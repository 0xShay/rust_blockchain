use byteorder::{ByteOrder, LittleEndian};
use hex;
use rand::Rng;
use rsa::{BigUint, RsaPrivateKey, RsaPublicKey, traits::PrivateKeyParts, traits::PublicKeyParts};

pub fn generate_priv_key() -> RsaPrivateKey {
    let mut rng = rand::thread_rng();
    let bits = 128;
    RsaPrivateKey::new(&mut rng, bits).expect("Key generation failed")
}

pub fn generate_pub_key(priv_key: &RsaPrivateKey) -> RsaPublicKey {
    RsaPublicKey::from(priv_key)
}

pub fn priv_key_to_hex(priv_key: &RsaPrivateKey) -> String {
    format!("{:0>32x}{:0>32x}", priv_key.n(), priv_key.d())
}

pub fn pub_key_to_hex(pub_key: &RsaPublicKey) -> String {
    format!("{:0>32x}", pub_key.n())
}

pub fn hex_to_priv_key(hex_str: &String) -> Option<RsaPrivateKey> {
    if let Ok(bytes) = hex::decode(&hex_str) {
        let n: [u8; 16];
        let n: BigUint = BigUint::from_bytes_be(&bytes) >> 128;
    
        let e: BigUint = BigUint::from(65537u32);

        let d: [u8; 16];
        let big128: BigUint = BigUint::from(1u32) << 128;
        let d: BigUint = BigUint::from_bytes_be(&bytes) % big128;

        let primes: Vec<BigUint> = Vec::new();

        match RsaPrivateKey::from_components(
            n,
            e,
            d,
            primes
        ) {
            Ok(key) => Some(key),
            Err(_) => None
        }
    } else { None }
}

pub fn hex_to_pub_key(hex_str: &String) -> Option<RsaPublicKey> {
    if let Ok(bytes) = hex::decode(&hex_str) {
        let n: [u8; 16];
        let n: BigUint = BigUint::from_bytes_be(&bytes);

        let e: BigUint = BigUint::from(65537u32);

        match RsaPublicKey::new(n, e) {
            Ok(key) => Some(key),
            Err(_) => None
        }
    } else { None }
}