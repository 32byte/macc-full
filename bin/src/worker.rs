use log::{debug, error, info};
use macc_lib::{
    blockchain::{difficulty, utils, Block, Blockchain, Transaction, TxStore},
    ecdsa,
    utils::current_time,
};

use crate::types::{share, Shared};

use super::types::Data;

fn request_blockchain(_node: &str) -> Option<(Blockchain, TxStore, [u8; 32])> {
    // TODO: implement this
    // request blockchain
    // check if blockchain is valid
    None
}

fn process_blocks(data: &Data) -> bool {
    // clone the blocks which are to process
    let i_blocks = data
        .i_blocks
        .read()
        .expect("Couldn't lock i_blocks for reading!")
        .clone();
    let blocks_to_process = i_blocks.len();

    if blocks_to_process == 0 {
        debug!("No blocks to process");
        return false;
    }
    info!("Processing {} new blocks!", blocks_to_process);

    // clone the current blockchain
    let mut blockchain = data
        .blockchain
        .read()
        .expect("Couldn't lock blockchain for reading!")
        .clone();
    // clone the current store
    let mut store = data
        .store
        .read()
        .expect("Couldn't lock blockchain for reading!")
        .clone();
    // clone the current difficulty
    let mut difficulty = *data
        .difficulty
        .read()
        .expect("Couldn't lock blockchain for reading!");

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
            if let Some((bc, st, di)) = request_blockchain(&node) {
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
            blockchain.add(&mut store, block);
            difficulty = diff;
            modified = true;
        }
    }

    if modified {
        debug!("New blocks updated the state, updating it!");
        // drain the incoming blocks
        data.i_blocks
            .write()
            .expect("Couldn't lock blockchain for writing")
            .drain(0..blocks_to_process);
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

    modified
}

fn proces_transactions(data: &Data) -> bool {
    // store state
    let i_transactions = data
        .i_transactions
        .read()
        .expect("Couldn't lock i_transactions for reading!")
        .clone();
    let i_transactions_len = i_transactions.len();
    if i_transactions_len == 0 {
        debug!("No transactions to process!");
        return false;
    }
    info!("Processing {} transactions!", i_transactions_len);

    let mut mem_transactions = data
        .mem_transactions
        .read()
        .expect("Couldn't lock mem_transactions for reading!")
        .clone();
    let mut mem_store = data
        .mem_store
        .read()
        .expect("Couldn't lock mem_store for reading!")
        .clone();
    let mut modified = false;

    // process all transaction
    for tx in i_transactions {
        // check if transaction is valid
        if utils::is_valid_tx(&tx, &mem_store) {
            debug!("New valid transaction found!");
            // add transaction to the mem store
            utils::add_tx_to_store(&tx, &mut mem_store);
            // add transaction the the mem transaction
            mem_transactions.push(tx);

            modified = true;
        } else {
            debug!("Invalid transaction found!")
        }
    }

    // update state
    if modified {
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
    }
    modified
}

fn prepare_transactions(data: &Data, transactions: &mut Vec<Transaction>) {
    // get block height
    let block_height = data
        .blockchain
        .read()
        .expect("Couldn't lock blockchain for reading!")
        .height();
    // calculate reward
    let reward = utils::calculate_mining_reward(block_height, &data.settings);
    // create lock
    let lock = ecdsa::create_lock_with_addr(&data.config.address);
    // create coinbase transaction
    let coinbase_tx = Transaction::new_coinbase(block_height, reward, lock);

    // add coinbase transaction
    transactions.push(coinbase_tx);
}

fn add_block(data: &Data, block: Block) {
    let block_height = data
        .blockchain
        .read()
        .expect("Couldn't lock blockchain for reading!")
        .height();
    debug!("Adding new block from miner!");

    // TODO: localhost:port
    data.i_blocks
        .write()
        .expect("Couldn't lock i_blocks for writing!")
        .push(("".to_string(), block_height + 1, block));
}

async fn miner_thread(
    data: Data,
    mining_data: (Shared<bool>, Shared<Option<Block>>),
    difficulty: [u8; 32],
    previous: [u8; 32],
    mut transactions: Vec<Transaction>,
) {
    *mining_data
        .0
        .write()
        .expect("Couldn't lock mining_data.0 for writing!") = true;

    let mut running = *data
        .running
        .read()
        .expect("Couldn't read running for reading!");
    

    // setup for coinbase transactions
    prepare_transactions(&data, &mut transactions);

    // nonce to bruteforce
    // TODO: start at a random position
    let mut nonce = 0_u128;
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
        if !*mining_data
            .0
            .read()
            .expect("Couldn't lock mining_data.0 for reading!")
        {
            // stop miner
            debug!("Miner was stopped externally!");
            return;
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
            return;
        }

        // update running
        running = *data
            .running
            .read()
            .expect("Couldn't read running for reading!");
    }
    // block was found

    // set nonce
    block.nonce = nonce;
    // set block
    *mining_data
        .1
        .write()
        .expect("Couldn't lock mining_data.1 for writing!") = Some(block);
    // set running = false for miner
    *mining_data
        .0
        .write()
        .expect("Couldn't lock mining_data.0 for writing!") = false;
}

fn handle_mining(
    data: &Data,
    mining_data: &(Shared<bool>, Shared<Option<Block>>),
    state_modified: bool,
) {
    let block = (*mining_data
        .1
        .read()
        .expect("Couldn't lock mining_data.1 for writing!"))
    .clone();

    // the state was modified
    if state_modified {
        info!("The state was modified, miner restarting!");
        // restart miner
        *mining_data
            .0
            .write()
            .expect("Couldn't lock mining_data.0 for writing!") = false;
    }

    // check if miner finished
    if let Some(b) = block {
        info!("Miner has found a new block with the nonce: {}!", b.nonce);
        // push block as incoming
        add_block(data, b);
        // clear block the miner found
        *mining_data
            .0
            .write()
            .expect("Couldn't lock mining_data.0 for writing!") = false;
        *mining_data
            .1
            .write()
            .expect("Couldn't lock mining_data.1 for writing!") = None;
        // you don't want to instantly start miner after he found a block
        // since you need to first wait until the block is added to the blockchain
        return;
    }

    let miner_running = *mining_data
        .0
        .read()
        .expect("Couldn't lock mining_data.0 for writing!");

    // check if miner is running
    if !miner_running {
        // store state
        // clone the current blockchain
        let blockchain = data
            .blockchain
            .read()
            .expect("Couldn't lock blockchain for reading!")
            .clone();
        let difficulty = *data
            .difficulty
            .read()
            .expect("Couldn't lock difficulty for reading!");
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
        let transactions = data
            .mem_transactions
            .read()
            .expect("Couldn't lock difficulty for reading!")
            .clone();

        info!("Starting miner at block_height={}!", blockchain.height());
        // start miner
        tokio::spawn(miner_thread(
            data.clone(),
            mining_data.clone(),
            difficulty,
            previous,
            transactions,
        ));
    }
}

pub async fn start(data: Data) {
    let mut running = *data
        .running
        .read()
        .expect("Couldn't read running for reading!");
    let mining_data: (Shared<bool>, Shared<Option<Block>>) = (share(false), share(None));

    info!("Starting worker thread!");

    while running {
        let state_modified = process_blocks(&data);

        let _ = proces_transactions(&data);

        handle_mining(&data, &mining_data, state_modified);

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // update running
        running = *data
            .running
            .read()
            .expect("Couldn't read running for reading!");
    }

    // shutdown requested
    info!("Shutting down worker thread!");
}
