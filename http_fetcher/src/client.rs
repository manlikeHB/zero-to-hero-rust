use reqwest::Client;
use std::time::Duration;
use crate::config::Cli;
use crate::utils::{save_file, generate_filename};
use tokio::fs;

pub struct HttpClient {
    client: reqwest::Client,
}

impl HttpClient {
  pub fn new(timeout: u64) -> Result<Self, Box<dyn std::error::Error>> {
      let client = reqwest::Client::builder()
          .timeout(Duration::from_secs(timeout))
          .build()?;
      Ok(HttpClient { client })
  }

  pub fn get_client(&self) -> &Client {
      &self.client
  }
}

pub async fn make_request(client: &Client, cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    let urls = cli.urls.clone();
    let mut handles = Vec::new();
    for url in urls {
        println!("=== Fetching: {} ===", url);
        let url_clone = url.clone();
        let client_clone = client.clone();
        let handle = tokio::spawn(async move {
            let result = client_clone.get(url).send().await;
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

    Ok(())
}