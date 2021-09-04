use ledger_canister::{BlockHeight, Transaction, Transfer};

#[derive(Clone)]
pub struct TransactionDetails {
    pub transaction_index: u64,
    pub from: String,
    pub to: String,
    pub amount: String,
}

impl From<(BlockHeight, Transaction)> for TransactionDetails {
    fn from((block_height, transaction): (BlockHeight, Transaction)) -> Self {
        let from_s: String;
        let to_s: String;
        let amount_s: String;
        match transaction.transfer {
            Transfer::Burn { from: _, amount: _ } => {
                // Notifications are (currently) only on receipt of ICP
                unimplemented!();
            }
            Transfer::Mint { to, amount } => {
                from_s = "Minting Account".to_string();
                to_s = to.to_string();
                amount_s = amount.to_string();
            }
            Transfer::Send {
                from,
                to,
                amount,
                fee: _,
            } => {
                from_s = from.to_string();
                to_s = to.to_string();
                amount_s = amount.to_string();
            }
        };

        TransactionDetails {
            transaction_index: block_height,
            from: from_s,
            to: to_s,
            amount: amount_s,
        }
    }
}
