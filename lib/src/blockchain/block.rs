use std::convert::TryInto;

use serde::{Deserialize, Serialize};

use super::{
    blockchain::Blockchain,
    difficulty::Difficulty,
    helper::current_unix_time,
    transaction::Transaction,
    txstore::TxStore,
};

#[cfg(feature = "mining")]
use super::helper::SharedData;

#[cfg(feature = "mining")]
use rocket::tokio::sync::RwLock;

use crate::blockchain::blockchain::BlockChainMethods;
use crate::{
    blockchain::{difficulty::DifficultyMethods, helper::calculate_mining_reward},
    hashes,
};

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone, Hash)]
pub struct Block {
    pub nonce: u128,
    pub timestamp: u64,
    pub previous: [u8; 32],
    pub difficulty: [u8; 32],
    pub height: usize,

    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(
        previous: [u8; 32],
        difficulty: Difficulty,
        block_height: usize,
        transactions: Vec<Transaction>,
    ) -> Block {
        Block {
            nonce: 0,
            timestamp: current_unix_time(),
            previous: previous,
            difficulty: difficulty,
            height: block_height,
            transactions: transactions,
        }
    }

    pub fn default_genesis() -> Block {
        Block {
            nonce: 0,
            timestamp: current_unix_time(),
            previous: [0; 32],
            difficulty: Difficulty::from_zeros(2),
            height: 0,
            transactions: Vec::new(),
        }
    }

    #[cfg(feature = "mining")]
    pub async fn genesis_mined() -> Block {
        let mut genesis = Block::default_genesis();

        log::debug!("Mining genesis block!");
        let nonce = genesis.mine(SharedData::new(RwLock::new(true))).await;

        genesis.nonce = nonce.expect("Couldn't mine nonce for genesis block!");

        genesis
    }

    pub fn hash(&self, custom_nonce: Option<u128>) -> [u8; 32] {
        let nonce = if let Some(n) = custom_nonce {
            n
        } else {
            self.nonce
        };

        let nonce_bytes: Vec<u8> = bincode::serialize(&nonce).unwrap();
        let timestamp_bytes: Vec<u8> = bincode::serialize(&self.timestamp).unwrap();
        let previous_bytes: Vec<u8> = bincode::serialize(&self.previous).unwrap();
        let difficulty_bytes: Vec<u8> = bincode::serialize(&self.difficulty).unwrap();
        let block_height_bytes: Vec<u8> = bincode::serialize(&self.height).unwrap();
        let transaction_bytes: Vec<u8> = bincode::serialize(&self.transactions).unwrap();

        let mut raw_data: Vec<u8> = Vec::new();
        raw_data.extend_from_slice(&nonce_bytes);
        raw_data.extend_from_slice(&timestamp_bytes);
        raw_data.extend_from_slice(&previous_bytes);
        raw_data.extend_from_slice(&difficulty_bytes);
        raw_data.extend_from_slice(&block_height_bytes);
        raw_data.extend_from_slice(&transaction_bytes);

        hashes::sha256(&raw_data).try_into().unwrap()
    }

    pub fn verify(&self, tx_store: &TxStore, last_block: Option<&Block>) -> bool {
        // verify previous
        if self.height == 0 {
            if self.previous != Block::default_genesis().previous {
                return false;
            }
        } else if let Some(last) = last_block {
            if self.previous != last.hash(None) {
                log::debug!("Invalid previous!");
                return false;
            }
            if last.timestamp > self.timestamp {
                return false;
            }
        } else {
            return false;
        }

        log::debug!("Valid previous!");

        // verify nonce
        if !self.valid_nonce(None) {
            log::debug!("Invalid nonce! {} {:?}", self.height, self.difficulty);
            return false;
        }

        log::debug!("Valid nonce!");

        let mut used_txs: TxStore = TxStore::new();

        // verify transactions
        let mut coinbase_tx: Option<Transaction> = None;
        let mut fees = 0u128;
        let mining_reward = calculate_mining_reward(self.height);

        for tx in &self.transactions {
            if tx.vin.len() == 0 {
                if coinbase_tx.is_none() {
                    coinbase_tx = Some(tx.clone());
                    continue;
                } else {
                    log::debug!("Two coinbase txs found!");
                    return false;
                }
            }

            if !tx.verify(tx_store, &mut used_txs) {
                log::debug!("Invalid tx: {:?}", tx);
                return false;
            }

            fees += tx.vin_total(tx_store) - tx.vout_total();
        }
        log::debug!("Valid txs!");

        if coinbase_tx.is_some() {
            if coinbase_tx.unwrap().vout_total() > fees + mining_reward {
                log::debug!("Invalid coinbase fees");
                return false;
            }
        }
        log::debug!("Valid coinbase!");

        true
    }

    pub fn add(
        &self,
        blockchain: &mut Blockchain,
        tx_store: &mut TxStore,
        mem_tx_store: &mut TxStore,
        mempool: &mut Vec<Transaction>,
    ) {
        // add block to blockchain, update tx_store
        blockchain.add_block(tx_store, self);

        // update mempool and mem_tx_store
        for tx in &self.transactions {
            for utxo in &tx.vin {
                match mem_tx_store.0.get_mut(&utxo.tx_hash) {
                    Some(utxos) => {
                        if utxos.contains_key(&utxo.index) {
                            utxos.remove(&utxo.index);
                        }
                    }
                    None => (),
                }
            }
        }
        mempool.drain(0..self.transactions.len() - 1);
    }

    pub fn valid_nonce(&self, nonce: Option<u128>) -> bool {
        self.difficulty.check(self.hash(nonce))
    }

    #[cfg(feature = "mining")]
    pub async fn mine(&self, running: SharedData<bool>) -> Option<u128> {
        let mut nonce: u128 = 0;

        while !self.valid_nonce(Some(nonce)) {
            nonce += 1;

            if !*running.read().await {
                return None;
            }
        }

        Some(nonce)
    }
}
