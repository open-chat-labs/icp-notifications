use crate::transaction_details::TransactionDetails;
use async_trait::async_trait;
use types::Error;

#[async_trait]
pub trait SmsSender: Send + Sync {
    async fn send(
        &self,
        phone_number: String,
        transaction_details: TransactionDetails,
    ) -> Result<(), Error>;
}
