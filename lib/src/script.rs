use secp256k1::{
    bitcoin_hashes::hex::{FromHex},
    Message, PublicKey, Secp256k1, Signature,
};

use crate::ecdsa::AsPublicAddress;

fn to_addr(stack: &mut Vec<String>) -> bool {
    let pk = match stack.pop() {
        Some(x) => x,
        None => return false,
    };

    let pk_bytes = match Vec::from_hex(&pk) {
        Ok(x) => x,
        Err(_) => return false,
    };

    let pub_key: PublicKey = if let Ok(p) = PublicKey::from_slice(&pk_bytes) {
        p
    } else {
        return false;
    };

    let addr = if let Ok(a) = pub_key.as_address() {
        a
    } else {
        return false;
    };

    stack.push(addr);
    true
}

fn op_eq(stack: &mut Vec<String>) -> bool {
    let val1 = match stack.pop() {
        Some(x) => x,
        None => return false,
    };

    let val2 = match stack.pop() {
        Some(x) => x,
        None => return false,
    };

    val1 == val2
}

fn verify_signature(stack: &mut Vec<String>) -> bool {
    let utxo_hash = match stack.pop() {
        Some(x) => x,
        None => return false,
    };

    let sign = match stack.pop() {
        Some(x) => x,
        None => return false,
    };

    let pub_key = match stack.pop() {
        Some(x) => x,
        None => return false,
    };

    let utxo_hash: secp256k1::bitcoin_hashes::sha256::Hash = secp256k1::bitcoin_hashes::Hash::hash(&if let Ok(data) = Vec::from_hex(&utxo_hash) {
        data
    } else {
        return false;
    });
    
    let msg = Message::from(utxo_hash);
    
    let sig = if let Ok(m) = Signature::from_der(&if let Ok(data) = Vec::from_hex(&sign) {
        data
    } else {
        return false;
    }) {
        m
    } else {
        return false;
    };

    let pk = if let Ok(m) = PublicKey::from_slice(&if let Ok(data) = Vec::from_hex(&pub_key) {
        data
    } else {
        return false;
    }) {
        m
    } else {
        return false;
    };
    

    Secp256k1::new().verify(&msg, &sig, &pk).is_ok()
}

pub fn eval(script: String) -> Option<Vec<String>> {
    let mut stack: Vec<String> = Vec::new();

    for val in script.split(" ") {
        if val == "" {
            continue;
        }
        if !match val.to_lowercase().as_str() {
            "op_eq" => op_eq(&mut stack),
            "to_addr" => to_addr(&mut stack),
            "verify_sign" => verify_signature(&mut stack),
            _ => {
                stack.push(val.to_string());
                true
            }
        } {
            log::error!("{}: {}", val, script);
            return None;
        }
    }
    Some(stack)
}

/*
example script: [pub_key (pub_key sign utxo_hash   {verify_sign)    to_addr addr eq]}
*/
