use std::path::Path;

use clap::Parser;
use rwget::download_file;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Cli {
    #[arg(short, long)]
    out_file: Option<String>,

    #[clap()]
    url: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let url = cli.url;
    let path = Path::new(&url);
    let file_name = cli
        .out_file
        .unwrap_or(path.file_name().unwrap().to_str().unwrap().to_string());
    download_file(&url, &file_name).await.unwrap();
}
