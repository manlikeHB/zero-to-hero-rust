use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WeatherResponse {
    pub name: String,
    pub main: MainWeather,
    pub weather: Vec<WeatherCondition>,
    pub wind: Option<Wind>,
    pub clouds: Option<Clouds>,
    pub sys: Option<Sys>,
    pub visibility: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct MainWeather {
    pub temp: f64,
    pub feels_like: f64,
    pub humidity: u8,
    pub pressure: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct WeatherCondition {
    pub main: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct Wind {
    pub speed: f64,
    pub deg: Option<u16>,
}

#[derive(Debug, Deserialize)]
pub struct Clouds {
    pub all: u8,
}

#[derive(Debug, Deserialize)]
pub struct Sys {
    pub country: Option<String>,
    pub sunrise: Option<u64>,
    pub sunset: Option<u64>,
}

impl WeatherResponse {
    pub fn temp_celsius(&self) -> f64 {
        self.main.temp - 273.15
    }

    pub fn temp(&self) -> f64 {
        self.main.temp
    }

    pub fn feels_like(&self) -> f64 {
        self.main.feels_like
    }

    pub fn feels_like_celsius(&self) -> f64 {
        self.main.feels_like - 273.15
    }

    pub fn feels_like_fahrenheit(&self) -> f64 {
        (self.main.feels_like - 273.15) * 9.0 / 5.0 + 32.0
    }

    pub fn temp_fahrenheit(&self) -> f64 {
        (self.main.temp - 273.15) * 9.0 / 5.0 + 32.0
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn humidity(&self) -> u8 {
        self.main.humidity
    }

    pub fn description(&self) -> &str {
        &self.weather[0].description
    }

    pub fn display(&self, units: &str) {
        println!("\n Weather in {}", self.name);

        match units.to_lowercase().as_str() {
            "metric" => {
                println!("Temperature: {:.1}°C", self.temp_celsius());
                println!("Feels like: {:.1}°C", self.feels_like_celsius());
            }
            "imperial" => {
                println!("Temperature: {:.1}°F", self.temp_fahrenheit());
                println!("Feels like: {:.1}°F", self.feels_like_fahrenheit());
            }
            "kelvin" => {
                println!("Temperature: {:.1}K", self.temp());
                println!("Feels like: {:.1}K", self.feels_like());
            }
            _ => {
                println!("Invalid units, defaulting to Celsius");
                println!("Temperature: {:.1}°C", self.temp_celsius());
            }
        }

        println!("Conditions: {}", self.description());
        println!("Humidity: {}%", self.humidity());
    }

    /// Display detailed weather information
    pub fn display_detailed(&self, units: &str) {
        // Basic info
        self.display(units);

        println!("\n Additional Details:");

        // Country
        if let Some(country) = self.country() {
            println!("Country: {}", country);
        }

        // Pressure
        if let Some(pressure) = self.pressure() {
            println!("Pressure: {} hPa", pressure);
        }

        // Visibility
        if let Some(visibility) = self.visibility_km() {
            println!("Visibility: {:.1} km", visibility);
        }

        // Wind
        println!("\n Wind:");
        match units {
            "imperial" => {
                if let Some(speed) = self.wind_speed_mph() {
                    print!("  Speed: {:.1} mph", speed);
                    if let Some(dir) = self.wind_direction() {
                        print!(" ({})", dir);
                    }
                    println!();
                }
            }
            "metric" => {
                if let Some(speed) = self.wind_speed_kmh() {
                    print!("  Speed: {:.1} km/h", speed);
                    if let Some(dir) = self.wind_direction() {
                        print!(" ({})", dir);
                    }
                    println!();
                }
            }
            _ => {
                if let Some(speed) = self.wind_speed_ms() {
                    print!("  Speed: {:.1} m/s", speed);
                    if let Some(dir) = self.wind_direction() {
                        print!(" ({})", dir);
                    }
                    println!();
                }
            }
        }

        // Clouds
        if let Some(clouds) = self.cloud_coverage() {
            println!("\n  Cloudiness: {}%", clouds);
        }

        // Sunrise/Sunset
        println!("\n Sun Times:");
        if let Some(sunrise) = self.sunrise_time() {
            println!("  Sunrise: {}", sunrise);
        }
        if let Some(sunset) = self.sunset_time() {
            println!("  Sunset: {}", sunset);
        }
    }

    /// Get wind speed in different units
    pub fn wind_speed_ms(&self) -> Option<f64> {
        self.wind.as_ref().map(|w| w.speed)
    }

    pub fn wind_speed_kmh(&self) -> Option<f64> {
        self.wind.as_ref().map(|w| w.speed * 3.6)
    }

    pub fn wind_speed_mph(&self) -> Option<f64> {
        self.wind.as_ref().map(|w| w.speed * 2.237)
    }

    /// Get wind direction as compass direction
    pub fn wind_direction(&self) -> Option<String> {
        self.wind.as_ref()?.deg.map(|deg| {
            match deg {
                0..=22 | 338..=360 => "N",
                23..=67 => "NE",
                68..=112 => "E",
                113..=157 => "SE",
                158..=202 => "S",
                203..=247 => "SW",
                248..=292 => "W",
                293..=337 => "NW",
                _ => "?",
            }
            .to_string()
        })
    }

    /// Get cloud coverage percentage
    pub fn cloud_coverage(&self) -> Option<u8> {
        self.clouds.as_ref().map(|c| c.all)
    }

    /// Get country code
    pub fn country(&self) -> Option<&str> {
        self.sys.as_ref()?.country.as_deref()
    }

    /// Get pressure in hPa
    pub fn pressure(&self) -> Option<i32> {
        self.main.pressure.map(|p| p as i32)
    }

    /// Get visibility in km
    pub fn visibility_km(&self) -> Option<f64> {
        self.visibility.map(|v| v as f64 / 1000.0)
    }

    /// Format sunrise time (returns HH:MM or None)
    pub fn sunrise_time(&self) -> Option<String> {
        let timestamp = self.sys.as_ref()?.sunrise?;
        format_timestamp(timestamp)
    }

    /// Format sunset time (returns HH:MM or None)
    pub fn sunset_time(&self) -> Option<String> {
        let timestamp = self.sys.as_ref()?.sunset?;
        format_timestamp(timestamp)
    }
}

/// Helper function to format Unix timestamp to HH:MM
fn format_timestamp(timestamp: u64) -> Option<String> {
    use chrono::DateTime;

    let date_time = DateTime::from_timestamp(timestamp as i64, 0).expect("Invalid timestamp");
    let formatted_time = date_time.format("%H:%M").to_string();

    Some(formatted_time)
}
