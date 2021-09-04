use crate::{RuntimeState, RUNTIME_STATE};
use ic_cdk_macros::update;
use notifications_canister_api::unsubscribe::{Response::*, *};

#[update]
fn unsubscribe(args: Args) -> Response {
    RUNTIME_STATE.with(|state| unsubscribe_impl(args, &mut state.borrow_mut()))
}

fn unsubscribe_impl(args: Args, runtime_state: &mut RuntimeState) -> Response {
    let caller = runtime_state.env.caller();
    runtime_state
        .data
        .subscriptions
        .remove(caller, args.account_identifier, args.targets);

    Success
}
