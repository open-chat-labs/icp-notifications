use crate::NotificationTarget;
use candid::CandidType;
use ledger_canister::AccountIdentifier;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct TransactionNotification {
    pub from: AccountIdentifier,
    pub to: AccountIdentifier,
    pub amount_e8s: u64,
    pub targets: Vec<NotificationTarget>,
}
