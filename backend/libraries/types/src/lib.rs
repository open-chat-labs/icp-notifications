use candid::Principal;

mod notification_target;
mod subscription;
mod transaction_notification;

pub use notification_target::*;
pub use subscription::*;
pub use transaction_notification::*;

pub type CanisterId = Principal;
pub type TimestampMillis = u64;
