use anyhow::Result;
use iroh::{Endpoint, SecretKey};
use rand::TryRngCore;
use rand_core::OsRng;

#[tokio::main]
async fn main() -> Result<()> {
    // Generate a secret key. This is the source of identity for your node. If you want to have
    // the same identity each time you open the app, you would need to store and load it each time.
    let mut key_bytes = [0u8; 32];
    // Fill the key bytes with random data
    OsRng.try_fill_bytes(&mut key_bytes).unwrap();

    let secret_key = SecretKey::from_bytes(&key_bytes);
    println!("Our Secret key: {secret_key}");

    let endpoint = Endpoint::builder().discovery_n0().bind().await?;

    println!("> our node id: {}", endpoint.node_id());

    Ok(())
}
