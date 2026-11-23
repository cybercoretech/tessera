pub mod nav;
pub mod page;
pub mod style;
pub mod toolchain;

use crate::token::TokenSet;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::Builder;
use tessera_config::ProjectConfig;

pub struct Engine<'a> {
    tokens: &'a TokenSet,
    config: Option<&'a ProjectConfig>,
    root: PathBuf,
    content_dir: PathBuf,
    output_dir: PathBuf,
}

impl<'a> Engine<'a> {
    pub fn new(tokens: &'a TokenSet) -> Self {
        Self::at_root(
            tokens,
            None,
            std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        )
    }

    pub fn with_config(tokens: &'a TokenSet, config: &'a ProjectConfig) -> Self {
        Self::at_root(
            tokens,
            Some(config),
            std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        )
    }

    pub fn at_root(tokens: &'a TokenSet, config: Option<&'a ProjectConfig>, root: PathBuf) -> Self {
        Self {
            tokens,
            config,
            content_dir: root.join("content"),
            output_dir: root.join("dist"),
            root,
        }
    }

    /// Build the site: render all markdown files in `content/` to HTML in `dist/` and compile styles.
    pub fn build(&self) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
        let output_parent = self
            .output_dir
            .parent()
            .filter(|path| !path.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."));
        let stage = Builder::new()
            .prefix(".tessera-build-")
            .tempdir_in(output_parent)?;
        let staged_output = stage.path().join("dist");
        fs::create_dir_all(&staged_output)?;

        let mut css_content = if let Some(config) = self.config {
            style::StyleCompiler::compile_with_config(self.tokens, &self.root, &config.style)?
        } else {
            style::StyleCompiler::compile(self.tokens, &self.root)?
        };
        if let Some(config) = self.config {
            if let Some(utility_css) =
                toolchain::compile_utility_css(config, self.tokens, &self.root, stage.path())?
            {
                css_content.push_str("\n/* Optional utility engine output */\n");
                css_content.push_str(&utility_css);
            }
        }
        let css_path = staged_output.join("styles.css");
        fs::write(css_path, css_content)?;

        let script_config = self.config.map(|config| &config.scripts);
        let mut script_outputs = Vec::new();
        if let Some(config) = self.config {
            script_outputs = toolchain::bundle_scripts(config, &self.root, &staged_output)?;
            if config.scripts.interactions {
                fs::write(staged_output.join("tessera.js"), include_str!("tessera.js"))?;
            }
        }
        let script_tags = Self::script_tags(
            &script_outputs,
            script_config.is_some_and(|scripts| scripts.interactions),
        );

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
                    let full_html = self.apply_layout(&page, &body_html, &script_tags)?;

                    let file_stem = path.file_stem().unwrap().to_str().unwrap();
                    let mut out_path = staged_output.clone();
                    out_path.push(file_stem);
                    out_path.set_extension("html");
                    fs::write(&out_path, full_html)?;

                    let url = if file_stem == "index" {
                        "/".to_string()
                    } else {
                        format!("/{file_stem}.html")
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
            let title = title.replace('\\', "\\\\").replace('"', "\\\"");
            let desc = desc.replace('\\', "\\\\").replace('"', "\\\"");
            search_json.push_str(&format!(
                "  {{\"title\": \"{title}\", \"description\": \"{desc}\", \"url\": \"{url}\"}}{comma}\n"
            ));
        }
        search_json.push_str("]\n");
        fs::write(staged_output.join("search-index.json"), search_json)?;

        // 6. Copy public/ directory assets if present.
        let public_dir = self.root.join("public");
        if public_dir.is_dir() {
            Self::copy_dir_all(&public_dir, &staged_output)?;
        }

        let backup = stage.path().join("previous-dist");
        let had_previous_output = self.output_dir.exists();
        if had_previous_output {
            fs::rename(&self.output_dir, &backup)?;
        }
        if let Err(error) = fs::rename(&staged_output, &self.output_dir) {
            if had_previous_output {
                let _ = fs::rename(&backup, &self.output_dir);
            }
            return Err(error.into());
        }

        Ok(count)
    }

