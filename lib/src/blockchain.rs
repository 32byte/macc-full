use std::{collections::HashMap, error::Error};

use bitcoin_hashes::hex::ToHex;
use serde::{Deserialize, Serialize};

use crate::{hashes, settings::Settings};

use self::utils::{calculate_mining_reward, is_valid_tx};

// logging
// NICE-TO-HAVE: fix this
// NOTE: it should work with the #[cfg] but somehow it doesnt
//       it always uses the log::debug
// https://stackoverflow.com/questions/67087597/is-it-possible-to-use-rusts-log-info-for-tests
// #[cfg(not(test))]
// Use log crate when building application
use log::debug; 
// #[cfg(test)]
// use std::{println as debug}; // Workaround to use prinltn! for logs.

pub mod difficulty {
    use std::{
        io::{Error, ErrorKind},
        vec,
    };

    use crate::{
        settings::Settings,
        utils::{arr_to_bi, bi_to_arr},
    };

    // create a difficulty array
    pub fn create(num_zeros: usize) -> Result<[u8; 32], Error> {
        // will crash otherwise
        if num_zeros > 32 {
            return Err(Error::new(ErrorKind::Other, "num_zeros cannot exceed 32!"));
        }

        // create zeros
        let mut diff = vec![0x00_u8; num_zeros];
        // fill other bytes
        diff.append(&mut vec![0xFF_u8; 32 - num_zeros]);
        // try to convert to array
        diff.try_into()
            .map_err(|_| Error::new(ErrorKind::Other, "failed to convert vec<u8> to [u8; 32]"))
    }

    // check if the difficulty is satisfied
    // basically if the arrays were represented as a number
    // the hash has to be lower or equal to the difficulty
    pub fn satisfies(difficulty: &[u8; 32], hash: &[u8; 32]) -> bool {
        for i in 0_usize..32_usize {
            if hash[i] > difficulty[i] {
                return false;
            }

            if difficulty[i] > hash[i] {
                return true;
            }
        }
        true
    }

    // will adjust the difficulty to try to reach target block time
    pub fn adjusted(current: &[u8; 32], time_interval: u64, settings: &Settings) -> [u8; 32] {
        // calculate the current ratio
        let ratio = ((settings.target_time * settings.adjustment_interval as u64) as f64)
            / (time_interval as f64);

        // convert difficulty to bigint for easier calculations
        let mut diff_bi = arr_to_bi(current);
        // divide first to avoid buffer overflow
        diff_bi /= (ratio * (10u64.pow(settings.precision) as f64)) as u64;
        diff_bi *= 10u64.pow(settings.precision);

        bi_to_arr(&diff_bi)
    }
}

pub mod utils {
    use std::error::Error;

    use crate::hex::ToHex;
    use crate::settings::Settings;
    use crate::{hashes, script};

    use super::{debug, Transaction, TxStore};

    pub fn hash_utxou(utxou: (&[u8; 32], &usize)) -> Result<[u8; 32], Box<dyn Error>> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend(bincode::serialize(utxou.0)?);
        bytes.extend(bincode::serialize(utxou.1)?);

