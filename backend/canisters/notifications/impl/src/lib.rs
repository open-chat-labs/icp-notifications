use crate::env::{EmptyEnvironment, Environment};
use crate::ledger::{EmptyLedger, Ledger};
use crate::model::notifications::Notifications;
use crate::model::subscriptions::Subscriptions;
use crate::model::transaction_import_state::TransactionImportState;
use candid::{CandidType, Principal};
use serde::Deserialize;
use std::cell::RefCell;
use std::collections::HashSet;
use std::sync::Arc;

mod env;
mod ledger;
mod lifecycle;
mod model;
mod queries;
mod updates;

thread_local! {
    pub static RUNTIME_STATE: RefCell<RuntimeState> = RefCell::default();
}

pub struct RuntimeState {
    pub env: Box<dyn Environment>,
    pub data: Data,
    pub ledger: Arc<dyn Ledger>,
}

impl RuntimeState {
    pub fn new(env: Box<dyn Environment>, data: Data, ledger: Arc<dyn Ledger>) -> RuntimeState {
        RuntimeState { env, data, ledger }
    }
}

impl Default for RuntimeState {
    fn default() -> Self {
        RuntimeState {
            env: Box::new(EmptyEnvironment {}),
            data: Data::default(),
            ledger: Arc::new(EmptyLedger {}),
        }
    }
}

#[derive(CandidType, Deserialize, Default)]
pub struct Data {
    subscriptions: Subscriptions,
    notifications: Notifications,
    transaction_import_state: TransactionImportState,
    push_service_principals: HashSet<Principal>,
}
