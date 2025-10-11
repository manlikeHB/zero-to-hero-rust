# 🦀 Learning Rust: A Project-Based Journey

A hands-on approach to mastering Rust through progressively challenging projects. This repository documents my intensive learning sprint, focusing on practical implementation over theoretical knowledge.

## 📚 Learning Philosophy

**Learn → Build → Review → Refactor**

Instead of reading exhaustively before coding, I:
1. Learn just enough from [The Rust Book](https://doc.rust-lang.org/book/) to start building
2. Implement projects that reinforce specific concepts
3. Seek feedback and review my approach
4. Refactor as I discover better patterns and idiomatic Rust

> **⚠️ Disclaimer:** This is a learning repository. Code quality improves as projects progress. Earlier projects may be revisited and refactored as I gain deeper understanding of Rust's patterns and best practices.

## 🎯 Learning Objectives

- Master Rust fundamentals: ownership, borrowing, lifetimes
- Build confidence with traits, generics, and error handling
- Understand async programming and concurrency
- Learn to work with Rust's ecosystem (crates, Cargo)
- Develop muscle memory for idiomatic Rust patterns

## 📋 Project Roadmap

### Core Concepts & Systems Thinking

| Status | Project | Focus Areas | Key Concepts |
|--------|---------|-------------|--------------|
| ✅ | [Guessing Game](./guessing_game) | Syntax basics, random numbers, loops | `rand` crate, user input, pattern matching |
| ✅ | [Temperature Converter](./temp_converter) | Functions, I/O, data types | Ownership basics, string handling |
| ✅ | [Word Counter CLI](./word_counter) | File I/O, collections | `HashMap`, file operations, iterators |
| ✅ | [To-Do List CLI](./to-do_list) | Structs, enums, persistence | Data modeling, vector operations |
| ✅ | [Simple Calculator](./simple_calculator) | Pattern matching, error handling | `match` expressions, `Result` type |
| ✅ | [Contact Book CLI](./contact_book) | Serialization, data structures | `serde`, JSON, CRUD operations |
| ✅ | [Matrix Library](./matrix_lib) | Traits, generics, operator overloading | Generic programming, `std::ops` traits |
| ✅ | [Mini CSV Parser](./mini_csv_parser) | Iterators, closures | Iterator adapters, functional patterns |
| ✅ | [Markdown → HTML Converter](./markdown_to_html_converter) | String parsing, regex, enums | Text processing, `regex` crate |

### Async, Networking & Backend

| Status | Project | Focus Areas | Key Concepts |
|--------|---------|-------------|--------------|
| ✅ | [HTTP Fetcher CLI](./http_fetcher) | Async basics, concurrency | `tokio`, `reqwest`, `async`/`await` |
| ✅ | [Chat Server (TCP)](./chat_server) | Networking, channels | TCP sockets, message passing |
| ✅ | [Weather CLI](./weather_cli/) | External APIs, JSON | REST APIs, error propagation |
| ✅ | [Mini Blockchain Ledger](./mini_blockchain/) | Systems thinking, hashing | Cryptography, data structures |
| 🔨 | Task Manager REST API | Backend fundamentals | `axum`, HTTP handlers, routing |
| 📅 | Blockchain REST API | Systems + Backend integration | Combining previous concepts |

**Legend:**
- ✅ Completed
- 🔨 In Progress
- 📅 Planned

## 🛠️ Tech Stack & Tools

- **Language:** Rust 2024 Edition
- **Package Manager:** Cargo
- **Key Crates:** `serde`, `tokio`, `axum`, `reqwest`, `regex`, `rand`, `anyhow`
- **Learning Resources:** The Rust Book, Rustlings, Exercism

## 🏗️ Repository Structure

```
.
├── README.md                    # This file
├── guessing_game/              # Level 1 projects
├── temp_converter/
├── word_counter/
├── to-do_list/
├── simple_calculator/
├── contact_book/               # Level 2 projects
├── matrix_lib/                 # Level 3 projects
├── mini_csv_parser/
├── markdown_to_html_converter/
├── http_fetcher/               # Level 4+ projects
├── chat_server/
└── weather_cli/
└── mini_blockchain/
```

Each project is a standalone Cargo workspace with its own `Cargo.toml` and dependencies.

## 🚀 Getting Started

To run any project:

```bash
cd project_name
cargo run
```

To run tests:

```bash
cargo test
```

## 📈 Progress Tracking

- **Current Focus:** Mini Blockchain Ledger
- **Completed Projects:** 12/15
- **Lines of Code Written:** ~4,000+

## 🔄 Iteration & Improvement

As I progress, earlier projects may be refactored to incorporate:
- Better error handling patterns
- More idiomatic Rust code
- Additional features that demonstrate new concepts
- Improved documentation and tests

This iterative approach mirrors real-world software development and reinforces learning.

## 🤝 Feedback Welcome

This is a learning journey, and I'm actively seeking:
- Code review and suggestions for improvement
- Idiomatic Rust patterns I might have missed
- Better approaches to problem-solving in Rust
- General feedback on project structure and architecture

## 📝 Notes

- Projects increase in complexity and introduce new Rust concepts progressively
- Each project folder contains its own README with specific learning objectives
- Some projects may have multiple iterations as I refactor with new knowledge
- Focus is on understanding **why** Rust works this way, not just **how** to write it

## 🔗 Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings)
- [Exercism Rust Track](https://exercism.org/tracks/rust)

---

*"The best way to learn Rust is to write Rust."* - Building in public, one project at a time.