        Ok(hashes::sha256(&bytes))
    }

    pub fn is_valid_tx(tx: &Transaction, store: &TxStore) -> bool {
        let vout_total = tx.vout_total();
        let mut vin_total = 0_u128;

        // validate inputs
        for (hash, index, solution) in &tx.vin {
            // check if utxo exists
            if let Some((value, lock)) = store.get(hash, index) {
                // check if utxou is hashable
                if let Ok(utxou_hash) = hash_utxou((hash, index)) {
                    // validate script
                    if script::eval(format!("{} {} {}", solution, utxou_hash.to_hex(), lock))
                        .is_none()
                    {
                        debug!("Invalid solution!");
                        return false;
                    }

                    vin_total += value;
                } else {
                    debug!("Couldn't hash utxou!");
                    return false;
                }
            } else {
                debug!("UTXO not found!");
                return false;
            }
        }

        vin_total > 0 && vin_total >= vout_total
    }

    pub fn add_tx_to_store(tx: &Transaction, store: &mut TxStore) {
        // remove used transaction outputs
        for (hash, index, _) in &tx.vin {
            store.remove(hash, index)
        }

        // add new utxo's
        for (index, utxo) in tx.vout.iter().enumerate() {
            store.set(
                &tx.hash().expect("Transaction couldn't be hashed!"),
                index,
                utxo.clone(),
            );
        }
    }

    pub fn calculate_mining_reward(block_height: usize, settings: &Settings) -> u128 {
        // based on formula reward = floor( start_reward / 2^( floor(block_height / halving_interval) ) )
        // https://www.desmos.com/calculator

        let num_halvings = (block_height / settings.halvings_interval) as u32;

        settings.start_mining_reward / 2_i32.pow(num_halvings) as u128
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    // unique number generated by the transaction sender to identify transaction
    pub nonce: u128,

    // vector of inputs
    //
    // an input consists of:
    //  - transaction hash of the utxo
    //  - index of the utxo
    //  - solution to the utxo
    pub vin: Vec<([u8; 32], usize, String)>,

    // vector of outputs
    //
    // an output consists of:
    //  - value
    //  - lock
    pub vout: Vec<(u128, String)>,
}

impl Transaction {
    pub fn new_coinbase(block_height: usize, reward: u128, lock: String) -> Self {
        Self {
            nonce: block_height as u128,
            vin: vec![],
            vout: vec![(reward, lock)],
        }
    }

    pub fn hash(&self) -> Result<[u8; 32], Box<dyn Error>> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend(bincode::serialize(&self.nonce)?);
        bytes.extend(bincode::serialize(&self.vin)?);
        bytes.extend(bincode::serialize(&self.vout)?);

