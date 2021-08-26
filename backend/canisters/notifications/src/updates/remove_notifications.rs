use crate::{RuntimeState, RUNTIME_STATE};
use candid::CandidType;
use ic_cdk_macros::query;
use serde::Deserialize;

#[derive(CandidType, Deserialize, Debug)]
struct Args {
    up_to_index: u64,
}

#[derive(CandidType, Deserialize, Debug)]
enum Response {
    Success,
    NotAuthorized,
}

#[query]
fn remove_notifications(args: Args) -> Response {
    RUNTIME_STATE
        .with(|state| remove_notifications_impl(args, state.borrow_mut().as_mut().unwrap()))
}

fn remove_notifications_impl(args: Args, runtime_state: &mut RuntimeState) -> Response {
    let caller = runtime_state.env.caller();
    if runtime_state.data.push_service_principals.contains(&caller) {
        runtime_state.data.notifications.remove(args.up_to_index);

        Response::Success
    } else {
        Response::NotAuthorized
    }
}
