use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{transaction::Transaction, txstore::TxStore};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MemPool {
    pub transactions: HashMap<Transaction, ()>
}

impl MemPool {
    pub fn new() -> Self {
        MemPool {
            transactions: HashMap::new()
        }
    }

    pub fn add_tx(&mut self, tx: &Transaction) {
        self.transactions.insert(tx.clone(), ());
    }

    pub fn remove_tx(&mut self, tx: &Transaction) {
        self.transactions.remove(tx);
    }

    pub fn sorted_txs(&self, tx_store: &TxStore) -> Vec<Transaction> {
        let mut sorted: Vec<Transaction> = self.transactions.keys().cloned().collect();
        sorted.sort_by(|a, b| a.fees(tx_store).cmp(&b.fees(tx_store)));
        sorted
    }
}
