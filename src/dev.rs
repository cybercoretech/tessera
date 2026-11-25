use crate::site::Engine;
use crate::token::TokenSet;
use log::{error, info, warn};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::io::Cursor;
use std::net::{IpAddr, SocketAddr};
use std::path::{Component, Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tessera_config::ProjectConfig;
use tiny_http::{Header, Response, Server, StatusCode};

type DynResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Polled by the injected reload script; answers with the build generation.
const RELOAD_PATH: &str = "/__tessera/reload";

/// Injected into HTML responses served by `tessera dev` only (never into
/// `dist/`). It polls the build generation and reloads the page when a
/// rebuild has succeeded since the page loaded.
const RELOAD_SCRIPT: &str = r#"<script>(() => {
  let seen = null;
  const poll = async () => {
    try {
      const res = await fetch("/__tessera/reload", { cache: "no-store" });
      const generation = await res.text();
      if (seen !== null && generation !== seen) { location.reload(); return; }
      seen = generation;
    } catch (_) { /* server restarting: keep polling */ }
    setTimeout(poll, 1000);
  };
  poll();
})();</script>"#;

const WATCH_DIRS: [&str; 6] = [
    "content",
    "layouts",
    "templates",
    "styles",
    "scripts",
    "public",
];

/// Runs the development server for the project in the current directory:
/// serves `dist/`, rebuilds when sources change, and reloads open pages.
pub fn run(host: IpAddr, port: u16) -> DynResult<()> {
    let root = std::env::current_dir()?;
    let generation = Arc::new(AtomicU64::new(0));
    // The watcher stops when dropped, so keep it alive for the server's lifetime.
    let _watcher = watch(&root, Arc::clone(&generation))?;

    let addr = SocketAddr::new(host, port);
    let server = Server::http(addr)?;
    let shown = if host.is_loopback() || host.is_unspecified() {
        format!("http://localhost:{port}")
    } else {
        format!("http://{addr}")
    };
    info!("[dev] Server listening on {shown}");
    println!("[dev] Server listening on {shown}");
    if host.is_unspecified() {
        println!("[dev] Warning: listening on all network interfaces ({addr}); other devices on your network can reach this server");
    }
    println!("[dev] Live reload on: open pages reload after each successful rebuild");

    serve(&server, &root.join("dist"), &generation);
    Ok(())
}

/// Watches the project's source folders and `tessera.toml`, rebuilding on change.
fn watch(root: &Path, generation: Arc<AtomicU64>) -> DynResult<RecommendedWatcher> {
    let build_root = root.to_path_buf();
    let mut watcher =
        notify::recommended_watcher(move |res: Result<notify::Event, notify::Error>| match res {
            Ok(event) => {
                info!("[dev] Detected file change: {:?}", event.paths);
                match rebuild(&build_root) {
                    Ok(count) => {
                        generation.fetch_add(1, Ordering::SeqCst);
                        info!("[dev] Rebuilt {count} page(s)");
                        println!("[dev] Rebuilt {count} page(s)");
                    }
                    Err(e) => {
                        error!("[dev] Rebuild error: {e}");
                        eprintln!("[dev] Rebuild error: {e}");
                    }
                }
            }
            Err(e) => {
                warn!("[dev] watcher error: {e:?}");
                eprintln!("[dev] watcher error: {e:?}");
            }
        })?;

    for dir in WATCH_DIRS {
        let path = root.join(dir);
        if path.is_dir() {
            match watcher.watch(&path, RecursiveMode::Recursive) {
                Ok(()) => info!("[dev] Watching {}", path.display()),
                Err(e) => warn!("[dev] Could not watch {}: {e}", path.display()),
            }
        }
    }
    let config_path = root.join("tessera.toml");
    if config_path.is_file() {
        if let Err(e) = watcher.watch(&config_path, RecursiveMode::NonRecursive) {
            warn!("[dev] Could not watch {}: {e}", config_path.display());
        }
    }
    Ok(watcher)
}

fn rebuild(root: &Path) -> DynResult<usize> {
    let contents = std::fs::read_to_string(root.join("tessera.toml"))?;
    let config = ProjectConfig::parse(&contents)?;
    let token_set = TokenSet::from_toml(&contents)?;
    Engine::at_root(&token_set, Some(&config), root.to_path_buf()).build()
}

/// Answers requests until the server stops. A failed response (for example
/// a browser that closed the connection) is logged and does not stop the server.
fn serve(server: &Server, dist: &Path, generation: &AtomicU64) {
    for request in server.incoming_requests() {
        let response = respond(request.url(), dist, generation.load(Ordering::SeqCst));
        if let Err(e) = request.respond(response) {
            warn!("[dev] Could not send response: {e}");
        }
    }
}

