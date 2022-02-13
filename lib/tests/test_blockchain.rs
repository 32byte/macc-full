extern crate macc_lib;

#[cfg(test)]
mod tests {
    use std::error::Error;

    use macc_lib::blockchain::*;
    use macc_lib::ecdsa::*;
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


        // setup for ecdsa & script testing
        let secp = create_secp();
        let mut rng = create_rng()?;

        let mut miner_client = Client::new_random(&secp, &mut rng);
        let my_client = Client::new_random(&secp, &mut rng);

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
        let lock = create_lock(&miner_client.pb_key);
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
        // let lock = create_lock(&miner_client.pb_key);
        // let cb_tx = Transaction::new_coinbase(0, reward, lock);
        // add coinbase transaction to the block
        // b.transactions.push(cb_tx);

        // create our own transaction in which the miner sends
        // half of his previous mining reward to our address
        // and the other half back to himself
        // create input
        let input = vec![(cb_hash, 0)];
        // create output
        let output = vec![
            // send half to my_client
            (reward / 2, my_client.pb_key),
            // send other half back to himself
            (reward - (reward / 2), miner_client.pb_key)
        ];
        // create transaciton
        let tx = miner_client.create_transaction(
            &secp,
            input,
            output
        ).expect("Couldn't create the transaction!");
        // the code below would do this manually
        // create the "message" which is the hash of the utxo we want to unlock
        // let message_str = utils::hash_utxou((&cb_hash, &0))?.to_hex();
        // create the message object
        // let message = msg_from_str(&message_str);
        // create the solution
        // let miner_solution = create_solution(&secp, &miner_client, &message);
        // create new lock (using the address of 'my_client')
        // let my_client_lock = create_lock(&my_client.pb_key);
        // create new lock (using the address of 'miner_client')
        // let miner_client_lock = create_lock(&miner_client.pb_key);
        // the transaction to my_client
        // let tx = Transaction {
            // make sure this is always a new number
            // NOTE: actually it might be ok to reuse a number
            //       since the hash will be always different anyway
            //       because the utxou is always different
            // nonce: 0,

            // use the first utxo of the coinbase transaction
            // from the last block
            // vin: vec![(cb_hash, 0, miner_solution)],
            // send the amount = reward to my account
            // vout: vec![(reward / 2, my_client_lock), (reward - (reward / 2), miner_client_lock)]
        // };
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

        println!("{:?}", store);

        Ok(())
    }
}
