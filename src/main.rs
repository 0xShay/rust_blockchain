use std::net::{IpAddr, Ipv4Addr};
use rand::Rng;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey, traits::PrivateKeyParts, traits::PublicKeyParts, BigUint};
use serde::{Deserialize, Serialize};
use serde_json::Result;

mod block_utils;
use block_utils::Block;

mod account_utils;
mod peers;
// use account_utils;

fn main() {

    println!("---");
    println!("Generating keys...");

    let priv_key: RsaPrivateKey = account_utils::generate_priv_key();
    let priv_key_hex: String = account_utils::priv_key_to_hex(&priv_key);
    // let priv_key_hex: String = String::from("aff3f015a8a7b8037a68bd8a9a1257fb0065b8a113e46d11f3fb177180a533e1");
    // let priv_key: RsaPrivateKey = account_utils::hex_to_priv_key(&priv_key_hex).expect("Hex to private key conversion failed");
    let pub_key: RsaPublicKey = account_utils::generate_pub_key(&priv_key);
    let pub_key_hex: String = account_utils::pub_key_to_hex(&pub_key);

    println!("Generated private key: {}", priv_key_hex);
    println!("Generated public key: {}", pub_key_hex);

    println!("---");
    println!("Generating genesis block...");

    let genesis: Block = Block::generate_genesis_block();
    println!("{:#?}", genesis);

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
    println!("Known peers now: {:#?}", peers);
    println!("Saving to known_peers.txt...");
    peers.save_known_peers();
    println!("Saved.\nReloading known peers...");
    peers.load_known_peers();
    println!("Successfully loaded. Known peers now: {:#?}", peers);
}
