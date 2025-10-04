# 🌤️ Weather CLI

A command-line weather application that fetches current weather data from OpenWeatherMap API, built as part of my Rust learning journey.

## 🎯 Learning Objectives

- **Async Programming**: Using `tokio` for async/await and concurrent requests
- **Error Handling**: Custom error types with `thiserror` and `anyhow`
- **JSON Parsing**: Deserializing complex nested JSON with `serde`
- **CLI Development**: Building user-friendly CLIs with `clap`
- **HTTP Requests**: Making API calls with `reqwest`
- **Option Handling**: Working with `Option<T>`, `.as_ref()`, `.as_deref()`, and the `?` operator
- **Type Safety**: Choosing appropriate types and handling nullable fields
- **Concurrent Execution**: Spawning multiple async tasks with `tokio::spawn`

## ✨ Features

- 🌍 Fetch weather for multiple cities simultaneously
- 🌡️ Support for multiple temperature units (Celsius, Fahrenheit, Kelvin)
- 📊 Detailed weather information (wind speed/direction, humidity, pressure, sunrise/sunset)
- ⚡ Concurrent API requests for fast performance
- 🔒 Secure API key management with environment variables
- 🎯 Clean error handling with context-specific messages
- 🕐 Formatted sunrise/sunset times using `chrono`

## 📋 Prerequisites

- Rust 1.70 or higher
- OpenWeatherMap API key (get one free at [openweathermap.org](https://openweathermap.org/api))

## 🚀 Installation

1. Clone the repository:
```bash
git clone <your-repo-url>
cd weather_cli
```

2. Create a `.env` file in the project root:
```bash
OPENWEATHER_API_KEY=your_api_key_here
```

3. Build the project:
```bash
cargo build --release
```

## 💻 Usage

### Basic Usage

```bash
# Single city (default: metric/Celsius)
cargo run -- London

# Multiple cities
cargo run -- London Paris Tokyo

# Imperial units (Fahrenheit)
cargo run -- "New York" --units imperial

# Detailed information
cargo run -- London --detailed

# Short flags
cargo run -- Paris -u metric -d
```

### Command-Line Options

- `cities`: One or more city names (required)
- `-u, --units <UNITS>`: Temperature units: `metric`, `imperial`, or `kelvin` (default: `metric`)
- `-d, --detailed`: Show detailed weather information including wind, pressure, visibility, and sun times
- `-h, --help`: Print help information

### Examples

```bash
# Get weather for multiple cities in Fahrenheit
cargo run -- "New York" Miami Seattle --units imperial

# Detailed weather for London in Celsius
cargo run -- London --detailed

# Multiple cities with detailed info
cargo run -- London Paris Tokyo -d

# Single city in Kelvin
cargo run -- Tokyo --units kelvin
```

## 🏗️ Project Structure

```
weather_cli/
├── src/
│   ├── main.rs       # Application entry point, async runtime, concurrent fetching
│   ├── models.rs     # Data structures, helper methods, display logic
│   ├── cli.rs        # CLI argument parsing with clap
│   └── error.rs      # Custom error types with thiserror
├── Cargo.toml        # Dependencies and project metadata
├── .env              # API key (not committed to version control)
└── README.md         # This file
```

## 🧪 Example Output

### Basic Output
```
🌤️ Weather in London
Temperature: 11.8°C
Feels like: 11.3°C
Conditions: overcast clouds
Humidity: 87%
```

### Detailed Output
```
🌤️ Weather in London
Temperature: 11.8°C
Feels like: 11.3°C
Conditions: overcast clouds
Humidity: 87%

📊 Additional Details:
Country: GB
Pressure: 1012 hPa
Visibility: 10.0 km

💨 Wind:
  Speed: 14.8 km/h (SW)

☁️ Cloudiness: 75%

🌅 Sun Times:
  Sunrise: 07:30
  Sunset: 16:45
```

## 📦 Dependencies

```toml
[dependencies]
anyhow = "1.0"        # Flexible error handling for applications
chrono = "0.4"        # Date and time library
clap = { version = "4.5", features = ["derive"] }  # CLI argument parsing
dotenv = "0.15"       # Environment variable management
reqwest = { version = "0.12", features = ["json"] }  # HTTP client
serde = { version = "1.0", features = ["derive"] }   # Serialization framework
serde_json = "1.0"    # JSON support for serde
thiserror = "2.0"     # Custom error type derivation
tokio = { version = "1", features = ["full"] }       # Async runtime
```

## 💡 Key Implementation Details

### Custom Error Types
```rust
#[derive(Error, Debug)]
pub enum WeatherError {
    #[error("City not found: {0}")]
    CityNotFound(String),
    
    #[error("Invalid API key")]
    InvalidApiKey,
    
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    
    #[error("Invalid units '{0}'. Use: metric, imperial, or kelvin")]
    InvalidUnits(String),
}
```

### Concurrent API Requests
```rust
for city in &cli.cities {
    let handle = tokio::spawn(async move {
        fetch_weather(&city, &api_key).await
    });
    handles.push((city.clone(), handle));
}
```

### Option Chaining Patterns
```rust
// Double ? pattern for nested Options
pub fn sunrise_time(&self) -> Option<String> {
    let timestamp = self.sys.as_ref()?.sunrise?;
    format_timestamp(timestamp)
}

// as_deref() for Option<String> → Option<&str>
pub fn country(&self) -> Option<&str> {
    self.sys.as_ref()?.country.as_deref()
}
```

## 🧠 What I Learned

### Async/Await Fundamentals
- How `#[tokio::main]` creates an async runtime
- Using `tokio::spawn` for concurrent task execution
- Awaiting multiple futures with handles

### Error Handling Patterns
- Creating typed errors with `thiserror`
- Using `anyhow::Result` in application code
- Automatic error conversion with `#[from]`
- Error propagation with the `?` operator

### Working with Options
- `.as_ref()` to borrow from `Option<T>` → `Option<&T>`
- `.as_deref()` to convert `Option<String>` → `Option<&str>`
- Chaining `?` for early returns on `None`
- `.map()` for transforming inner values

### JSON Deserialization
- Deriving `Deserialize` for automatic JSON parsing
- Using `Option<T>` for nullable API fields
- Nested struct deserialization
- Type-safe data modeling

### CLI Design
- Using `clap`'s derive macros for argument parsing
- Default values and validation
- Multiple arguments with `Vec<String>`
- Short and long flag variants

## 🔄 Possible Improvements

- [ ] Cache weather data to reduce API calls
- [ ] Output formatting options (JSON, table)
- [ ] Colored terminal output
- [ ] Progress indicators for multiple cities
- [ ] Timezone-aware sunrise/sunset times
- [ ] Unit tests for helper functions
- [ ] Integration tests for API calls (with mocking)
- [ ] Config file for saving user preferences

## 📚 Relevant Rust Book Chapters

- [Chapter 12: Building a Command Line Program](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
- [Chapter 16: Fearless Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- [Chapter 17: Async and Await](https://doc.rust-lang.org/book/ch17-00-async-await.html)

## 📝 License

MIT

---

**Status**: ✅ Completed | **Difficulty**: Intermediate