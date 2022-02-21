use wasm_bindgen::prelude::*;
use macc_lib::{hex::{ToHex, FromHex}, settings::Settings, blockchain::{utils, Transaction, TxStore, Block, Blockchain}, ecdsa::{Client, pb_key_to_addr, create_lock_with_addr, create_lock, create_secp}};

// utils

#[wasm_bindgen]
pub fn to_hex(data: &[u8]) -> String {
    data.to_hex()
}

#[wasm_bindgen]
pub fn calculate_mining_reward(block_height: usize) -> String {
    utils::calculate_mining_reward(block_height, &Settings::default()).to_string()
}

#[wasm_bindgen]
pub fn current_time() -> u64 {
    macc_lib::utils::current_time()
}

// transaction
#[wasm_bindgen]
pub fn tx_hash(tx_str: String) -> Option<String> {
    let tx: Transaction = serde_json::from_str(&tx_str).ok()?;

    Some(tx.hash().ok()?.to_hex())
}

#[wasm_bindgen]
pub fn tx_vout_total(tx_str: String) -> Option<String> {
    let tx: Transaction = serde_json::from_str(&tx_str).ok()?;

    Some(tx.vout_total().to_string())
}

#[wasm_bindgen]
pub fn tx_vin_total(tx_str: String, store_str: String) -> Option<String> {
    let tx: Transaction = serde_json::from_str(&tx_str).ok()?;
    let store: TxStore = serde_json::from_str(&store_str).ok()?;

    Some(tx.vin_total(&store)?.to_string())
}

#[wasm_bindgen]
pub fn block_hash(block_str: String) -> Option<String> {
    let block: Block = serde_json::from_str(&block_str).ok()?;

    Some(block.hash(None).ok()?.to_hex())
}

#[wasm_bindgen]
pub fn get_tx(blockchain_str: String, hash_str: String) -> Option<String> {
    let bc: Blockchain = serde_json::from_str(&blockchain_str).ok()?;
    let hash: [u8; 32] = Vec::from_hex(&hash_str).ok()?.try_into().ok()?;

    let found = bc.get_transaction(&hash)?;

    Some(serde_json::to_string(&found).ok()?)
}

// Wallet

#[wasm_bindgen]
pub fn get_client(sk_key: String) -> Option<String> {
    let client = Client::from_sk_key(sk_key.clone()).ok()?;

    let pb_key = client.pb_key.to_hex();
    let addr = pb_key_to_addr(&client.pb_key.serialize());

    Some(serde_json::to_string(&(sk_key, pb_key, addr)).ok()?)
}

#[wasm_bindgen]
pub fn my_utxos(store_str: String, addr: String) -> Option<String> {
    let store: TxStore = serde_json::from_str(&store_str).ok()?;
    let owned = store.get_owned_fast(addr)?;

    Some(serde_json::to_string(&owned).ok()?)
}

#[wasm_bindgen]
pub fn send(owned_str: String, sk_key: String, addr: String, amount_str: String) -> Option<String> {
    let owned: (u128, Vec<(String, usize, u128)>) = serde_json::from_str(&owned_str).ok()?;
    let amount: u128 = amount_str.parse().ok()?;

    let mut client = Client::from_sk_key(sk_key).ok()?;

    if amount > owned.0 {
        return None;
    }

    let mut sending = 0_u128;
    let mut input: Vec<([u8; 32], usize)> = Vec::new();
    let mut i = 0_usize;

    while sending < amount {
        let hash: [u8; 32] = Vec::from_hex(&owned.1[i].0).ok()?.try_into().ok()?;
        let index: usize = owned.1[i].1;
        let value: u128 = owned.1[i].2;

        sending += value;
        input.push((hash, index));

        i += 1;
    }

    let change = sending - amount;
    let mut output: Vec<(u128, String)> = Vec::new();
    // send to addr
    output.push((amount, create_lock_with_addr(&addr)));
    // send change to self
    if change > 0 {
        output.push((change, create_lock(&client.pb_key)));
    }

    let secp = create_secp();

    let tx: Transaction = client.create_transaction_addr(&secp, input, output)?;
    let tx_str = serde_json::to_string(&tx).ok()?;

    Some(tx_str)
}