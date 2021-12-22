use super::{
    block::Block,
    difficulty::{Difficulty, DifficultyMethods},
    txstore::TxStore,
};

pub type Blockchain = Vec<Block>;

pub trait BlockChainMethods {
    fn add_block(&mut self, tx_store: &mut TxStore, block: &Block);

    fn height(&self) -> usize;
    fn block_at(&self, i: i32) -> Block;
    fn last_diffculty(&self) -> [u8; 32];

    fn verify(&self) -> Option<TxStore>;
}

impl BlockChainMethods for Blockchain {
    fn add_block(&mut self, tx_store: &mut TxStore, block: &Block) {
        // store block on the blockchain
        self.push(block.clone());

        // update internal values
        for tx in &block.transactions {
            tx.add(tx_store, block.height);
        }
    }

    fn height(&self) -> usize {
        self.len()
    }

    fn block_at(&self, i: i32) -> Block {
        let idx: usize = if i < 0 {
            self.len() - i.abs() as usize
        } else {
            i as usize
        };
        self[idx].clone()
    }

    fn last_diffculty(&self) -> [u8; 32] {
        self.last().unwrap().difficulty
    }

    fn verify(&self) -> Option<TxStore> {
        let mut tx_store: TxStore = TxStore::new();
        let mut new_blockchain: Blockchain = Blockchain::new();

        let mut last_block: Option<&Block> = None;
        for (block_height, block) in self.into_iter().enumerate() {
            // verify diffculty
            if block_height > 0 {
                // check if difficulty has to be adjusted
                if block.difficulty
                    != Difficulty::adjusted(
                        self,
                        self[block_height - 1].difficulty,
                        Some(block_height - 1),
                    )
                {
                    return None;
                }
            }

            if !block.verify(&tx_store, last_block) {
                log::debug!("Invalid block {}!", block_height);
                return None;
            }
            log::debug!("Valid block {}!", block_height);
            new_blockchain.add_block(&mut tx_store, block);
            last_block = Some(block);
        }

        Some(tx_store)
    }
}
