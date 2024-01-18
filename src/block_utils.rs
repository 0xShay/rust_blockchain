use serde::{Deserialize, Serialize};
use serde_json::Result;
use sha256;
use num_bigint::BigUint;

use crate::account_utils;

const DIFFICULTY: usize = 8; // number of zeros needed to prefix hash (bits)

#[derive(Debug, Deserialize, Serialize)]
pub struct Block {

    index: u32,
    timestamp: u32, // timestamp in seconds since 1970-01-01 00:00 UTC (epoch)
    transactions: String, // JSON data for all transactions
    previous: String, // hash for the previous block
    nonce: u32,
    hash: String,

}

impl Block {

    pub fn from_json(json: &str) -> Self {
        serde_json::from_str(json).expect("Error deserializing block JSON")
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("Error serializing block JSON")
    }
    
    pub fn generate_genesis_block() -> Block {
        let mut genesis = Block {
            index: 0,
            timestamp: 0,
            transactions: String::from("[]"),
            previous: String::from("0000000000000000000000000000000000000000000000000000000000000000"),
            nonce: 0,
            hash: String::from("0000000000000000000000000000000000000000000000000000000000000000"),        
        };
        genesis.nonce = crate::block_utils::Block::generate_work(&mut genesis).expect("Genesis work generation failed");
        genesis.hash = crate::block_utils::Block::generate_hash(&genesis);
        genesis
    }

    pub fn generate_hash(block: &Block) -> String {
        sha256::digest(format!(
            "{}{}{}{}{}",
            block.index,
            block.timestamp,
            block.transactions,
            block.previous,
            block.nonce
        ))
    }

    pub fn generate_work(block: &mut Block) -> Option<u32> {
        for nonce in 0.. {
            block.nonce = nonce;
            let hash = Block::generate_hash(block);
            let hash_bits = BigUint::parse_bytes(hash.as_bytes(), 16)?.to_radix_be(2);
            let mut ctr: usize = 0;
            if hash_bits.len() <= 256-DIFFICULTY { return Some(nonce) };
        };
        None
    }

    pub fn verify_block_is_valid(block: &Block) -> bool {
        // check if the previous block exists and is a valid block

        // check that the timestamp of the block is greater than that of the previous block and less than an hour into the future

        // verify that work (nonce) is valid

        // with the ordered transaction list, loop through and ensure that no balances go into negative

        // add a "block reward" transaction to the transaction list

        // return true
        true
    }

}