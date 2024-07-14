use clap::Parser;
use rwget::{rwget, Args};

#[tokio::main]
async fn main() {
    let args = Args::parse();
    rwget(args).await;
}
