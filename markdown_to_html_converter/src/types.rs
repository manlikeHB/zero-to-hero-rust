//! Core data structures and configuration for the markdown converter.
//!
//! This module defines the fundamental types used throughout the library,
//! including the abstract syntax tree for markdown elements and configuration
//! options for customizing conversion behavior.

use anyhow::Result;
use std::default::Default;

/// Represents different elements that can appear in a Markdown document
///
/// This enum captures the structure of parsed markdown content before
/// it gets rendered to a specific output format.
///
/// # Examples
///
/// ```rust
/// use markdown_to_html_converter::MarkdownElement;
///
/// let header = MarkdownElement::Header(1, "Title".to_string());
/// let paragraph = MarkdownElement::Paragraph("Some text".to_string());
/// let list_item = MarkdownElement::List("List item".to_string());
/// ```
#[derive(Debug)]
pub enum MarkdownElement {
    /// A header with level (1-6) and text content
    ///
    /// # Examples
    /// - `Header(1, "Main Title")` represents `# Main Title`
    /// - `Header(3, "Subsection")` represents `### Subsection`
    Header(u8, String),

    /// A paragraph of text
    ///
    /// Contains the raw text content which may include inline formatting
    /// that will be processed during rendering.
    Paragraph(String),

    /// A list item
    ///
    /// Individual list items are grouped together during HTML rendering
    /// to create proper `<ul>` structures.
    List(String),
}

/// Configuration options for the markdown converter
///
/// Provides fine-grained control over parsing and rendering behavior.
/// Uses the builder pattern for ergonomic configuration.
///
/// # Examples
///
/// ```rust
/// use markdown_to_html_converter::Config;
///
/// // Basic configuration
/// let config = Config::new("input.md", "output.html");
///
/// // Advanced configuration with builder pattern
/// let config = Config::new("document.md", "document.html")
///     .with_full_html(true)
///     .with_max_header_level(4);
/// ```
#[derive(Debug, Clone)]
pub struct Config {
    /// Input file path
    pub input_path: String,
    /// Output file path  
    pub output_path: String,
    /// Whether to wrap output in full HTML document
    pub full_html_document: bool,
    /// Maximum allowed header level (1-6)
    pub max_header_level: u8,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            input_path: "test.md".to_string(),
            output_path: "output.html".to_string(),
            full_html_document: false,
            max_header_level: 6,
        }
    }
}

/// Trait for rendering markdown elements to different output formats
///
/// This trait enables the library to support multiple output formats
/// while maintaining a consistent interface. Implementors define how
/// to convert the abstract syntax tree to their target format.
///
/// # Examples
///
/// ```rust
/// use markdown_to_html_converter::{Renderer, MarkdownElement};
/// use anyhow::Result;
///
/// struct CustomRenderer;
///
/// impl Renderer for CustomRenderer {
///     fn render(&self, elements: &[MarkdownElement]) -> Result<String> {
///         elements.iter()
///             .map(|e| self.render_element(e))
///             .collect::<Result<Vec<_>>>()
///             .map(|parts| parts.join("\n"))
///     }
///     
///     fn render_element(&self, element: &MarkdownElement) -> Result<String> {
///         match element {
///             MarkdownElement::Header(level, text) => {
///                 Ok(format!("{} {}", "#".repeat(*level as usize), text))
///             }
///             MarkdownElement::Paragraph(text) => Ok(text.clone()),
///             MarkdownElement::List(text) => Ok(format!("- {}", text)),
///         }
///     }
/// }
/// ```
pub trait Renderer {
    /// Render a list of markdown elements to a string
    ///
    /// Takes a slice of parsed markdown elements and converts them
    /// to the target output format.
    ///
    /// # Arguments
    /// * `elements` - The parsed markdown elements to render
    ///
    /// # Returns
    /// * `Result<String>` - The rendered output or an error
    fn render(&self, elements: &[MarkdownElement]) -> Result<String>;

    /// Render a single markdown element
    ///
    /// Converts an individual markdown element to the target format.
    /// This method is typically called by `render` for each element.
    ///
    /// # Arguments
    /// * `element` - The markdown element to render
    ///
    /// # Returns
    /// * `Result<String>` - The rendered element or an error
    fn render_element(&self, element: &MarkdownElement) -> Result<String>;
}

impl Config {
    /// Create a new config with custom input/output paths
    ///
    /// # Arguments
    /// * `input_path` - Path to the input markdown file
    /// * `output_path` - Path where the output should be written
    ///
    /// # Examples
    ///
    /// ```rust
    /// use markdown_to_html_converter::Config;
    ///
    /// let config = Config::new("README.md", "README.html");
    /// ```
    pub fn new(input_path: &str, output_path: &str) -> Self {
        Self {
            input_path: input_path.to_string(),
            output_path: output_path.to_string(),
            ..Default::default()
        }
    }

    /// Builder pattern for full HTML document
    ///
    /// When enabled, the output will be wrapped in a complete HTML document
    /// with `<html>`, `<head>`, and `<body>` tags.
    ///
    /// # Arguments
    /// * `full_html` - Whether to generate a complete HTML document
    ///
    /// # Examples
    ///
    /// ```rust
    /// use markdown_to_html_converter::Config;
    ///
    /// let config = Config::new("input.md", "output.html")
    ///     .with_full_html(true);
    /// ```
    pub fn with_full_html(mut self, full_html: bool) -> Self {
        self.full_html_document = full_html;
        self
    }

    /// Builder pattern for max header level
    ///
    /// Sets the maximum allowed header level. Headers exceeding this
    /// level will cause parsing to fail with an error.
    ///
    /// # Arguments
    /// * `header_level` - Maximum header level (1-6)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use markdown_to_html_converter::Config;
    ///
    /// let config = Config::new("input.md", "output.html")
    ///     .with_max_header_level(4); // Only allow # through ####
    /// ```
    pub fn with_max_header_level(mut self, header_level: u8) -> Self {
        self.max_header_level = header_level;
        self
    }

    /// Builder pattern for input path
    ///
    /// # Arguments
    /// * `input_path` - Path to the input markdown file
    pub fn with_input_path(mut self, input_path: &str) -> Self {
        self.input_path = input_path.to_string();
        self
    }

    /// Builder pattern for output path
    ///
    /// # Arguments
    /// * `output_path` - Path where the output should be written
    pub fn with_output_path(mut self, output_path: &str) -> Self {
        self.output_path = output_path.to_string();
        self
    }
}
