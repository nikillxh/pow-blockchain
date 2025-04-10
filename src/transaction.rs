use chrono::Utc;
use libp2p::{swarm::NetworkBehaviour, Swarm};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use crate::{coin::ECoin, wallet::Wallet};

// #[derive(Debug, Serialize, Deserialize, Clone)]
// Ecoin is UTXO
#[derive(Serialize, Deserialize)]
pub struct Transaction {
    pub inputs: Vec<ECoin>,
    pub outputs: Vec<ECoin>,
    pub id: [u8; 32],
}

impl Transaction {
    pub fn transact(from_wallet: Wallet, to_wallet: Wallet, inputs: Vec<ECoin>, value: u64) -> Self {
        let miner_fee: u64 = 1;
        let total_input: u64 = inputs.iter().map(|coin| coin.value).sum();
        let id = Transaction::generate_id();

        // Change ECoin (UTXO)
        let change_val = total_input - value - miner_fee;
        let change_hash = ECoin::hash_coin(from_wallet.public_key, id);
        let change = ECoin {
            curr_owner: from_wallet.public_key,
            prev_tx: id,
            hash: ECoin::hash_coin(from_wallet.public_key, id),
            signature: ECoin::sign_coin(&from_wallet, change_hash),
            value: change_val
        };

        // Receiver ECoin (UTXO)
        let output_val = value;
        let output_hash = ECoin::hash_coin(to_wallet.public_key, id);
        let output = ECoin {
            curr_owner: to_wallet.public_key,
            prev_tx: id,
            hash: ECoin::hash_coin(to_wallet.public_key, id),
            signature: ECoin::sign_coin(&from_wallet, output_hash),
            value: output_val,
        };

        // Outputs ECoin
        let outputs = vec![change, output];

        let message = format!("{} ECoins transferred from {} to {}.", output_val, from_wallet.get_address(), to_wallet.get_address());

        // Transaction
        Transaction { inputs, outputs, id }
    }

    pub fn generate_id() -> [u8; 32] {
        let mut hasher = Sha256::new();

        let timestamp_opt = Utc::now().timestamp_nanos_opt();
        let timestamp = timestamp_opt.unwrap_or(0);
        hasher.update(timestamp.to_be_bytes());

        let result = hasher.finalize();
        let mut id = [0u8; 32];
        id.copy_from_slice(&result);
        id
    }
}