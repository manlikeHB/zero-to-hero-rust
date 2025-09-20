//! # Markdown to HTML Converter
//!
//! A fast, configurable Markdown to HTML converter written in Rust.
//!
//! This library provides a flexible architecture for parsing Markdown documents
//! and rendering them to various output formats. Currently supports HTML output
//! with plans for additional formats.
//!
//! ## Features
//!
//! - **Headers** - All 6 levels (`#` through `######`)
//! - **Text formatting** - Bold (`**text**`), italic (`*text*`), and inline code (`` `code` ``)
//! - **Links** - Standard markdown links (`[text](url)`)
//! - **Lists** - Unordered lists with proper grouping
//! - **Error handling** - Comprehensive error reporting with `anyhow`
//! - **Configurable** - Flexible configuration with builder pattern
//! - **Extensible** - Trait-based rendering system
//!
//! ## Quick Start
//!
//! ```rust
//! use markdown_to_html_converter::{Config, HtmlRenderer, MarkdownElement, Renderer};
//!
//! // Create configuration
//! let config = Config::new("input.md", "output.html")
//!     .with_full_html(true)
//!     .with_max_header_level(4);
//!
//! // Create renderer
//! let renderer = HtmlRenderer::new(config);
//!
//! // Example of rendering elements directly
//! let elements = vec![
//!     MarkdownElement::Header(1, "Title".to_string()),
//!     MarkdownElement::Paragraph("Hello world!".to_string()),
//! ];
//!
//! let html = renderer.render(&elements).unwrap();
//! assert!(html.contains("<h1>Title</h1>"));
//! assert!(html.contains("<p>Hello world!</p>"));
//! ```
//!
//! ## Architecture
//!
//! The library is organized into several modules:
//!
//! - [`types`] - Core data structures and configuration
//! - [`parser`] - Markdown parsing logic  
//! - [`html`] - HTML rendering implementation
//!
//! ## Adding New Output Formats
//!
//! The library uses a trait-based system that makes adding new output formats simple:
//!
//! ```rust
//! use markdown_to_html_converter::{Renderer, MarkdownElement};
//! use anyhow::Result;
//!
//! struct PlainTextRenderer;
//!
//! impl Renderer for PlainTextRenderer {
//!     fn render(&self, elements: &[MarkdownElement]) -> Result<String> {
//!         // Your implementation here
//!         todo!()
//!     }
//!     
//!     fn render_element(&self, element: &MarkdownElement) -> Result<String> {
//!         // Your implementation here  
//!         todo!()
//!     }
//! }
//! ```

pub mod file;
pub mod html;
pub mod parser;
pub mod types;

// Re-export commonly used items for convenience
pub use html::HtmlRenderer;
pub use parser::parse_md;
pub use types::{Config, MarkdownElement, Renderer};
