use secp256k1::{rand, Keypair, PublicKey, Secp256k1, SecretKey};

// Implemented using the ECC, can also be implemeted using RSA
// Private and Public Key of the Wallet
#[derive(Debug)]
struct Wallet {
    private_key: SecretKey,
    public_key: PublicKey,
}

impl Wallet {
    pub fn new() -> Self {
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut rand::thread_rng());
        Wallet {
            private_key: secret_key,
            public_key,
        }
    }

    pub fn get_address(&self) -> String {
        format!("{}", self.public_key)
    }
}