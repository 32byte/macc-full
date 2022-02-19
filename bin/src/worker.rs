use log::{debug, error, info, warn};
use macc_lib::{
    blockchain::{difficulty, utils, Block, Blockchain, Transaction, TxStore},
    ecdsa::{self, create_rng},
    rand::{rngs::OsRng, Rng},
    settings::Settings,
    utils::current_time,
};

use crate::{netio::NetIO, types::Shared};

use super::types::Data;

async fn request_blockchain(
    settings: &Settings,
    net_client: &NetIO,
    node: &str,
) -> Option<(Blockchain, TxStore, [u8; 32])> {
    let bc = net_client.get_blockchain(node).await?;
    let (store, diff) = bc.is_valid(settings)?;

    Some((bc, store, diff))
}

async fn process_blocks(data: &Data, net_client: &NetIO) -> Option<bool> {
    // clone the blocks which are to process
    let i_blocks = data.i_blocks.try_read().ok()?.clone();
    let blocks_to_process = i_blocks.len();

    if blocks_to_process == 0 {
        debug!("No blocks to process");
        return Some(false);
    }
    info!("Processing {} new blocks!", blocks_to_process);

    // clone the current blockchain
    let mut blockchain = data.blockchain.try_read().ok()?.clone();
    // clone the current store
    let mut store = data.store.try_read().ok()?.clone();
    // clone the current difficulty
    let mut difficulty = *data.difficulty.try_read().ok()?;

    let mut modified = false;

    // adjust difficulty to be the one of the next block
    for (node, block_height, block) in i_blocks {
        // the block belongs to a blockchain no longer than ours
        // -> ignore this block
        if block_height <= blockchain.height() {
            debug!("{} has sent a block from a smaller blockchain", node);
            continue;
        }

        // advance the difficulty
        let diff = blockchain.adjust_difficulty(difficulty, &data.settings);

        let block_valid_as_next = blockchain
            .valid_next(&block, &store, &diff, &data.settings)
            .unwrap_or(false);

        // this block belongs to a longer blockchain (at least +2)
        // or this block belongs to a blockchain with lenght + 1 but the blockchain
        // is not identical to this one
        if block_height > blockchain.height() + 1 || !block_valid_as_next {
            debug!(
                "{} has sent a block from a bigger or other blockchain!",
                node
            );
            // request the blockchain from the node
            if let Some((bc, st, di)) = request_blockchain(&data.settings, net_client, &node).await
            {
                info!(
                    "{} has a bigger blockchain, this blockchain will be replaced!",
                    node
                );
                blockchain = bc;
                store = st;
                difficulty = di;

                modified = true;
            }
        } else {
            // the block belongs to a blockchain exactly 1 block longer than our
            // and the block is valid

            info!("{} has sent a new valid block!", node);
            // TODO: add to node trusted nodes
            blockchain.add(&mut store, block);
            difficulty = diff;
            modified = true;
        }
    }

    // drain the incoming blocks
    data.i_blocks
        .write()
        .expect("Couldn't lock blockchain for writing")
        .drain(0..blocks_to_process);

    if modified {
        debug!("New blocks updated the state, updating it!");
        // replace the blockchain
        *data
            .blockchain
            .write()
            .expect("Couldn't lock blockchain for writing") = blockchain;
        // replace the store
        *data.store.write().expect("Couldn't lock store for writing") = store.clone();
        // replace the difficulty
        *data
            .difficulty
            .write()
            .expect("Couldn't lock difficulty for writing") = difficulty;

        // update mem_store
        *data
            .mem_store
            .write()
            .expect("Couldn't lock mem_store for writing") = store;
        // push mem_transactions back to incoming transactions
        // since they need to be reverified
        data.i_transactions
            .write()
            .expect("Couldn't lock i_transactions for writing")
            .append(
                &mut data
                    .mem_transactions
                    .write()
                    .expect("Couldn't lock mem_transactions for writing"),
            );
    }

    Some(modified)
}

