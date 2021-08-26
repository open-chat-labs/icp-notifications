use crate::{RuntimeState, RUNTIME_STATE};
use candid::CandidType;
use ic_cdk_macros::query;
use serde::Deserialize;
use types::Notification;

#[derive(CandidType, Deserialize, Debug)]
struct Args {
    from_index: u64,
    max_results: u32,
}

#[derive(CandidType, Deserialize, Debug)]
enum Response {
    Success(SuccessResult),
    NotAuthorized,
}

#[derive(CandidType, Deserialize, Debug)]
struct SuccessResult {
    notifications: Vec<Notification>,
}

#[query]
fn notifications(args: Args) -> Response {
    RUNTIME_STATE.with(|state| notifications_impl(args, state.borrow().as_ref().unwrap()))
}

fn notifications_impl(args: Args, runtime_state: &RuntimeState) -> Response {
    let caller = runtime_state.env.caller();
    if runtime_state.data.push_service_principals.contains(&caller) {
        let notifications = runtime_state
            .data
            .notifications
            .get_range(args.from_index, args.max_results);

        Response::Success(SuccessResult { notifications })
    } else {
        Response::NotAuthorized
    }
}
