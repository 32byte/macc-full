use std::{collections::HashMap, convert::TryInto};

use serde::{Deserialize, Serialize};

use crate::{hashes, script};

use super::txstore::TxStore;

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone, Hash)]
pub struct UTXOU {
    pub tx_hash: [u8; 32],
    pub index: usize,
    pub solution: String,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone, Hash)]
pub struct UTXO {
    pub value: u128,
    pub lock: String,
}

impl UTXOU {
    pub fn new(tx_hash: [u8; 32], index: usize, solution: String) -> UTXOU {
        UTXOU {
            tx_hash: tx_hash,
            index: index,
            solution: solution,
        }
    }
}

impl UTXO {
    pub fn new(value: u128, lock: String) -> UTXO {
        UTXO {
            value: value,
            lock: lock,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone, Hash)]
pub struct Transaction {
    pub vin: Vec<UTXOU>,
    pub vout: Vec<UTXO>,
}

impl Transaction {
    pub fn new(vin: Vec<UTXOU>, vout: Vec<UTXO>) -> Transaction {
        Transaction {
            vin: vin,
            vout: vout,
        }
    }

    pub fn coinbase(reward: u128, lock: String) -> Transaction {
        let utxo = UTXO::new(reward, lock);

        Transaction {
            vin: Vec::new(),
            vout: vec![utxo],
        }
    }

    pub fn hash(&self, block_height: usize) -> [u8; 32] {
        let tx_bytes = bincode::serialize(self).unwrap();

        let block_height_bytes: Vec<u8> = bincode::serialize(&block_height).unwrap();

        let mut raw_data: Vec<u8> = Vec::new();
        raw_data.extend_from_slice(&tx_bytes);
        raw_data.extend_from_slice(&block_height_bytes);

        hashes::sha256(&raw_data).try_into().unwrap()
    }

    pub fn verify(&self, tx_store: &TxStore, used_tx_store: &mut TxStore) -> bool {
        let output_value: u128 = self.vout_total();
        let mut input_value: u128 = 0;

        // validate inputs
        for utxou in &self.vin {
            // check if UTXO with the tx_hash exists
            if let Some(utxos) = tx_store.0.get(&utxou.tx_hash) {
                // check if UTXO with the index exists
                if let Some(utxo) = utxos.get(&utxou.index) {
                    // validate script
                    if script::eval(format!("{} {}", &utxou.solution, &utxo.lock)).is_none() {
                        log::debug!("Invalid script!");
                        return false;
                    }

                    // check if utxou isn't already used
                    if let Some(u) = used_tx_store.0.get(&utxou.tx_hash) {
                        if let Some(_) = u.get(&utxou.index) {
                            return false;
                        }
                    }

                    input_value += utxo.value;
                } else {
                    log::debug!("UTXO index not found!");
                    return false;
                }
            } else {
                log::debug!("{:?}", tx_store);
                log::debug!("TX HASH not found!");
                return false;
            }
        }

        // invalid output value
        if output_value > input_value {
            log::debug!("{} {}", output_value, input_value);
            return false;
        }

        // add used UTXO's to used_tx_store
        self.vin.iter().for_each(|utxou| {
            if used_tx_store.0.get_mut(&utxou.tx_hash).is_none() {
                used_tx_store.0.insert(utxou.tx_hash, HashMap::new());
            }

            // using expects here should be safe since the edge cases are handled before
            used_tx_store
                .0
                .get_mut(&utxou.tx_hash)
                .expect("Something went wrong!")
                .insert(
                    utxou.index,
                    tx_store
                        .0
                        .get(&utxou.tx_hash)
                        .expect("Something went wrong!")
                        .get(&utxou.index)
                        .expect("Something went wrong!")
                        .clone(),
                );
        });

        true
    }

    pub fn add(&self, tx_store: &mut TxStore, block_height: usize) {
        // remove used utxo
        for utxou in &self.vin {
            tx_store
                .0
                .get_mut(&utxou.tx_hash)
                .expect("Can't add invalid transaction!")
                .remove(&utxou.index);
        }

        // add new utxo
        let mut new_utxos: HashMap<usize, UTXO> = HashMap::new();
        for (idx, utxo) in self.vout.iter().enumerate() {
            new_utxos.insert(idx, utxo.clone());
        }
        tx_store.0.insert(self.hash(block_height), new_utxos);
    }

    pub fn vin_total(&self, tx_store: &TxStore) -> u128 {
        let mut input_value: u128 = 0;
        // calculate input value
        for utxou in &self.vin {
            // NOTE: .unwrap() should be safe since this should be only called on already verified tx's
            input_value += tx_store
                .0
                .get(&utxou.tx_hash)
                .unwrap()
                .get(&utxou.index)
                .unwrap()
                .value;
        }
        input_value
    }

    pub fn vout_total(&self) -> u128 {
        let mut output_value: u128 = 0;
        // get the output value
        for utxo in &self.vout {
            output_value += utxo.value;
        }
        output_value
    }

    pub fn fees(&self, tx_store: &TxStore) -> u128 {
        self.vin_total(tx_store) - self.vout_total()
    }
}
