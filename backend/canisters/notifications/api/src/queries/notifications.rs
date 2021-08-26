use candid::CandidType;
use serde::Deserialize;
use types::Notification;

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
    pub from_index: u64,
    pub max_results: u32,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum Response {
    Success(SuccessResult),
    NotAuthorized,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct SuccessResult {
    pub notifications: Vec<Notification>,
}
