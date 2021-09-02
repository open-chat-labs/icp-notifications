use async_trait::async_trait;
use ledger_canister::{BlockHeight, Transaction};
use notification_sender::sms_sender::SmsSender;
use std::borrow::BorrowMut;
use std::sync::Mutex;
use types::Error;

pub struct DummySmsSender {
    sms_messages_sent: Mutex<Vec<(String, BlockHeight, Transaction)>>,
}

impl DummySmsSender {
    pub fn new() -> DummySmsSender {
        DummySmsSender {
            sms_messages_sent: Mutex::new(Vec::new()),
        }
    }
}

#[async_trait]
impl SmsSender for DummySmsSender {
    async fn send(
        &self,
        phone_number: String,
        block_height: BlockHeight,
        transaction: Transaction,
    ) -> Result<(), Error> {
        match self.sms_messages_sent.lock() {
            Ok(mut mutex) => {
                mutex
                    .borrow_mut()
                    .push((phone_number, block_height, transaction));
                Ok(())
            }
            Err(error) => Err(error.to_string().into()),
        }
    }
}
