use candid::CandidType;
use ledger_canister::AccountIdentifier;
use serde::Deserialize;
use types::NotificationTarget;

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
    pub account_identifier: AccountIdentifier,
    pub targets: Vec<NotificationTarget>,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum Response {
    Success,
}
