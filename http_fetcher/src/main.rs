use reqwest;
use tokio;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    let mut valid_urls = Vec::<String>::new();
    let mut handles = vec![];

    if args.is_empty() {
        return Err("Please provide a URL".into());
    }

    for url in args {
        match check_valid_url(&url) {
            Ok(_) => valid_urls.push(url),
            Err(e) => println!("Error: {}", e),
        }
    }

    let start = std::time::Instant::now();
    for url in valid_urls {
        println!("=== Fetching: {} ===", url);
        let handle = tokio::spawn(async move { reqwest::get(url).await });

        handles.push(handle);
    }

    for handle in handles {
        let result = handle.await?;
        match &result {
            Ok(res) => println!("{:?}", result),
            _ => println!("Failed to fetch URL: ..."),
        }
    }
    // Total time: 2.818446209s for sync
    println!("Total time: {:?}", start.elapsed());

    Ok(())
}

fn check_valid_url(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    if Url::parse(&url).is_err() {
        return Err(format!("Invalid URL: {}", url).into());
    }

    Ok(())
}
