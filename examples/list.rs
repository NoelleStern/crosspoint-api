//! 
//! Lists files and directories at the root
//! 
//! Run with:
//!    cargo run --example list
//! 


use crosspoint_api::CrossPointClient;


#[tokio::main]
async fn main() -> crosspoint_api::Result<()> {
    let client = CrossPointClient::new(None)?;
    let files = client.list("/".to_owned()).await?;
    println!("{files:#?}");
    Ok(())
}