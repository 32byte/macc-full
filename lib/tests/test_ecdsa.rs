extern crate macc_lib;
extern crate secp256k1;

#[cfg(test)]
mod tests {
    use macc_lib::{ecdsa::AsPublicAddress, blockchain::transaction::UTXOU};
    use secp256k1::{SecretKey, PublicKey, bitcoin_hashes::hex::{ToHex}};

    #[test]
    fn test_with_script() {
        let utxou = UTXOU::new([0; 32], 0, String::new());

        let secp = secp256k1::Secp256k1::new();

        let rng = &mut secp256k1::rand::rngs::OsRng::new().unwrap();

        let secret: SecretKey = macc_lib::ecdsa::new_secret_key(rng);
        let public: PublicKey = PublicKey::from_secret_key(&secp, &secret);

        let addr = public.as_address().unwrap();

        let lock = macc_lib::ecdsa::create_lock(&addr);

        let solution = macc_lib::ecdsa::create_solution(secret, &utxou);

        assert!(macc_lib::script::eval(format!("{} {} {}", solution, utxou.hash().to_hex(), lock)).is_some());
    }
}