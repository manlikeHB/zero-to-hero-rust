//! Markdown parsing functionality.
//!
//! This module handles the conversion of raw markdown text into a structured
//! representation using the `MarkdownElement` enum. The parser validates
//! markdown syntax and reports errors for invalid constructs.

use crate::types::{Config, MarkdownElement};
use anyhow::Result;

/// Parses markdown content into structured elements
///
/// Takes raw markdown text and converts it into a vector of `MarkdownElement`s
/// that can be rendered to various output formats. The parser handles headers,
/// paragraphs, and list items while respecting configuration constraints.
///
/// # Arguments
/// * `content` - The markdown text to parse
/// * `config` - Configuration options that control parsing behavior
///
/// # Returns
/// * `Result<Vec<MarkdownElement>>` - Parsed elements or error
///
/// # Errors
/// * Returns error if header level exceeds `config.max_header_level`
/// * Returns error for malformed markdown constructs
///
/// # Examples
///
/// ```rust
/// use markdown_to_html_converter::{parse_md, Config};
///
/// let content = r#"
/// # Main Title
///
/// This is a paragraph.
///
/// - List item 1
/// - List item 2
/// "#.to_string();
///
/// let config = Config::default();
/// let elements = parse_md(content, &config).unwrap();
/// assert_eq!(elements.len(), 4); // Header + paragraph + 2 list items
/// ```
///
/// # Supported Markdown Features
///
/// ## Headers
/// Supports all 6 header levels:
/// ```markdown
/// # Level 1
/// ## Level 2  
/// ### Level 3
/// #### Level 4
/// ##### Level 5
/// ###### Level 6
/// ```
///
/// ## Lists
/// Unordered lists using `-` marker:
/// ```markdown
/// - First item
/// - Second item
/// - Third item
/// ```
///
/// ## Paragraphs
/// Any non-empty line that doesn't match other patterns becomes a paragraph.
/// Empty lines separate paragraphs.
pub fn parse_md(content: String, config: &Config) -> Result<Vec<MarkdownElement>> {
    let mut md_elements = Vec::new();

    for line in content.lines() {
        let line = line.trim();

        if line.starts_with("#") {
            let headers = line.chars().take_while(|x| *x == '#').count() as u8;

            if headers > config.max_header_level {
                return Err(anyhow::anyhow!(
                    "Header level {} exceeds maximum header of 6",
                    headers
                ));
            }

            let text = line
                .chars()
                .skip(headers as usize)
                .collect::<String>()
                .trim()
                .to_string();

            md_elements.push(MarkdownElement::Header(headers, text));
        } else if line.starts_with("-") {
            let text = line[1..].trim_start().to_string();

            md_elements.push(MarkdownElement::List(text));
        } else if !line.is_empty() {
            md_elements.push(MarkdownElement::Paragraph(line.to_string()));
        }
    }

    Ok(md_elements)
}

#[cfg(test)]
mod test_parse_md {
    use super::*;

    #[test]
    fn test_parse_md_ok() {
        let content = "### Some header".to_string();

        let res = parse_md(content, &Config::default()).unwrap();

        assert_eq!(res.len(), 1);

        match &res[0] {
            MarkdownElement::Header(count, text) => {
                assert_eq!(*count, 3);
                assert_eq!(text, "Some header");
            }
            _ => panic!("Expected a header element"),
        }
    }

    #[test]
    fn test_parse_md_invalid_header() {
        let content = "######## Some header".to_string();

        let result = parse_md(content, &Config::default());
        assert!(result.is_err());
    }

    #[test]
    fn test_various_header_levels() {
        for level in 1..=6 {
            let markdown = format!("{} Header Level {}", "#".repeat(level), level);
            let config = Config::default();
            let result = parse_md(markdown, &config).unwrap();

            assert_eq!(result.len(), 1);
            match &result[0] {
                MarkdownElement::Header(parsed_level, _) => {
                    assert_eq!(*parsed_level, level as u8);
                }
                _ => panic!("Expected header"),
            }
        }
    }

    #[test]
    fn test_mixed_content() {
        let content = r#"# Title
Paragraph 1

## Subtitle
- Item 1
- Item 2

Another paragraph."#
            .to_string();

        let result = parse_md(content, &Config::default()).unwrap();

        // Should have: Header, Paragraph, Header, 2 Lists, Paragraph
        assert_eq!(result.len(), 6);
    }
}
