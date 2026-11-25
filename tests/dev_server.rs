use std::fs;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

/// Kills the dev server when the test ends, even if an assertion fails.
struct DevServer(Child);

impl Drop for DevServer {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

fn tessera(dir: &Path, args: &[&str]) {
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

fn get(addr: SocketAddr, path: &str) -> Option<String> {
    let mut stream = TcpStream::connect_timeout(&addr, Duration::from_secs(1)).ok()?;
    stream.set_read_timeout(Some(Duration::from_secs(5))).ok()?;
    write!(
        stream,
        "GET {path} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n"
    )
    .ok()?;
    let mut response = String::new();
    stream.read_to_string(&mut response).ok()?;
    Some(response)
}

fn wait_for(what: &str, mut check: impl FnMut() -> bool) {
    let start = Instant::now();
    while !check() {
        assert!(
            start.elapsed() < Duration::from_secs(15),
            "timed out waiting for {what}"
        );
        std::thread::sleep(Duration::from_millis(100));
    }
}

#[test]
fn dev_serves_the_current_project_and_reports_rebuilds() {
    let dir = tempfile::tempdir().unwrap();
    tessera(dir.path(), &["new", "site"]);
    let project = dir.path().join("site");
    tessera(&project, &["build"]);

    let port = TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port();
    let addr: SocketAddr = ([127, 0, 0, 1], port).into();
    let _server = DevServer(
        Command::new(env!("CARGO_BIN_EXE_tessera"))
            .args(["dev", "--port", &port.to_string()])
            .current_dir(&project)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("failed to start tessera dev"),
    );
    wait_for("the dev server", || get(addr, "/").is_some());

    // Serves this project's dist/, with the reload script injected (#6, #9).
    let home = get(addr, "/").unwrap();
    assert!(home.starts_with("HTTP/1.1 200"), "{home}");
    assert!(home.contains("Welcome to site"), "{home}");
    assert!(home.contains("/__tessera/reload"), "{home}");
    assert!(
        !fs::read_to_string(project.join("dist/index.html"))
            .unwrap()
            .contains("/__tessera/reload"),
        "the reload script must never be written into dist/"
    );

    // An edit triggers a rebuild, which the reload endpoint reports.
    let before = get(addr, "/__tessera/reload").unwrap();
    fs::write(
        project.join("content/index.md"),
        "---\ntitle: Edited\nlayout: base\n---\n# Edited live\n",
    )
    .unwrap();
    wait_for("a rebuild", || {
        get(addr, "/__tessera/reload").is_some_and(|now| now != before)
    });
    assert!(get(addr, "/").unwrap().contains("Edited live"));
}
