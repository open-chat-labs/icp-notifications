use candid::CandidType;
use ledger_canister::AccountIdentifier;
use serde::Deserialize;
use types::Subscription;

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
    pub account_identifier: AccountIdentifier,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum Response {
    Success(SuccessResult),
    NotFound,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct SuccessResult {
    pub subscription: Subscription,
}
