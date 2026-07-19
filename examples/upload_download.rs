//! 
//! Creates a temporary directory, uploads a file
//! in there, downloads it back and then deletes it all
//! 
//! Run with:
//!    cargo run --example upload_download
//! 


use crosspoint_api::CrossPointClient;


const TEST_DIR: &str = "/api-test-dir";
const TEST_FILE: &str = "test.txt";
const TEST_TEXT: &str = "meow-meow-meow :3";


#[tokio::main]
async fn main() -> crosspoint_api::Result<()> {
    let file_path = format!("{}/{}", TEST_DIR, TEST_FILE);
    let client: CrossPointClient = CrossPointClient::new(None)?;

    // Create a test directory
    client.mkdir(TEST_DIR.to_owned()).await?;
    println!("> Test folder created");

    // Upload test file to the test directory
    client.upload(TEST_DIR.to_owned(), TEST_FILE.to_owned(), TEST_TEXT.as_bytes()).await?;
    let files = client.list(TEST_DIR.to_owned()).await?;
    println!("> Test file uploaded: {files:#?}");

    // Read the file back
    let bytes = client.download(file_path.to_owned()).await?;
    match str::from_utf8(&bytes) {
        Ok(v) => println!("> Test file downloaded: \"{v}\""),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
  
    // Delete the file
    client.delete(file_path.to_owned()).await?;
    println!("> Test file deleted");

    // Delete the folder (Doesn't work if non-empty)
    client.delete(TEST_DIR.to_owned()).await?;
    println!("> Test folder deleted");

    // // Or we could have force-deleted the folder instead
    // client.force_delete_directory(TEST_DIR).await?;

    // Done!
    println!("> Test finished successfully!");
    Ok(())
}