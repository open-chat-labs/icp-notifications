use async_trait::async_trait;
use aws_sdk_ses::model::{Body, Content, Destination, Message};
use aws_sdk_ses::Client;
use log::info;
use notification_sender::email_sender::EmailSender;
use notification_sender::transaction_details::TransactionDetails;
use types::Error;

pub struct SesClient {
    client: Client,
}

impl SesClient {
    pub fn build() -> Result<SesClient, Error> {
        let config = aws_sdk_ses::Config::builder().build();

        let client = Client::from_conf(config);

        info!("SesClient created");

        Ok(SesClient { client })
    }
}

#[async_trait]
impl EmailSender for SesClient {
    async fn send(
        &self,
        email_address: String,
        transaction_details: TransactionDetails,
    ) -> Result<(), Error> {
        let destination = Destination::builder()
            .set_to_addresses(Some(vec![email_address]))
            .build();

        let title_content = Content::builder()
            .data(format!(
                "ICP transaction notification ({})",
                transaction_details.transaction_index
            ))
            .build();

        let body_content = Content::builder()
            .data(format!(
                "TransactionIndex: {}\nFrom: {}\nTo: {}\nAmount: {}",
                transaction_details.transaction_index,
                transaction_details.from,
                transaction_details.to,
                transaction_details.amount,
            ))
            .build();

        let message = Message::builder()
            .subject(title_content)
            .body(Body::builder().text(body_content).build())
            .build();

        self.client
            .send_email()
            .source("notifications@icpnotifier.com")
            .destination(destination)
            .message(message)
            .send()
            .await?;

        Ok(())
    }
}
