use std::collections::HashMap;
use std::convert::TryInto;

use secp256k1::bitcoin_hashes::hex::{FromHex, ToHex};
use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct TxStore(pub HashMap<[u8; 32], HashMap<usize, UTXO>>);

impl TxStore {
    pub fn new() -> Self {
        TxStore { 0: HashMap::new() }
    }
}

impl Serialize for TxStore {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.0.len()))?;

        for (k, v) in &self.0 {
            map.serialize_entry(&k.to_hex(), v)?;
        }

        map.end()
    }
}

// scuffed TxStore deserialization
// https://serde.rs/impl-deserialize.html
use serde::de::{MapAccess, Visitor};
use std::marker::PhantomData;

use super::transaction::UTXO;

pub struct TxStoreVisitor {
    marker: PhantomData<fn() -> TxStore>,
}

impl TxStoreVisitor {
    pub fn new() -> Self {
        TxStoreVisitor {
            marker: PhantomData,
        }
    }
}

impl<'de> Visitor<'de> for TxStoreVisitor {
    type Value = TxStore;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("expecting a TxStore!")
    }

    fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut map: HashMap<[u8; 32], HashMap<usize, UTXO>> = HashMap::new();

        while let Some((key, value)) = access.next_entry()? {
            let k: String = key;

            map.insert(Vec::from_hex(&k).unwrap().try_into().unwrap(), value);
        }

        Ok(TxStore { 0: map })
    }
}

impl<'de> Deserialize<'de> for TxStore {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(TxStoreVisitor::new())
    }
}
