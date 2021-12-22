use std::collections::HashMap;

use serde::{ser::SerializeSeq, Deserialize, Serialize};

use super::{transaction::Transaction, txstore::TxStore};

#[derive(Clone, Debug)]
pub struct MemPool {
    pub transactions: Vec<Transaction>,
    pub hashmap: HashMap<[u8; 32], usize>,
}

impl MemPool {
    pub fn new() -> Self {
        MemPool {
            transactions: Vec::new(),
            hashmap: HashMap::new(),
        }
    }

    pub fn add_tx(&mut self, tx: &Transaction) {
        self.transactions.push(tx.clone());
        self.hashmap.insert(tx.hash(0), self.transactions.len() - 1);
    }

    pub fn remove_tx(&mut self, tx: &Transaction) {
        if let Some(index) = self.hashmap.get(&tx.hash(0)) {
            self.transactions.remove(*index);
        }
    }

    pub fn sorted_txs(&self, tx_store: &TxStore) -> Vec<Transaction> {
        let mut sorted = self.transactions.clone();
        sorted.sort_by(|a, b| a.fees(tx_store).cmp(&b.fees(tx_store)));
        sorted
    }
}

impl Serialize for MemPool {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.transactions.len()))?;

        for e in &self.transactions {
            seq.serialize_element(e)?;
        }

        seq.end()
    }
}

use serde::de::Visitor;
use std::marker::PhantomData;

pub struct MemPoolVisitor {
    marker: PhantomData<fn() -> MemPool>,
}

impl MemPoolVisitor {
    pub fn new() -> Self {
        MemPoolVisitor {
            marker: PhantomData,
        }
    }
}

impl<'de> Visitor<'de> for MemPoolVisitor {
    type Value = MemPool;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("expecting a MemPool!")
    }

    fn visit_seq<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut txs: Vec<Transaction> = Vec::new();
        let mut map: HashMap<[u8; 32], usize> = HashMap::new();

        while let Some(tx) = access.next_element()? {
            let tx: Transaction = tx;
            map.insert(tx.hash(0), txs.len());
            txs.push(tx);
        }

        Ok(MemPool {
            transactions: txs,
            hashmap: map,
        })
    }
}

impl<'de> Deserialize<'de> for MemPool {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(MemPoolVisitor::new())
    }
}
