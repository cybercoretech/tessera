use std::fs;
use std::path::Path;
use pulldown_cmark::{Parser, Options, html};

pub struct Page {
    source: String,
    // future: front matter fields could be added
}

impl Page {
    pub fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let source = fs::read_to_string(path)?;
        Ok(Self { source })
    }

    /// Render the markdown source to HTML.
    pub fn render(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        let parser = Parser::new_ext(&self.source, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        Ok(html_output)
    }
}
