use anyhow::Result;
use clap::Parser;
use weather_cli::cli::Cli;
use weather_cli::error::WeatherError;
use weather_cli::model::WeatherResponse;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    validate_units(&cli.units)?;

    dotenv::dotenv().ok();

    let api_key = std::env::var("OPENWEATHER_API_KEY").map_err(|_| WeatherError::InvalidApiKey)?;

    let cities = cli.city;
    let mut handles = Vec::new();

    for city in cities {
        let api_key_clone = api_key.clone();

        let handle = tokio::spawn(async move { fetch_weather(&city, &api_key_clone).await });
        handles.push(handle);
    }

    for handle in handles {
        let weather = handle.await??;

        // Display based on flags
        if cli.detailed {
            weather.display_detailed(&cli.units);
        } else {
            weather.display(&cli.units);
        }
    }

    Ok(())
}

fn validate_units(units: &str) -> Result<(), WeatherError> {
    match units.to_lowercase().as_str() {
        "metric" | "imperial" | "kelvin" => Ok(()),
        _ => Err(WeatherError::InvalidUnits(units.to_string())),
    }
}

async fn fetch_weather(city: &str, api_key: &str) -> Result<WeatherResponse, WeatherError> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&APPID={}",
        city, api_key
    );

    let response = reqwest::get(&url).await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let weather: WeatherResponse = response.json().await?;
            Ok(weather)
        }
        reqwest::StatusCode::NOT_FOUND => Err(WeatherError::CityNotFound(city.to_string())),
        reqwest::StatusCode::UNAUTHORIZED => Err(WeatherError::InvalidApiKey),
        _ => Err(WeatherError::Unknown),
    }
}
