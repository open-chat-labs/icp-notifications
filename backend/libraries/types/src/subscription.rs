use crate::NotificationTarget;
use candid::CandidType;
use ledger_canister::AccountIdentifier;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Subscription {
    pub account_identifier: AccountIdentifier,
    pub targets: Vec<NotificationTarget>,
}
