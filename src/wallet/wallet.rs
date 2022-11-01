use super::{genreate_keypair, get_public_key_address};
use secp256k1::{PublicKey, SecretKey};
use web3::types::Address;
pub struct Wallet {
    pub secret: SecretKey,
    pub public: PublicKey,
    pub address: Address,
}

impl Wallet {
    pub fn new() -> Self {
        let (secret, public) = genreate_keypair();
        let address = get_public_key_address(&public);
        Wallet {
            secret,
            public,
            address,
        }
    }
}
