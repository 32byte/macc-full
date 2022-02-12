extern crate macc_lib;

#[cfg(test)]
mod tests {
    use std::error::Error;

    use macc_lib::blockchain::*;
    use macc_lib::utils::*;

    #[test]
    fn test_blockchain_full() -> Result<(), Box<dyn Error>> {
        static TARGET_TIME: u64 = 2; 
        static ADJUSTMENT_INTERVAL: u32 = 30;
        static PRECISION: u32 = 5;
        

        // create blockchain
        let mut bc = Blockchain::new_empty();
        // create store
        let mut store = TxStore::new_empty();
        // create start difficulty
        let mut diff = difficulty::create(1)?;

        // create a block
        let mut b = Block {
            timestamp: current_time(),

            // the first block can have any previous hash
            // it is simply ignored
            previous: [0u8; 32],

            // nonce is not found yet
            nonce: 0,

            // no transactions
            transactions: vec![],
        };

        // find & set nonce for block
        b.nonce = find_nonce(&b, &diff)?;

        // check if block can be added

        // calculate difficulty for the block
        bc.adjust_difficulty(&mut diff, TARGET_TIME, ADJUSTMENT_INTERVAL, PRECISION);
        // check if block can be added
        let can_be_added = bc.valid_next(&b, &store, &diff).unwrap_or(false);
        
        assert!(can_be_added);

        // add block
        
        bc.add(&mut store, b);


        Ok(())
    }
}
