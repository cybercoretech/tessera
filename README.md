# Tessera

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![GitHub Repository](https://img.shields.io/badge/github-cybercoretech%2Ftessera-black.svg)](https://github.com/cybercoretech/tessera)

**Tessera** is a lightweight, token-driven static site generator and scaffolding CLI built in Rust. It pairs markdown-based content generation with a composable design token system and a built-in development server with automatic file watching and live rebuilding.

---

## Features

- **Project Scaffolding**: Quickly spin up structured static sites with `tessera new <name>`.
- **Design Token System**: Define and update theme tokens (`primary`, `secondary`, spacing, colors) in `tessera.toml` and reference them across layouts and components.
- **Content & Component Generators**: Scaffold markdown pages (`tessera add-page <name>`) and reusable component stubs (`tessera add-component <name>`).
- **Live Development Server**: `tessera dev` serves your built static site and automatically rebuilds when markdown source files change.
- **Project Doctor**: `tessera doctor` validates project configuration, verifies token definitions, and checks directory health.
- **Static Site Engine**: Fast Markdown-to-HTML rendering into `dist/`.
- **Configurable & Clean**: Custom port flags (`-p` / `--port`), clean command (`tessera clean`), and structured logging support via `env_logger`.

---

## Installation & Build

Build Tessera directly from source using Cargo:

```bash
# Clone the repository
git clone https://github.com/cybercoretech/tessera.git
cd tessera

# Build the release binary
cargo build --release

# The executable will be available at:
./target/release/tessera --help
```

---

## Quickstart

### 1. Initialize a New Project
```bash
tessera new my-blog
cd my-blog
```

This creates the canonical project structure:
```text
my-blog/
├── content/           # Markdown content files
│   └── index.md
├── dist/              # Generated HTML files
├── templates/         # Page templates
│   └── page.md
├── src/
│   └── components/    # Reusable component stubs
└── tessera.toml       # Site metadata and design tokens
```

### 2. Check Project Health
```bash
tessera doctor
```
Output:
```text
[doctor] ✓ tessera.toml parsed successfully
[doctor]   Tokens defined: 2
[doctor] ✓ content/ exists with 1 markdown file(s)
[doctor] ✓ Site engine instantiated
```

### 3. Add Pages and Components
```bash
# Add an "about" page in content/about.md
tessera add-page about

# Add a "navbar" component stub in src/components/navbar.rs
tessera add-component navbar
```

### 4. Manage Design Tokens
Update or insert theme tokens directly in `tessera.toml`:
```bash
tessera theme-set primary "#0070f3"
tessera theme-set accent "#ff4081"
```

### 5. Build the Site
Compile all markdown pages to static HTML in `dist/`:
```bash
tessera build
```

### 6. Run the Live Development Server
Start the development server with live watching:
```bash
# Default port (8000)
tessera dev

# Or specify a custom port
tessera dev --port 3000
```
Open [http://localhost:8000](http://localhost:8000) in your browser. Any edits in `content/` will automatically trigger a site rebuild.

### 7. Clean Output
Remove generated artifacts:
```bash
tessera clean
```

---

## CLI Reference

| Command | Arguments / Flags | Description |
|---|---|---|
| `new` | `<name>` | Scaffolds a new project directory with templates and configuration |
| `add-page` | `<name>` | Generates a new markdown file in `content/<name>.md` |
| `add-component`| `<name>` | Creates a new component placeholder in `src/components/<name>.rs` |
| `theme-set` | `<key> <value>` | Sets or updates a design token under `[theme.tokens]` in `tessera.toml` |
| `dev` | `-p, --port <PORT>` | Starts the HTTP server (default: `8000`) and watches `content/` |
| `doctor` | — | Runs diagnostic checks on tokens, content, and engine instantiation |
| `build` | — | Compiles markdown sources and design tokens into static files in `dist/` |
| `clean` | — | Deletes the generated `dist/` build directory |

---

## Configuration (`tessera.toml`)

Project configuration and design tokens live in `tessera.toml`:

```toml
name = "my-site"
version = "0.1.0"

[theme.tokens]
primary = "#0066ff"
secondary = "#ff6600"
font-main = "system-ui, sans-serif"
```

---

## Logging & Debugging

Tessera includes structured logging via `env_logger`. To enable verbose diagnostic logs:

```bash
RUST_LOG=info tessera dev
```

---

## License

Dual-licensed under either:
- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [MIT license](http://opensource.org/licenses/MIT)

at your option.
