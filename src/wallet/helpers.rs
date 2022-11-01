use secp256k1::{
    rand::{rngs, SeedableRng}, PublicKey, Secp256k1, SecretKey
};
use sha3::{Digest, Keccak256};
use web3::types::Address;

pub fn genreate_keypair() -> (SecretKey, PublicKey) {
    let secp = Secp256k1::new();
    let mut rng = rngs::StdRng::seed_from_u64(111);
    let (secret_key, public_key) = secp.generate_keypair(&mut rng);
    (secret_key, public_key)
}

pub fn get_public_key_address(public_key: &PublicKey) -> Address {
    let public_key = public_key.serialize_uncompressed();
    let hash = Keccak256::digest(&public_key[1..]);
    Address::from_slice(&hash[12..])
}