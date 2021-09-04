use crate::email_sender::EmailSender;
use crate::index_store::IndexStore;
use crate::notification_reader::NotificationReader;
use crate::sms_sender::SmsSender;
use crate::transaction_details::TransactionDetails;
use futures::future;
use types::{Error, NotificationTarget};

pub async fn run(
    notification_reader: &dyn NotificationReader,
    index_store: &dyn IndexStore,
    sms_sender: &dyn SmsSender,
    email_sender: &dyn EmailSender,
) -> Result<(), Error> {
    let from_index = index_store
        .get_index_processed_up_to()
        .await?
        .map_or(0, |i| i + 1);

    let notifications_response = notification_reader.get(from_index).await?;

    if let Some(latest_index) = notifications_response.notifications.last().map(|e| e.index) {
        let futures: Vec<_> = notifications_response
            .notifications
            .into_iter()
            .flat_map(|n| {
                let transaction_details: TransactionDetails =
                    (n.block_height, n.transaction).into();
                n.targets.into_iter().map(move |t| match t {
                    NotificationTarget::Sms(phone_number) => {
                        sms_sender.send(phone_number, transaction_details.clone())
                    }
                    NotificationTarget::Email(email_address) => {
                        email_sender.send(email_address, transaction_details.clone())
                    }
                })
            })
            .collect();

        future::join_all(futures).await;

        index_store.set_index_processed_up_to(latest_index).await?;
    }

    Ok(())
}
