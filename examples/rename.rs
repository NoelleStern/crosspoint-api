//! 
//! Creates a temporary directory, uploads a file
//! in there, downloads it back and then deletes it all
//! 
//! Run with:
//!    cargo run --example rename
//! 


use crosspoint_api::CrossPointClient;


const TEST_FILE: &str = "test.txt";
const TEST_FILE_NEW_NAME: &str = "test_renamed.txt";
const TEST_TEXT: &str = "empty :<";


#[tokio::main]
async fn main() -> crosspoint_api::Result<()> {
    let client: CrossPointClient = CrossPointClient::new(None)?;

    // Upload test file to root
    client.upload("/".to_owned(), TEST_FILE.to_owned(), TEST_TEXT.as_bytes()).await?;
    let files = client.list("/".to_owned()).await?;
    let file = files.iter().find(|&f| f.name == TEST_FILE);
    println!("> Test file uploaded: {file:#?}");

    // Rename the file
    client.rename(format!("/{}", TEST_FILE), TEST_FILE_NEW_NAME.to_owned()).await?;
    let files = client.list("/".to_owned()).await?;
    let file = files.iter().find(|&f| f.name == TEST_FILE_NEW_NAME);
    println!("> Test file renamed: {file:#?}");
  
    // Delete the file
    client.delete(format!("/{}", TEST_FILE_NEW_NAME)).await?;
    println!("> Test file deleted");

    // Done!
    println!("> Test finished successfully!");
    Ok(())
}