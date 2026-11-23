use crate::site::Engine;
use crate::token::TokenSet;
use std::fs;
use std::path::Path;
use tessera_config::ProjectConfig;

/// Runs a simple health‑check of the current project.
///
/// Checks performed:
/// 1. `tessera.toml` exists and parses as a `TokenSet`.
/// 2. The `content/` directory exists.
/// 3. Counts markdown files in `content/`.
/// 4. Reports the number of design tokens defined.
pub fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 1. Check config file.
    if !Path::new("tessera.toml").exists() {
        println!("[doctor] ✗ tessera.toml not found");
    } else {
        match fs::read_to_string("tessera.toml") {
            Ok(txt) => match (ProjectConfig::parse(&txt), TokenSet::from_toml(&txt)) {
                (Ok(config), Ok(tok)) => {
                    println!("[doctor] ✓ tessera.toml parsed successfully");
                    let token_count = tok.tokens.len();
                    println!("[doctor]   Tokens defined: {token_count}");
                    for (tool, major) in crate::site::toolchain::required_tools(&config) {
                        match crate::site::toolchain::check_tool(tool, major) {
                            Ok(version) => println!("[doctor] ✓ {tool} {version}"),
                            Err(error) => println!("[doctor] ✗ {error}"),
                        }
                    }
                }
                (Err(error), _) => {
                    println!("[doctor] ✗ Failed to parse tessera.toml: {error}")
                }
                (_, Err(error)) => {
                    println!("[doctor] ✗ Failed to parse tessera.toml: {error}")
                }
            },
            Err(e) => println!("[doctor] ✗ Could not read tessera.toml: {e}"),
        }
    }

    // 2. Content directory.
    let content_path = Path::new("content");
    if !content_path.is_dir() {
        println!("[doctor] ✗ content/ directory missing");
    } else {
        // 3. Count markdown files.
        let md_count = fs::read_dir(content_path)?
            .filter_map(Result::ok)
            .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("md"))
            .count();
        println!("[doctor] ✓ content/ exists with {md_count} markdown file(s)");
    }

    // 4. Verify site engine can be instantiated.
    let token_set = TokenSet::default();
    let _engine = Engine::new(&token_set);
    println!("[doctor] ✓ Site engine instantiated");
    println!(
        "[doctor] ✓ Kit catalog loaded with {} patterns",
        crate::kit::catalog::ENTRIES.len()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_instantiation() {
        let tokens = TokenSet::default();
        let _engine = Engine::new(&tokens);
    }

    #[test]
    fn test_token_parsing_valid() {
        let sample = r##"
        [tokens]
        color-primary = "#ff0000"
        "##;
        let parsed: Result<TokenSet, _> = toml::from_str(sample);
        assert!(parsed.is_ok());
    }
}
