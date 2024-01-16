use rsa::{BigUint, RsaPrivateKey, RsaPublicKey, traits::PrivateKeyParts, traits::PublicKeyParts};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use sha256;

use crate::account_utils;

#[derive(Debug, Deserialize, Serialize)]
pub struct Block {

    hash: String,
    height: i32, // current block height

    send_pub_key: String, // sender public key
    recv_pub_key: String, // receiver public key
    amount: i32, // amount to send (deduct) from sender account
    balance: i32, // resulting balance of sender account after amount deduction

    signature: String, // signature

    previous: String, // previous block hash
    local_timestamp: i32, // timestamp of block

    work: i32,

}

impl Block {

    pub fn from_json(json: &str) -> Self {
        serde_json::from_str(json).expect("Error deserializing block JSON")
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("Error serializing block JSON")
    }
    
    pub fn generate_genesis_block(priv_key: &RsaPrivateKey) -> Block {
        let priv_key: RsaPrivateKey = account_utils::generate_priv_key();
        let priv_key_hex: String = account_utils::priv_key_to_hex(&priv_key);
        let pub_key: RsaPublicKey = account_utils::generate_pub_key(&priv_key);
        let pub_key_hex: String = account_utils::pub_key_to_hex(&pub_key);
        let mut genesis = Block {
            hash: sha256::digest("rust_blockchain"),
            height: 0,
            send_pub_key: pub_key_hex.clone(),
            recv_pub_key: pub_key_hex.clone(),
            amount: 0,
            balance: 100,
            signature: String::from("genesis"),
            previous: String::from("0000000000000000000000000000000000000000000000000000000000000000"),
            local_timestamp: 0,
            work: 0
        };
        genesis.hash = genesis.generate_hash();
        genesis.work = Block::generate_work(&genesis.hash).expect("Work generation for genesis hash failed");
        genesis
    }
    
    pub fn generate_hash(&self) -> String {
        sha256::digest(format!(
            "{} {} {} {} {} {} {} {}",
            self.hash,
            self.height,
            self.send_pub_key,
            self.recv_pub_key,
            self.amount,
            self.balance,
            self.signature,
            self.previous
        ))
    }

    pub fn generate_work(block_hash: &str) -> Option<i32> {
        for nonce in 1.. {
            let hash: String = sha256::digest(format!("{}{}", nonce, block_hash));
            match &hash[..4] {
                "0000" => return Some(nonce),
                _ => continue,
            };
        };
        None
    }

}