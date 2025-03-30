use core::hash;

use sha2::{Digest, Sha256};

use crate::block::Block;

pub struct Blockchain {
    pub genesis: Block, // Store Genesis Block
    pub latest: Option<Box<BlockNode>>, // Point to the latest block on chain
}

pub struct BlockNode {
    pub block: Block, // Current Block
    pub next: Option<Box<BlockNode>>, // Next Block(or None)
    pub hash: [u8; 32] // Current Hash
}

impl BlockNode {
    pub fn create_node(curr_block: Block, prev_hash: [u8; 32]) -> Self {
        let node_hash = Self::compute_hash(prev_hash, curr_block.hash);
        BlockNode { block: curr_block, next: None, hash: node_hash}
    }

    pub fn compute_hash(prev_hash: [u8; 32], curr_block: [u8; 32]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(prev_hash);
        hasher.update(curr_block);
        let result = hasher.finalize();

        let mut hash_bytes = [0u8; 32];
        hash_bytes.copy_from_slice(&result);
        hash_bytes
    }
}

impl Blockchain {
    // Genesis Block
    pub fn genesis(genesis_block: Block) -> Self {
        Blockchain { genesis: genesis_block, latest: None }
    }

    // Compute Hash + Create Node and then Update the Blockchain
    pub fn update(&mut self, new_block: BlockNode) {
        let new_node = Box::new(new_block);
        
        match self.latest.as_mut() {
            Some(latest_node) => latest_node.next = Some(new_node),
            None => self.latest = Some(new_node), // Genesis Case
        }
    }
}