// general settings

pub static CC_PRECISION: u32 = 3;

// mining settings
pub static BLOCK_TX_LIMIT: usize = 1000;

pub static START_MINING_REWARD: u128 = 100 * 10_i32.pow(CC_PRECISION) as u128;
pub static HALVINGS_INTERVAL: usize = 10_000;

// difficulty settings
pub static BLOCK_TIME_TARGET: u64 = 2; // in seconds
pub static DIFFICULTY_ADJUSTMENT_INTERVAL: usize = 30;
pub static DIFFICULTY_ADJUSTMENT_PRECISION: u32 = 5;