fn proces_transactions(data: &Data, net_client: &NetIO) -> Option<()> {
    // store state
    let i_transactions = data.i_transactions.try_read().ok()?.clone();

    let i_transactions_len = i_transactions.len();
    if i_transactions_len == 0 {
        debug!("No transactions to process!");
        return Some(());
    }
    info!("Processing {} transactions!", i_transactions_len);

    let mut mem_transactions = data.mem_transactions.try_read().ok()?.clone();
    let mut mem_store = data.mem_store.try_read().ok()?.clone();

    // process all transaction
    for tx in i_transactions {
        // check if transaction is valid
        if utils::is_valid_tx(&tx, &mem_store) {
            debug!("New valid transaction found!");
            // broadcast transaction
            if net_client.b_transaction(&tx).is_err() {
                error!("Node failed to broadcast transaction!");
            }
            // add transaction to the mem store
            utils::add_tx_to_store(&tx, &mut mem_store);
            // add transaction the the mem transaction
            mem_transactions.push(tx);
        } else {
            debug!("Invalid transaction found!")
        }
    }

    // update state
    debug!("New transactions updated the state, updating it!");
    data.i_transactions
        .write()
        .expect("Couldn't lock i_transactions for writing")
        .drain(0..i_transactions_len);
    *data
        .mem_transactions
        .write()
        .expect("Couldn't lock mem_transactions for writing") = mem_transactions.clone();
    *data
        .mem_store
        .write()
        .expect("Couldn't lock mem_store for writing") = mem_store.clone();

    Some(())
}

fn prepare_transactions(data: &Data, transactions: &mut Vec<Transaction>) -> Option<()> {
    // get block height
    let block_height = data.blockchain.try_read().ok()?.height();
    // calculate reward
    let reward = utils::calculate_mining_reward(block_height, &data.settings);
    // create lock
    let lock = ecdsa::create_lock_with_addr(&data.config.address);
    // create coinbase transaction
    let coinbase_tx = Transaction::new_coinbase(block_height, reward, lock);

    // add coinbase transaction
    transactions.push(coinbase_tx);

    Some(())
}

fn add_block(data: &Data, block: Block) -> Option<()> {
    let block_height = data.blockchain.try_read().ok()?.height();
    debug!("Adding new block from miner!");

    let port = data.config.port;
    let self_addr = format!("http://127.0.0.1:{}", port);

    data.i_blocks
        .write()
        .expect("Couldn't lock i_blocks for writing!")
        .push((self_addr, block_height + 1, block));

    Some(())
}

fn handle_mining(
    data: &Data,
    mining_data: &(
        Shared<Option<([u8; 32], [u8; 32], Vec<Transaction>)>>,
        Shared<Option<Block>>,
    ),
    state_modified: bool,
    net_client: &NetIO,
) -> Option<()> {
    let block = mining_data.1.try_read().ok()?.clone();

    // the state was modified, clear miner
    if state_modified {
        info!("The state was modified, miner restarting!");
        // remove task
        *mining_data
            .0
            .write()
            .expect("Couldn't lock mining_data.0 for writing!") = None;
        // remove block
        *mining_data
            .1
            .write()
            .expect("Couldn't lock mining_data.0 for writing!") = None;
    }

    // check if miner finished
    if let Some(b) = block {
        info!("Miner has found a new block with the nonce: {}!", b.nonce);
        // broadcast block
        let block_height = data.blockchain.try_read().ok()?.height();
        if net_client.b_block(&b, block_height).is_err() {
            error!("Node failed to broadcast block!")
        }
        // push block as incoming
        add_block(data, b);
        // remove task
        *mining_data
            .0
            .write()
            .expect("Couldn't lock mining_data.0 for writing!") = None;
        // remove block
        *mining_data
            .1
            .write()
            .expect("Couldn't lock mining_data.1 for writing!") = None;
        // you don't want to instantly start miner after he found a block
        // since you need to first wait until the block is added to the blockchain
        return None;
    }

    // check if miner is running
    if !is_miner_running(mining_data)? {
        // store state
        // clone the current blockchain
        let blockchain = data.blockchain.try_read().ok()?.clone();
        let difficulty = *data.difficulty.try_read().ok()?;

        // adjust difficulty for the new block
        let difficulty = blockchain.adjust_difficulty(difficulty, &data.settings);
        // get the previous has (also handle if this will be the first block to be mined)
        let previous = if blockchain.height() == 0 {
            [0x00_u8; 32]
        } else {
            blockchain
                .at(-1)
                .hash(None)
                .expect("Block in the blockchain couldn't be hashed!")
        };
        let transactions = data.mem_transactions.try_read().ok()?.clone();

        info!("Starting miner at block_height={}!", blockchain.height());

        // create task
        let task = Some((difficulty, previous, transactions));

        // set task
        *mining_data
            .0
            .write()
            .expect("Couldn't lock mining_data.0 for writing!") = task;
    }

    None
}

