mod wallet;
fn main() {
    let new_wallet = wallet::Wallet::new();
    println!("Address: {:?}", new_wallet.get_address());
    println!("Public Key: {:?}", new_wallet.get_public_key().to_string());
    println!("Secret Key: {:?}", new_wallet.get_secret_key());
}

