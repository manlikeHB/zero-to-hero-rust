use anyhow::Result;
use std::fs;
use std::fs::File;
use std::io::Write;

pub fn read_md_file(path: &str) -> Result<String> {
    Ok(fs::read_to_string(path)?)
}

pub fn write_html_file(content: String, path: &str) -> Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
