use secp256k1::bitcoin_hashes::{ripemd160, sha256, Hash};

pub fn sha256(data: &Vec<u8>) -> Vec<u8> {
    sha256::Hash::hash(data).to_vec()
}

pub fn ripemd160(data: &Vec<u8>) -> Vec<u8> {
    ripemd160::Hash::hash(data).to_vec()
}

// Will return the first 4 bytes of the double hash in hex format
pub fn checksum(data: &Vec<u8>) -> Vec<u8> {
    // hash 1
    let hash = sha256(data);

    // hash 2
    let hash = sha256(&hash);

    hash[0..4].to_vec()
}
