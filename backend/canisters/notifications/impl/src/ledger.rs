use async_trait::async_trait;
use ledger_canister::{BlockHeight, GetBlocksRes, TipOfChainRes};

#[async_trait]
pub trait Ledger {
    async fn tip_of_chain(&self) -> Result<TipOfChainRes, String>;

    async fn get_blocks_since(
        &self,
        start: BlockHeight,
        length: usize,
    ) -> Result<GetBlocksRes, String>;
}

pub struct LedgerCanister {}

#[async_trait]
impl Ledger for LedgerCanister {
    async fn tip_of_chain(&self) -> Result<TipOfChainRes, String> {
        todo!()
    }

    async fn get_blocks_since(&self, _start: u64, _length: usize) -> Result<GetBlocksRes, String> {
        todo!()
    }
}

pub struct EmptyLedger {}

#[async_trait]
impl Ledger for EmptyLedger {
    async fn tip_of_chain(&self) -> Result<TipOfChainRes, String> {
        unimplemented!()
    }

    async fn get_blocks_since(&self, _start: u64, _length: usize) -> Result<GetBlocksRes, String> {
        unimplemented!()
    }
}
