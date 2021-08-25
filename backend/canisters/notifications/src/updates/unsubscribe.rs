use crate::{RuntimeState, RUNTIME_STATE};
use candid::CandidType;
use ic_cdk_macros::update;
use ledger_canister::AccountIdentifier;
use serde::Deserialize;
use types::NotificationTarget;

#[derive(CandidType, Deserialize, Debug)]
struct Args {
    account_identifier: AccountIdentifier,
    targets: Vec<NotificationTarget>,
}

#[derive(CandidType, Deserialize, Debug)]
enum Response {
    Success,
}

#[update]
fn unsubscribe(args: Args) -> Response {
    RUNTIME_STATE.with(|state| unsubscribe_impl(args, state.borrow_mut().as_mut().unwrap()))
}

fn unsubscribe_impl(args: Args, runtime_state: &mut RuntimeState) -> Response {
    let caller = runtime_state.env.caller();
    runtime_state
        .data
        .subscriptions
        .remove(caller, args.account_identifier, args.targets);
    Response::Success
}
