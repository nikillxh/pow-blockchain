use secp256k1::{ecdsa::Signature, PublicKey};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use crate::wallet::Wallet;

#[derive(Serialize, Deserialize)]
pub struct ECoin {
    pub curr_owner: PublicKey,
    pub prev_tx: [u8; 32],
    pub hash: [u8; 32],
    pub signature: Signature,
    pub value: u64,
}

impl ECoin {
    pub fn hash_coin(curr_owner: PublicKey, prev_tx: [u8; 32]) -> [u8; 32] {
        // Hash (curr_owner + prev_tx)
        let mut hasher = Sha256::new();
        hasher.update(curr_owner.serialize());
        hasher.update(prev_tx);
        let hash_result = hasher.finalize();

        // Convert hash to [u8; 32]
        let mut hash_bytes = [0u8; 32];
        hash_bytes.copy_from_slice(&hash_result);

        hash_bytes
    }

    pub fn sign_coin(prev_owner: &Wallet, hash: [u8;32]) -> Signature {
        let signature = prev_owner.sign_hash(hash);
        signature
    }

    pub fn coinbase(miner: &Wallet, id: u64) -> Self {
        let prev_tx = [0u8; 32];

        let hash = Self::hash_coin(miner.public_key, prev_tx);
        let signature = Self::sign_coin(miner, hash);

        let reward = 50 / id;

        ECoin {
            curr_owner: miner.public_key,
            prev_tx,
            hash,
            signature,
            value: reward,
        }
    }
}