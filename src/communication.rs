use crate::wallet::{self, Wallet};

pub struct Broadcast {
    message: String,
    broadcast: u8,
    wallet: Wallet,
}

impl Broadcast {
    pub fn new(wallet: Wallet) -> Self {
        Broadcast {message: String::from(""), broadcast: 0, wallet}
    }

    pub fn get_nonce(noncestr: String) -> u64 {
        let nonce: u64 = noncestr.parse().expect("Unable to decode nonce");
        nonce
    }
}