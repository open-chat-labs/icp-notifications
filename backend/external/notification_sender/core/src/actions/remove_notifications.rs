use crate::index_store::IndexStore;
use crate::notification_reader::NotificationReader;
use types::Error;

pub async fn run(
    notification_reader: &dyn NotificationReader,
    index_store: &dyn IndexStore,
) -> Result<(), Error> {
    let maybe_index_processed_up_to = index_store.get_index_processed_up_to().await?;

    if let Some(index_processed_up_to) = maybe_index_processed_up_to {
        notification_reader.remove(index_processed_up_to).await?;
    }

    Ok(())
}
