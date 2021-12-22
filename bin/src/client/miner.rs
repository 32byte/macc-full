use super::{config::Config, data::Data};
use macc_lib::{
    blockchain::{
        block::Block,
        blockchain::{BlockChainMethods, Blockchain},
        difficulty::{Difficulty, DifficultyMethods},
        helper::{calculate_mining_reward, SharedData},
        transaction::Transaction,
        txstore::TxStore,
    },
    settings::BLOCK_TX_LIMIT,
};

fn create_lock(config: &Config) -> String {
    // TODO: update script module and generate lock
    format!("{}", config.address)
}

pub fn setup_mining_block(
    config: &Config,
    blockchain: &Blockchain,
    tx_store: &TxStore,
    valid_txs: Vec<Transaction>,
) -> Block {
    // get transactions for the block
    let mut usuable_tx = valid_txs[0..valid_txs.len().min(BLOCK_TX_LIMIT - 1)].to_vec();
    // calculate the reward = fees + mining_reward
    let total_fees: u128 = usuable_tx.iter().map(|tx| tx.fees(tx_store)).sum();
    // create coinbase transaction
    let coinbase_tx = Transaction::coinbase(
        total_fees + calculate_mining_reward(blockchain.height()),
        create_lock(config),
    );

    // final transactions vector for the block
    let mut transactions = vec![coinbase_tx];
    transactions.append(&mut usuable_tx);

    // calculate difficulty
    let previous_block = blockchain.block_at(-1).clone();
    let difficulty = Difficulty::adjusted(&blockchain, previous_block.difficulty, None);

    Block::new(
        previous_block.hash(None),
        difficulty,
        blockchain.height(),
        transactions,
    )
}

pub async fn mine_block(block: Block, running: SharedData<bool>, data: SharedData<Data>) {
    let nonce = block.mine(running.clone()).await;

    if let Some(nonce) = nonce {
        log::info!("Mined block successfully!");

        let mined_block = Block {
            nonce: nonce,
            ..block
        };

        data.read().await.net_io.broadcast(&mined_block);
        data.write()
            .await
            .new_blocks
            .insert("self".to_string(), vec![mined_block]);
    }
}
