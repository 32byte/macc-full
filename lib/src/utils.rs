use std::error::Error;
use std::time::SystemTime;

use num::BigUint;

use crate::blockchain::difficulty;
use crate::blockchain::Block;

// returns current unix time
pub fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

// bruteforces a nonce until the difficulty is satisfied
pub fn find_nonce(block: &Block, difficulty: &[u8; 32]) -> Result<u128, Box<dyn Error>> {
    let mut nonce = 0;

    while !difficulty::satisfies(difficulty, &block.hash(Some(nonce))?) {
        nonce += 1;
    }

    Ok(nonce)
}

pub fn arr_to_bi(arr: &[u8; 32]) -> BigUint {
    BigUint::from_bytes_be(arr)
}

// convert BigUInt back to [u8; 32]
pub fn bi_to_arr(bi: &BigUint) -> [u8; 32] {
    let mut diff_bytes = bi.to_bytes_be();
    let mut arr = vec![0x00_u8; 32 - diff_bytes.len()];
    arr.append(&mut diff_bytes);
    // unreachable panic since the created vector has the lenght of 32
    arr.try_into().expect("UNREACHABLE!")
}
