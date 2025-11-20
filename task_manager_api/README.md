# 🚀 Task Manager REST API

A production-ready REST API built with Rust and Axum, featuring advanced querying, filtering, sorting, and pagination capabilities.

## 🎯 Learning Objectives

- **REST API Design**: Building RESTful endpoints with proper HTTP methods and status codes
- **Async Programming**: Using Tokio for non-blocking concurrent request handling
- **Thread-Safe State**: Managing shared mutable state with `Arc<Mutex<T>>`
- **Query Parameters**: Extracting and processing complex URL parameters
- **Custom Error Handling**: Creating type-safe error responses with `IntoResponse`
- **Generic Types**: Building reusable response structures with generics
- **Complex Type Patterns**: Working with `Option<Option<T>>` for PATCH semantics
- **Integration Testing**: Testing complete HTTP workflows with Tower
- **Code Organization**: Separating concerns across multiple modules
- **Functional Programming**: Using iterators, closures, and combinators

## ✨ Features

- **Full CRUD Operations**: Create, read, update, and delete tasks
- **Advanced Filtering**: Filter tasks by completion status
- **Search Functionality**: Case-insensitive full-text search across task titles
- **Multi-Field Sorting**: Sort by title, creation date, or completion status (ascending/descending)
- **Pagination**: Efficient pagination with metadata (page, limit, total, total_pages)
- **Composable Queries**: Combine all features in a single request
- **Thread-Safe**: Concurrent request handling with Rust's safety guarantees
- **Type-Safe**: Compile-time guarantees through Rust's type system

## 🚀 Running the API
```bash
# Start the server
cargo run

# Server starts on http://127.0.0.1:3000
```

## 📝 Example Usage

### Create a Task
```bash
curl -X POST http://localhost:3000/tasks \
  -H "Content-Type: application/json" \
  -d '{"title": "Learn Rust", "description": "Build a REST API"}'

# Response (201 Created):
{
  "id": 1,
  "title": "Learn Rust",
  "description": "Build a REST API",
  "completed": false,
  "created_at": "2025-11-20T14:30:00Z"
}
```

### List All Tasks
```bash
curl http://localhost:3000/tasks

# Response:
{
  "data": [
    {"id": 1, "title": "Learn Rust", ...},
    {"id": 2, "title": "Build API", ...}
  ],
  "pagination": {
    "page": 1,
    "limit": 2,
    "total": 2,
    "total_pages": 1
  }
}
```

### Filter Completed Tasks
```bash
curl "http://localhost:3000/tasks?completed=true"
```

### Search and Sort
```bash
curl "http://localhost:3000/tasks?search=rust&sort_by=title&order=asc"
```

### Paginate Results
```bash
curl "http://localhost:3000/tasks?page=1&limit=10"
```

### Combined Query
```bash
curl "http://localhost:3000/tasks?completed=false&search=api&sort_by=created_at&order=desc&page=1&limit=5"
```

### Update a Task
```bash
curl -X PATCH http://localhost:3000/tasks/1 \
  -H "Content-Type: application/json" \
  -d '{"completed": true}'
```

### Delete a Task
```bash
curl -X DELETE http://localhost:3000/tasks/1
```

## 🔑 Key Concepts Demonstrated

### Thread-Safe Shared State
```rust
pub struct TaskRepository {
    tasks: Arc<Mutex<HashMap<u64, Task>>>,
    next_id: Arc<Mutex<u64>>,
}

// Arc for shared ownership across threads
// Mutex for safe mutation
```

### Custom Error Handling
```rust
pub enum ApiError {
    NotFound,
    BadRequest(String),
    InternalError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Resource not found"),
            // ...
        };
        (status, Json(json!({"error": error_message}))).into_response()
    }
}
```

### Option<Option<T>> for Partial Updates
```rust
#[derive(Deserialize)]
pub struct UpdateTaskRequest {
    pub title: Option<String>,
    pub description: Option<Option<String>>,  // Distinguishes missing vs null
    pub completed: Option<bool>,
}

// None = field not provided
// Some(None) = field set to null
// Some(Some("text")) = field set to value
```

### Generic Pagination Response
```rust
#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: PaginationMeta,
}

// Works with any data type
```

