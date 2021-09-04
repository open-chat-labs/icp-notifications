use crate::{RuntimeState, RUNTIME_STATE};
use ic_cdk_macros::update;
use notifications_canister_api::subscribe::{Response::*, *};

#[update]
fn subscribe(args: Args) -> Response {
    RUNTIME_STATE.with(|state| subscribe_impl(args, &mut state.borrow_mut()))
}

fn subscribe_impl(args: Args, runtime_state: &mut RuntimeState) -> Response {
    let caller = runtime_state.env.caller();
    for account_identifier in args.account_identifiers.into_iter() {
        runtime_state
            .data
            .subscriptions
            .add(caller, account_identifier, args.targets.clone());
    }
    Success
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::env::TestEnvironment;
    use crate::ledger::LedgerCanister;
    use crate::Data;
    use candid::Principal;
    use ledger_canister::AccountIdentifier;
    use std::sync::Arc;
    use types::NotificationTarget;

    const PRINCIPAL1: Principal = Principal::from_slice(&[1]);

    #[test]
    fn subscribe() {
        let account_identifier = AccountIdentifier::new(PRINCIPAL1.into(), None);
        let targets = vec![NotificationTarget::Email("1@1.com".to_string())];

        let args = Args {
            account_identifiers: vec![account_identifier],
            targets: targets.clone(),
        };
        let mut runtime_state = build_runtime_state();

        let result = super::subscribe_impl(args, &mut runtime_state);
        assert!(matches!(result, Success));

        if let Some(subscription) = runtime_state
            .data
            .subscriptions
            .get_by_principal(runtime_state.env.caller(), account_identifier)
        {
            assert_eq!(subscription.principal, PRINCIPAL1);
            assert_eq!(subscription.account_identifier, account_identifier);
            assert_eq!(subscription.targets, targets);
        } else {
            panic!("Subscription not added");
        }
    }

    fn build_runtime_state() -> RuntimeState {
        let test_env = TestEnvironment {
            now: 100,
            caller: PRINCIPAL1,
        };
        let ledger = LedgerCanister {};
        RuntimeState::new(Box::new(test_env), Data::default(), Arc::new(ledger))
    }
}
