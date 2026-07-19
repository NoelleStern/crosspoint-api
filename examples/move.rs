//! 
//! Uploads a file to the root, creates a temporary directory,
//! moves the file to the temporary directory and then deletes it all
//! 
//! Run with:
//!    cargo run --example move
//! 


use crosspoint_api::CrossPointClient;


const TEST_DIR: &str = "/api-test-dir";
const TEST_FILE: &str = "test.txt";
const TEST_TEXT: &str = "empty :<";


#[tokio::main]
async fn main() -> crosspoint_api::Result<()> {
    let client: CrossPointClient = CrossPointClient::new(None)?;

    // Upload test file to root
    client.upload("/".to_owned(), TEST_FILE.to_owned(), TEST_TEXT.as_bytes()).await?;
    let files = client.list("/".to_owned()).await?;
    let file = files.iter().find(|&f| f.name == TEST_FILE);
    println!("> Test file uploaded: {file:#?}");

    // Create a test directory
    client.mkdir(TEST_DIR.to_owned()).await?;
    println!("> Test folder created");

    // Move file to the test directory
    client.r#move(format!("/{}", TEST_FILE), TEST_DIR.to_owned()).await?;
    let files = client.list(TEST_DIR.to_owned()).await?;
    println!("> Test file moved: {files:#?}");

    // Delete the file
    client.delete(format!("{}/{}", TEST_DIR, TEST_FILE)).await?;
    println!("> Test file deleted");

    // Delete the folder (Doesn't work if non-empty)
    client.delete(TEST_DIR.to_owned()).await?;
    println!("> Test folder deleted");

    // // Or we could have force-deleted the folder instead
    // client.force_delete_directory(TEST_DIR.to_owned()).await?;

    // Done!
    println!("> Test finished successfully!");
    Ok(())
}