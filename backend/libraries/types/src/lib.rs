use candid::Principal;

mod error;
mod notification;
mod notification_target;
mod subscription;

pub use error::*;
pub use notification::*;
pub use notification_target::*;
pub use subscription::*;

pub type CanisterId = Principal;
pub type TimestampMillis = u64;
