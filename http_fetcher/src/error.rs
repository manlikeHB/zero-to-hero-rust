use thiserror::Error;

#[derive(Error, Debug)]
pub enum FetcherError {
    #[error("Request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("IO operation failed: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Tokio task failed: {0}")]
    TokioJoinError(#[from] tokio::task::JoinError),

    #[error("No urls provided")]
    NoUrls,
}