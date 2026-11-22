use log::{error, info, warn};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use tiny_http::{Response, Server};
use crate::site::Engine;
use crate::token::TokenSet;

/// Returns the project root directory (two levels up from the executable path).
fn project_root() -> PathBuf {
    let exe = std::env::current_exe().expect("cannot get current exe path");
    let mut dir = exe.parent().expect("no parent dir").to_path_buf();
    dir.pop(); // debug/release
    dir.pop(); // target or .cargo-target
    dir
}

/// Runs a simple development server with live‑reload.
pub fn run(port: Option<u16>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let port = port.unwrap_or(8000);

    // ---------- file watcher ----------
    let content_path = project_root().join("content");
    let mut watcher: RecommendedWatcher = notify::recommended_watcher(move |res: Result<notify::Event, notify::Error>| {
        match res {
            Ok(event) => {
                info!("[dev] Detected file change: {:?}", event.paths);
                // Rebuild the site using a default token set.
                let token_set = TokenSet::default();
                let engine = Engine::new(&token_set);
                match engine.build() {
                    Ok(cnt) => {
                        info!("[dev] Rebuilt {} page(s)", cnt);
                        println!("[dev] Rebuilt {} page(s)", cnt);
                    }
                    Err(e) => {
                        error!("[dev] Rebuild error: {}", e);
                        eprintln!("[dev] Rebuild error: {}", e);
                    }
                }
            }
            Err(e) => {
                warn!("[dev] watcher error: {:?}", e);
                eprintln!("[dev] watcher error: {:?}", e);
            }
        }
    })?;

    if content_path.exists() {
        watcher.watch(&content_path, RecursiveMode::Recursive)?;
        info!("[dev] Watching for changes in {}", content_path.display());
    } else {
        warn!("[dev] content/ directory does not exist at {}", content_path.display());
    }

    // ---------- HTTP server ----------
    let addr = format!("0.0.0.0:{}", port);
    let server = match Server::http(&addr) {
        Ok(s) => s,
        Err(e) => return Err(e.into()),
    };
    info!("[dev] Server listening on http://localhost:{}", port);
    println!("[dev] Server listening on http://localhost:{}", port);

    for request in server.incoming_requests() {
        let url_path = request.url().trim_start_matches('/');
        let mut path = project_root();
        path.push("dist");
        if url_path.is_empty() {
            path.push("index.html");
        } else {
            path.push(url_path);
        }

        info!("[dev] {} -> {}", request.url(), path.display());

        let response = if path.is_file() {
            let body = std::fs::read(&path)?;
            Response::from_data(body)
        } else {
            Response::from_string("Not Found".to_string())
        };
        request.respond(response)?;
    }
    Ok(())
}
