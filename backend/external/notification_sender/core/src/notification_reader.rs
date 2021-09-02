use async_trait::async_trait;
use notifications_canister_api::notifications;
use types::Error;

#[async_trait]
pub trait NotificationReader: Send + Sync {
    async fn get(&self, from_index: u64) -> Result<notifications::SuccessResult, Error>;
    async fn remove(&self, up_to_index: u64) -> Result<(), Error>;
}
