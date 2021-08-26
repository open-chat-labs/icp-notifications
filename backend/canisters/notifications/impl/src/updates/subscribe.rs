use crate::{RuntimeState, RUNTIME_STATE};
use ic_cdk_macros::update;
use notifications_canister_api::subscribe::{Response::*, *};

#[update]
fn subscribe(args: Args) -> Response {
    RUNTIME_STATE.with(|state| subscribe_impl(args, state.borrow_mut().as_mut().unwrap()))
}

fn subscribe_impl(args: Args, runtime_state: &mut RuntimeState) -> Response {
    let caller = runtime_state.env.caller();
    for account_identifier in args.account_identifiers.into_iter() {
        runtime_state
            .data
            .subscriptions
            .add(caller, account_identifier, args.targets.clone());
    }
    Success
}
