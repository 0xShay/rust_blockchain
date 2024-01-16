use rand::Rng;
use rsa::{RsaPrivateKey, RsaPublicKey, traits::PrivateKeyParts, traits::PublicKeyParts, BigUint};
use hex;
use byteorder::{ByteOrder, LittleEndian};

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

pub fn hex_to_priv_key(hex_str: &String) -> Result<RsaPrivateKey, rsa::Error> {
    let n_o = hex::decode(&hex_str).expect("Hex to private key conversion failed");
    let n: [u8; 16];
    let n: BigUint = BigUint::from_bytes_be(&n_o) >> 128;

    let e: BigUint = BigUint::from(65537u32);

    let d_o = hex::decode(&hex_str).expect("Hex to private key conversion failed");
    let d: [u8; 16];
    let big128: BigUint = BigUint::from(1u32) << 128;
    let d: BigUint = BigUint::from_bytes_be(&d_o) % big128;

    let primes: Vec<BigUint> = Vec::new();
    RsaPrivateKey::from_components(
        n,
        e,
        d,
        primes
    )
}

pub fn hex_to_pub_key(hex_str: &String) -> Result<RsaPublicKey, rsa::Error> {
    let n_o = hex::decode(&hex_str).expect("Hex to public key conversion failed");
    let n: [u8; 16];
    let n: BigUint = BigUint::from_bytes_be(&n_o);

    let e: BigUint = BigUint::from(65537u32);

    RsaPublicKey::new(n, e)
}