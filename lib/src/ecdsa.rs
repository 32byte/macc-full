use bitcoin_hashes::sha256;
use rand::rngs::OsRng;
use secp256k1::ecdsa::Signature;
use secp256k1::rand;
use secp256k1::{All, Message, PublicKey, Secp256k1, SecretKey};
use std::error::Error;

use crate::blockchain::utils::hash_utxou;
use crate::blockchain::Transaction;
use crate::hashes;
use crate::hex::ToHex;

// wrapper functions for creating Secp and Rng

pub fn create_secp() -> Secp256k1<All> {
    Secp256k1::new()
}

pub fn create_rng() -> Result<OsRng, rand::Error> {
    rand::rngs::OsRng::new()
}

// client with wrapper & helper functions
pub struct Client {
    pub pb_key: PublicKey,
    pub sk_key: SecretKey,

    nonce: u128,
}

impl Client {
    pub fn new_random(secp: &Secp256k1<All>, rng: &mut OsRng) -> Self {
        let (sk_key, pk_key) = secp.generate_keypair(rng);

        Self {
            sk_key: sk_key,
            pb_key: pk_key,
            nonce: 0,
        }
    }

    pub fn sign(&self, secp: &Secp256k1<All>, message: &Message) -> Signature {
        secp.sign_ecdsa(message, &self.sk_key)
    }

    pub fn create_transaction(
        &mut self,
        secp: &Secp256k1<All>,
        input: Vec<([u8; 32], usize)>,
        output: Vec<(u128, PublicKey)>,
    ) -> Option<Transaction> {
        // for each input create a solution
        let vin: Vec<([u8; 32], usize, String)> = input
            .iter()
            .filter_map(|(hash, index)| {
                let message = hash_utxou((hash, index)).ok()?.to_hex();
                let solution = create_solution(secp, self, &msg_from_str(&message));

                Some((*hash, *index, solution))
            })
            .collect();

        // if they aren't equally big that means
        // that the hash_utxou failed somewhere meaning
        // that one of the inputs is invalid
        //  -> invalid inputs -> can't create transaction
        if vin.len() != input.len() {
            return None;
        }

        let vout: Vec<(u128, String)> = output
            .iter()
            .map(|(amount, receiver)| {
                let lock = create_lock(receiver);

                (*amount, lock)
            })
            .collect();

        // create transaction
        let tx = Transaction {
            nonce: self.nonce,
            vin: vin,

            vout: vout,
        };
        // update nonce
        self.nonce += 1;
        // return transaction
        Some(tx)
    }
}

// helper functions
pub fn msg_from_str(message: &str) -> Message {
    Message::from_hashed_data::<sha256::Hash>(message.as_bytes())
}

pub fn msg_from_bytes(bytes: &[u8]) -> Result<Message, Box<dyn Error>> {
    Ok(Message::from_slice(bytes)?)
}

pub fn sig_from_bytes(bytes: &[u8]) -> Result<Signature, Box<dyn Error>> {
    Ok(Signature::from_compact(bytes)?)
}

pub fn pb_key_from_bytes(bytes: &[u8]) -> Result<PublicKey, Box<dyn Error>> {
    Ok(PublicKey::from_slice(&bytes)?)
}

pub fn pb_key_to_addr(pk: &Vec<u8>) -> String {
    let mut address = hashes::ripemd160(&hashes::sha256(&pk).to_vec());
    // prepend 0x00
    address.insert(0, 0u8);
    // append checksum
    address.append(&mut hashes::checksum(&address));
    // encode in base58
    bs58::encode(&address).into_string()
}

pub fn valid_signature(
    secp: &Secp256k1<All>,
    message: &Message,
    signature: &Signature,
    pk_key: &PublicKey,
) -> bool {
    secp.verify_ecdsa(&message, signature, pk_key).is_ok()
}

// NOTE: this is just my standart, the script technically allows for more complex locks
pub fn create_lock(pb_key: &PublicKey) -> String {
    // my address
    let my_addr = pb_key_to_addr(&pb_key.serialize().to_vec());
    // standart lock format
    format!("verify_sig to_addr {} eq", my_addr)
}

// NOTE: this is just my standart, the script technically allows for more complex locks
pub fn create_solution(secp: &Secp256k1<All>, client: &Client, message: &Message) -> String {
    // save public key as hex
    let pb_key_hex = client.pb_key.serialize().to_hex();
    // create signature
    let sig = client.sign(secp, message);
    // save signature as hex
    let sig_hex = sig.serialize_compact().to_hex();

    // standart solution format for the lock
    format!("{} {}", pb_key_hex, sig_hex)
}

/*
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
