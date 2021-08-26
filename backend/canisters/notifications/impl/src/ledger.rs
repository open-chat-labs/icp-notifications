use async_trait::async_trait;
use ic_base_types::CanisterId;
use ledger_canister::protobuf::ArchiveIndexResponse;
use ledger_canister::{BlockHeight, GetBlocksRes, TipOfChainRes};

#[async_trait]
pub trait Ledger {
    async fn tip_of_chain(&self) -> Result<TipOfChainRes, String>;
    async fn archive_index(&self) -> Result<ArchiveIndexResponse, String>;
    async fn get_blocks(
        &self,
        canister_id: CanisterId,
        start: BlockHeight,
        length: usize,
    ) -> Result<GetBlocksRes, String>;
}

#[derive(Clone)]
pub struct LedgerCanister {}

#[async_trait]
impl Ledger for LedgerCanister {
    async fn tip_of_chain(&self) -> Result<TipOfChainRes, String> {
        todo!()
    }

    async fn archive_index(&self) -> Result<ArchiveIndexResponse, String> {
        todo!()
    }

    async fn get_blocks(
        &self,
        _canister_id: CanisterId,
        _start: u64,
        _length: usize,
    ) -> Result<GetBlocksRes, String> {
        todo!()
    }
}
