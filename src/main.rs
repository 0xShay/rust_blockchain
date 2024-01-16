use rand::Rng;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey, traits::PrivateKeyParts, traits::PublicKeyParts, BigUint};
use serde::{Deserialize, Serialize};
use serde_json::Result;

mod block_utils;
use block_utils::Block;

mod account_utils;
// use account_utils;

fn main() {
    // let genesis: Block = Block::generate_test_block();
    // println!("{:#?}", genesis);
    // println!("{}", genesis.to_json());
    // println!("{:#?}", genesis.generate_hash());

    // let data = b"hello world";

    for i in (0..1) {
        let priv_key: RsaPrivateKey = account_utils::generate_priv_key();
        let priv_key_hex: String = account_utils::priv_key_to_hex(&priv_key);
        let pub_key: RsaPublicKey = account_utils::generate_pub_key(&priv_key);
        let pub_key_hex: String = account_utils::pub_key_to_hex(&pub_key);

        let pk_2 = account_utils::hex_to_priv_key(&priv_key_hex).expect("Hex to private key conversion failed");
        let pk_2_hex = account_utils::priv_key_to_hex(&pk_2);
        let puk_2 = account_utils::hex_to_pub_key(&pub_key_hex).expect("Hex to public key conversion failed");
        let puk_2_hex = account_utils::pub_key_to_hex(&puk_2);

        // println!("{} | {} | {}", priv_key_hex, pub_key_hex, pk_2_hex);
        println!("{} | {} | {} | {}", priv_key_hex, pub_key_hex, pk_2_hex, puk_2_hex);
    }

    // let priv_key_json = serde_json::to_string(&priv_key).expect("Key serialization failed");    
    // let loaded_key: RsaPrivateKey = serde_json::from_str(&priv_key_json).expect("Key deserialization failed");

    // let enc_data = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, &data[..]).expect("failed to encrypt");
    // println!("{:?} | {:?}", &data[..5], &enc_data[..5]);

}
