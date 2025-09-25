use tokio;
use http_fetcher::config::Cli;
use clap::Parser;
use http_fetcher::client::{HttpClient, make_request};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let client = HttpClient::new(cli.timeout)?;

    make_request(client.get_client(), &cli).await?;
    Ok(())
}

