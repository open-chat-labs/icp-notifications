use async_trait::async_trait;
use ledger_canister::{BlockHeight, Transaction};
use notification_sender::email_sender::EmailSender;
use std::borrow::BorrowMut;
use std::sync::Mutex;
use types::Error;

pub struct DummyEmailSender {
    emails_sent: Mutex<Vec<(String, BlockHeight, Transaction)>>,
}

impl DummyEmailSender {
    pub fn new() -> DummyEmailSender {
        DummyEmailSender {
            emails_sent: Mutex::new(Vec::new()),
        }
    }
}

#[async_trait]
impl EmailSender for DummyEmailSender {
    async fn send(
        &self,
        email_address: String,
        block_height: BlockHeight,
        transaction: Transaction,
    ) -> Result<(), Error> {
        match self.emails_sent.lock() {
            Ok(mut mutex) => {
                mutex
                    .borrow_mut()
                    .push((email_address, block_height, transaction));
                Ok(())
            }
            Err(error) => Err(error.to_string().into()),
        }
    }
}
