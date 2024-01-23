use serde::{Deserialize, Serialize};
use sha256;
use std::time::SystemTime;
use num_bigint::BigUint;

use crate::db_utils;
use crate::transaction_utils::Transaction;

const DIFFICULTY: usize = 8; // number of zeros needed to prefix hash (bits)

#[derive(Debug, Deserialize, Serialize)]
pub struct Block {

    pub index: u32,
    pub timestamp: u32, // timestamp in seconds since 1970-01-01 00:00 UTC (epoch)
    pub transactions: Vec<Transaction>, // data for all transactions
    pub previous: String, // hash for the previous block
    pub nonce: u32,
    pub hash: String,

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
            transactions: Vec::new(),
            previous: String::from("0000000000000000000000000000000000000000000000000000000000000000"),
            nonce: 0,
            hash: String::from("0000000000000000000000000000000000000000000000000000000000000000"),        
        };
        genesis.nonce = crate::block_utils::Block::generate_work(&mut genesis).expect("Genesis work generation failed");
        genesis.hash = crate::block_utils::Block::generate_hash(&genesis);
        genesis
    }

    pub fn create_block(
        index: u32,
        timestamp: u32,
        transactions: Vec<Transaction>,
        previous: String,
        nonce: u32,
        hash: String
    ) -> Block {
        Block {
            index,
            timestamp,
            transactions,
            previous,
            nonce,
            hash,        
        }
    }

    pub fn generate_hash(block: &Block) -> String {
        let mut tx_hash_concat: String = String::new();
        for tx in &block.transactions {
            tx_hash_concat += &Transaction::generate_hash(&tx);
        };
        sha256::digest(format!(
            "{}{}{}{}{}",
            block.index,
            block.timestamp,
            tx_hash_concat,
            block.previous,
            block.nonce
        ))
    }

    pub fn generate_transactions_json(block: &Block) -> String {
        serde_json::to_string(&block.transactions).expect("Error serializing block.transactions JSON")
    }

    pub fn generate_work(block: &mut Block) -> Option<u32> {
        for nonce in 0.. {
            block.nonce = nonce;
            if Block::is_work_valid(block) { return Some(nonce) };
        };
        None
    }

    pub fn is_work_valid(block: &Block) -> bool {
        let hash = Block::generate_hash(block);
        match BigUint::parse_bytes(hash.as_bytes(), 16) {
            Some(parsed_bytes) => {
                let hash_bits = parsed_bytes.to_radix_be(2);
                hash_bits.len() <= 256-DIFFICULTY
            },
            None => false
        }
    }

    pub fn verify_block_is_valid(block: &Block) -> bool {
        // check if the previous block exists and is a valid block
        let is_genesis = block.index == 0;
        let prev_block_option: Option<Block> = db_utils::get_block(&block.previous);
        match prev_block_option {
            Some(_) => (),
            None => { if !is_genesis { return false; } }
        };

        // check that the timestamp of the block is greater than that of the previous block and less than 10 minutes into the future
        let secs_since_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        match prev_block_option {
            Some(prev_block) => {
                // not genesis
                if !(block.timestamp > prev_block.timestamp) { return false; };
            },
            None => ()
        };
        if !(block.timestamp < (secs_since_epoch as u32 + 600)) { return false; };

        // verify that work (nonce) is valid
        if !(Block::is_work_valid(block)) { return false; };

        // with the ordered transaction list, loop through and ensure that no balances go into negative

        // add a "block reward" action to the transaction list

        // return true
        true
    }

}