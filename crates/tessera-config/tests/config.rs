use std::{fs, path::Path};

use tempfile::tempdir;
use tessera_config::{ProjectConfig, StyleMode};

fn write_manifest(root: &Path, contents: &str) {
    fs::write(root.join("tessera.toml"), contents).unwrap();
}

#[test]
fn loads_project_metadata_and_scss_style() {
    let dir = tempdir().unwrap();
    write_manifest(
        dir.path(),
        r#"
            name = "Example"
            version = "1.2.3"

            [style]
            mode = "scss"
        "#,
    );

    let config = ProjectConfig::load(dir.path()).unwrap();

    assert_eq!(config.name, "Example");
    assert_eq!(config.version.as_deref(), Some("1.2.3"));
    assert_eq!(config.style.mode, StyleMode::Scss);
}

#[test]
fn rejects_unknown_style_mode_with_field_context() {
    let dir = tempdir().unwrap();
    write_manifest(
        dir.path(),
        r#"
            name = "Example"

            [style]
            mode = "less"
        "#,
    );

    let error = ProjectConfig::load(dir.path()).unwrap_err().to_string();

    assert!(error.contains("style.mode"), "unexpected error: {error}");
}

#[test]
fn rejects_unknown_manifest_keys() {
    let dir = tempdir().unwrap();
    write_manifest(
        dir.path(),
        r#"
            name = "Example"
            mystery = true

            [style]
            mode = "css"
        "#,
    );

    assert!(ProjectConfig::load(dir.path()).is_err());
}

#[test]
fn loads_theme_tokens_from_the_project_manifest() {
    let dir = tempdir().unwrap();
    write_manifest(
        dir.path(),
        r##"
            name = "my-site"
            version = "0.1.0"

            [theme.tokens]
            primary = "#0066ff"
        "##,
    );

    let config = ProjectConfig::load(dir.path()).unwrap();

    assert_eq!(config.name, "my-site");
    assert_eq!(config.style.mode, StyleMode::Scss);
    assert_eq!(
        config.theme.tokens.get("primary"),
        Some(&"#0066ff".to_owned())
    );
}
