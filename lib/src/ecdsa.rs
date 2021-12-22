use crate::hashes;

use secp256k1::bitcoin_hashes::hex;
use secp256k1::bitcoin_hashes::hex::{FromHex, ToHex};
use secp256k1::{All, PublicKey, Secp256k1, SecretKey};

pub trait ToSecretKey {
    fn to_secret_key(&self) -> Result<SecretKey, Result<hex::Error, secp256k1::Error>>;
}

impl ToSecretKey for String {
    fn to_secret_key(&self) -> Result<SecretKey, Result<hex::Error, secp256k1::Error>> {
        let key = match Vec::from_hex(self) {
            Ok(x) => match SecretKey::from_slice(&x) {
                Ok(x) => x,
                Err(e) => return Err(Err(e)),
            },
            Err(e) => return Err(Ok(e)),
        };

        Ok(key)
    }
}

pub trait ToPublicKey {
    fn to_public_key(&self, secp: &Secp256k1<All>) -> PublicKey;
}

impl ToPublicKey for SecretKey {
    fn to_public_key(&self, secp: &Secp256k1<All>) -> PublicKey {
        PublicKey::from_secret_key(secp, self)
    }
}

pub trait ToHexString {
    fn to_hex_string(&self) -> String;
}

impl ToHexString for SecretKey {
    fn to_hex_string(&self) -> String {
        self.as_ref().to_hex()
    }
}

pub trait WifFormat {
    fn as_wif(&self) -> Result<String, hex::Error>;
    fn from_wif(&self) -> Result<String, bs58::decode::Error>;
}

impl WifFormat for String {
    fn as_wif(&self) -> Result<String, hex::Error> {
        let mut wif = match Vec::from_hex(&("80".to_string() + self)) {
            Ok(x) => x,
            Err(e) => return Err(e),
        };

        let mut checksum = hashes::checksum(&wif);

        wif.append(&mut checksum);

        Ok(bs58::encode(wif).into_string())
    }

    fn from_wif(&self) -> Result<String, bs58::decode::Error> {
        let byte_data = match bs58::decode(self).into_vec() {
            Ok(x) => x,
            Err(e) => return Err(e),
        };

        let hex_string = byte_data.to_hex();

        Ok(hex_string[2..hex_string.len() - 8].to_string())
    }
}

pub trait AsPublicAddress {
    fn as_address(&self) -> Result<String, hex::Error>;
}

impl AsPublicAddress for String {
    fn as_address(&self) -> Result<String, hex::Error> {
        let self_bytes = match Vec::from_hex(self) {
            Ok(x) => x,
            Err(e) => return Err(e),
        };

        let mut address = hashes::ripemd160(&hashes::sha256(&self_bytes));

        // prepend 0x00
        address.insert(0, 0u8);

        address.append(&mut hashes::checksum(&address));

        Ok(bs58::encode(&address).into_string())
    }
}

impl AsPublicAddress for PublicKey {
    fn as_address(&self) -> Result<String, hex::Error> {
        self.serialize().to_hex().as_address()
    }
}
