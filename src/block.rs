use std::time::{UNIX_EPOCH, SystemTime};
use sha2::{Sha256, Digest};

use crate::transaction::{self, Transaction};

pub struct Block {
    pub index: u64,
    pub previous_hash: String,
    pub timestamp: u128,
    pub transactions: Vec<Transaction>,
    pub nonce: u64,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, previous_hash: String, transactions: Vec<Transaction>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time error")
            .as_millis();

        let mut block = Block {
            index,
            previous_hash,
            timestamp,
            transactions,
            nonce: 0,
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!(
            "{}{}{:?}{}",
            self.index, self.previous_hash, self.transactions, self.nonce
        ));
        format!("{:?}", hasher.finalize())
    }

    pub fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        while !self.hash.starts_with(&target) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
    }
}