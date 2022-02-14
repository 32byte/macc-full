use macc_lib::{blockchain::*, settings::Settings};
use std::sync::{Arc, RwLock};

use super::Config;

pub type Shared<T> = Arc<RwLock<T>>;

pub fn share<T>(data: T) -> Shared<T> {
    Arc::new(RwLock::new(data))
}

#[derive(Clone)]
pub struct Data {
    pub running: Shared<bool>,

    pub settings: Settings,
    pub config: Config,

    // current state of the blockchain
    pub blockchain: Shared<Blockchain>,
    // current state of the store
    // can be technically derived form the blockchain
    pub store: Shared<TxStore>,
    // current state of the difficulty
    // can be technically derived from the blockchain and settings
    pub difficulty: Shared<[u8; 32]>,

    // incoming blocks from outside nodes (not verified)
    // vector of node-addr, block_height, block
    pub i_blocks: Shared<Vec<(String, usize, Block)>>,
    // incoming transactions from outside nodes (not verified)
    pub i_transactions: Shared<Vec<Transaction>>,

    // transactions which are verified but not in the memory yet
    pub mem_transactions: Shared<Vec<Transaction>>,
    // updated store with the values of mem_transactions
    // to prevent having double transactions
    // only for verified that, not an accurate representation
    // of the next state of the store
    pub mem_store: Shared<TxStore>,
}

impl Data {
    pub fn new(
        running: bool,
        settings: Option<Settings>,
        config: Config,
        blockchain: Option<Blockchain>,
        store: Option<TxStore>,
        difficulty: Option<[u8; 32]>,
        i_blocks: Option<Vec<(String, usize, Block)>>,
        i_transactions: Option<Vec<Transaction>>,
    ) -> Self {
        Self {
            running: share(running),
            settings: settings.unwrap_or_default(),
            config,
            blockchain: share(blockchain.unwrap_or_else(Blockchain::new_empty)),
            store: share(store.unwrap_or_else(TxStore::new_empty)),
            difficulty: share(if let Some(diff) = difficulty {
                diff
            } else {
                settings.unwrap_or_default().start_difficulty
            }),
            i_blocks: share(i_blocks.unwrap_or_default()),
            i_transactions: share(i_transactions.unwrap_or_default()),

            // NICE-TO-HAVE: maybe serialize this?
            mem_transactions: share(Vec::new()),
            mem_store: share(TxStore::new_empty()),
        }
    }

    pub fn save(&self, path: &str) -> Option<()> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.extend(
            &(bincode::serialize(&(
                (*self.blockchain.read().ok()?).clone(),
                (*self.store.read().ok()?).clone(),
                (*self.difficulty.read().ok()?).clone(),
            ))
            .ok()?),
        );

        std::fs::write(path, bytes).ok()?;

        Some(())
    }

    pub fn from_file(path: &str, config: Config) -> Option<Self> {
        let bytes = std::fs::read(path).ok()?;

        let de: (Blockchain, TxStore, [u8; 32]) = bincode::deserialize(&bytes).ok()?;

        Some(Self::new(
            true,
            None,
            config,
            Some(de.0),
            Some(de.1),
            Some(de.2),
            None,
            None,
        ))
    }

    // NICE-TO-HAVE: helper functions for reading / writing since its ugly
}
