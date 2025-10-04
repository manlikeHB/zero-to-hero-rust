use thiserror::Error;

#[derive(Error, Debug)]
pub enum WeatherError {
    #[error("City not found: {0}")]
    CityNotFound(String),

    #[error("Invalid API key. Please check your OPENWEATHER_API_KEY environment variable")]
    InvalidApiKey,

    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("Invalid units '{0}'. Use: metric, imperial, or kelvin")]
    InvalidUnits(String),

    #[error("Invalid timestamp: {0}")]
    InvalidTimestamp(i64),

    #[error("Unknown error occurred")]
    Unknown,
}
