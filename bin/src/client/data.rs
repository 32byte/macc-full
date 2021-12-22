use std::collections::HashMap;

use super::{config::Config, netio::NetIOModule};
use macc_lib::blockchain::{
    block::Block,
    blockchain::{BlockChainMethods, Blockchain},
    mempool::MemPool,
    txstore::TxStore,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Data {
    pub running: bool,

    pub blockchain: Blockchain,
    pub tx_store: TxStore,

    pub new_transactions: MemPool,
    pub new_blocks: HashMap<String, Blockchain>,

    pub net_io: NetIOModule,
}

impl Data {
    pub async fn new(config: &Config) -> Self {
        let default_genesis_block = Block::default_genesis();

        let default_data = Data {
            running: false,

            blockchain: vec![default_genesis_block],
            tx_store: TxStore::new(),

            new_transactions: MemPool::new(),
            new_blocks: HashMap::new(),

            net_io: NetIOModule::new(),
        };

        if let Ok(bytes) = std::fs::read(&config.data_file) {
            if let Ok(d) = bincode::deserialize(&bytes) {
                log::info!("Loaded node data from file!");

                if (&d as &Data).blockchain.verify().is_none() {
                    log::error!("Blockchain is invalid!");
                    std::process::exit(1);
                } else {
                    log::info!("Blockchain is valid!");
                }

                d
            } else {
                log::error!("Couldn't deserealize node data!");
                std::process::exit(1);
            }
        } else {
            log::warn!("{} not found!", config.data_file);
            Data {
                running: true,
                blockchain: vec![Block::genesis_mined().await],
                ..default_data
            }
        }
    }

    pub fn save(&self, config: &Config) {
        std::fs::write(
            &config.data_file,
            bincode::serialize(self).expect("Couldn't serealize data!"),
        )
        .expect("Couldn't save data to file!");
        log::info!("Saved data to {}!", config.data_file);
    }
}
