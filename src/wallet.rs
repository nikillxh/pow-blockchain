use secp256k1::{rand, Keypair, Message, PublicKey, Secp256k1, SecretKey};
use sha2::{Digest, Sha256};
use serde::{Deserialize, Serialize};

// Implemented using the ECC, can also be implemeted using RSA
// Private and Public Key of the Wallet
#[derive(Debug, Deserialize, Serialize)]
pub enum WalletType {
    User,
    Miner,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Wallet {
    private_key: SecretKey,
    pub public_key: PublicKey,
    pub identity: WalletType,
}

impl Wallet {
    pub fn new(identity: WalletType) -> Self {
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut rand::thread_rng());
        Wallet {
            private_key: secret_key,
            public_key,
            identity,
        }
    }

    pub fn get_address(&self) -> String {
        format!("{}", self.public_key)
    }

    pub fn sign_hash(&self, hash: [u8; 32]) -> secp256k1::ecdsa::Signature {
        let secp = Secp256k1::new();
        
        // Use from_digest_slice() instead of from_slice()
        let msg = Message::from_digest_slice(&hash).expect("Message must be 32 bytes");
        
        secp.sign_ecdsa(&msg, &self.private_key)
    }
}