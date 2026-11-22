use clap::{Parser, Subcommand};
use log::info;
use std::path::Path;
use std::fs;
use toml;

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
        let target_path = content_dir.join(format!("{}.md", name));
        if target_path.exists() {
            return Err(format!("Page '{}' already exists", name).into());
        }
        let template_path = Path::new(TEMPLATE_DIR).join("page.md");
        let template = if template_path.exists() {
            fs::read_to_string(template_path)?
        } else {
            // fallback simple template
            format!("# {}\n\n*Write content here.*", name)
        };
        let mut file = fs::File::create(&target_path)?;
        file.write_all(template.as_bytes())?;
        Ok(())
    }

    /// Create a stub Rust component file.
    pub fn generate_component(name: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let comp_dir = Path::new("src").join("components");
        fs::create_dir_all(&comp_dir)?;
        let file_name = format!("{}.rs", name);
        let target_path = comp_dir.join(&file_name);
        if target_path.exists() {
            return Err(format!("Component '{}' already exists", name).into());
        }
        let mut file = fs::File::create(&target_path)?;
        writeln!(file, "// {} component placeholder", name)?;
        writeln!(file, "pub fn render() -> &'static str {{")?;
        writeln!(file, "    \"<div class=\"{}\"></div>\"", name)?;
        writeln!(file, "}}")?;
        Ok(())
    }

    /// Update or add a token entry in `tessera.toml` under `[theme.tokens]`.
    pub fn set_token(key: &str, value: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
                } else if line.trim_start().starts_with(&format!("{} =", key)) {
                    *line = format!("{} = \"{}\"", key, value);
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
                    lines.insert(i + 1, format!("{} = \"{}\"", key, value));
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
            println!("Created page '{}.md' in content/", name);
        }
        Commands::AddComponent { name } => {
            generators::generate_component(&name)?;
            println!("Created component file '{}.rs' in src/components/", name);
        }
        Commands::ThemeSet { key, value } => {
            generators::set_token(&key, &value)?;
            println!("Set token '{} = {}' in tessera.toml", key, value);
        }
        Commands::Dev { port } => {
            dev::run(Some(port))?;
        }
        Commands::Doctor => {
            doctor::run()?;
        }
        Commands::Build => {
            // Load token set directly from tessera.toml
            let contents = fs::read_to_string("tessera.toml")?;
            let token_set: token::TokenSet = toml::from_str(&contents)?;
            // Demo: show primary color via Kit
            let theme = kit::Theme::new(&token_set);
            println!("Primary color from Kit: {}", theme.foundations.primary_color());
            // Build site
            let engine = site::Engine::new(&token_set);
            let built = engine.build()?;
            println!("Built {} page(s) into dist/", built);
            // Print tokens if any
            if !token_set.tokens.is_empty() {
                println!("Defined design tokens:");
                for (k, v) in token_set.iter() {
                    println!("  {} = {}", k, v);
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
