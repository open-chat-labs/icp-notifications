use crate::{RuntimeState, RUNTIME_STATE};
use candid::CandidType;
use ic_cdk_macros::query;
use ledger_canister::AccountIdentifier;
use serde::Deserialize;
use types::Subscription;

#[derive(CandidType, Deserialize, Debug)]
struct Args {
    account_identifier: AccountIdentifier,
}

#[derive(CandidType, Deserialize, Debug)]
enum Response {
    Success(SuccessResult),
    NotFound,
}

#[derive(CandidType, Deserialize, Debug)]
struct SuccessResult {
    subscription: Subscription,
}

#[query]
fn subscription(args: Args) -> Response {
    RUNTIME_STATE.with(|state| subscription_impl(args, state.borrow().as_ref().unwrap()))
}

fn subscription_impl(args: Args, runtime_state: &RuntimeState) -> Response {
    let caller = runtime_state.env.caller();
    if let Some(subscription) = runtime_state
        .data
        .subscriptions
        .get(caller, args.account_identifier)
    {
        Response::Success(SuccessResult { subscription })
    } else {
        Response::NotFound
    }
}