fn is_miner_running(
    mining_data: &(
        Shared<Option<([u8; 32], [u8; 32], Vec<Transaction>)>>,
        Shared<Option<Block>>,
    ),
) -> Option<bool> {
    let has_task = mining_data.0.try_read().ok()?.is_some();

    let has_output = mining_data.1.try_read().ok()?.is_some();

    Some(has_task && !has_output)
}

fn mine(
    data: &Data,
    mining_data: &(
        Shared<Option<([u8; 32], [u8; 32], Vec<Transaction>)>>,
        Shared<Option<Block>>,
    ),
    rng: &mut OsRng,
) -> Option<()> {
    // get task
    let task = mining_data.0.try_read().ok()?.clone();

    // check if task exists
    if task.is_none() {
        debug!("Miner has currently no task!");
        return None;
    }
    let task = task.expect("UNREACHABLE!");

    // read task parameters
    let difficulty = task.0;
    let previous = task.1;
    let mut transactions = task.2;

    let mut running = *data.running.try_read().ok()?;

    // setup for coinbase transactions
    prepare_transactions(&data, &mut transactions);

    // nonce to bruteforce
    let mut nonce = rng.gen::<u128>();
    // setup block
    let mut block = Block {
        // set timestamp
        timestamp: current_time(),
        // set previous hash
        previous,
        // nonce is not found yet
        nonce,
        // transactions to be inlcuded in the block
        transactions,
    };

    while running {
        // check if miner was stopped externally
        if !is_miner_running(mining_data)? {
            // stop miner
            debug!("Miner was stopped externally!");
            return None;
        }

        // bruteforce nonce
        if let Ok(hash) = block.hash(Some(nonce)) {
            if difficulty::satisfies(&difficulty, &hash) {
                // stop bruteforcing
                debug!("Miner found nonce: {}!", nonce);
                break;
            }
            // increment nonce
            nonce += 1;
        } else {
            error!("Something went wront in miner: Couldn't hash block!");
            // something went wrong in the block hash
            return None;
        }

        // update running
        running = *data.running.try_read().ok()?;
    }
    // block was found

    // set nonce
    block.nonce = nonce;
    // set block
    *mining_data
        .1
        .write()
        .expect("Couldn't lock mining_data.1 for writing!") = Some(block);

    None
}

pub async fn start_miner(
    data: Data,
    mining_data: (
        Shared<Option<([u8; 32], [u8; 32], Vec<Transaction>)>>,
        Shared<Option<Block>>,
    ),
) {
    let mut running = if let Some(r) = data.running.try_read().ok() {
        *r
    } else {
        true
    };

    let mut rng = create_rng().expect("Miner could not create a RNG!");

    while running {
        mine(&data, &mining_data, &mut rng);

        std::thread::sleep(std::time::Duration::from_millis(100));

        // update running
        if let Some(r) = data.running.try_read().ok() {
            running = *r;
        } else {
            break;
        };
    }

    // shutdown requested
    info!("Shutting down miner thread!");
}

pub async fn start(
    data: Data,
    mining_data: (
        Shared<Option<([u8; 32], [u8; 32], Vec<Transaction>)>>,
        Shared<Option<Block>>,
    ),
) {
    let mut running = if let Some(r) = data.running.try_read().ok() {
        *r
    } else {
        true
    };

    let net_client = NetIO::new(&data.config);
    // TODO: sync with trusted

    info!("Starting worker thread!");

    while running {
        let state_modified = if let Some(modified) = process_blocks(&data, &net_client).await {
            modified
        } else {
            warn!("process blocks failed to lock something!");
            false
        };

        if proces_transactions(&data, &net_client).is_none() {
            warn!("process transactions failed to lock something!");
        }

        handle_mining(&data, &mining_data, state_modified, &net_client);

        std::thread::sleep(std::time::Duration::from_millis(100));

        // update running
        if let Some(r) = data.running.try_read().ok() {
            running = *r;
        } else {
            break;
        };
    }

    // shutdown requested
    info!("Shutting down worker thread!");
    info!("Saving config..");
    if net_client.save().is_err() {
        error!("The config couldn't be stored!");
    }
}
