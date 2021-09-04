use crate::{RuntimeState, RUNTIME_STATE};
use ic_cdk_macros::query;
use notifications_canister_api::remove_notifications::{Response::*, *};

#[query]
fn remove_notifications(args: Args) -> Response {
    RUNTIME_STATE.with(|state| remove_notifications_impl(args, &mut state.borrow_mut()))
}

fn remove_notifications_impl(args: Args, runtime_state: &mut RuntimeState) -> Response {
    let caller = runtime_state.env.caller();
    if runtime_state.data.push_service_principals.contains(&caller) {
        runtime_state.data.notifications.remove(args.up_to_index);

        Success
    } else {
        NotAuthorized
    }
}
