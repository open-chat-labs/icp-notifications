use crate::env::Environment;
use crate::ledger::Ledger;
use crate::model::subscriptions::Subscriptions;
use std::cell::RefCell;

mod env;
mod ledger;
mod lifecycle;
mod model;
mod queries;
mod updates;

thread_local! {
    pub static RUNTIME_STATE: RefCell<Option<RuntimeState>> = RefCell::default();
}

pub struct RuntimeState {
    pub env: Box<dyn Environment>,
    pub data: Data,
    pub ledger: Box<dyn Ledger>,
}

impl RuntimeState {
    pub fn new(env: Box<dyn Environment>, data: Data, ledger: Box<dyn Ledger>) -> RuntimeState {
        RuntimeState { env, data, ledger }
    }
}

#[derive(Default)]
pub struct Data {
    subscriptions: Subscriptions,
}
