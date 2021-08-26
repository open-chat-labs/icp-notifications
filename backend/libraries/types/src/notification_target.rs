use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
pub enum NotificationTarget {
    Email(String),
    Sms(String),
}
