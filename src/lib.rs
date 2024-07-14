use clap::Parser;
use reqwest::Client;
use std::fs::File;
use std::io::copy;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub out_file: Option<String>,

    #[clap()]
    pub url: String,
}

pub async fn download_file(url: &str, out_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Create a client
    let client = Client::new();

    // Send a GET request
    let response = client.get(url).send().await?;

    // Check if the response status is OK
    if response.status().is_success() {
        let p = std::path::PathBuf::from(out_file);
        let out_dir = p.parent().unwrap();
        if !out_dir.exists() {
            std::fs::create_dir_all(out_dir).unwrap();
        }
        // Create or open the file
        let mut dest = File::create(out_file)?;

        // Copy the response content to the file
        let content = response.bytes().await?;
        copy(&mut content.as_ref(), &mut dest)?;
    }
    Ok(())
}

pub async fn rwget(args: Args) {
    let url = args.url;
    let path = Path::new(&url);
    let file_name = args
        .out_file
        .unwrap_or(path.file_name().unwrap().to_str().unwrap().to_string());
    download_file(&url, &file_name).await.unwrap();
}
