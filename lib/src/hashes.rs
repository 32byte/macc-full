use std::error::Error;

use bitcoin_hashes::{sha256, Hash};

pub fn sha256(data: &Vec<u8>) -> Result<[u8; 32], Box<dyn Error>> {
    Ok(sha256::Hash::hash(data).as_ref().try_into()?)
}

/*
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
 */
