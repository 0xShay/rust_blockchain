use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use serde_json::Result;

mod block_utils;
use block_utils::Block;

mod account_utils;
mod peers;
mod server;
// use account_utils;

mod transaction_utils;
use transaction_utils::Transaction;

fn main() {


    println!("---");
    println!("Generating keys...");

    let priv_key: SigningKey = account_utils::generate_priv_key();
    let priv_key_hex: String = account_utils::priv_key_to_hex(&priv_key);
    
    let pub_key: VerifyingKey = account_utils::generate_pub_key(&priv_key);
    let pub_key_hex: String = account_utils::pub_key_to_hex(&pub_key);

    println!("Generated private key: {}", priv_key_hex);
    println!("Generated public key: {}", pub_key_hex);


    println!("---");
    println!("Generating genesis block...");

    let genesis: Block = Block::generate_genesis_block();
    println!("{:#?}", genesis);
    println!("{}", genesis.to_json());


    println!("---");
    println!("Generating transaction...");
    let tx: Transaction = Transaction::new(
        &pub_key_hex,
        &pub_key_hex,
        10,
        10,
        &priv_key_hex
    ).expect("Generating transaction failed.");
    println!("{:?}", tx);
    

    // let genesis: Block = Block::generate_test_block();
    // println!("{:#?}", genesis);
    // println!("{}", genesis.to_json());
    // println!("{:#?}", genesis.generate_hash());

    // Test peers.rs code
    let mut peers = peers::Peers::new();
    println!("Original known peers: {:#?}", peers);
    println!("Requesting known peers: {:#?}", peers.get_known_peers(IpAddr::V4(Ipv4Addr::new(70, 116, 167, 167))));
    println!("Known peers now: {:#?}", peers);
    println!("Requesting known peers again: {:#?}", peers.get_known_peers(IpAddr::V4(Ipv4Addr::new(204, 14, 12, 61))));
    println!("Requesting known peers again: {:#?}", peers.get_known_peers(IpAddr::V4(Ipv4Addr::new(188, 80, 225, 31))));
    println!("Reqiesting known peers again: {:#?}", peers.get_known_peers(IpAddr::V6(Ipv6Addr::from_str("::1").unwrap())));
    println!("Known peers now: {:#?}", peers);
    println!("Saving to known_peers.txt...");
    peers.save_known_peers();
    println!("Saved.\nReloading known peers...");
    peers.load_known_peers();
    println!("Successfully loaded. Known peers now: {:#?}", peers);

    println!();
    println!("Now testing the server code.");
    server::Server::new().start_server();
}
