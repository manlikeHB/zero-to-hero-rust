use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "weather_cli", about = "A simple CLI to fetch weather data")]
pub struct Cli {
    /// City name to fetch weather for
    #[arg(required = true)]
    pub city: Vec<String>,

    /// Temperature units: metric (Celsius), imperial (Fahrenheit), or kelvin
    #[arg(short, long, default_value = "metric")]
    pub units: String,

    /// Show detailed weather information
    #[arg(short, long)]
    pub detailed: bool,
}

impl Cli {
    pub fn is_metric(&self) -> bool {
        self.units.to_lowercase() == "metric"
    }

    pub fn is_imperial(&self) -> bool {
        self.units.to_lowercase() == "imperial"
    }

    pub fn is_kelvin(&self) -> bool {
        self.units.to_lowercase() == "kelvin"
    }
}
