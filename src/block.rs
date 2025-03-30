use secp256k1::PublicKey;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use crate::transaction::Transaction;

#[derive(Serialize, Deserialize)]
pub struct Block {
    pub nonce: u64,
    pub prev_hash: [u8; 32],
    pub txns: Vec<Transaction>,
    pub txns_hash: [u8; 32],
    pub miner_address: PublicKey,
    pub hash: [u8; 32],
    pub timestamp: u64,
}

impl Block {
    // New Block
    pub fn new(prev_hash: [u8; 32], txns_hash: [u8; 32], miner_address: PublicKey) -> Self {
        let timestamp = chrono::Utc::now().timestamp() as u64;
        let mut block = Block {
            nonce: 0,
            prev_hash,
            txns: vec![],
            txns_hash,
            miner_address,
            hash: [0u8; 32],
            timestamp,
        };
        block.hash = block.calculate_hash();
        block
    }

    // Calculate the hash for new block
    pub fn calculate_hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(self.prev_hash);
        hasher.update(self.txns_hash);
        hasher.update(self.miner_address.serialize());
        hasher.update(self.nonce.to_le_bytes());
        hasher.update(self.timestamp.to_le_bytes());

        let hash_result = hasher.finalize();
        let mut hash_bytes = [0u8; 32];
        hash_bytes.copy_from_slice(&hash_result);
        hash_bytes
    }

    // Mine Block with a certain difficulty
    pub fn mine_block(&mut self, difficulty: usize) {
        let target_prefix = "0".repeat(difficulty);
        loop {
            self.hash = self.calculate_hash();
            if hex::encode(self.hash).starts_with(&target_prefix) {
                println!("✅ Block mined! Nonce: {}", self.nonce);
                break;
            }
            self.nonce += 1;
        }
    }

    // New Block Validation
    pub fn is_valid_proof(&self, difficulty: usize) -> bool {
        let target_prefix = "0".repeat(difficulty);
        hex::encode(self.hash).starts_with(&target_prefix)
    }

    // Block -> Hash
    pub fn serialize(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("Serialization failed")
    }
    
    // Hash -> Block
    pub fn deserialize(data: &[u8]) -> Self {
        serde_json::from_slice(data).expect("Deserialization failed")
    }
}