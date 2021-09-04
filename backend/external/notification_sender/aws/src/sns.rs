use async_trait::async_trait;
use aws_sdk_sns::Client;
use log::info;
use notification_sender::sms_sender::SmsSender;
use notification_sender::transaction_details::TransactionDetails;
use types::Error;

pub struct SnsClient {
    client: Client,
    sns_topic_arn: String,
}

impl SnsClient {
    pub fn build() -> Result<SnsClient, Error> {
        let config = aws_sdk_sns::Config::builder().build();

        let client = Client::from_conf(config);

        let sns_topic_arn = dotenv::var("SNS_TOPIC_ARN")?;

        info!("SnsClient created");

        Ok(SnsClient {
            client,
            sns_topic_arn,
        })
    }
}

#[async_trait]
impl SmsSender for SnsClient {
    async fn send(
        &self,
        phone_number: String,
        transaction_details: TransactionDetails,
    ) -> Result<(), Error> {
        self.client
            .publish()
            .set_topic_arn(Some(self.sns_topic_arn.clone()))
            .phone_number(phone_number)
            .subject("ICP transaction notifications")
            .message(format!(
                "TransactionIndex: {}\nFrom: {}\nTo: {}\nAmount: {}",
                transaction_details.transaction_index,
                transaction_details.from,
                transaction_details.to,
                transaction_details.amount,
            ))
            .send()
            .await?;

        Ok(())
    }
}
