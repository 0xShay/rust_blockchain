use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use serde_json::Result;

mod block_utils;
use block_utils::Block;

mod account_utils;
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

}
