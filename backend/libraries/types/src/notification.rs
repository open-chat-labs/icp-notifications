use crate::NotificationTarget;
use candid::CandidType;
use ledger_canister::{BlockHeight, Transaction};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Notification {
    pub block_height: BlockHeight,
    pub transaction: Transaction,
    pub targets: Vec<NotificationTarget>,
}
