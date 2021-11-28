mod cli;
mod github;

use cli::parse_opts;
use github::download;
use std::process;

#[tokio::main]
async fn main() {
    let opts = parse_opts();
    let rs = download(&opts).await;

    if let Err(e) = rs {
        eprintln!("Download failed: {}", e);
        process::exit(1);
    }
}
