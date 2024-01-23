use rand::rngs::OsRng;
use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use hex;

use crate::transaction_utils;

pub fn generate_priv_key() -> SigningKey {
    let mut csprng = OsRng;
    let private_key: SigningKey = SigningKey::generate(&mut csprng);
    private_key
}

pub fn generate_pub_key(private_key: &SigningKey) -> VerifyingKey {
    let public_key: VerifyingKey = private_key.verifying_key();
    public_key
}

pub fn priv_key_to_hex(private_key: &SigningKey) -> String { hex::encode(private_key.to_bytes()) }

pub fn pub_key_to_hex(pub_key: &VerifyingKey) -> String { hex::encode(pub_key.to_bytes()) }

pub fn signature_to_hex(signature: &Signature) -> String { hex::encode(signature.to_bytes()) }

pub fn priv_key_from_hex(hex_str: &str) -> Result<SigningKey, Box<dyn std::error::Error>> {
    let private_key_bytes: &[u8] = &hex::decode(hex_str)?;
    let mut private_array: [u8; 32] = [0; 32];
    private_array.copy_from_slice(&private_key_bytes);
    let private_key = SigningKey::from_bytes(&private_array);
    Ok(private_key)
}

pub fn pub_key_from_hex(hex_str: &str) -> Result<VerifyingKey, Box<dyn std::error::Error>> {
    let public_key_bytes: &[u8] = &hex::decode(hex_str)?;
    let mut public_array: [u8; 32] = [0; 32];
    public_array.copy_from_slice(&public_key_bytes);
    Ok(VerifyingKey::from_bytes(&public_array)?)
}

pub fn signature_from_hex(hex_str: &str) -> Result<Signature, Box<dyn std::error::Error>> {
    let sig_bytes: &[u8] = &hex::decode(hex_str)?;
    let mut sig_array: [u8; 64] = [0; 64];
    sig_array.copy_from_slice(&sig_bytes);
    Ok(Signature::from_bytes(&sig_array))
}

pub fn sign_transaction(key: &SigningKey, tx: &transaction_utils::Transaction) -> String {
    // NEEDS IMPLEMENTING
    let signature: Signature = key.sign(transaction_utils::Transaction::generate_hash(&tx).as_bytes());
    hex::encode(signature.to_bytes())
}