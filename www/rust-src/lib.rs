use std::collections::HashMap;

use wasm_bindgen::prelude::*;

use macc_lib::blockchain::{transaction::{Transaction, UTXO, UTXOU}, txstore::TxStore};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn big_computation() {
    alert("Big computation in Rust");
}

#[wasm_bindgen]
pub fn welcome(name: &str) {
   alert(&format!("Hello {}, from Rust!", name));
}

#[wasm_bindgen]
pub fn hash_tx(tx: &str, height: usize) -> Option<String> {
    let tx: Transaction = if let Ok(t) = serde_json::from_str(tx) {
        t
    } else { return None; };

    let hash = tx.hash(height);

    if let Ok(json) = serde_json::to_string(&hash) {
        return Some(json);
    } 
    None
}

#[wasm_bindgen]
pub fn get_mine(tx_store: &str, i: &str) -> String {
    let tx_store: TxStore = serde_json::from_str(tx_store).unwrap();
    let mut mine: TxStore = TxStore::new();
    let mut bal: u128 = 0;

    for (tx_hash, utxos) in &tx_store.0 {
        for (index, utxo) in utxos {
            if utxo.lock == i {
                bal += utxo.value;

                if mine.0.get(tx_hash).is_none() {
                    mine.0.insert(tx_hash.clone(), HashMap::new());
                }
                mine.0.get_mut(tx_hash).unwrap().insert(*index, utxo.clone());
            }
        }
    }

    format!("{{ \"mine\": {}, \"bal\": {} }}", serde_json::to_string(&mine).unwrap(), bal)
}

#[wasm_bindgen]
pub fn get_send_body(mine: &str, mut bal: u32, amount: u32, receiver: &str, solution: &str, change_lock: &str, fee: u32) -> Option<String> {
    if bal < amount { return None; }

    let mut mine: TxStore = serde_json::from_str(mine).unwrap();

    let mut sending: u32 = 0;
    let mut vin: Vec<UTXOU> = Vec::new();
    let mut vout: Vec<UTXO> = Vec::new();

    while sending < amount {
        let mine_cloned = mine.0.clone();
        let (tx_hash, utxos) = mine_cloned.iter().next().unwrap();
        let (index, utxo) = utxos.iter().next().unwrap();

        sending += utxo.value as u32;
        bal -= utxo.value as u32;

        vin.push(UTXOU::new(tx_hash.clone(), *index, solution.to_string()));

        mine.0.get_mut(tx_hash).unwrap().remove(index);
        if mine.0.get(tx_hash).unwrap().len() == 0 {
            mine.0.remove(tx_hash);
        }
    }

    vout.push(UTXO::new(amount as u128, receiver.to_string()));

    if amount + fee < sending {
        vout.push(UTXO::new((sending - amount - fee) as u128, change_lock.to_string()));
    }

    let tx = Transaction::new(vin, vout);

    Some(serde_json::to_string(&tx).unwrap())
}