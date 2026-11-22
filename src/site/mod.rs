pub mod nav;
pub mod page;
pub mod style;

use crate::token::TokenSet;
use std::fs;
use std::path::{Path, PathBuf};

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

    /// Build the site: render all markdown files in `content/` to HTML in `dist/` and compile styles.
    pub fn build(&self) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
        // 1. Ensure output directory exists
        fs::create_dir_all(&self.output_dir)?;

        // 2. Compile and write stylesheet (dist/styles.css)
        let css_content = style::StyleCompiler::compile(self.tokens, Path::new("."))?;
        let css_path = self.output_dir.join("styles.css");
        fs::write(css_path, css_content)?;

        // 3. Write progressive enhancement script (dist/tessera.js)
        let js_path = self.output_dir.join("tessera.js");
        fs::write(js_path, include_str!("tessera.js"))?;

        // 4. Render markdown pages inside layouts and build search index
        let mut count = 0usize;
        let mut search_items = Vec::new();

        if self.content_dir.is_dir() {
            for entry in fs::read_dir(&self.content_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("md") {
                    let page = page::Page::load(&path)?;
                    let body_html = page.render()?;
                    let full_html = self.apply_layout(&page, &body_html)?;

                    let file_stem = path.file_stem().unwrap().to_str().unwrap();
                    let mut out_path = self.output_dir.clone();
                    out_path.push(file_stem);
                    out_path.set_extension("html");
                    fs::write(&out_path, full_html)?;

                    let url = if file_stem == "index" {
                        "/".to_string()
                    } else {
                        format!("/{}.html", file_stem)
                    };

                    search_items.push((page.title.clone(), page.description.clone(), url));
                    count += 1;
                }
            }
        }

        // 5. Emit search-index.json
        let mut search_json = String::from("[\n");
        for (i, (title, desc, url)) in search_items.iter().enumerate() {
            let comma = if i + 1 < search_items.len() { "," } else { "" };
            search_json.push_str(&format!(
                "  {{\"title\": \"{}\", \"description\": \"{}\", \"url\": \"{}\"}}{}\n",
                title.replace('\\', "\\\\").replace('"', "\\\""),
                desc.replace('\\', "\\\\").replace('"', "\\\""),
                url,
                comma
            ));
        }
        search_json.push_str("]\n");
        fs::write(self.output_dir.join("search-index.json"), search_json)?;

        // 6. Copy user custom scripts from scripts/ to dist/
        let scripts_dir = Path::new("scripts");
        if scripts_dir.is_dir() {
            Self::copy_dir_all(scripts_dir, &self.output_dir)?;
        }

        // 7. Copy public/ directory assets if present
        let public_dir = Path::new("public");
        if public_dir.is_dir() {
            Self::copy_dir_all(public_dir, &self.output_dir)?;
        }

        Ok(count)
    }

    /// Resolves privacy-friendly analytics script from tessera.toml
    fn resolve_analytics_tag() -> String {
        #[derive(serde::Deserialize, Default)]
        struct AnalyticsManifest {
            analytics: Option<AnalyticsConfig>,
        }
        #[derive(serde::Deserialize, Default)]
        struct AnalyticsConfig {
            provider: Option<String>,
            domain: Option<String>,
            website_id: Option<String>,
            token: Option<String>,
        }

        let content = fs::read_to_string("tessera.toml").unwrap_or_default();
        if let Ok(m) = toml::from_str::<AnalyticsManifest>(&content) {
            if let Some(cfg) = m.analytics {
                let provider = cfg.provider.as_deref().unwrap_or_default();
                if provider == "plausible" {
                    if let Some(domain) = cfg.domain {
                        return format!("<script defer data-domain=\"{}\" src=\"https://plausible.io/js/script.js\"></script>\n", domain);
                    }
                } else if provider == "umami" {
                    if let Some(id) = cfg.website_id {
                        return format!("<script defer src=\"https://cloud.umami.is/script.js\" data-website-id=\"{}\"></script>\n", id);
                    }
                } else if provider == "cloudflare" {
                    if let Some(token) = cfg.token {
                        return format!("<script defer src=\"https://static.cloudflareinsights.com/beacon.min.js\" data-cf-beacon='{{\"token\": \"{}\"}}'></script>\n", token);
                    }
                }
            }
        }
        String::new()
    }

    /// Wraps rendered page content in an HTML layout template.
    fn apply_layout(
        &self,
        page: &page::Page,
        body_html: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let layout_candidates = [
            PathBuf::from(format!("layouts/{}.html", page.layout)),
            PathBuf::from(format!("templates/{}.html", page.layout)),
            PathBuf::from("layouts/base.html"),
            PathBuf::from("templates/base.html"),
        ];

        let template = layout_candidates
            .iter()
            .find(|p| p.is_file())
            .and_then(|p| fs::read_to_string(p).ok())
            .unwrap_or_else(Self::default_layout);

        let user_script_tag = if Path::new("scripts/main.js").is_file()
            || self.output_dir.join("main.js").is_file()
        {
            "<script src=\"/main.js\" defer></script>\n"
        } else {
            ""
        };
        let analytics_tag = Self::resolve_analytics_tag();

        let mut output = template
            .replace("{{title}}", &page.title)
            .replace("{{description}}", &page.description)
            .replace("{{content}}", body_html);

        if !user_script_tag.is_empty() && !output.contains("/main.js") {
            output = output.replace("</head>", &format!("  {}</head>", user_script_tag));
        }
        if !analytics_tag.is_empty() {
            output = output.replace("</head>", &format!("  {}</head>", analytics_tag));
        }

        Ok(output)
    }

    /// Default HTML5 layout shell.
    fn default_layout() -> String {
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>{{title}}</title>
  <meta name="description" content="{{description}}">
  <link rel="stylesheet" href="/styles.css">
  <script src="/tessera.js" defer></script>
</head>
<body>
  <div class="ts-reading-progress" data-ts-progress></div>
  <header style="border-bottom: 1px solid var(--ts-border-color, #e5e7eb);">
    <div class="ts-container">
      <nav class="ts-nav">
        <a href="/" style="font-weight: 700; font-size: 1.25rem;">⚡ Tessera</a>
        <div style="display: flex; align-items: center; gap: 1rem;">
          <ul class="ts-nav__links">
            <li><a href="/">Home</a></li>
          </ul>
          <button class="ts-search-btn" data-ts-search-trigger aria-label="Search">🔍 <span class="ts-search-btn__hint">Ctrl+K</span></button>
          <button class="ts-theme-toggle" data-ts-theme-toggle aria-label="Toggle dark/light theme">🌓</button>
        </div>
      </nav>
    </div>
  </header>
  <main class="ts-container ts-section">
    {{content}}
  </main>
  <footer class="ts-section" style="border-top: 1px solid var(--ts-border-color, #e5e7eb); margin-top: 4rem; padding-top: 1.5rem; padding-bottom: 1.5rem;">
    <div class="ts-container" style="display: flex; justify-content: space-between; align-items: center; color: #6b7280; font-size: 0.875rem;">
      <span>Built with <a href="https://github.com/cybercoretech/tessera">Tessera</a></span>
      <span>Lightweight & Zero-JS Fallback</span>
    </div>
  </footer>
</body>
</html>"#.to_string()
    }

    /// Recursively copies directory contents.
    fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
        fs::create_dir_all(dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let ty = entry.file_type()?;
            let target = dst.join(entry.file_name());
            if ty.is_dir() {
                Self::copy_dir_all(&entry.path(), &target)?;
            } else {
                fs::copy(entry.path(), target)?;
            }
        }
        Ok(())
    }
}
