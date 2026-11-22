use pulldown_cmark::{html, Options, Parser};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct Page {
    pub title: String,
    pub description: String,
    pub layout: String,
    #[allow(dead_code)] // Retained for site checks and templates as they are implemented.
    pub metadata: HashMap<String, String>,
    pub content: String,
    #[allow(dead_code)] // Retained for diagnostics and future source-aware rendering.
    source: String,
}

impl Page {
    pub fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let source = fs::read_to_string(path)?;
        let default_title = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Untitled")
            .to_string();

        let (metadata, content) = Self::parse_front_matter(&source);

        let title = metadata.get("title").cloned().unwrap_or_else(|| {
            // Capitalize first letter of file stem
            let mut c = default_title.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        });
        let description = metadata.get("description").cloned().unwrap_or_default();
        let layout = metadata
            .get("layout")
            .cloned()
            .unwrap_or_else(|| "base".to_string());

        Ok(Self {
            title,
            description,
            layout,
            metadata,
            content,
            source,
        })
    }

    /// Extract YAML/TOML-style front matter between `---` delimiters.
    fn parse_front_matter(source: &str) -> (HashMap<String, String>, String) {
        let mut metadata = HashMap::new();
        let trimmed = source.trim_start();
        if !trimmed.starts_with("---") {
            return (metadata, source.to_string());
        }

        // Find the second --- delimiter
        let after_first = &trimmed[3..];
        let after_first_trimmed = after_first.trim_start_matches(['\r', '\n']);
        if let Some(end_idx) = after_first_trimmed.find("---") {
            let front_matter = &after_first_trimmed[..end_idx];
            let body = &after_first_trimmed[end_idx + 3..];
            let body = body.trim_start_matches(['\r', '\n']);

            for line in front_matter.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                if let Some((key, val)) = line.split_once(':') {
                    let key = key.trim().to_string();
                    let val = val.trim().trim_matches('"').trim_matches('\'').to_string();
                    metadata.insert(key, val);
                } else if let Some((key, val)) = line.split_once('=') {
                    let key = key.trim().to_string();
                    let val = val.trim().trim_matches('"').trim_matches('\'').to_string();
                    metadata.insert(key, val);
                }
            }

            (metadata, body.to_string())
        } else {
            (metadata, source.to_string())
        }
    }

    /// Render the markdown content body to an HTML fragment.
    pub fn render(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        let parser = Parser::new_ext(&self.content, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        Ok(html_output)
    }

    /// Returns the raw source before front-matter parsing.
    #[allow(dead_code)] // Used by source-aware checks planned for the site engine.
    pub fn raw_source(&self) -> &str {
        &self.source
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_front_matter_parsing() {
        let md = r#"---
title: "Custom Title"
description: 'A test description'
layout: blog
---
# Main Heading

Content here.
"#;
        let (meta, body) = Page::parse_front_matter(md);
        assert_eq!(meta.get("title").map(|s| s.as_str()), Some("Custom Title"));
        assert_eq!(
            meta.get("description").map(|s| s.as_str()),
            Some("A test description")
        );
        assert_eq!(meta.get("layout").map(|s| s.as_str()), Some("blog"));
        assert!(body.starts_with("# Main Heading"));
    }

    #[test]
    fn test_empty_front_matter() {
        let md = "# Simple Page\nNo front matter.";
        let (meta, body) = Page::parse_front_matter(md);
        assert!(meta.is_empty());
        assert_eq!(body, md);
    }
}