### Axum Extractors
```rust
async fn list_tasks(
    State(repo): State<Arc<TaskRepository>>,  // Shared state
    Query(query): Query<TaskQuery>,           // URL parameters
) -> Json<PaginatedResponse<Task>> {
    // Axum automatically extracts and deserializes
}
```

### Filtering with Closures
```rust
tasks
    .into_iter()
    .filter(|task| {
        let matches_completed = completed.map_or(true, |f| task.completed == f);
        let matches_search = search.as_ref().map_or(true, |term| {
            task.title.to_lowercase().contains(&term.to_lowercase())
        });
        matches_completed && matches_search
    })
    .collect()
```

### Dynamic Sorting
```rust
filtered.sort_by(|a, b| {
    let cmp = match field {
        SortField::Title => a.title.to_lowercase().cmp(&b.title.to_lowercase()),
        SortField::CreatedAt => a.created_at.cmp(&b.created_at),
        SortField::Completed => a.completed.cmp(&b.completed),
    };
    match order {
        SortOrder::Asc => cmp,
        SortOrder::Desc => cmp.reverse(),
    }
});
```

### Pagination Math
```rust
let offset = page.saturating_sub(1) * limit;
let paginated = tasks
    .into_iter()
    .skip(offset as usize)
    .take(limit as usize)
    .collect();
```

## 💡 What I Learned

1. **Arc<Mutex<T>> Pattern**: Understanding when and why to use atomic reference counting with mutexes for thread-safe shared state
2. **Axum Extractors**: How web frameworks use type-safe extraction for request data
3. **Custom Serde Deserialization**: Solving the "missing vs null" problem with `deserialize_with`
4. **Option Combinators**: Using `.map_or()`, `.as_ref()`, `.ok_or()` for clean optional handling
5. **Iterator Chaining**: Composing `.filter()`, `.skip()`, `.take()`, `.collect()` for data processing
6. **Separation of Concerns**: Organizing code into layers (handlers, repository, utils, models)
7. **Integration Testing**: Using Tower's `oneshot()` to test HTTP endpoints without network calls
8. **Builder Patterns**: Creating ergonomic APIs with method chaining
9. **Type-Safe Enums**: Using Serde's `rename_all` for clean JSON mapping
10. **Lock Scope Management**: Releasing locks early to maximize concurrency

## 🧪 Testing
```bash
# Run all tests (39 integration tests)
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_pagination
```

Tests cover:
- All CRUD operations
- Filtering by completion
- Search functionality
- Sorting (all fields, both orders)
- Pagination (with filters)
- Error cases (404, validation)
- Edge cases (empty results, out-of-bounds pages)

## 🏗️ Project Structure
```
task_manager_api/
├── src/
│   ├── lib.rs           # Application setup
│   ├── handlers.rs      # handlers
│   ├── main.rs          # Entry point
│   ├── models.rs        # Data models, request/response types
│   ├── repository.rs    # Data access layer
│   ├── errors.rs        # Custom error types
│   ├── sort.rs          # Sorting enums
│   └── utils.rs         # Filtering, sorting, pagination utilities
├── tests/
│   └── api_tests.rs     # Integration tests
└── Cargo.toml
```

## 🔄 Possible Improvements

- [ ] Database persistence (SQLite/PostgreSQL with `sqlx`)
- [ ] User authentication (JWT tokens)
- [ ] Rate limiting (per-IP or per-user)
- [ ] Request logging with `tracing`
- [ ] Input validation with `validator` crate
- [ ] Task due dates and priorities
- [ ] Task tags/categories
- [ ] Bulk operations (delete multiple, update multiple)
- [ ] Search in description field
- [ ] Export tasks (JSON, CSV)
- [ ] Docker containerization
- [ ] CI/CD pipeline
- [ ] API documentation (OpenAPI/Swagger)
- [ ] WebSocket support for real-time updates

## 📦 Dependencies
```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }

[dev-dependencies]
tower = { version = "0.4", features = ["util"] }
http-body-util = "0.1"
```

## 📚 Relevant Rust Concepts

- [Chapter 15: Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html) - `Arc`, `Mutex`
- [Chapter 16: Fearless Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- [Chapter 17: Async Programming](https://doc.rust-lang.org/book/ch17-00-async-await.html)
- [Axum Documentation](https://docs.rs/axum/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)

---

**Status**: ✅ Completed | **Difficulty**: Intermediate-Advanced