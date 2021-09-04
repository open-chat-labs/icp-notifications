use crate::{RuntimeState, RUNTIME_STATE};
use ic_cdk_macros::query;
use notifications_canister_api::notifications::{Response::*, *};

#[query]
fn notifications(args: Args) -> Response {
    RUNTIME_STATE.with(|state| notifications_impl(args, &state.borrow()))
}

fn notifications_impl(args: Args, runtime_state: &RuntimeState) -> Response {
    let caller = runtime_state.env.caller();
    if runtime_state.data.push_service_principals.contains(&caller) {
        let notifications = runtime_state
            .data
            .notifications
            .get_range(args.from_index, args.max_results);

        Success(SuccessResult { notifications })
    } else {
        NotAuthorized
    }
}
