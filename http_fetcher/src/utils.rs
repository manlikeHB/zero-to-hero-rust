use tokio::fs;
use crate::error::FetcherError;

pub async fn save_file(dir: &str, filename: &str, content: &str) -> Result<(), FetcherError> {
    let path = format!("{}/{}.txt", dir, filename);
    fs::write(path, content).await?;
    Ok(())
}

pub fn generate_filename(url: &str) -> String {
    let sanitized_url = url
        .replace("http://", "")
        .replace("https://", "")
        .replace("/", "_")
        .replace("?", "_")
        .replace("&", "_")
        .replace("=", "_");
    format!("{}.txt", sanitized_url)
}