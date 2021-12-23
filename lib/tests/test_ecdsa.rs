extern crate macc_lib;
extern crate secp256k1;

#[cfg(test)]
mod tests {
    use macc_lib::{blockchain::transaction::UTXOU, ecdsa};
    use secp256k1::{bitcoin_hashes::hex::ToHex, rand::rngs::OsRng, PublicKey, SecretKey};

    #[test]
    fn test_with_script() {
        let utxou = UTXOU::new([0; 32], 0, String::new());
        let secp = secp256k1::Secp256k1::new();
        let mut rng = OsRng::new().unwrap();

        let secret: SecretKey = SecretKey::new(&mut rng);
        let public: PublicKey = PublicKey::from_secret_key(&secp, &secret);

        let addr = ecdsa::pk_to_address(&public.serialize().to_vec());
        let lock = macc_lib::ecdsa::create_lock(&addr);
        let solution = ecdsa::create_solution(&secp, secret, &utxou);

        assert!(
            macc_lib::script::eval(format!("{} {} {}", solution, utxou.hash().to_hex(), lock))
                .is_some()
        );
    }
}
