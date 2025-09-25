use clap::{Parser};

#[derive(Parser)]
#[command(name = "http_fetcher")]
#[command(about = "A simple HTTP client that fetches multiple URLs concurrently")]
#[command(version = "1.0")]
pub struct Cli {
    /// Urls to fetch
    pub urls: Vec<String>,

    /// Timeout in seconds for each request
    #[arg(short, long, default_value = "30")]
    pub timeout: u64,

    /// Maximum number of concurrent request
    #[arg(short, long, default_value = "10")]
    pub max_concurrent: usize,

    /// show only status and header no response body
    #[arg(short = 's', long)]
    pub status_only: bool,

    /// Directory to save responses
    #[arg(short = 'd', long)]
    pub save_dir: Option<String>,
}