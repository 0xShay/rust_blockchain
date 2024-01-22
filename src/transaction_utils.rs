use rsa::{BigUint, RsaPrivateKey, RsaPublicKey, traits::PrivateKeyParts, traits::PublicKeyParts};
use serde::{Deserialize, Serialize};
use sha256;

// use crate::account_utils;

#[derive(Debug, Deserialize, Serialize)]
pub struct Transaction {

    sender: String, // sender address (in hex)
    recipient: String, // recipient address (in hex)
    value: u32, // transfer amount in raw units of currency (excl. fee)
    fee: u32, // fee amount in raw units of currency
    hash: String, // hash for the transaction data
    signature: String, // signed hash with sender private key

}

impl Transaction {

    pub fn from_json(json: &str) -> Self {
        serde_json::from_str(json).expect("Error deserializing transaction JSON")
    }

    pub fn vec_from_json(json: &str) -> Vec<Self> {
        let json: Vec<Self> = serde_json::from_str(json).expect("Error deserializing transaction list JSON");
        json
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("Error serializing transaction JSON")
    }
    
    pub fn new(sender: &str,
        recipient: &str,
        value: u32,
        fee: u32,
        sender_key: &str
    ) -> Result<Transaction, Box<dyn std::error::Error>> {
        let mut tx = Transaction {
            sender: String::from(sender),
            recipient: String::from(recipient),
            value,
            fee,
            hash: String::from("0000000000000000000000000000000000000000000000000000000000000000"),
            signature: String::from("0000000000000000000000000000000000000000000000000000000000000000"),
        };
        let sender_key = crate::account_utils::priv_key_from_hex(sender_key)?;
        tx.hash = crate::transaction_utils::Transaction::generate_hash(&tx);
        tx.signature = crate::account_utils::sign_transaction(&sender_key, &tx);
        Ok(tx)
    }

    pub fn verify_transaction() -> bool {
        true
        // NEEDS IMPLEMENTING
    }

    pub fn generate_hash(tx: &Transaction) -> String {
        sha256::digest(format!(
            "{}{}{}{}",
            tx.sender,
            tx.recipient,
            tx.value,
            tx.fee
        ))
    }

}