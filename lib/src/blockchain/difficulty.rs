use std::convert::TryInto;

use num::BigUint;

use crate::settings::*;

use super::blockchain::{BlockChainMethods, Blockchain};

pub type Difficulty = [u8; 32];

pub trait DifficultyMethods {
    fn from_zeros(zeros: usize) -> Self;
    fn adjusted(blockchain: &Blockchain, old: Difficulty, block_height: Option<usize>) -> Self;

    fn check(&self, hash: [u8; 32]) -> bool;
}

impl DifficultyMethods for Difficulty {
    fn from_zeros(zeros: usize) -> Self {
        let mut diff = vec![0x00u8; zeros];
        diff.append(&mut vec![0xFFu8; 32 - zeros]);
        diff.try_into().unwrap()
    }

    fn adjusted(blockchain: &Blockchain, old: Difficulty, block_height: Option<usize>) -> Self {
        let block_height = if let Some(height) = block_height {
            height
        } else {
            blockchain.len() - 1
        };

        if block_height < DIFFICULTY_ADJUSTMENT_INTERVAL
            || block_height % DIFFICULTY_ADJUSTMENT_INTERVAL != 0
        {
            return old;
        }

        let interval = blockchain.block_at(block_height as i32).timestamp
            - blockchain
                .block_at((block_height - DIFFICULTY_ADJUSTMENT_INTERVAL) as i32)
                .timestamp;

        let ratio = (BLOCK_TIME_TARGET as f64) * (DIFFICULTY_ADJUSTMENT_INTERVAL as f64)
            / (interval as f64);

        // adjust difficulty

        // convert current difficutly to BitInt so its easier to work with
        let mut diff_bi = BigUint::from_bytes_be(&old);
        // divide first to avoid overflow
        diff_bi /= (ratio * (10u64.pow(DIFFICULTY_ADJUSTMENT_PRECISION) as f64)) as u64;
        diff_bi *= 10u32.pow(DIFFICULTY_ADJUSTMENT_PRECISION);

        let mut diff_bytes: Vec<u8> = diff_bi.to_bytes_be();
        let mut new_diff: Vec<u8> = vec![0x00; 32 - diff_bytes.len()];
        new_diff.append(&mut diff_bytes);

        let difficulty: [u8; 32] = new_diff.try_into().unwrap();
        difficulty
    }

    fn check(&self, hash: [u8; 32]) -> bool {
        for i in 0_usize..32_usize {
            if hash[i] > self[i] {
                return false;
            }
            if self[i] > hash[i] {
                return true;
            }
        }
        true
    }
}
