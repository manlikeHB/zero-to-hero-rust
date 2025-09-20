use reqwest;
use std::time::Instant;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().skip(1).collect::<Vec<String>>();

    if args.is_empty() {
        return Err("Please provide at least one URL".into());
    };

    let start = Instant::now();

    let mut handles = Vec::new();
    for url in args {
        println!("=== Fetching: {} ===", url);
        let url_clone = url.clone();
        let handle = tokio::spawn(async move {
            let result = reqwest::get(&url).await;
            (url_clone, result)
        });
        handles.push(handle);
    }

    // Collect all results
    for handle in handles {
        let (url, result) = handle.await?;
        match result {
            Ok(response) => println!("✅ {}: {:?}", url, response),
            Err(e) => eprintln!("❌ {}: {}", url, e),
        }
    }

    println!("Total time: {:?}", start.elapsed());
    Ok(())
}
