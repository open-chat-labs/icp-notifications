use crate::{RuntimeState, RUNTIME_STATE};
use candid::CandidType;
use ic_cdk_macros::query;
use serde::Deserialize;
use types::Subscription;

#[derive(CandidType, Deserialize, Debug)]
struct Args {}

#[derive(CandidType, Deserialize, Debug)]
enum Response {
    Success(SuccessResult),
}

#[derive(CandidType, Deserialize, Debug)]
struct SuccessResult {
    subscriptions: Vec<Subscription>,
}

#[query]
fn subscriptions(_: Args) -> Response {
    RUNTIME_STATE.with(|state| subscriptions_impl(state.borrow().as_ref().unwrap()))
}

fn subscriptions_impl(runtime_state: &RuntimeState) -> Response {
    let caller = runtime_state.env.caller();
    let subscriptions = runtime_state.data.subscriptions.get_all(caller);
    Response::Success(SuccessResult { subscriptions })
}
