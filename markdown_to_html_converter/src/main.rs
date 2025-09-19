use anyhow::Result;
use markdown_to_html_converter::html::HtmlRenderer;
use markdown_to_html_converter::types::Config;

fn main() -> Result<()> {
    let config = Config::new("test.md", "text.html");
    let renderer = HtmlRenderer::new(config);
    renderer.convert_file()
}
