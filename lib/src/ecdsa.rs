use bitcoin_hashes::hex::FromHex;
use bitcoin_hashes::sha256;
use rand::rngs::OsRng;
use secp256k1::ecdsa::Signature;
use secp256k1::rand;
use secp256k1::{All, Message, PublicKey, Secp256k1, SecretKey};
use serde::Serialize;
use serde::ser::SerializeStruct;
use std::error::Error;
use serde::de::{self, Deserialize, Deserializer, Visitor, SeqAccess, MapAccess};
use std::fmt;

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
#[derive(Clone, Debug)]
pub struct Client {
    pub pb_key: PublicKey,
    pub sk_key: SecretKey,

    nonce: u128,
}

impl Serialize for Client {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
            S: serde::Serializer {
        let mut state = serializer.serialize_struct("Client", 3)?;

        state.serialize_field("sk_key", &self.sk_key.serialize_secret().to_hex())?;
        state.serialize_field("pb_key", &self.pb_key.to_hex())?;
        state.serialize_field("nonce", &self.nonce)?;
        
        state.end()
    }
}

impl<'de> Deserialize<'de> for Client {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[allow(non_camel_case_types)]
        enum Field { Sk_Key, Pb_Key, Nonce }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`pb_key` or `sk_key` or `nonce`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "pb_key" => Ok(Field::Pb_Key),
                            "sk_key" => Ok(Field::Sk_Key),
                            "nonce" => Ok(Field::Nonce),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct ClientVisitor;

        impl<'de> Visitor<'de> for ClientVisitor {
            type Value = Client;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Client")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Client, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let sk_key = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let pb_key = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let nonce = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                Ok(Client::new(sk_key, pb_key, nonce).expect("sk_key or pb_key are wrong!"))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Client, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut sk_key = None;
                let mut pb_key = None;
                let mut nonce = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Sk_Key => {
                            if sk_key.is_some() {
                                return Err(de::Error::duplicate_field("sk_key"));
                            }
                            sk_key = Some(map.next_value()?);
                        }
                        Field::Pb_Key => {
                            if pb_key.is_some() {
                                return Err(de::Error::duplicate_field("pb_key"));
                            }
                            pb_key = Some(map.next_value()?);
                        }
                        Field::Nonce => {
                            if nonce.is_some() {
                                return Err(de::Error::duplicate_field("nonce"));
                            }
                            nonce = Some(map.next_value()?);
                        }
                    }
                }
                let sk_key = sk_key.ok_or_else(|| de::Error::missing_field("sk_key"))?;
                let pb_key = pb_key.ok_or_else(|| de::Error::missing_field("pb_key"))?;
                let nonce = nonce.ok_or_else(|| de::Error::missing_field("nonce"))?;
                Ok(Client::new(sk_key, pb_key, nonce).expect("sk_key or pb_key are wrong!"))
            }
        }

        const FIELDS: &'static [&'static str] = &["sk_key", "pb_key", "nonce"];
        deserializer.deserialize_struct("Client", FIELDS, ClientVisitor)
    }
}

impl Client {
    pub fn new(sk_key: String, pb_key: String, nonce: u128) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            sk_key: sk_key_from_bytes(&Vec::from_hex(&sk_key)?)?,
            pb_key: pb_key_from_bytes(&Vec::from_hex(&pb_key)?)?,
            nonce,
        })
    }

    pub fn new_random(secp: &Secp256k1<All>, rng: &mut OsRng) -> Self {
        let (sk_key, pk_key) = secp.generate_keypair(rng);

        Self {
            sk_key,
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
            vin,

            vout,
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
    Ok(PublicKey::from_slice(bytes)?)
}

pub fn sk_key_from_bytes(bytes: &[u8]) -> Result<SecretKey, Box<dyn Error>> {
    Ok(SecretKey::from_slice(bytes)?)
}

pub fn pb_key_to_addr(pk: &[u8]) -> String {
    let mut address = hashes::ripemd160(&hashes::sha256(pk).to_vec());
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
    secp.verify_ecdsa(message, signature, pk_key).is_ok()
}

// NOTE: this is just my standart, the script technically allows for more complex locks
pub fn create_lock(pb_key: &PublicKey) -> String {
    // my address
    let my_addr = pb_key_to_addr(&pb_key.serialize().to_vec());
    // standart lock format
    format!("verify_sig to_addr {} eq", my_addr)
}

pub fn create_lock_with_addr(address: &String) -> String {
    // standart lock format
    format!("verify_sig to_addr {} eq", address)
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
