use candid::CandidType;
use serde::Deserialize;

mod heartbeat;
mod init;
mod inspect_message;
mod post_upgrade;
mod pre_upgrade;

#[derive(CandidType, Deserialize, Copy, Clone, Debug)]
pub(crate) enum StableMemoryVersion {
    V1,
}
