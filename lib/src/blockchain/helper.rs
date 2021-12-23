use std::time::SystemTime;

use crate::settings::{HALVINGS_INTERVAL, START_MINING_REWARD};

#[cfg(feature = "mining")]
use rocket::tokio::sync::RwLock;
#[cfg(feature = "mining")]
use std::sync::Arc;
#[cfg(feature = "mining")]
pub type SharedData<T> = Arc<RwLock<T>>;

pub fn current_unix_time() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn calculate_mining_reward(block_height: usize) -> u128 {
    // based on formula reward = floor( start_reward / 2^( floor(block_height / halving_interval) ) )
    // https://www.desmos.com/calculator

    let num_halvings = (block_height / HALVINGS_INTERVAL) as u32;

    // No need to calculate 2^x with x >= 7
    if num_halvings >= 7 {
        return 0;
    }

    START_MINING_REWARD / 2_i32.pow(num_halvings) as u128
}
