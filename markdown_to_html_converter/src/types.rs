use anyhow::Result;
use std::default::Default;

/// Represents different elements that can appear in a Markdown document
#[derive(Debug)]
pub enum MarkdownElement {
    /// A header with level (1-6) and text content
    Header(u8, String),
    /// A paragraph of text
    Paragraph(String),
    /// A list item
    List(String),
}

/// Configuration options for the markdown converter
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
pub trait Renderer {
    /// Render a list of markdown elements to a string
    fn render(&self, elements: &[MarkdownElement]) -> Result<String>;
    /// Render a single markdown element
    fn render_element(&self, element: &MarkdownElement) -> Result<String>;
}

impl Config {
    /// Create a new config with custom input/output paths
    pub fn new(input_path: &str, output_path: &str) -> Self {
        Self {
            input_path: input_path.to_string(),
            output_path: output_path.to_string(),
            ..Default::default()
        }
    }

    /// Builder pattern for full HTML document
    pub fn with_full_html(mut self, full_html: bool) -> Self {
        self.full_html_document = full_html;
        self
    }

    /// Builder pattern for max header level
    pub fn with_max_header_level(mut self, header_level: u8) -> Self {
        self.max_header_level = header_level;
        self
    }

    pub fn with_input_path(mut self, input_path: &str) -> Self {
        self.input_path = input_path.to_string();
        self
    }

    pub fn with_output_path(mut self, output_path: &str) -> Self {
        self.output_path = output_path.to_string();
        self
    }
}
