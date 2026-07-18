//! 
//! Prints device status
//! 
//! Run with:
//!    cargo run --example status
//! 


use crosspoint_api::CrossPointClient;


#[tokio::main]
async fn main() -> crosspoint_api::Result<()> {
    let client = CrossPointClient::new(None)?;
    let status = client.status().await?;
    println!("{status:#?}");
    Ok(())
}