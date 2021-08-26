use crate::ledger::Ledger;
use crate::{RuntimeState, RUNTIME_STATE};
use ic_base_types::CanisterId;
use ic_cdk_macros::heartbeat;
use ic_nns_constants::LEDGER_CANISTER_ID;
use ledger_canister::protobuf::ArchiveIndexEntry;
use ledger_canister::Transfer::{Mint, Send};
use ledger_canister::{BlockHeight, Transaction};
use std::cmp::{max, min};
use std::ops::RangeInclusive;
use std::sync::Arc;

const MAX_TRANSACTIONS_PER_BATCH: usize = 1000;

#[heartbeat]
async fn heartbeat() {
    if let PrepareResult::Ready(ledger, maybe_synced_up_to) =
        RUNTIME_STATE.with(|state| prepare(state.borrow_mut().as_mut().unwrap()))
    {
        let tip_of_chain = ledger
            .tip_of_chain()
            .await
            .expect("Failed to call 'tip_of_chain'")
            .tip_index;
        if let Some(synced_up_to) = maybe_synced_up_to {
            if tip_of_chain > synced_up_to {
                let transactions = get_transactions(ledger, synced_up_to + 1, tip_of_chain)
                    .await
                    .expect("Failed to get transactions");
                RUNTIME_STATE.with(|state| {
                    process_transactions(transactions, state.borrow_mut().as_mut().unwrap())
                });
            }
        } else {
            // This will only happen on the first iteration. We set synced_up_to to be the current
            // tip_of_chain so that subsequent processing continues from there since we don't care
            // about transactions prior to this.
            RUNTIME_STATE
                .with(|state| set_synced_up_to(tip_of_chain, state.borrow_mut().as_mut().unwrap()));
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
    tip_of_chain: BlockHeight,
) -> Result<Vec<(BlockHeight, Transaction)>, String> {
    let archive_index_entries = ledger.archive_index().await?.entries;

    let (canister_id, range) =
        determine_canister_for_blocks(from, tip_of_chain, archive_index_entries);

    let count = min(
        (range.end() - range.start() + 1) as usize,
        MAX_TRANSACTIONS_PER_BATCH,
    );

    let blocks = ledger
        .get_blocks(canister_id, *range.start(), count)
        .await?
        .0?;

    let results: Vec<_> = blocks
        .into_iter()
        .enumerate()
        .map(|(index, block)| {
            (
                range.start() + (index as u64),
                block.decode().unwrap().transaction,
            )
        })
        .collect();

    Ok(results)
}

fn process_transactions(
    transactions: Vec<(BlockHeight, Transaction)>,
    runtime_state: &mut RuntimeState,
) {
    for (block_height, transaction) in transactions.into_iter() {
        let recipient = match &transaction.transfer {
            Mint { to, amount: _ } => to,
            Send {
                from: _,
                to,
                amount: _,
                fee: _,
            } => to,
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

fn determine_canister_for_blocks(
    from: BlockHeight,
    tip_of_chain: BlockHeight,
    archive_index_entries: Vec<ArchiveIndexEntry>,
) -> (CanisterId, RangeInclusive<BlockHeight>) {
    for archive_index_entry in archive_index_entries.into_iter().rev() {
        if archive_index_entry.height_to < from {
            break;
        } else if archive_index_entry.height_from > from {
            continue;
        } else {
            let range_start = max(from, archive_index_entry.height_from);
            let range_end = min(tip_of_chain, archive_index_entry.height_to);
            return (
                CanisterId::new(archive_index_entry.canister_id.unwrap()).unwrap(),
                range_start..=range_end,
            );
        }
    }

    (LEDGER_CANISTER_ID, from..=tip_of_chain)
}
