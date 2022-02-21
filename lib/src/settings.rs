use crate::blockchain::difficulty;

#[derive(Debug, Clone, Copy)]
pub struct Settings {
    // difficulty settings
    pub target_time: u64,
    pub adjustment_interval: u32,
    pub precision: u32,

    // mining settings
    pub halvings_interval: usize,
    pub start_mining_reward: u128,

    // general settings
    pub crypto_precision: u32,
    pub start_difficulty: [u8; 32],
}

impl Settings {
    pub fn new(
        target_time: u64,
        adjustment_interval: u32,
        precision: u32,
        halvings_interval: usize,
        start_mining_reward: u128,
        crypto_precision: u32,
        start_difficulty: [u8; 32],
    ) -> Self {
        Self {
            target_time,
            adjustment_interval,
            precision,

            halvings_interval,
            start_mining_reward,

            crypto_precision,
            start_difficulty,
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        let crypto_precision: u32 = 3;

        Self {
            target_time: 2,
            adjustment_interval: 30,
            precision: 5,

            halvings_interval: 43_200,
            start_mining_reward: 3000_u128 * 10_u32.pow(crypto_precision) as u128,

            crypto_precision,
            start_difficulty: difficulty::create(1).expect("UNREACHABLE"),
        }
    }
}

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