    /// Resolves privacy-friendly analytics script from tessera.toml
    fn resolve_analytics_tag(root: &Path) -> String {
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

        let content = fs::read_to_string(root.join("tessera.toml")).unwrap_or_default();
        if let Ok(m) = toml::from_str::<AnalyticsManifest>(&content) {
            if let Some(cfg) = m.analytics {
                let provider = cfg.provider.as_deref().unwrap_or_default();
                if provider == "plausible" {
                    if let Some(domain) = cfg.domain {
                        return format!("<script defer data-domain=\"{domain}\" src=\"https://plausible.io/js/script.js\"></script>\n");
                    }
                } else if provider == "umami" {
                    if let Some(id) = cfg.website_id {
                        return format!("<script defer src=\"https://cloud.umami.is/script.js\" data-website-id=\"{id}\"></script>\n");
                    }
                } else if provider == "cloudflare" {
                    if let Some(token) = cfg.token {
                        return format!("<script defer src=\"https://static.cloudflareinsights.com/beacon.min.js\" data-cf-beacon='{{\"token\": \"{token}\"}}'></script>\n");
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
        script_tags: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let layout = &page.layout;
        let layout_candidates = [
            self.root.join(format!("layouts/{layout}.html")),
            self.root.join(format!("templates/{layout}.html")),
            self.root.join("layouts/base.html"),
            self.root.join("templates/base.html"),
        ];

        let template = layout_candidates
            .iter()
            .find(|p| p.is_file())
            .and_then(|p| fs::read_to_string(p).ok())
            .unwrap_or_else(Self::default_layout);

        let analytics_tag = Self::resolve_analytics_tag(&self.root);

        let mut output = template
            .replace("{{title}}", &page.title)
            .replace("{{description}}", &page.description)
            .replace("{{content}}", body_html);

        if !script_tags.is_empty() {
            output = output.replace("</head>", &format!("  {script_tags}</head>"));
        }
        if !analytics_tag.is_empty() {
            output = output.replace("</head>", &format!("  {analytics_tag}</head>"));
        }

        Ok(output)
    }

    fn script_tags(script_outputs: &[PathBuf], interactions: bool) -> String {
        let mut tags = String::new();
        for output in script_outputs {
            let output = output.display();
            tags.push_str(&format!(
                "<script type=\"module\" src=\"/{output}\"></script>\n"
            ));
        }
        if interactions {
            tags.push_str("<script src=\"/tessera.js\" defer></script>\n");
        }
        tags
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
</head>
<body>
  <header style="border-bottom: 1px solid var(--ts-border-color, #e5e7eb);">
    <div class="ts-container">
      <nav class="ts-nav">
        <a href="/" style="font-weight: 700; font-size: 1.25rem;">⚡ Tessera</a>
        <div style="display: flex; align-items: center; gap: 1rem;">
          <ul class="ts-nav__links">
            <li><a href="/">Home</a></li>
          </ul>
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

#[cfg(test)]
mod tests {
    use super::Engine;
    use crate::token::TokenSet;
    use std::fs;
    use tempfile::tempdir;
    use tessera_config::ProjectConfig;

    #[test]
    fn default_build_does_not_emit_javascript() {
        let root = tempdir().unwrap();
        let config = ProjectConfig::parse("name = \"test\"\n").unwrap();
        let tokens = TokenSet::default();
        let engine = Engine::at_root(&tokens, Some(&config), root.path().to_path_buf());

        engine.build().unwrap();

        assert!(!root.path().join("dist/tessera.js").exists());
        assert!(!root.path().join("dist/assets").exists());
    }

    #[test]
    fn failed_utility_build_preserves_previous_dist() {
        let root = tempdir().unwrap();
        fs::create_dir(root.path().join("dist")).unwrap();
        fs::write(root.path().join("dist/previous.txt"), "keep me").unwrap();
        let config =
            ProjectConfig::parse("name = \"test\"\n[style]\nutility = \"uno-css\"\n").unwrap();
        let tokens = TokenSet::default();
        let engine = Engine::at_root(&tokens, Some(&config), root.path().to_path_buf());

        assert!(engine.build().is_err());

        assert_eq!(
            fs::read_to_string(root.path().join("dist/previous.txt")).unwrap(),
            "keep me"
        );
    }

    #[test]
    fn failed_script_build_preserves_previous_dist() {
        let root = tempdir().unwrap();
        fs::create_dir(root.path().join("dist")).unwrap();
        fs::write(root.path().join("dist/previous.txt"), "keep me").unwrap();
        fs::create_dir(root.path().join("scripts")).unwrap();
        fs::write(root.path().join("scripts/main.css"), "not a script").unwrap();
        let config = ProjectConfig::parse(
            "name = \"test\"\n[scripts]\nenabled = true\nentry_points = [\"scripts/main.css\"]\n",
        )
        .unwrap();
        let tokens = TokenSet::default();
        let engine = Engine::at_root(&tokens, Some(&config), root.path().to_path_buf());

        assert!(engine.build().is_err());

        assert_eq!(
            fs::read_to_string(root.path().join("dist/previous.txt")).unwrap(),
            "keep me"
        );
    }
}
