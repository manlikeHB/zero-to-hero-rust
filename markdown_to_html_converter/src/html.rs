use crate::file::{read_md_file, write_html_file};
use crate::parser::parse_md;
use crate::types::{Config, MarkdownElement, Renderer};
use anyhow::Result;
use regex::Regex;
use std::sync::LazyLock;

static BOLD_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\*\*([^*]+)\*\*").unwrap());
static ITALICS_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\*([^*]+)\*").unwrap());
static CODE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"`([^`]+)`").unwrap());
static LINK_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\[(.*?)\]\((.*?)\)").unwrap());

pub struct HtmlRenderer {
    pub config: Config,
}

impl HtmlRenderer {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn convert_file(&self) -> Result<()> {
        let res = read_md_file(&self.config.input_path)?;
        let elements = parse_md(res, &self.config)?;

        let html = self.render(&elements[0..])?;

        write_html_file(html, &self.config.output_path)?;
        Ok(())
    }
}

impl Renderer for HtmlRenderer {
    fn render(&self, elements: &[MarkdownElement]) -> Result<String> {
        let mut html_elements = Vec::new();

        for el in elements {
            match el {
                MarkdownElement::Header(x, y) => html_elements.push(parse_header(*x, y)),
                MarkdownElement::List(text) => html_elements.push(parse_list(text)),
                MarkdownElement::Paragraph(text) => html_elements.push(parse_paragraph(text)),
            }
        }

        Ok(group_list(&html_elements).join("\n"))
    }

    fn render_element(&self, element: &MarkdownElement) -> Result<String> {
        match element {
            MarkdownElement::Header(x, y) => Ok(parse_header(*x, y)),
            MarkdownElement::List(text) => Ok(parse_list(text)),
            MarkdownElement::Paragraph(text) => Ok(parse_paragraph(text)),
        }
    }
}

pub fn parse_header(count: u8, text: &str) -> String {
    let text = parse_inner(text);
    format!("<h{}>{}</h{}>", count, text, count)
}

pub fn parse_paragraph(text: &str) -> String {
    let text = parse_inner(text);
    format!("<p>{}</p>", text)
}

pub fn parse_list(text: &str) -> String {
    let text = parse_inner(text);
    format!("<li>{}</li>", text)
}

// Note: Complex nested formatting like **bold with *italic* inside**
// requires lookahead/lookbehind regex features not supported by Rust's
// regex crate. This handles the majority of real-world cases correctly.
pub fn parse_inner(text: &str) -> String {
    let replaced = BOLD_REGEX
        .replace_all(&text, "<strong>$1</strong>")
        .to_string();
    let replaced = ITALICS_REGEX
        .replace_all(&replaced, "<em>$1</em>")
        .to_string();
    let replaced = CODE_REGEX
        .replace_all(&replaced, "<code>$1</code>")
        .to_string();
    LINK_REGEX
        .replace_all(&replaced, r#"<a href="$2">$1</a>"#)
        .to_string()
}

pub fn group_list(html_el: &Vec<String>) -> Vec<String> {
    let mut new_html = Vec::new();
    let mut new_group = Vec::new();

    for i in 0..html_el.len() {
        let cur = html_el.get(i).unwrap();
        let next = html_el.get(i + 1);

        if cur.starts_with("<li>") {
            new_group.push(cur.clone());
        } else {
            new_html.push(cur.clone());
        }

        if next.is_some() && !next.unwrap().starts_with("<li>") && new_group.len() > 0 {
            let list = new_group.join("\n");
            new_group.clear();
            let prop_list = format!("<ul>\n{}\n</ul>", list);
            new_html.push(prop_list);
        }
    }

    if new_group.len() > 0 {
        let list = new_group.join("\n");
        let prop_list = format!("<ul>\n{}\n</ul>", list);
        new_html.push(prop_list);
    }

    new_html
}
