use candid::CandidType;
use ledger_canister::{BlockHeight, Transaction};
use serde::Deserialize;
use std::collections::VecDeque;
use types::{Notification, NotificationTarget};

#[derive(CandidType, Deserialize, Default)]
pub struct Notifications {
    notifications: VecDeque<Notification>,
    latest_notification_index: u64,
}

#[allow(dead_code)]
impl Notifications {
    pub fn add(
        &mut self,
        block_height: BlockHeight,
        transaction: Transaction,
        targets: Vec<NotificationTarget>,
    ) -> u64 {
        let index = self.latest_notification_index + 1;
        self.notifications.push_back(Notification {
            index,
            block_height,
            transaction,
            targets,
        });
        self.latest_notification_index = index;
        index
    }

    pub fn get_range(&self, _start: u64, _max_count: u32) -> Vec<Notification> {
        todo!()
    }

    pub fn remove(&mut self, _up_to: u64) {
        todo!()
    }
}
