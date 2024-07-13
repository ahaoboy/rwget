use reqwest::Client;
use std::fs::File;
use std::io::copy;

pub async fn download_file(url: &str, out_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Create a client
    let client = Client::new();

    // Send a GET request
    let response = client.get(url).send().await?;

    // Check if the response status is OK
    if response.status().is_success() {
        // Create or open the file
        let mut dest = File::create(out_file)?;

        // Copy the response content to the file
        let content = response.bytes().await?;
        copy(&mut content.as_ref(), &mut dest)?;
        println!("File downloaded successfully to {}", out_file);
    } else {
        eprintln!("Failed to download file: {}", response.status());
    }
    Ok(())
}
