use serde::{Deserialize, Serialize};
use serde_json::Result;
use sha256::{digest, try_digest};

// println!("{}", digest("test"));

#[derive(Debug, Deserialize, Serialize)]
pub struct Block {

    height: i32, // current block height

    send_pub_key: String, // sender public key
    recv_pub_key: String, // receiver public key
    amount: i32, // amount to send (deduct) from sender account
    balance: i32, // resulting balance of sender account after amount deduction

    signature: String, // signature

    previous: String, // previous block hash
    local_timestamp: i32, // timestamp of block

}

impl Block {

    pub fn from_json(json: &str) -> Self {
        serde_json::from_str(json).expect("Error deserializing block JSON")
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("Error serializing block JSON")
    }

    pub fn test() {
        println!("test");
    }
    
    pub fn generate_test_block() -> Block {
        Block {
            height: 0,
            send_pub_key: String::from("qbf"),
            recv_pub_key: String::from("ld"),
            amount: 0,
            balance: 0,
            signature: String::from("shaysig"),
            previous: String::from("genesis"),
            local_timestamp: 0
        }
    }
    
    pub fn generate_hash(&self) -> String {
        digest(self.to_json())
    }

}