# 📝 Markdown → HTML Converter

A configurable Markdown parser and HTML renderer built from scratch using regex for inline formatting.

## 🎯 Learning Objectives

- **Regular Expressions**: Using `regex` crate for pattern matching
- **Enums for AST**: Representing document structure
- **Trait System**: Creating extensible rendering architecture
- **Module Organization**: Structuring larger projects with multiple modules
- **Builder Pattern**: Ergonomic configuration design
- **Integration Testing**: Testing complete workflows
- **Documentation**: Writing comprehensive doc comments

## ✨ Supported Markdown Features

### Block Elements
- Headers (`#` through `######`)
- Paragraphs
- Unordered lists (`-`)

### Inline Formatting
- **Bold** (`**text**`)
- *Italic* (`*text*`)
- `Code` (`` `text` ``)
- [Links](url) (`[text](url)`)

## 🚀 Running the Converter

```bash
cargo run
```

Converts `test.md` to `text.html` using the default configuration.

## 📝 Example Conversion

**Input (Markdown):**
```markdown
# Main Title

This is a paragraph with **bold** and *italic* text.

- First item
- Second item with `code`
- Third [link](https://example.com)
```

**Output (HTML):**
```html
<h1>Main Title</h1>
<p>This is a paragraph with <strong>bold</strong> and <em>italic</em> text.</p>
<ul>
<li>First item</li>
<li>Second item with <code>code</code></li>
<li>Third <a href="https://example.com">link</a></li>
</ul>
```

## 🏗️ Architecture

### Module Structure
```
markdown_to_html_converter/
├── src/
│   ├── lib.rs          # Public API, re-exports
│   ├── types.rs        # Core types (Config, MarkdownElement, Renderer trait)
│   ├── parser.rs       # Markdown parsing logic
│   ├── html.rs         # HTML rendering implementation
│   └── file.rs         # File I/O operations
├── tests/
│   └── integration_test.rs
└── Cargo.toml
```

## 🔑 Key Concepts Demonstrated

### Enum for AST
```rust
#[derive(Debug)]
pub enum MarkdownElement {
    Header(u8, String),      // level, text
    Paragraph(String),
    List(String),
}
```

### Trait for Extensibility
```rust
pub trait Renderer {
    fn render(&self, elements: &[MarkdownElement]) -> Result<String>;
    fn render_element(&self, element: &MarkdownElement) -> Result<String>;
}
```

### Builder Pattern for Config
```rust
let config = Config::new("input.md", "output.html")
    .with_full_html(true)
    .with_max_header_level(4);
```

### Regex-Based Inline Parsing
```rust
static BOLD_REGEX: LazyLock<Regex> = 
    LazyLock::new(|| Regex::new(r"\*\*([^*]+)\*\*").unwrap());

pub fn parse_inner(text: &str) -> String {
    let replaced = BOLD_REGEX
        .replace_all(&text, "<strong>$1</strong>")
        .to_string();
    // ... more replacements
}
```

### List Grouping Algorithm
```rust
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

        // Wrap group when next element isn't a list item
        if next.is_some() && !next.unwrap().starts_with("<li>") && new_group.len() > 0 {
            let list = new_group.join("\n");
            new_group.clear();
            let prop_list = format!("<ul>\n{}\n</ul>", list);
            new_html.push(prop_list);
        }
    }
    
    new_html
}
```

### Integration Testing
```rust
#[test]
fn test_full_conversion_pipeline() {
    let dir = tempdir().unwrap();
    let input_path = dir.path().join("test.md");
    let output_path = dir.path().join("test.html");

    fs::write(&input_path, "# Test Header\n\nParagraph").unwrap();

    let config = Config::new(
        input_path.to_str().unwrap(), 
        output_path.to_str().unwrap()
    );
    let renderer = HtmlRenderer::new(config);
    renderer.convert_file().unwrap();

    let output = fs::read_to_string(&output_path).unwrap();
    assert!(output.contains("<h1>Test Header</h1>"));
}
```

## 💡 What I Learned

1. **LazyLock for Regex**: Compiling regex patterns once at initialization
2. **Trait Objects**: Creating extensible systems with trait-based architecture
3. **Module Privacy**: Using `pub` strategically for clean APIs
4. **Doc Comments**: Writing comprehensive documentation with examples
5. **Error Propagation**: Using `anyhow::Result` for flexible error handling
6. **Integration Testing**: Testing complete workflows with `tempfile`
7. **Re-exports**: Using `lib.rs` to create convenient public APIs
8. **String Replacement**: Chaining `.replace_all()` for multiple transformations

## 🧪 Testing Strategy

### Unit Tests (in modules)
- Parser validation for different markdown constructs
- Header level validation
- Individual element rendering

### Integration Tests (in `tests/`)
- Full file conversion pipeline
- Error handling for invalid inputs
- Temporary file management with `tempfile`

## 📚 Library API

```rust
use markdown_to_html_converter::{Config, HtmlRenderer, parse_md};

// Parse markdown
let config = Config::default();
let elements = parse_md(markdown_content, &config)?;

// Render to HTML
let renderer = HtmlRenderer::new(config);
let html = renderer.render(&elements)?;

// Or convert file directly
let config = Config::new("input.md", "output.html");
let renderer = HtmlRenderer::new(config);
renderer.convert_file()?;
```

## 🔄 Possible Improvements

- [ ] Code blocks with syntax highlighting
- [ ] Ordered lists (numbered)
- [ ] Blockquotes
- [ ] Horizontal rules
- [ ] Tables
- [ ] Images
- [ ] Nested lists
- [ ] Better handling of nested formatting (e.g., bold inside italic)
- [ ] Custom rendering backends (LaTeX, plain text)
- [ ] Incremental parsing for large documents

## 🐛 Known Limitations

- Complex nested formatting requires lookahead/lookbehind regex (not supported in Rust's regex crate)
- Example: `**bold with *italic* inside**` may not render perfectly
- Workaround: Use simpler formatting or implement a proper parser

## 📚 Relevant Rust Book Chapters

- [Chapter 7: Managing Growing Projects with Packages, Crates, and Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [Chapter 10: Generic Types, Traits, and Lifetimes](https://doc.rust-lang.org/book/ch10-00-generics.html)
- [Chapter 14: More About Cargo and Crates.io](https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html)

## 📦 Dependencies

```toml
[dependencies]
anyhow = "1.0"
regex = "1.10"

[dev-dependencies]
tempfile = "3.8"
```

---

**Status**: ✅ Completed | **Difficulty**: Intermediate-Advanced