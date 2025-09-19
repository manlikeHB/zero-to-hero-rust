use crate::types::{Config, MarkdownElement};
use anyhow::Result;

/// Parses markdown content into structured elements
///
/// # Arguments
/// * `content` - The markdown text to parse
///
/// # Returns
/// * `Result<Vec<MarkdownElement>>` - Parsed elements or error
///
/// # Errors
/// * Returns error if header level exceeds 6
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
}
