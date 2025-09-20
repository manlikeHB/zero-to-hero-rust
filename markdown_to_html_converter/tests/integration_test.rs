use markdown_to_html_converter::{Config, HtmlRenderer};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_full_conversion_pipeline() {
    // Create temporary directory for test files
    let dir = tempdir().unwrap();
    let input_path = dir.path().join("test.md");
    let output_path = dir.path().join("test.html");

    // Write test markdown
    let markdown_content = r#"# Test Header

This is a test paragraph with **bold** text.

- First item
- Second item
"#;
    fs::write(&input_path, markdown_content).unwrap();

    // Convert using your library
    let config = Config::new(input_path.to_str().unwrap(), output_path.to_str().unwrap());
    let renderer = HtmlRenderer::new(config);
    renderer.convert_file().unwrap();

    // Verify the output file was created and contains expected HTML
    let output = fs::read_to_string(&output_path).unwrap();
    assert!(output.contains("<h1>Test Header</h1>"));
    assert!(output.contains("<p>This is a test paragraph with <strong>bold</strong> text.</p>"));
    assert!(output.contains("<ul>"));
    assert!(output.contains("<li>First item</li>"));
}

#[test]
fn test_error_handling_invalid_file() {
    let config = Config::new("nonexistent.md", "output.html");
    let renderer = HtmlRenderer::new(config);

    // This should return an error, not panic
    let result = renderer.convert_file();
    assert!(result.is_err());
}
