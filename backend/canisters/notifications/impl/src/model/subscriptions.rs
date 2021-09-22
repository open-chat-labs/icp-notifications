use candid::{CandidType, Principal};
use ledger_canister::AccountIdentifier;
use serde::Deserialize;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{HashMap, HashSet};
use types::{NotificationTarget, Subscription};

#[derive(CandidType, Deserialize, Default)]
pub struct Subscriptions {
    subscriptions: HashMap<AccountIdentifier, HashMap<Principal, HashSet<NotificationTarget>>>,
    by_principal: HashMap<Principal, HashSet<AccountIdentifier>>,
}

impl Subscriptions {
    pub fn get_all_by_principal(&self, principal: Principal) -> Vec<Subscription> {
        if let Some(account_identifiers) = self.by_principal.get(&principal) {
            account_identifiers
                .iter()
                .filter_map(|a| self.get_by_principal(principal, *a))
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn get_by_principal(
        &self,
        principal: Principal,
        account_identifier: AccountIdentifier,
    ) -> Option<Subscription> {
        if let Some(map) = self.subscriptions.get(&account_identifier) {
            if let Some(targets) = map.get(&principal) {
                let subscription = Subscription {
                    account_identifier,
                    principal,
                    targets: targets.iter().cloned().collect(),
                };
                return Some(subscription);
            }
        }
        None
    }

    pub fn get_by_account(&self, account_identifier: AccountIdentifier) -> Vec<Subscription> {
        if let Some(map) = self.subscriptions.get(&account_identifier) {
            map.iter()
                .map(|(principal, targets)| Subscription {
                    account_identifier,
                    principal: *principal,
                    targets: targets.iter().cloned().collect(),
                })
                .collect()
        } else {
            Vec::new()
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use ledger_canister::Subaccount;

    const PRINCIPAL1: Principal = Principal::from_slice(&[1]);
    const PRINCIPAL2: Principal = Principal::from_slice(&[2]);

    #[test]
    fn add_then_get_by_principal() {
        let mut subscriptions = Subscriptions::default();

        let account_identifier1 = AccountIdentifier::new(PRINCIPAL1.into(), None);
        let account_identifier2 =
            AccountIdentifier::new(PRINCIPAL1.into(), Some(build_sub_account(1)));

        let target1 = NotificationTarget::Email("1@1.com".to_string());
        let target2 = NotificationTarget::Email("2@2.com".to_string());
        let target3 = NotificationTarget::Email("3@3.com".to_string());

        subscriptions.add(PRINCIPAL1, account_identifier1, vec![target1]);
        subscriptions.add(PRINCIPAL1, account_identifier2, vec![target2.clone()]);
        subscriptions.add(
            PRINCIPAL2,
            AccountIdentifier::new(PRINCIPAL2.into(), None),
            vec![target3],
        );

        if let Some(subscription) = subscriptions.get_by_principal(PRINCIPAL1, account_identifier2)
        {
            assert_eq!(subscription.principal, PRINCIPAL1);
            assert_eq!(subscription.account_identifier, account_identifier2);
            assert_eq!(subscription.targets, vec![target2]);
        } else {
            panic!("Subscription was not returned");
        }
    }

    #[test]
    fn add_then_get_by_account() {
        let mut subscriptions = Subscriptions::default();

        let account_identifier1 = AccountIdentifier::new(PRINCIPAL1.into(), None);
        let account_identifier2 = AccountIdentifier::new(PRINCIPAL2.into(), None);

        let target1 = NotificationTarget::Email("1@1.com".to_string());
        let target2 = NotificationTarget::Email("2@2.com".to_string());
        let target3 = NotificationTarget::Email("3@3.com".to_string());

        subscriptions.add(PRINCIPAL1, account_identifier1, vec![target1]);
        subscriptions.add(PRINCIPAL1, account_identifier2, vec![target2.clone()]);
        subscriptions.add(PRINCIPAL2, account_identifier2, vec![target3.clone()]);

        let matches = subscriptions.get_by_account(account_identifier2);
        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0].principal, PRINCIPAL1);
        assert_eq!(matches[0].account_identifier, account_identifier2);
        assert_eq!(matches[0].targets, vec![target2]);
        assert_eq!(matches[1].principal, PRINCIPAL2);
        assert_eq!(matches[1].account_identifier, account_identifier2);
        assert_eq!(matches[1].targets, vec![target3]);
    }

    fn build_sub_account(index: u8) -> Subaccount {
        let mut sub_account: [u8; 32] = [0; 32];
        sub_account[31] = index;
        Subaccount(sub_account)
    }
}
