use anyhow::Result;
use iroh::{protocol::Router, Endpoint, SecretKey};
use iroh_gossip::net::Gossip;
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

    let endpoint = Endpoint::builder()
        .secret_key(secret_key)
        // Pass in your secret key. If you don't pass in a secret key a new one will be generated
        // for you each time.
        .discovery_n0()
        // Enable n0 discovery. This allows you to dial by `NodeId`, and allows you to be
        // dialed by `NodeId`.
        .bind()
        // Bind the endpoint to the socket.
        .await?;

    //NEVER DO THIS IN PRODUCTION
    println!("> our node id: {}", endpoint.node_id());

    // Build and instance of the gossip protocol
    // and add a clone of the endpoint we have built.
    // The gossip protocol will use the endpoint to
    // make connections.
    let gossip = Gossip::builder().spawn(endpoint.clone()).await?;

    // The Router is how we manage protocols on top
    // of the iroh endpoint. It handles all incoming
    // messages and routes them to the correct protocol.

    let router = Router::builder(endpoint.clone())
        .accept(iroh_gossip::ALPN, gossip.clone())
        .spawn()
        .await?;

    // Cleanly shutdown the router.
    router.shutdown().await?;

    Ok(())
}
