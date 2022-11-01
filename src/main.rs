mod wallet;
fn main() {
    let (secret, public) = wallet::genreate_keypair();
    let pub_key = wallet::get_public_key_address(&public);
    println!("secret: {:?}\npublic: {:?}\npublic_key: {:?}", secret, public.to_string(), pub_key);
}

