use crate::lifecycle::StableMemoryVersion;
use crate::{Data, RUNTIME_STATE};
use ic_cdk_macros::pre_upgrade;

#[pre_upgrade]
fn pre_upgrade() {
    let stable_bytes =
        RUNTIME_STATE.with(|state| serialize_state(StableMemoryVersion::V1, &state.borrow().data));

    ic_cdk::storage::stable_save((stable_bytes,)).expect("Failed saving bytes to stable memory");
}

pub(crate) fn serialize_state(version: StableMemoryVersion, data: &Data) -> Vec<u8> {
    let version_bytes = candid::encode_one(version).expect("Failed to serialize version");
    let state_bytes = candid::encode_one(data).expect("Failed to serialize state");

    candid::encode_args((version_bytes, state_bytes)).unwrap()
}