        Ok(hashes::sha256(&bytes))
    }

    pub fn vout_total(&self) -> u128 {
        self.vout.iter().map(|utxo| utxo.0).sum()
    }

    pub fn vin_total(&self, store: &TxStore) -> Option<u128> {
        let mut total = 0_u128;

        for (hash, index, _) in &self.vin {
            total += store.get(hash, index)?.0
        }

        Some(total)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    // when was block mined
    pub timestamp: u64,

    // hash of the previous block
    pub previous: [u8; 32],

    // number with which the block satisfies the difficulty
    pub nonce: u128,

    // transactions included in the block
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn hash(&self, nonce: Option<u128>) -> Result<[u8; 32], Box<dyn Error>> {
        let nonce = nonce.unwrap_or(self.nonce);

        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend(bincode::serialize(&self.timestamp)?);
        bytes.extend(bincode::serialize(&self.previous)?);
        bytes.extend(bincode::serialize(&nonce)?);
        bytes.extend(bincode::serialize(&self.transactions)?);

        Ok(hashes::sha256(&bytes))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
// Blockchain stores only the vector of blocks
pub struct Blockchain(Vec<Block>);

impl Blockchain {
    pub fn new_empty() -> Self {
        Self { 0: Vec::new() }
    }

    pub fn height(&self) -> usize {
        self.0.len()
    }

    pub fn at(&self, i: i32) -> &Block {
        let idx: usize = if i < 0 {
            self.height() - i.abs() as usize
        } else {
            i as usize
        };
        &self.0[idx]
    }

    pub fn valid_next(
        &self,
        block: &Block,
        store: &TxStore,
        difficulty: &[u8; 32],
        settings: &Settings,
    ) -> Option<bool> {
        // validate timestamp
        // doesn't need validation if first block
        if !self.0.is_empty() && block.timestamp < (self.0.last()?).timestamp {
            debug!("Invalid timestamp!");
            return Some(false);
        }

        // validate previous
        // doesn't need validation if first block
        if !self.0.is_empty() && block.previous != (self.0.last()?).hash(None).ok()? {
            debug!("Invalid previous hash!");
            return Some(false);
        }

        // validate nonce
        if !difficulty::satisfies(difficulty, &block.hash(None).ok()?) {
            debug!("Invalid nonce!");
            return Some(false);
        }

        // validate transaction
        let mut coinbase_tx: Option<Transaction> = None;
        let mining_reward = calculate_mining_reward(self.height(), settings);
        let mut fees = 0_u128;

        for tx in &block.transactions {
            // possible coinbase transaction
            if tx.vin.is_empty() {
                // check if there wasn't already a coinbase transaction
                if coinbase_tx.is_some() {
                    debug!("Two coinbase transactions found!");
                    return Some(false);
                }

                // so that every new coinbase transaction gets a new hash
                // it is set that the nonce is the block_height
                if tx.nonce != self.height() as u128 {
                    debug!("Invalid coinbase nonce!");
                    return Some(false);
                }

                // valid coinbase transaction found
                coinbase_tx = Some(tx.clone());
                continue;
            }

            // check if transaction valid
            if !is_valid_tx(tx, store) {
                debug!("Invalid transaction!");
                return Some(false);
            }

            // add fees
            fees += tx.vin_total(store)? - tx.vout_total();
        }

        // check if reward isn't too high
        if let Some(tx) = coinbase_tx {
            if tx.vout_total() > mining_reward + fees {
                debug!("Invalid coinbase transaction reward!");
                return Some(false);
            }
        }

        Some(true)
    }

    // adds a block to the blockchain
    // doesn't check if the block is valid,
    // so run valid_next(block) first
    pub fn add(&mut self, store: &mut TxStore, block: Block) {
        for tx in &block.transactions {
            utils::add_tx_to_store(tx, store);
        }

        self.0.push(block);
    }

    pub fn adjust_difficulty(&self, difficulty: [u8; 32], settings: &Settings) -> [u8; 32] {
        if (self.height() as u32) < settings.adjustment_interval
            || (self.height() as u32) % settings.adjustment_interval != 0
        {
            return difficulty;
        }

        let time_interval =
            self.at(-1).timestamp - self.at(-(settings.adjustment_interval as i32)).timestamp;

        debug!(
            "Targeted block time: {}, Current block time: {}",
            settings.target_time,
            time_interval as f64 / settings.adjustment_interval as f64
        );

        difficulty::adjusted(&difficulty, time_interval, settings)
    }

    pub fn is_valid(&self, settings: &Settings) -> Option<(TxStore, [u8; 32])> {
        let mut blockchain = Blockchain::new_empty();
        let mut store = TxStore::new_empty();
        let mut difficulty = settings.start_difficulty;

        for block in &self.0 {
            // adjust difficulty
            difficulty = blockchain.adjust_difficulty(difficulty, settings);

            // if next block isn't valid, whole blockchain isn't valid
            if !blockchain.valid_next(block, &store, &difficulty, settings)? {
                return None;
            }

            // add block to blockchain
            blockchain.add(&mut store, block.clone());
        }

        Some((store, difficulty))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TxStore(HashMap<String, HashMap<usize, (u128, String)>>);

impl TxStore {
    pub fn new_empty() -> Self {
        TxStore { 0: HashMap::new() }
    }

    pub fn get(&self, hash: &[u8; 32], index: &usize) -> Option<&(u128, String)> {
        let key = hash.to_hex();

        self.0.get(&key)?.get(index)
    }

    pub fn set(&mut self, hash: &[u8; 32], index: usize, utxo: (u128, String)) {
        let key = hash.to_hex();

        if self.0.get_mut(&key).is_none() {
            self.0.insert(key.clone(), HashMap::new());
        }

        self.0
            .get_mut(&key)
            .expect("UNREACHABLE!")
            .insert(index, utxo);
    }

    pub fn remove(&mut self, hash: &[u8; 32], index: &usize) {
        let key = hash.to_hex();

        if let Some(map) = self.0.get_mut(&key) {
            map.remove(index);

            if map.is_empty() {
                self.0.remove(&key);
            }
        }
    }
}
