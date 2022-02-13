extern crate macc_lib;

#[cfg(test)]
mod tests {
    use std::error::Error;

    use macc_lib::{ecdsa::*, script, hex::ToHex};

    #[test]
    fn test_ecdsa_with_script() -> Result<(), Box<dyn Error>> {
        // test ecdsa
        let secp = create_secp();
        let mut rng = create_rng()?;

        let client = Client::new_random(&secp, &mut rng);

        let message = msg_from_str("Hello, World!");

        let sig = client.sign(&secp, &message);

        let msg = msg_from_bytes(message.as_ref())?;

        assert!(valid_signature(&secp, &msg, &sig, &client.pb_key));

        // test script
        let lock = create_lock(&client.pb_key);

        let solution = create_solution(&secp, &client, &message);

        let stack = script::eval(format!(
            "{} {} {}",
            solution,
            message.as_ref().to_hex(),
            lock
        ));
        
        let valid = stack.is_some();

        assert!(valid);

        Ok(())
    }
}
