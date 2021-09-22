use candid::CandidType;
use ledger_canister::BlockHeight;
use serde::Deserialize;

#[derive(CandidType, Deserialize, Default)]
pub struct TransactionImportState {
    in_progress: bool,
    synced_up_to: Option<BlockHeight>,
}

impl TransactionImportState {
    pub fn try_start_import(&mut self) -> bool {
        if self.in_progress {
            false
        } else {
            self.in_progress = true;
            true
        }
    }

    pub fn mark_import_finished(&mut self) {
        self.in_progress = false;
    }

    pub fn synced_up_to(&self) -> Option<u64> {
        self.synced_up_to
    }

    pub fn set_synced_up_to(&mut self, value: BlockHeight) {
        self.synced_up_to = Some(value);
    }
}