fn respond(url: &str, dist: &Path, generation: u64) -> Response<Cursor<Vec<u8>>> {
    let path = url.split(['?', '#']).next().unwrap_or("/");
    if path == RELOAD_PATH {
        return with_headers(
            Response::from_string(generation.to_string()),
            "text/plain; charset=utf-8",
        );
    }
    let Some(file) = resolve(dist, path) else {
        info!("[dev] {url} -> 404");
        return with_headers(
            Response::from_string("Not Found").with_status_code(StatusCode(404)),
            "text/plain; charset=utf-8",
        );
    };
    info!("[dev] {url} -> {}", file.display());
    match std::fs::read(&file) {
        Ok(mut body) => {
            let mime = content_type(&file);
            if mime.starts_with("text/html") {
                body = inject_reload(body);
            }
            with_headers(Response::from_data(body), mime)
        }
        Err(e) => {
            warn!("[dev] Could not read {}: {e}", file.display());
            with_headers(
                Response::from_string("Internal Server Error").with_status_code(StatusCode(500)),
                "text/plain; charset=utf-8",
            )
        }
    }
}

fn with_headers(response: Response<Cursor<Vec<u8>>>, mime: &str) -> Response<Cursor<Vec<u8>>> {
    [
        ("Content-Type", mime),
        ("Cache-Control", "no-store"),
        ("X-Content-Type-Options", "nosniff"),
    ]
    .into_iter()
    .filter_map(|(name, value)| Header::from_bytes(name.as_bytes(), value.as_bytes()).ok())
    .fold(response, |response, header| response.with_header(header))
}

/// Maps a request path to a file inside `dist/`, or `None` when it does not
/// name a file there. The path is percent-decoded; `..`, absolute paths, NUL
/// and backslashes are rejected, and the final file must still resolve inside
/// `dist/` after following symlinks.
fn resolve(dist: &Path, url_path: &str) -> Option<PathBuf> {
    let decoded = percent_decode(url_path)?;
    if decoded.contains('\0') || decoded.contains('\\') {
        return None;
    }
    let mut path = dist.to_path_buf();
    for component in Path::new(decoded.trim_start_matches('/')).components() {
        match component {
            Component::Normal(part) => path.push(part),
            Component::CurDir => {}
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => return None,
        }
    }
    if path.is_dir() {
        path.push("index.html");
    }
    let file = path.canonicalize().ok()?;
    let dist = dist.canonicalize().ok()?;
    (file.starts_with(&dist) && file.is_file()).then_some(file)
}

fn percent_decode(input: &str) -> Option<String> {
    let bytes = input.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' {
            let hex = std::str::from_utf8(bytes.get(i + 1..i + 3)?).ok()?;
            out.push(u8::from_str_radix(hex, 16).ok()?);
            i += 3;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    String::from_utf8(out).ok()
}

fn content_type(path: &Path) -> &'static str {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(str::to_ascii_lowercase)
        .unwrap_or_default();
    match ext.as_str() {
        "html" | "htm" => "text/html; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "js" | "mjs" => "text/javascript; charset=utf-8",
        "json" | "map" => "application/json",
        "txt" => "text/plain; charset=utf-8",
        "xml" => "application/xml",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "avif" => "image/avif",
        "ico" => "image/x-icon",
        "woff" => "font/woff",
        "woff2" => "font/woff2",
        "ttf" => "font/ttf",
        "otf" => "font/otf",
        "wasm" => "application/wasm",
        "pdf" => "application/pdf",
        "mp4" => "video/mp4",
        "webm" => "video/webm",
        "webmanifest" => "application/manifest+json",
        _ => "application/octet-stream",
    }
}

