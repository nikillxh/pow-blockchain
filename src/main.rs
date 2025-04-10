mod wallet;
mod transaction;
mod coin;
mod block;
mod blockchain;
mod network;
mod communication;

use std::{
    collections::hash_map::DefaultHasher, error::Error, hash::{Hash, Hasher}, sync::{Arc, Mutex}, time::Duration
};
use communication::Broadcast;
use futures::{stream::StreamExt};
use libp2p::{
    gossipsub, mdns, noise,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux,
};
use network::{task1, task2};
use tokio::{io, select, sync::mpsc, task, time};
use tracing_subscriber::EnvFilter;
use wallet::{Wallet, WalletType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let wallet = Wallet::new(WalletType::Miner);
    let mut bcwallet = Arc::new(Mutex::new(Broadcast::new(wallet)));

    let mut bcwallet1 = Arc::clone(&bcwallet);
    let mut bcwallet2 = Arc::clone(&bcwallet);

    // Create a channel for task2 to send messages to task1
    let (tx, rx) = mpsc::channel::<String>(100);
    
    // Spawn both tasks with communication channel
    let t1 = task::spawn(task1(&bcwallet1, &mut rx));
    let t2 = task::spawn(task2(&bcwallet2,tx));

    // Wait for tasks to complete (they won't normally complete unless there's an error)
    let results: Vec<Result<Result<(), Box<dyn Error + Send + Sync>>, task::JoinError>> = futures::future::join_all(vec![t1, t2]).await;
    
    // Report any errors from tasks
    for (i, result) in results.into_iter().enumerate() {
        if let Err(e) = result {
            println!("Task {} failed with error: {:?}", i+1, e);
        }
    }

    Ok(())
}