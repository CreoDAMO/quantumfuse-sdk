use ipfs_api_backend_actix::{IpfsApi, IpfsClient, TryFromUri};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = IpfsClient::from_str("http://localhost:5001")?; 

    let mut file = File::open("audio.opus").await?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await?;

    let response = client.add(buffer.as_slice()).await?;
    println!("🔗 IPFS Hash: {}", response.hash);

    Ok(())
}
