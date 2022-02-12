use crate::blockchain::transaction::UTXOU;
use crate::hashes;

use secp256k1::{bitcoin_hashes::{
    hex::{FromHex, ToHex},
    sha256,
}, Signature};
use secp256k1::{All, Message, PublicKey, Secp256k1, SecretKey};

pub fn sk_to_hex(sk: &SecretKey) -> String {
    sk.as_ref().to_hex()
}

pub fn sk_from_hex(sk: &str) -> Option<SecretKey> {
    if let Ok(bytes) = Vec::from_hex(sk) {
        if let Ok(key) = SecretKey::from_slice(&bytes) {
            return Some(key);
        }
    }

    None
}

pub fn pk_to_address(pk: &Vec<u8>) -> String {
    let mut address = hashes::ripemd160(&hashes::sha256(&pk));

    // prepend 0x00
    address.insert(0, 0u8);

    address.append(&mut hashes::checksum(&address));

    bs58::encode(&address).into_string()
}

pub fn pk_from_hex(p: &str) -> Option<PublicKey> {
    if let Ok(bytes) = Vec::from_hex(p) {
        if let Ok(pk) = PublicKey::from_slice(&bytes) {
            return Some(pk);
        }
    }

    None
}

pub fn msg_from_hex(m: &str) -> Option<Message> {
    if let Ok(bytes) = Vec::from_hex(m) {
        return Some(Message::from_hashed_data::<sha256::Hash>(&bytes));
    }

    None
}

pub fn sig_from_hex(s: &str) -> Option<Signature> {
    if let Ok(bytes) = Vec::from_hex(s) {
        if let Ok(sig) = Signature::from_der(&bytes) {
            return Some(sig);
        }
    }

    None
}

pub fn create_lock(addr: &str) -> String {
    format!("verify_sign to_addr {} op_eq", addr)
}

pub fn create_solution(secp: &Secp256k1<All>, sk: SecretKey, utxou: &UTXOU) -> String {
    let kp: PublicKey = PublicKey::from_secret_key(&secp, &sk);

    let msg: Message = Message::from_hashed_data::<sha256::Hash>(&utxou.hash().to_vec());
    let sign = secp.sign(&msg, &sk);

    format!(
        "{} {} {}",
        kp.to_hex(),
        kp.to_hex(),
        sign.serialize_der().to_hex()
    )
}
*/