use std::{fs, path::Path};

use tempfile::tempdir;
use tessera_config::{ProjectConfig, SassCompiler, StyleMode, StyleSource, UtilityEngine};

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
    assert_eq!(config.style.source, StyleSource::Scss);
    assert_eq!(config.style.sass_compiler, SassCompiler::Grass);
    assert_eq!(config.style.utility, UtilityEngine::None);
}

#[test]
fn defaults_css_source_and_preserves_legacy_scss_mode_without_optional_tools() {
    let config = ProjectConfig::parse("name = \"Example\"\n").unwrap();

    assert_eq!(config.style.mode, StyleMode::Scss);
    assert_eq!(config.style.source, StyleSource::Css);
    assert_eq!(config.style.sass_compiler, SassCompiler::Grass);
    assert_eq!(config.style.utility, UtilityEngine::None);
    assert!(!config.scripts.enabled);
    assert!(!config.scripts.typecheck);
    assert!(!config.scripts.interactions);
}

#[test]
fn parses_independent_sass_utility_and_script_options() {
    let config = ProjectConfig::parse(
        r#"
            name = "Example"

            [style]
            source = "scss"
            sass_compiler = "dart-sass"
            utility = "uno-css"

            [style.unocss]
            preset = "wind4"

            [scripts]
            enabled = true
            entry_points = ["scripts/main.ts"]
            typecheck = true
        "#,
    )
    .unwrap();

    assert_eq!(config.style.source, StyleSource::Scss);
    assert_eq!(config.style.sass_compiler, SassCompiler::DartSass);
    assert_eq!(config.style.utility, UtilityEngine::UnoCss);
    assert_eq!(config.style.unocss.preset.as_deref(), Some("wind4"));
    assert!(config.scripts.enabled);
    assert_eq!(config.scripts.entry_points.len(), 1);
    assert!(config.scripts.typecheck);
}

#[test]
fn supports_legacy_tailwind_mode_as_scss_with_tailwind_adapter() {
    let config =
        ProjectConfig::parse("name = \"Example\"\n[style]\nmode = \"tailwind\"\n").unwrap();

    assert_eq!(config.style.mode, StyleMode::Tailwind);
    assert_eq!(config.style.source, StyleSource::Scss);
    assert_eq!(config.style.utility, UtilityEngine::Tailwind);
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
fn rejects_conflicting_legacy_mode_and_new_source() {
    let error =
        ProjectConfig::parse("name = \"Example\"\n[style]\nmode = \"css\"\nsource = \"scss\"\n")
            .unwrap_err()
            .to_string();

    assert!(error.contains("style.mode"), "unexpected error: {error}");
}

#[test]
fn rejects_conflicting_legacy_tailwind_and_unocss_selection() {
    let error = ProjectConfig::parse(
        "name = \"Example\"\n[style]\nmode = \"tailwind\"\nutility = \"uno-css\"\n",
    )
    .unwrap_err()
    .to_string();

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
