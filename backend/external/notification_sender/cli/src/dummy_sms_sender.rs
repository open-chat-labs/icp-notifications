use async_trait::async_trait;
use notification_sender::sms_sender::SmsSender;
use notification_sender::transaction_details::TransactionDetails;
use std::borrow::BorrowMut;
use std::sync::Mutex;
use types::Error;

pub struct DummySmsSender {
    sms_messages_sent: Mutex<Vec<(String, TransactionDetails)>>,
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
        transaction_details: TransactionDetails,
    ) -> Result<(), Error> {
        match self.sms_messages_sent.lock() {
            Ok(mut mutex) => {
                mutex.borrow_mut().push((phone_number, transaction_details));
                Ok(())
            }
            Err(error) => Err(error.to_string().into()),
        }
    }
}
