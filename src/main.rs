use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;
use tessera_config::ProjectConfig;

// Import new subcommands
mod dev;
mod doctor;
mod scaffolding;

mod kit;
mod site;
mod token;

/// Top‑level CLI definition.
#[derive(Parser)]
#[command(name = "tessera", version, about = "Static‑site scaffolding CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Tessera project directory with scaffolding.
    New { name: String },
    /// Add a new markdown page under `content/`.
    AddPage { name: String },
    /// Add a new component stub under `src/components/`.
    AddComponent { name: String },
    /// Set or update a design token in `tessera.toml`.
    ThemeSet { key: String, value: String },
    /// Run the local development server with file watching and live reload.
    Dev {
        /// Port to listen on
        #[arg(short, long, default_value_t = 8000)]
        port: u16,
        /// Address to listen on; use 0.0.0.0 to allow other devices on your network
        #[arg(long, default_value_t = std::net::IpAddr::from([127, 0, 0, 1]))]
        host: std::net::IpAddr,
    },
    /// Run diagnostic health-checks on the current project.
    Doctor,
    /// Build the static site into `dist/`.
    Build,
    /// Clean the generated `dist/` directory.
    Clean,
}

/// Generator helpers used by the CLI.
mod generators {
    use super::*;
    use std::io::Write;

    const TEMPLATE_DIR: &str = "templates";

    /// Create a new markdown page using the template.
    pub fn generate_page(name: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let content_dir = Path::new("content");
        fs::create_dir_all(content_dir)?;
        let target_path = content_dir.join(format!("{name}.md"));
        if target_path.exists() {
            return Err(format!("Page '{name}' already exists").into());
        }
        let template_path = Path::new(TEMPLATE_DIR).join("page.md");
        let template = if template_path.exists() {
            fs::read_to_string(template_path)?
        } else {
            // fallback simple template
            format!("# {name}\n\n*Write content here.*")
        };
        let mut file = fs::File::create(&target_path)?;
        file.write_all(template.as_bytes())?;
        Ok(())
    }

    /// Component names become a file name and an HTML class inside a Rust
    /// string, so keep them to a letter followed by letters, digits, `-` or `_`.
    fn is_valid_component_name(name: &str) -> bool {
        let mut chars = name.chars();
        matches!(chars.next(), Some(c) if c.is_ascii_alphabetic())
            && chars.all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    }

    /// Create a stub Rust component file.
    pub fn generate_component(name: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if !is_valid_component_name(name) {
            return Err(format!(
                "Invalid component name '{name}': use a letter followed by letters, digits, '-' or '_'"
            )
            .into());
        }
        let comp_dir = Path::new("src").join("components");
        fs::create_dir_all(&comp_dir)?;
        let file_name = format!("{name}.rs");
        let target_path = comp_dir.join(&file_name);
        if target_path.exists() {
            return Err(format!("Component '{name}' already exists").into());
        }
        let mut file = fs::File::create(&target_path)?;
        writeln!(file, "// {name} component placeholder")?;
        writeln!(file, "pub fn render() -> &'static str {{")?;
        // A raw string keeps the HTML attribute quotes valid inside the Rust literal.
        writeln!(file, "    r#\"<div class=\"{name}\"></div>\"#")?;
        writeln!(file, "}}")?;
        Ok(())
    }

    /// Update or add a token entry in `tessera.toml` under `[theme.tokens]`.
    pub fn set_token(
        key: &str,
        value: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let toml_path = Path::new("tessera.toml");
        let mut contents = fs::read_to_string(toml_path)?;
        // Find the start of the [theme.tokens] table (or create it).
        if !contents.contains("[theme.tokens]") {
            // Append a new table at the end.
            contents.push_str("\n[theme.tokens]\n");
        }
        // Simple line‑based replace or append.
        let mut lines: Vec<String> = contents.lines().map(|l| l.to_string()).collect();
        let mut in_section = false;
        let mut updated = false;
        for line in lines.iter_mut() {
            if line.trim() == "[theme.tokens]" {
                in_section = true;
                continue;
            }
            if in_section {
                if line.starts_with('[') {
                    // End of section
                    in_section = false;
                } else if line.trim_start().starts_with(&format!("{key} =")) {
                    *line = format!("{key} = \"{value}\"");
                    updated = true;
                    break;
                }
            }
        }
        if !updated {
            // Append the key/value under the section.
            for i in 0..lines.len() {
                if lines[i].trim() == "[theme.tokens]" {
                    // Insert after the section header.
                    lines.insert(i + 1, format!("{key} = \"{value}\""));
                    break;
                }
            }
        }
        let new_contents = lines.join("\n");
        fs::write(toml_path, new_contents)?;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    env_logger::init();
    let cli = Cli::parse();
    match cli.command {
        Commands::New { name } => {
            scaffolding::create_project(&name)?;
        }
        Commands::AddPage { name } => {
            generators::generate_page(&name)?;
            println!("Created page '{name}.md' in content/");
        }
        Commands::AddComponent { name } => {
            generators::generate_component(&name)?;
            println!("Created component file '{name}.rs' in src/components/");
        }
        Commands::ThemeSet { key, value } => {
            generators::set_token(&key, &value)?;
            println!("Set token '{key} = {value}' in tessera.toml");
        }
        Commands::Dev { port, host } => {
            dev::run(host, port)?;
        }
        Commands::Doctor => {
            doctor::run()?;
        }
        Commands::Build => {
            // Load token set directly from tessera.toml
            let contents = fs::read_to_string("tessera.toml")?;
            let config = ProjectConfig::parse(&contents)?;
            let token_set = token::TokenSet::from_toml(&contents)?;
            // Demo: show primary color via Kit
            let theme = kit::Theme::new(&token_set);
            println!(
                "Primary color from Kit: {}",
                theme.foundations.primary_color()
            );
            // Build site
            let engine = site::Engine::with_config(&token_set, &config);
            let built = engine.build()?;
            println!("Built {built} page(s) into dist/");
            // Print tokens if any
            if !token_set.tokens.is_empty() {
                println!("Defined design tokens:");
                for (k, v) in token_set.iter() {
                    println!("  {k} = {v}");
                }
            }
        }
        Commands::Clean => {
            let dist = Path::new("dist");
            if dist.exists() {
                fs::remove_dir_all(dist)?;
                println!("Removed dist/ directory");
            } else {
                println!("dist/ directory does not exist, nothing to clean");
            }
        }
    }
    Ok(())
}
