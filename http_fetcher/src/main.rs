use clap::{Parser, builder::Str};
use reqwest;
use std::time::{Duration, Instant};
use tokio;
use tokio::fs;

#[derive(Parser)]
#[command(name = "http_fetcher")]
#[command(about = "A simple HTTP client that fetches multiple URLs concurrently")]
#[command(version = "1.0")]
struct Cli {
    /// Urls to fetch
    urls: Vec<String>,

    /// Timeout in seconds for each request
    #[arg(short, long, default_value = "30")]
    timeout: u64,

    /// Maximum number of concurrent request
    #[arg(short, long, default_value = "10")]
    max_concurrent: usize,

    /// show only status and header no response body
    #[arg(short = 's', long)]
    status_only: bool,

    /// Directory to save responses
    #[arg(short = 'd', long)]
    save_dir: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let urls = cli.urls;
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(cli.timeout))
        .build()?;

    if urls.is_empty() {
        return Err("Please provide at least one URL".into());
    };

    let start = Instant::now();

    let mut handles = Vec::new();
    for url in urls {
        println!("=== Fetching: {} ===", url);
        let url_clone = url.clone();
        let client_clone = client.clone();
        let handle = tokio::spawn(async move {
            let result = client_clone.get(&url).send().await;
            (url_clone, result)
        });
        handles.push(handle);
    }

    // Collect all results
    for handle in handles {
        let (url, result) = handle.await?;
        match result {
            Ok(response) => {
                let status = response.status();
                let filename = generate_filename(response.url().as_str());
                match response.text().await {
                    Ok(body) => {
                        match cli.save_dir {
                            Some(ref dir) => match fs::create_dir_all(dir).await {
                                Ok(_) => save_file(dir.as_str(), &filename, body.as_str())
                                    .await
                                    .unwrap_or(()),
                                Err(e) => eprintln!("Could not create directory {}: {}", dir, e),
                            },
                            None => (),
                        }

                        if cli.status_only {
                            println!("✅ {} [{}]", url, status);
                        } else {
                            if body.len() > 1000 {
                                println!("✅ {} [{}]: {} bytes", url, status, body.len());
                            } else {
                                println!("✅ {} [{}]:\n{}", url, status, body);
                            }
                        }
                    }
                    Err(e) => eprintln!("❌ {} - Failed to read body: {}", url, e),
                }
            }
            Err(e) => {
                if e.is_timeout() {
                    eprintln!(
                        "⏰ {}: Request timed out after {} seconds",
                        url, cli.timeout
                    );
                } else {
                    eprintln!("❌ {}: {}", url, e);
                }
            }
        }
    }

    println!("Total time: {:?}", start.elapsed());
    Ok(())
}

async fn save_file(dir: &str, filename: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = format!("{}/{}.txt", dir, filename);
    fs::write(path, content).await?;
    Ok(())
}

fn generate_filename(url: &str) -> String {
    let sanitized_url = url
        .replace("http://", "")
        .replace("https://", "")
        .replace("/", "_")
        .replace("?", "_")
        .replace("&", "_")
        .replace("=", "_");
    format!("{}.txt", sanitized_url)
}
