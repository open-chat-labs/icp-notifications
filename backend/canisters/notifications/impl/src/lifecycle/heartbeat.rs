use crate::ledger::Ledger;
use crate::{RuntimeState, RUNTIME_STATE};
use ic_cdk_macros::heartbeat;
use ledger_canister::Transfer::{Mint, Send};
use ledger_canister::{BlockHeight, Transaction};
use std::sync::Arc;

const MAX_TRANSACTIONS_PER_BATCH: usize = 1000;

#[heartbeat]
async fn heartbeat() {
    ingest_transactions::run().await;
}

mod ingest_transactions {
    use super::*;

    pub async fn run() {
        let prepare_result = RUNTIME_STATE.with(|state| prepare(&mut state.borrow_mut()));

        if let PrepareResult::Ready(ledger, maybe_synced_up_to) = prepare_result {
            if let Some(synced_up_to) = maybe_synced_up_to {
                let transactions = get_transactions(ledger, synced_up_to + 1)
                    .await
                    .expect("Failed to get transactions");

                RUNTIME_STATE
                    .with(|state| process_transactions(transactions, &mut state.borrow_mut()));
            } else {
                // This will only happen on the first iteration. We set synced_up_to to be the current
                // tip_of_chain so that subsequent processing continues from there since we don't care
                // about transactions prior to this.
                let tip_of_chain = ledger
                    .tip_of_chain()
                    .await
                    .expect("Failed to call 'tip_of_chain'")
                    .tip_index;

                RUNTIME_STATE.with(|state| set_synced_up_to(tip_of_chain, &mut state.borrow_mut()));
            }
        }
    }

    enum PrepareResult {
        AlreadyInProgress,
        Ready(Arc<dyn Ledger>, Option<u64>),
    }

    fn prepare(runtime_state: &mut RuntimeState) -> PrepareResult {
        if runtime_state
            .data
            .transaction_import_state
            .try_start_import()
        {
            let ledger = runtime_state.ledger.clone();
            PrepareResult::Ready(
                ledger,
                runtime_state.data.transaction_import_state.synced_up_to(),
            )
        } else {
            PrepareResult::AlreadyInProgress
        }
    }

    fn set_synced_up_to(synced_up_to: u64, runtime_state: &mut RuntimeState) {
        runtime_state
            .data
            .transaction_import_state
            .set_synced_up_to(synced_up_to);

        runtime_state
            .data
            .transaction_import_state
            .mark_import_finished();
    }

    async fn get_transactions(
        ledger: Arc<dyn Ledger>,
        from: BlockHeight,
    ) -> Result<Vec<(BlockHeight, Transaction)>, String> {
        let blocks = ledger
            .get_blocks_since(from, MAX_TRANSACTIONS_PER_BATCH)
            .await?
            .0?;

        let results: Vec<_> = blocks
            .into_iter()
            .enumerate()
            .map(|(index, block)| (from + (index as u64), block.decode().unwrap().transaction))
            .collect();

        Ok(results)
    }

    fn process_transactions(
        transactions: Vec<(BlockHeight, Transaction)>,
        runtime_state: &mut RuntimeState,
    ) {
        for (block_height, transaction) in transactions.into_iter() {
            let recipient = match &transaction.transfer {
                Mint { to, .. } => to,
                Send { to, .. } => to,
                _ => continue,
            };

            let subscriptions = runtime_state.data.subscriptions.get_by_account(*recipient);
            if subscriptions.is_empty() {
                continue;
            }

            let targets = subscriptions
                .into_iter()
                .map(|s| s.targets)
                .flatten()
                .collect();

            runtime_state
                .data
                .notifications
                .add(block_height, transaction, targets);
        }

        runtime_state
            .data
            .transaction_import_state
            .mark_import_finished();
    }
}
