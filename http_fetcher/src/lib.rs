pub mod config;
pub mod utils;
pub mod client;
pub mod error;

pub use config::Cli;
pub use client::{HttpClient, make_request};
pub use error::FetcherError;