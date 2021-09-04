use crate::{RuntimeState, RUNTIME_STATE};
use ic_cdk_macros::query;
use notifications_canister_api::subscription::{Response::*, *};

#[query]
fn subscription(args: Args) -> Response {
    RUNTIME_STATE.with(|state| subscription_impl(args, &state.borrow()))
}

fn subscription_impl(args: Args, runtime_state: &RuntimeState) -> Response {
    let caller = runtime_state.env.caller();
    if let Some(subscription) = runtime_state
        .data
        .subscriptions
        .get_by_principal(caller, args.account_identifier)
    {
        Success(SuccessResult { subscription })
    } else {
        NotFound
    }
}
