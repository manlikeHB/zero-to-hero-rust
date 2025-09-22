# 🌐 HTTP Fetcher CLI

A concurrent HTTP client that fetches multiple URLs in parallel and benchmarks response times.

## 🎯 What I'm Building

A command-line tool that:
- Accepts multiple URLs as arguments
- Fetches them concurrently using async/await
- Displays response status and timing information
- Demonstrates Rust's async ecosystem with `tokio` and `reqwest`

## 🎯 Learning Objectives

- **Async/Await**: Understanding Rust's async programming model
- **Tokio Runtime**: Using the async runtime
- **Reqwest**: Making HTTP requests
- **Concurrency**: Spawning concurrent tasks with `tokio::spawn`
- **Error Handling**: Dealing with network errors gracefully
- **Benchmarking**: Measuring performance with `Instant`

## 📦 Dependencies

```toml
[dependencies]
reqwest = "0.12"
tokio = { version = "1.47", features = ["full"] }
```

---

**Status**: 🔨 In Progress | **Difficulty**: Intermediate