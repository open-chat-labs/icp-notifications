use async_trait::async_trait;
use ledger_canister::{BlockHeight, Transaction};
use types::Error;

#[async_trait]
pub trait EmailSender {
    async fn send(
        &self,
        email_address: String,
        block_height: BlockHeight,
        transaction: Transaction,
    ) -> Result<(), Error>;
}
