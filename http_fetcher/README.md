# 🌐 HTTP Fetcher CLI

A concurrent HTTP client that fetches multiple URLs in parallel with professional features.

## 🎯 Learning Objectives

- **Async/Await Programming** - Rust's approach to non-blocking operations
- **Futures and Tokio Runtime** - How Rust handles concurrency
- **External Crates Integration** - Working with `reqwest` for HTTP requests
- **Error Handling with Async** - Managing multiple failure modes
- **Concurrent Programming Patterns** - Making multiple requests in parallel
- **Custom Error Types** - Professional error handling with `thiserror`
- **Module Organization** - Structuring larger projects

## ⚡ Features

- Fetch multiple URLs concurrently (true parallelism)
- Configurable request timeouts
- Save responses to files
- Status-only mode for quick checks
- Proper error handling with detailed messages
- Professional CLI with help and version info

## 🚀 Running the Program

```bash
cargo run -- <urls>
```

## 📝 Example Sessions

### Basic Usage
```bash
# Fetch a single URL
$ cargo run -- https://httpbin.org/get
=== Fetching: https://httpbin.org/get ===
✅ https://httpbin.org/get [200 OK]:
{
  "args": {},
  "headers": {
    ...
  }
}
Total time: 1.2s
```

### Multiple URLs (Concurrent)
```bash
$ cargo run -- https://httpbin.org/get https://httpbin.org/json
=== Fetching: https://httpbin.org/get ===
=== Fetching: https://httpbin.org/json ===
✅ https://httpbin.org/get [200 OK]: 222 bytes
✅ https://httpbin.org/json [200 OK]: 429 bytes
Total time: 1.5s
```

### With Timeout
```bash
$ cargo run -- --timeout 5 https://httpbin.org/delay/10
=== Fetching: https://httpbin.org/delay/10 ===
⏰ https://httpbin.org/delay/10: Request timed out after 5 seconds
Total time: 5.1s
```

### Status Only Mode
```bash
$ cargo run -- --status-only https://httpbin.org/get https://httpbin.org/json
=== Fetching: https://httpbin.org/get ===
=== Fetching: https://httpbin.org/json ===
✅ https://httpbin.org/get [200 OK]
✅ https://httpbin.org/json [200 OK]
Total time: 1.3s
```

### Save to Files
```bash
$ cargo run -- --save-dir ./responses https://httpbin.org/get https://httpbin.org/json
=== Fetching: https://httpbin.org/get ===
=== Fetching: https://httpbin.org/json ===
✅ https://httpbin.org/get [200 OK]: 222 bytes
✅ https://httpbin.org/json [200 OK]: 429 bytes
Total time: 1.4s

# Files saved to:
# ./responses/httpbin.org_get.txt.txt
# ./responses/httpbin.org_json.txt.txt
```

## 🔧 CLI Options

```bash
http_fetcher [OPTIONS] <URLS>...

Arguments:
  <URLS>...  URLs to fetch

Options:
  -t, --timeout <TIMEOUT>
          Timeout in seconds for each request [default: 30]
          
  -m, --max-concurrent <MAX_CONCURRENT>
          Maximum number of concurrent requests [default: 10]
          
  -s, --status-only
          Show only status and headers, not response body
          
  -d, --save-dir <SAVE_DIR>
          Directory to save responses (optional)
          
  -h, --help
          Print help
          
  -V, --version
          Print version
```

## 🔑 Key Concepts Demonstrated

### Concurrent Request Pattern
```rust
let mut handles = Vec::new();
for url in urls {
    let handle = tokio::spawn(async move {
        let result = client.get(&url).send().await;
        (url_clone, result)
    });
    handles.push(handle);
}

// Collect all results
for handle in handles {
    let (url, result) = handle.await?;
    // Process result...
}
```

### Custom Error Handling
```rust
#[derive(Error, Debug)]
pub enum FetcherError {
    #[error("Request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    
    #[error("IO operation failed: {0}")]
    IOError(#[from] std::io::Error),
    
    #[error("No urls provided")]
    NoUrls,
}
```

### Module Organization
```
src/
├── main.rs         # CLI entry point
├── lib.rs          # Public API
├── client.rs       # HTTP client logic
├── config.rs       # CLI configuration
├── utils.rs        # Utility functions
└── error.rs        # Custom error types
```

## 💡 What I Learned

1. **Async vs Concurrent** - Understanding the difference between async operations and true parallelism
2. **Tokio Runtime** - How `#[tokio::main]` sets up the async runtime
3. **HTTP Client Configuration** - Building custom clients vs using convenience functions
4. **Ownership in Async** - Cloning data for async tasks, understanding move semantics
5. **Error Type Design** - Creating custom error types with `thiserror`
6. **Module System** - Organizing code into logical modules with proper visibility
7. **CLI Parsing** - Using `clap` derive macros for professional argument parsing

## 🧪 Performance Comparison

**Test with delay URLs:**
```bash
# Sequential (hypothetical): 2 + 1 + 3 = 6 seconds
# Concurrent (actual): ~3-4 seconds (limited by slowest request)

$ cargo run -- https://httpbin.org/delay/2 https://httpbin.org/delay/1 https://httpbin.org/delay/3
Total time: 4.5s  # ~40% faster!
```

## 🔄 Architecture

### Request Flow
1. **CLI Parsing** - Parse arguments with `clap`
2. **Client Creation** - Build configured HTTP client with timeout
3. **Concurrent Spawning** - Launch all requests in parallel with `tokio::spawn`
4. **Result Collection** - Await all tasks and handle results individually
5. **Output/Save** - Display or save responses based on flags

### Error Handling Strategy
- **Individual URL failures** - Continue processing other URLs
- **Timeout handling** - Specific error message for timeouts
- **File I/O errors** - Graceful degradation, report but continue
- **Fatal errors** - Exit only on setup failures (no URLs, client creation)

## 📦 Dependencies

```toml
[dependencies]
clap = { version = "4.5.48", features = ["derive"] }
reqwest = "0.12.23"
thiserror = "2.0.16"
tokio = { version = "1.47.1", features = ["full"] }
```

## 🔄 Possible Improvements

- [ ] Progress bars with `indicatif`
- [ ] Actual concurrency limiting with semaphores
- [ ] JSON pretty-printing with `serde_json`
- [ ] Response streaming for large files
- [ ] Retry logic with exponential backoff
- [ ] HTTP/2 and HTTP/3 support
- [ ] Custom headers support
- [ ] Authentication options
- [ ] Request body support (POST/PUT)
- [ ] Integration tests with `mockito`

## 📚 Relevant Rust Concepts

- [Async Programming in Rust](https://rust-lang.github.io/async-book/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Error Handling with thiserror](https://docs.rs/thiserror/)
- [CLI Parsing with clap](https://docs.rs/clap/)

---

**Status**: ✅ Completed | **Difficulty**: Intermediate-Advanced