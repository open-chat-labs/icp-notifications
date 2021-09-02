use async_trait::async_trait;
use aws_sdk_sns::Client;
use ledger_canister::{BlockHeight, Transaction};
use log::info;
use notification_sender::email_sender::EmailSender;
use notification_sender::sms_sender::SmsSender;
use types::Error;

pub struct SnsClient {
    client: Client,
}

impl SnsClient {
    pub fn build() -> SnsClient {
        let config = aws_sdk_sns::Config::builder().build();

        let client = Client::from_conf(config);

        info!("SnsClient created");

        SnsClient { client }
    }
}

#[async_trait]
impl SmsSender for SnsClient {
    async fn send(
        &self,
        _phone_number: String,
        _block_height: BlockHeight,
        _transaction: Transaction,
    ) -> Result<(), Error> {
        todo!()
    }
}

#[async_trait]
impl EmailSender for SnsClient {
    async fn send(
        &self,
        _email_address: String,
        _block_height: BlockHeight,
        _transaction: Transaction,
    ) -> Result<(), Error> {
        todo!()
    }
}
