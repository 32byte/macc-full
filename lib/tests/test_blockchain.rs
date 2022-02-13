extern crate macc_lib;

#[cfg(test)]
mod tests {
    use std::error::Error;

    use macc_lib::blockchain::*;
    use macc_lib::hex::ToHex;
    use macc_lib::settings::Settings;
    use macc_lib::utils::*;

    #[test]
    fn test_blockchain_full() -> Result<(), Box<dyn Error>> {
        // init settings
        let settings = Settings::default();

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

        // create coinbase transaction
        let reward = utils::calculate_mining_reward(bc.height(), &settings);
        let lock = String::from("miner"); // TODO
        let cb_tx = Transaction::new_coinbase(bc.height(), reward, lock);

        let cb_hash = cb_tx.hash()?;
        // add coinbase transaction to the block
        b.transactions.push(cb_tx);

        // find & set nonce for block
        b.nonce = find_nonce(&b, &diff)?;

        // check if block can be added

        // calculate difficulty for the block
        bc.adjust_difficulty(&mut diff, &settings);
        // check if block can be added
        let can_be_added = bc.valid_next(&b, &store, &diff, &settings).unwrap_or(false);

        assert!(can_be_added);

        // add block

        bc.add(&mut store, b);



        // create new block and send a transaction

        // create the block
        let mut b = Block {
            timestamp: current_time(),

            // hash of the previous transaction
            // NOTE: could also use `b.hash(None)?`
            previous: bc.at(-1).hash(None)?,

            // nonce is not found yet
            nonce: 0,

            // will manually add new transactions
            transactions: vec![],
        };

        // NOTE: technically a coinbase transaction is not needed
        // create coinbase transaction
        // let reward = utils::calculate_mining_reward(bc.height(), &settings);
        // let lock = String::from("miner"); // TODO
        // let cb_tx = Transaction::new_coinbase(0, reward, lock);
        // add coinbase transaction to the block
        // b.transactions.push(cb_tx);

        // create our own transaction
        let tx = Transaction {
            // make sure this is always a new number
            // NOTE: actually it might be ok to reuse a number
            //       since the hash will be always different anyway
            //       because the utxou is always different
            nonce: 0,

            // use the first utxo of the coinbase transaction
            // from the last block
            // TODO: solution
            vin: vec![(cb_hash, 0, "solution".to_string())],
            // send the amount = reward to my account
            // TODO: lock
            vout: vec![(reward, "lock".to_string())]
        };
        let tx_hash = tx.hash()?;

        // add tx to block
        b.transactions.push(tx);

        // find & set nonce for block
        b.nonce = find_nonce(&b, &diff)?;

        // check if block can be added

        // calculate difficulty for the block
        bc.adjust_difficulty(&mut diff, &settings);
        // check if block can be added
        let can_be_added = bc.valid_next(&b, &store, &diff, &settings).unwrap_or(false);

        assert!(can_be_added);

        // add block

        bc.add(&mut store, b);

        println!("Transaction {} added!", tx_hash.to_hex());

        Ok(())
    }
}
