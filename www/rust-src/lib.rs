use std::collections::HashMap;

use wasm_bindgen::prelude::*;

use macc_lib::blockchain::{
    transaction::{Transaction, UTXO, UTXOU},
    txstore::TxStore,
};
use macc_lib::ecdsa::AsPublicAddress;
use secp256k1::{
    bitcoin_hashes::hex::{FromHex, ToHex},
    SecretKey, PublicKey, Secp256k1,
};

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
    } else {
        return None;
    };

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
            if utxo.lock.contains(i) {
                bal += utxo.value;

                if mine.0.get(tx_hash).is_none() {
                    mine.0.insert(tx_hash.clone(), HashMap::new());
                }
                mine.0
                    .get_mut(tx_hash)
                    .unwrap()
                    .insert(*index, utxo.clone());
            }
        }
    }

    format!(
        "{{ \"mine\": {}, \"bal\": {} }}",
        serde_json::to_string(&mine).unwrap(),
        bal
    )
}

#[wasm_bindgen]
pub fn get_send_body(
    mine: &str,
    mut bal: u32,
    amount: u32,
    receiver: &str,
    secret_key: &str,
    my_addr: &str,
    fee: u32,
) -> Option<String> {
    let sk = if let Ok(s) = SecretKey::from_slice(&if let Ok(data) = Vec::from_hex(secret_key) {
        data
    } else {
        alert("Invalid secret key hex!");
        return None;
    }) {
        s
    } else {
        alert("Invalid secret key!");
        return None;
    };

    if bal < amount {
        alert("Not enough balance!");
        return None;
    }

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

        let utxou = UTXOU::new(tx_hash.clone(), *index, "".to_string());
        let solution = macc_lib::ecdsa::create_solution(sk, &utxou);
        let utxou = UTXOU::new(tx_hash.clone(), *index, solution);

        vin.push(utxou);

        mine.0.get_mut(tx_hash).unwrap().remove(index);
        if mine.0.get(tx_hash).unwrap().len() == 0 {
            mine.0.remove(tx_hash);
        }
    }

    vout.push(UTXO::new(amount as u128, macc_lib::ecdsa::create_lock(receiver)));

    if amount + fee < sending {
        vout.push(UTXO::new(
            (sending - amount - fee) as u128,
            macc_lib::ecdsa::create_lock(my_addr),
        ));
    }

    let tx = Transaction::new(vin, vout);

    Some(serde_json::to_string(&tx).unwrap())
}

#[wasm_bindgen]
pub fn get_address(pk: &str) -> String {
    let pk: PublicKey = PublicKey::from_slice(&Vec::from_hex(pk).unwrap()).unwrap();
    
    pk.as_address().unwrap()
}

#[wasm_bindgen]
pub fn get_public_key(sk: &str) -> String {
    let sk: SecretKey = SecretKey::from_slice(&Vec::from_hex(sk).unwrap()).unwrap();
    
    PublicKey::from_secret_key(&Secp256k1::new(), &sk).to_hex()
}

#[wasm_bindgen]
pub fn generate_lock(addr: &str) -> String {
    macc_lib::ecdsa::create_lock(addr)
}

#[wasm_bindgen]
pub fn generate_solution(sk: &str, utxou: &str) -> Option<String> {
    let sk = if let Ok(s) = SecretKey::from_slice(&if let Ok(data) = Vec::from_hex(&sk) {
        data
    } else {
        return None;
    }) {
        s
    } else {
        return None;
    };

    let utxou: UTXOU = if let Ok(u) = serde_json::from_str(utxou) {
        u
    } else {
        return None;
    };

    Some(macc_lib::ecdsa::create_solution(sk, &utxou))
}
