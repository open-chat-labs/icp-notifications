use crate::env::CanisterEnvironment;
use crate::ledger::LedgerCanister;
use crate::{Data, RuntimeState, RUNTIME_STATE};
use ic_cdk_macros::init;
use std::sync::Arc;

#[init]
fn init() {
    ic_cdk::setup();

    let env = CanisterEnvironment {};
    let data = Data::default();
    let ledger = LedgerCanister {};
    let runtime_state = RuntimeState::new(Box::new(env), data, Arc::new(ledger));

    RUNTIME_STATE.with(|state| *state.borrow_mut() = runtime_state);
}
