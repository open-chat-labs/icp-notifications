use crate::env::CanisterEnvironment;
use crate::ledger::LedgerCanister;
use crate::lifecycle::StableMemoryVersion;
use crate::{Data, RuntimeState, RUNTIME_STATE};
use ic_cdk_macros::post_upgrade;
use std::sync::Arc;

#[post_upgrade]
fn post_upgrade() {
    ic_cdk::setup();

    let (stable_bytes,): (Vec<u8>,) =
        ic_cdk::storage::stable_restore().expect("Failed to read from stable memory");

    let env = CanisterEnvironment {};
    let data = deserialize_state(stable_bytes);
    let ledger = LedgerCanister {};
    let runtime_state = RuntimeState::new(Box::new(env), data, Arc::new(ledger));

    RUNTIME_STATE.with(|state| *state.borrow_mut() = runtime_state);
}

fn deserialize_state(stable_bytes: Vec<u8>) -> Data {
    let (version_bytes, state_bytes): (Vec<u8>, Vec<u8>) =
        candid::decode_args(&stable_bytes).expect("Failed to deserialize stable memory");

    let version: StableMemoryVersion =
        candid::decode_one(&version_bytes).expect("Failed to deserialize version");

    match version {
        StableMemoryVersion::V1 => {
            candid::decode_one(&state_bytes).expect("Failed to deserialize state")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lifecycle::pre_upgrade::serialize_state;
    use candid::Principal;
    use ic_base_types::PrincipalId;
    use ledger_canister::AccountIdentifier;
    use types::NotificationTarget;

    #[test]
    fn from_v1() {
        let mut data = Data::default();
        let principal = Principal::from_slice(&[1]);
        let account_identifier = AccountIdentifier::from(PrincipalId(principal));
        let targets = vec![NotificationTarget::Email("blah@blah.com".to_owned())];

        data.subscriptions
            .add(principal, account_identifier, targets.clone());

        // TODO populate 'data' with more data

        // Alternatively we could extract the stable memory from a running version of the app
        let bytes = serialize_state(StableMemoryVersion::V1, &data);

        let deserialized = deserialize_state(bytes);

        let subscription = deserialized
            .subscriptions
            .get_by_principal(principal, account_identifier)
            .unwrap();

        assert_eq!(subscription.principal, principal);
        assert_eq!(subscription.account_identifier, account_identifier);
        assert_eq!(subscription.targets, targets);
    }
}
