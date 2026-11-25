use std::fs;
use std::process::Command;

fn tessera(dir: &std::path::Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_tessera"))
        .args(args)
        .current_dir(dir)
        .output()
        .expect("failed to run tessera")
}

#[test]
fn add_component_writes_a_stub_that_compiles() {
    let dir = tempfile::tempdir().unwrap();
    let output = tessera(dir.path(), &["add-component", "site-hero"]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stub_path = dir.path().join("src/components/site-hero.rs");
    let stub = fs::read_to_string(&stub_path).unwrap();
    assert_eq!(
        stub,
        "// site-hero component placeholder\n\
         pub fn render() -> &'static str {\n    \
         r#\"<div class=\"site-hero\"></div>\"#\n\
         }\n"
    );

    // The generated file must be valid Rust (issue #5).
    let rustc = std::env::var("RUSTC").unwrap_or_else(|_| "rustc".to_string());
    let check = Command::new(rustc)
        .args([
            "--edition",
            "2021",
            "--crate-type",
            "lib",
            "--emit",
            "metadata",
            "--crate-name",
            "stub",
            "-o",
        ])
        .arg(dir.path().join("stub.rmeta"))
        .arg(&stub_path)
        .output()
        .expect("failed to run rustc");
    assert!(
        check.status.success(),
        "generated stub does not compile:\n{}",
        String::from_utf8_lossy(&check.stderr)
    );
}

#[test]
fn add_component_rejects_names_that_would_break_the_stub() {
    let dir = tempfile::tempdir().unwrap();
    for name in ["bad\"name", "9lives", "has space", "../escape", ""] {
        let output = tessera(dir.path(), &["add-component", name]);
        assert!(!output.status.success(), "name {name:?} should be rejected");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("Invalid component name"),
            "name {name:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    assert!(
        !dir.path().join("src").exists(),
        "rejected names must not create files"
    );
}
