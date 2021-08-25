use candid::Principal;
use ledger_canister::AccountIdentifier;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{HashMap, HashSet};
use types::{NotificationTarget, Subscription};

#[derive(Default)]
pub struct Subscriptions {
    subscriptions: HashMap<AccountIdentifier, HashMap<Principal, HashSet<NotificationTarget>>>,
    by_principal: HashMap<Principal, HashSet<AccountIdentifier>>,
}

impl Subscriptions {
    pub fn get_all(&self, principal: Principal) -> Vec<Subscription> {
        if let Some(account_identifiers) = self.by_principal.get(&principal) {
            account_identifiers
                .iter()
                .filter_map(|a| self.get(principal, *a))
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn get(
        &self,
        principal: Principal,
        account_identifier: AccountIdentifier,
    ) -> Option<Subscription> {
        if let Some(map) = self.subscriptions.get(&account_identifier) {
            if let Some(targets) = map.get(&principal) {
                let subscription = Subscription {
                    account_identifier,
                    targets: targets.iter().cloned().collect(),
                };
                return Some(subscription);
            }
        }
        None
    }

    pub fn add(
        &mut self,
        principal: Principal,
        account_identifier: AccountIdentifier,
        targets: Vec<NotificationTarget>,
    ) {
        match self.subscriptions.entry(account_identifier) {
            Occupied(e) => match e.into_mut().entry(principal) {
                Occupied(e) => {
                    let set = e.into_mut();
                    for t in targets.into_iter() {
                        set.insert(t);
                    }
                }
                Vacant(e) => {
                    e.insert(targets.into_iter().collect());
                }
            },
            Vacant(e) => {
                e.insert(
                    vec![(principal, targets.into_iter().collect())]
                        .into_iter()
                        .collect(),
                );
            }
        };

        match self.by_principal.entry(principal) {
            Occupied(e) => {
                e.into_mut().insert(account_identifier);
            }
            Vacant(e) => {
                e.insert(vec![account_identifier].into_iter().collect());
            }
        };
    }

    pub fn remove(
        &mut self,
        principal: Principal,
        account_identifier: AccountIdentifier,
        targets: Vec<NotificationTarget>,
    ) {
        if let Occupied(mut e) = self.subscriptions.entry(account_identifier) {
            let map = e.get_mut();
            match map.entry(principal) {
                Occupied(mut e) => {
                    let set = e.get_mut();
                    for t in targets.iter() {
                        set.remove(t);
                    }
                    if set.is_empty() {
                        e.remove();
                    }
                }
                Vacant(e) => {
                    e.insert(targets.into_iter().collect());
                }
            };
            if map.is_empty() {
                e.remove();
            }
        }

        if let Occupied(mut e) = self.by_principal.entry(principal) {
            let set = e.get_mut();
            set.remove(&account_identifier);
            if set.is_empty() {
                e.remove();
            }
        }
    }
}
