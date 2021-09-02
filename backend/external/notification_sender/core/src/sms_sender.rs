use async_trait::async_trait;
use ledger_canister::{BlockHeight, Transaction};
use types::Error;

#[async_trait]
pub trait SmsSender {
    async fn send(
        &self,
        phone_number: String,
        block_height: BlockHeight,
        transaction: Transaction,
    ) -> Result<(), Error>;
}
