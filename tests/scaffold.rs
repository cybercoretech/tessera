use std::fs;
use std::process::Command;

fn tessera(dir: &std::path::Path, args: &[&str]) {
    let output = Command::new(env!("CARGO_BIN_EXE_tessera"))
        .args(args)
        .current_dir(dir)
        .output()
        .expect("failed to run tessera");
    assert!(
        output.status.success(),
        "tessera {args:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn new_project_has_a_stylesheet_that_the_build_uses() {
    let dir = tempfile::tempdir().unwrap();
    tessera(dir.path(), &["new", "site"]);
    let project = dir.path().join("site");

    let main_css = fs::read_to_string(project.join("styles/main.css"))
        .expect("tessera new should create styles/main.css (#8)");
    assert!(main_css.contains("var(--ts-primary)"));

    tessera(&project, &["build"]);
    let built = fs::read_to_string(project.join("dist/styles.css")).unwrap();
    let kit = built
        .find("Tessera Kit Base")
        .expect("Kit styles in output");
    let custom = built
        .find("/* Custom Project Styles */")
        .expect("project styles in output");
    assert!(custom > kit, "project styles must come after the Kit");
    assert!(built[custom..].contains("background: var(--ts-primary);"));
}
