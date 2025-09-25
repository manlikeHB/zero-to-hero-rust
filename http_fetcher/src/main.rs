use tokio;
use clap::Parser;
use http_fetcher::{Cli, HttpClient, make_request, FetcherError};

#[tokio::main]
async fn main() -> Result<(), FetcherError> {
    let cli = Cli::parse();

    let client = HttpClient::new(cli.timeout)?;

    make_request(client.get_client(), &cli).await?;
    Ok(())
}

