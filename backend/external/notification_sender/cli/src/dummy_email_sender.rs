use async_trait::async_trait;
use notification_sender::email_sender::EmailSender;
use notification_sender::transaction_details::TransactionDetails;
use std::borrow::BorrowMut;
use std::sync::Mutex;
use types::Error;

pub struct DummyEmailSender {
    emails_sent: Mutex<Vec<(String, TransactionDetails)>>,
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
        transaction_details: TransactionDetails,
    ) -> Result<(), Error> {
        match self.emails_sent.lock() {
            Ok(mut mutex) => {
                mutex
                    .borrow_mut()
                    .push((email_address, transaction_details));
                Ok(())
            }
            Err(error) => Err(error.to_string().into()),
        }
    }
}
