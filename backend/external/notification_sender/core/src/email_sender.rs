use crate::transaction_details::TransactionDetails;
use async_trait::async_trait;
use types::Error;

#[async_trait]
pub trait EmailSender: Send + Sync {
    async fn send(
        &self,
        email_address: String,
        transaction_details: TransactionDetails,
    ) -> Result<(), Error>;
}
