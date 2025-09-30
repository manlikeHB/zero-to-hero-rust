use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChatError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Tokio Join error: {0}")]
    JoinError(#[from] tokio::task::JoinError),
    #[error("Send error: {0}")]
    SendError(#[from] tokio::sync::broadcast::error::SendError<String>),
    // #[error("UTF-8 error: {0}")]
    // Utf8(#[from] std::string::FromUtf8Error),
    #[error("Unknown error")]
    Unknown,
}
