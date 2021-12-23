use std::collections::HashMap;

use wasm_bindgen::prelude::*;

use macc_lib::blockchain::{
    transaction::{Transaction, UTXO, UTXOU},
    txstore::TxStore,
};
use macc_lib::ecdsa;
use secp256k1::{
    bitcoin_hashes::hex::{FromHex, ToHex},
    PublicKey, Secp256k1,
};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn get_owned_utxos(tx_store: &str, me: &str) -> String {
    let tx_store: TxStore = serde_json::from_str(tx_store).unwrap();
    let mut mine: TxStore = TxStore::new();
    let mut bal: u128 = 0;

    for (tx_hash, utxos) in &tx_store.0 {
        for (index, utxo) in utxos {
            // Note: should use script::eval for checking if owned
            if utxo.lock.contains(me) {
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
    // check balance
    if bal < amount {
        return None;
    }

    // try to parse secret-key
    let sk = match ecdsa::sk_from_hex(secret_key) {
        Some(s) => s,
        None => return None,
    };
    let secp = Secp256k1::new();
    
    let mut mine: TxStore = serde_json::from_str(mine).unwrap();
    let mut sending: u32 = 0;
    let mut vin: Vec<UTXOU> = Vec::new();
    let mut vout: Vec<UTXO> = Vec::new();

    // Note: a better algorithm should be possible
    while sending < amount {
        let mine_cloned = mine.0.clone();
        let (tx_hash, utxos) = mine_cloned.iter().next().unwrap();
        let (index, utxo) = utxos.iter().next().unwrap();

        sending += utxo.value as u32;
        bal -= utxo.value as u32;

        let mut utxou = UTXOU::new(tx_hash.clone(), *index, "".to_string());
        let solution = ecdsa::create_solution(&secp, sk, &utxou);
        utxou.solution = solution;

        vin.push(utxou);

        mine.0.get_mut(tx_hash).unwrap().remove(index);
        if mine.0.get(tx_hash).unwrap().len() == 0 {
            mine.0.remove(tx_hash);
        }
    }

    vout.push(UTXO::new(amount as u128, ecdsa::create_lock(receiver)));

    if amount + fee < sending {
        vout.push(UTXO::new(
            (sending - amount - fee) as u128,
            ecdsa::create_lock(my_addr),
        ));
    }

    let tx = Transaction::new(vin, vout);

    Some(serde_json::to_string(&tx).unwrap())
}

#[wasm_bindgen]
pub fn get_address(pk: &str) -> Option<String> {
    if let Ok(bytes) = Vec::from_hex(pk) {
        return Some(ecdsa::pk_to_address(&bytes));
    } 
    None
}

#[wasm_bindgen]
pub fn get_public_key(secret_key: &str) -> Option<String> {
    match ecdsa::sk_from_hex(secret_key) {
        Some(s) => Some(PublicKey::from_secret_key(&Secp256k1::new(), &s).to_hex()),
        None => return None,
    }
}

#[wasm_bindgen]
pub fn generate_lock(addr: &str) -> String {
    ecdsa::create_lock(addr)
}

#[wasm_bindgen]
pub fn generate_solution(secret_key: &str, utxou: &str) -> Option<String> {
    let sk = match ecdsa::sk_from_hex(secret_key) {
        Some(s) => s,
        None => return None,
    };

    let utxou: UTXOU = match serde_json::from_str(utxou) {
        Ok(u) => u,
        Err(_) => return None,
    };

    Some(ecdsa::create_solution(&Secp256k1::new(), sk, &utxou))
}
