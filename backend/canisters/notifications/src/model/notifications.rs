use std::collections::VecDeque;
use types::Notification;

#[derive(Default)]
pub struct Notifications {
    notifications: VecDeque<Notification>,
}

impl Notifications {
    pub fn add(&mut self, notification: Notification) {
        self.notifications.push_back(notification);
    }
}
