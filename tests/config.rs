use std::process::Command;

#[test]
fn test_cli_help() {
    let output = Command::new(env!("CARGO_BIN_EXE_tessera"))
        .arg("--help")
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("tessera"));
    assert!(stdout.contains("dev"));
    assert!(stdout.contains("doctor"));
    assert!(stdout.contains("clean"));
}

#[test]
fn test_cli_doctor() {
    let output = Command::new(env!("CARGO_BIN_EXE_tessera"))
        .arg("doctor")
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("[doctor]"));
}

#[test]
fn test_cli_build_and_clean() {
    // 1. Run build
    let build_output = Command::new(env!("CARGO_BIN_EXE_tessera"))
        .arg("build")
        .output()
        .expect("failed to execute tessera build");

    assert!(build_output.status.success());
    let stdout = String::from_utf8_lossy(&build_output.stdout);
    assert!(stdout.contains("Built"));

    // Check dist/styles.css exists
    let css = std::fs::read_to_string("dist/styles.css").expect("styles.css not found");
    assert!(css.contains("--ts-primary"));
    assert!(css.contains(".ts-container"));

    // Check dist/index.html exists and is wrapped in layout
    let html = std::fs::read_to_string("dist/index.html").expect("index.html not found");
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("<link rel=\"stylesheet\" href=\"/styles.css\">"));
    assert!(html.contains("<script src=\"/tessera.js\" defer></script>"));

    // Check dist/tessera.js exists
    let js = std::fs::read_to_string("dist/tessera.js").expect("tessera.js not found");
    assert!(js.contains("data-ts-theme-toggle"));
    assert!(js.contains("data-ts-toggle"));

    // 2. Run clean
    let clean_output = Command::new(env!("CARGO_BIN_EXE_tessera"))
        .arg("clean")
        .output()
        .expect("failed to execute tessera clean");
    assert!(clean_output.status.success());
    assert!(!std::path::Path::new("dist").exists());
}