/// Inserts the reload script before the last `</body>`, or appends it.
fn inject_reload(body: Vec<u8>) -> Vec<u8> {
    let lower = body.to_ascii_lowercase();
    let at = lower
        .windows(7)
        .rposition(|w| w == b"</body>")
        .unwrap_or(body.len());
    let mut out = Vec::with_capacity(body.len() + RELOAD_SCRIPT.len());
    out.extend_from_slice(&body[..at]);
    out.extend_from_slice(RELOAD_SCRIPT.as_bytes());
    out.extend_from_slice(&body[at..]);
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::{Read, Write};
    use std::net::TcpStream;

    /// A temp folder holding `dist/` plus a secret file beside it.
    fn site() -> tempfile::TempDir {
        let dir = tempfile::tempdir().unwrap();
        let dist = dir.path().join("dist");
        fs::create_dir_all(dist.join("docs")).unwrap();
        fs::write(
            dist.join("index.html"),
            "<html><body><h1>Home</h1></body></html>",
        )
        .unwrap();
        fs::write(dist.join("docs/index.html"), "<p>Docs</p>").unwrap();
        fs::write(dist.join("site.js"), "export {};").unwrap();
        fs::write(dist.join("my page.html"), "<p>spaces</p>").unwrap();
        fs::write(dir.path().join("secret.txt"), "TOP SECRET").unwrap();
        dir
    }

    #[test]
    fn resolve_serves_files_and_directory_indexes() {
        let dir = site();
        let dist = dir.path().join("dist");
        let canon = dist.canonicalize().unwrap();
        assert_eq!(resolve(&dist, "/"), Some(canon.join("index.html")));
        assert_eq!(
            resolve(&dist, "/docs/"),
            Some(canon.join("docs/index.html"))
        );
        assert_eq!(resolve(&dist, "/docs"), Some(canon.join("docs/index.html")));
        assert_eq!(resolve(&dist, "/site.js"), Some(canon.join("site.js")));
        assert_eq!(
            resolve(&dist, "/my%20page.html"),
            Some(canon.join("my page.html"))
        );
        assert_eq!(resolve(&dist, "/./site.js"), Some(canon.join("site.js")));
        assert_eq!(resolve(&dist, "/missing.html"), None);
    }

    #[test]
    fn resolve_rejects_paths_outside_dist() {
        let dir = site();
        let dist = dir.path().join("dist");
        for attempt in [
            "/../secret.txt",
            "/docs/../../secret.txt",
            "/%2e%2e/secret.txt",
            "/%2E%2E%2Fsecret.txt",
            "/..%2fsecret.txt",
            "/..\\secret.txt",
            "/%00",
            "/%zz",
            "/%2",
        ] {
            assert_eq!(resolve(&dist, attempt), None, "{attempt} must not resolve");
        }
    }

    #[cfg(unix)]
    #[test]
    fn resolve_rejects_symlinks_that_leave_dist() {
        let dir = site();
        let dist = dir.path().join("dist");
        std::os::unix::fs::symlink(dir.path().join("secret.txt"), dist.join("link.txt")).unwrap();
        assert_eq!(resolve(&dist, "/link.txt"), None);
    }

    #[test]
    fn content_types_follow_the_extension() {
        assert_eq!(
            content_type(Path::new("a.html")),
            "text/html; charset=utf-8"
        );
        assert_eq!(content_type(Path::new("a.CSS")), "text/css; charset=utf-8");
        assert_eq!(
            content_type(Path::new("a.mjs")),
            "text/javascript; charset=utf-8"
        );
        assert_eq!(content_type(Path::new("a.svg")), "image/svg+xml");
        assert_eq!(content_type(Path::new("a.woff2")), "font/woff2");
        assert_eq!(content_type(Path::new("noext")), "application/octet-stream");
    }

    #[test]
    fn reload_script_goes_before_the_last_body_tag() {
        let out = String::from_utf8(inject_reload(
            b"<html><BODY><p>x</p></BODY></html>".to_vec(),
        ))
        .unwrap();
        assert!(out.starts_with("<html><BODY><p>x</p><script>"));
        assert!(out.ends_with("</script></BODY></html>"));
        let fragment = String::from_utf8(inject_reload(b"<p>no body</p>".to_vec())).unwrap();
        assert!(fragment.starts_with("<p>no body</p><script>"));
    }

    fn get(addr: SocketAddr, path: &str) -> String {
        let mut stream = TcpStream::connect(addr).unwrap();
        write!(
            stream,
            "GET {path} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n"
        )
        .unwrap();
        let mut response = String::new();
        stream.read_to_string(&mut response).unwrap();
        response
    }

    #[test]
    fn running_server_blocks_traversal_and_sends_types_and_reload() {
        let dir = site();
        let dist = dir.path().join("dist");
        let server = Server::http("127.0.0.1:0").unwrap();
        let addr = server.server_addr().to_ip().unwrap();
        let generation = Arc::new(AtomicU64::new(7));
        let shared = Arc::clone(&generation);
        std::thread::spawn(move || serve(&server, &dist, &shared));

        for attempt in [
            "/../secret.txt",
            "/%2e%2e/secret.txt",
            "/docs/../../secret.txt",
        ] {
            let response = get(addr, attempt);
            assert!(
                response.starts_with("HTTP/1.1 404"),
                "{attempt}: {response}"
            );
            assert!(
                !response.contains("TOP SECRET"),
                "{attempt} leaked the file"
            );
        }

        let js = get(addr, "/site.js");
        assert!(js.starts_with("HTTP/1.1 200"), "{js}");
        assert!(
            js.to_ascii_lowercase()
                .contains("content-type: text/javascript"),
            "{js}"
        );
        assert!(
            js.to_ascii_lowercase().contains("cache-control: no-store"),
            "{js}"
        );

        let home = get(addr, "/");
        assert!(
            home.to_ascii_lowercase()
                .contains("content-type: text/html"),
            "{home}"
        );
        assert!(
            home.contains(RELOAD_PATH),
            "dev pages must include the reload script"
        );

        assert!(get(addr, RELOAD_PATH).ends_with("\r\n\r\n7"));
        generation.fetch_add(1, Ordering::SeqCst);
        assert!(get(addr, RELOAD_PATH).ends_with("\r\n\r\n8"));
    }
}
