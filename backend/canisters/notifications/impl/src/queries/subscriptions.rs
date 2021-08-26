use crate::{RuntimeState, RUNTIME_STATE};
use ic_cdk_macros::query;
use notifications_canister_api::subscriptions::{Response::*, *};

#[query]
fn subscriptions(_: Args) -> Response {
    RUNTIME_STATE.with(|state| subscriptions_impl(state.borrow().as_ref().unwrap()))
}

fn subscriptions_impl(runtime_state: &RuntimeState) -> Response {
    let caller = runtime_state.env.caller();
    let subscriptions = runtime_state
        .data
        .subscriptions
        .get_all_by_principal(caller);

    Success(SuccessResult { subscriptions })
}
