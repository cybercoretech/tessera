pub mod page;
pub mod nav;

use crate::token::TokenSet;
use std::path::{Path, PathBuf};
use std::fs;

pub struct Engine<'a> {
    tokens: &'a TokenSet,
    content_dir: PathBuf,
    output_dir: PathBuf,
}

impl<'a> Engine<'a> {
    pub fn new(tokens: &'a TokenSet) -> Self {
        Self {
            tokens,
            content_dir: Path::new("content").to_path_buf(),
            output_dir: Path::new("dist").to_path_buf(),
        }
    }

    /// Build the site: render all markdown files in `content/` to HTML in `dist/`.
    pub fn build(&self) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
        // Ensure output directory exists
        fs::create_dir_all(&self.output_dir)?;
        let mut count = 0usize;
        for entry in fs::read_dir(&self.content_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("md") {
                let page = page::Page::load(&path)?;
                let html = page.render()?;
                // Write to output with same stem but .html
                let mut out_path = self.output_dir.clone();
                out_path.push(path.file_stem().unwrap());
                out_path.set_extension("html");
                fs::write(out_path, html)?;
                count += 1;
            }
        }
        Ok(count)
    }
}
