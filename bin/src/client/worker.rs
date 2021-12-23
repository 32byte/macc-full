use std::collections::HashMap;

use super::{config::Config, data::Data, miner, netio::NetIOModule, node::Module};
use macc_lib::blockchain::{
    block::Block,
    blockchain::{BlockChainMethods, Blockchain},
    helper::SharedData,
    mempool::MemPool,
    txstore::TxStore, transaction::Transaction,
};
use rocket::tokio::{self, sync::RwLock, task::JoinHandle};

pub struct WorkerModule;

impl WorkerModule {
    async fn start_miner(
        config: &Config,
        data: &SharedData<Data>,
        miner_running: &SharedData<bool>,
        blockchain: &Blockchain,
        tx_store: &TxStore,
    ) {
        log::info!("Starting miner!");
        *miner_running.write().await = true;

        // mine new block
        let mut new_txs: MemPool = data.read().await.new_transactions.clone();
        let mut used_tx_store: TxStore = TxStore::new();

        // collect valid transactions
        for tx in &new_txs.transactions.keys().cloned().collect::<Vec<Transaction>>() {
            if !tx.verify(tx_store, &mut used_tx_store) {
                log::warn!("Removing invalid transaction!");
                new_txs.remove_tx(tx);
            }
        }
        data.write().await.new_transactions = new_txs.clone();

        let to_mine =
            miner::setup_mining_block(config, &blockchain, &tx_store, new_txs.sorted_txs(tx_store));

        tokio::spawn(miner::mine_block(
            to_mine,
            miner_running.clone(),
            data.clone(),
        ));
    }

    async fn get_valid_blocks(
        net_io: &NetIOModule,
        new_blocks: &HashMap<String, Blockchain>,
        blockchain: &Blockchain,
        tx_store: &TxStore,
    ) -> (Vec<Block>, Option<TxStore>) {
        let mut verified_blocks: Vec<Block> = Vec::new();
        let mut new_tx_store: Option<TxStore> = None;

        for (node, bc) in new_blocks {
            for block in bc {
                // verify block
                log::debug!("Verifying new blocks!");

                // block is next block
                if block.previous == blockchain.block_at(-1).hash(None) {
                    if block.verify(tx_store, Some(&blockchain.block_at(-1))) {
                        verified_blocks = vec![block.clone()];
                    } else {
                        log::warn!("Received invalid block from {}", node);
                    }
                } else {
                    if let Some(new_bc) = if node == "self" {
                        Some(blockchain.clone())
                    } else {
                        net_io.get_blockchain(node).await
                    } {
                        if new_bc.len() > blockchain.len() + verified_blocks.len() {
                            if let Some(new_txs) = new_bc.verify() {
                                new_tx_store = Some(new_txs);
                                verified_blocks = new_bc;
                                log::warn!("Received valid blockchain from {}", node);
                            }
                        } else {
                            log::warn!("Received block with a smaller blockchain from {}", node);
                        }
                    } else {
                        log::warn!("Couldn't get blockchain from {}", node);
                    }
                }
            }
        }

        (verified_blocks, new_tx_store)
    }

    async fn add_valid_blocks(
        data: &SharedData<Data>,
        valid_blocks: &Vec<Block>,
        blockchain: &mut Blockchain,
        tx_store: &mut TxStore,
    ) {
        for block in valid_blocks {
            log::warn!("Adding verified blocks!");

            blockchain.add_block(tx_store, block);

            // remove transactions from mempool
            let mut mempool = data.read().await.new_transactions.clone();

            for tx in &block.transactions {
                mempool.remove_tx(tx);
            }

            data.write().await.new_transactions = mempool;
        }
    }

    async fn start(config: Config, data: SharedData<Data>) {
        let miner_running: SharedData<bool> = SharedData::new(RwLock::new(false));

        while data.read().await.running {
            let mut blockchain: Blockchain = data.read().await.blockchain.clone();
            let mut tx_store: TxStore = data.read().await.tx_store.clone();
            let net_io: NetIOModule = data.read().await.net_io.clone();

            // process new blocks

            // get valid blocks
            let new_blocks = data.read().await.new_blocks.clone();
            let (valid_blocks, new_tx_store) =
                Self::get_valid_blocks(&net_io, &new_blocks, &blockchain, &tx_store).await;
            data.write().await.new_blocks.clear();

            // replace current blockchain and tx_store if received a longer chain
            if let Some(txs) = new_tx_store {
                log::warn!("Replacing current blockchain!");
                data.write().await.blockchain = valid_blocks.clone();
                data.write().await.tx_store = txs.clone();

                continue;
            }

            // add valid blocks & update tx_store
            Self::add_valid_blocks(&data, &valid_blocks, &mut blockchain, &mut tx_store).await;

            // update data
            data.write().await.blockchain = blockchain.clone();
            data.write().await.tx_store = tx_store.clone();

            // mine new block

            // stop miner if new blocks were added
            if !valid_blocks.is_empty() {
                *miner_running.write().await = false;
            }

            // if miner not running start it
            if !*miner_running.write().await {
                Self::start_miner(&config, &data, &miner_running, &blockchain, &tx_store).await;
            }
        }

        // stop miner when node stops
        *miner_running.write().await = false;
    }
}

impl Module for WorkerModule {
    fn start_thread(&self, config: &Config, data: SharedData<Data>) -> JoinHandle<()> {
        tokio::spawn(WorkerModule::start(config.clone(), data))
    }
}
