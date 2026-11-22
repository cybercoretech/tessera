# ⚡ Tessera

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![GitHub Repository](https://img.shields.io/badge/github-cybercoretech%2Ftessera-black.svg)](https://github.com/cybercoretech/tessera)

**Tessera** is a lightweight, token-driven static site generator and scaffolding CLI built in Rust. It pairs markdown-based content with front-matter metadata, customizable HTML layout templates, an integrated design token compiler, and a live development server with instant rebuilds.

---

## ✨ Features

- 🏗️ **Instant Scaffolding**: Spin up production-ready static site structures with `tessera new <name>`.
- 🎨 **Design Token Compiler**: Define theme tokens (`primary`, `secondary`, fonts, spacing) in `tessera.toml` and compile them into CSS custom properties (`:root { --ts-primary: ...; }`).
- 🧩 **Tessera Kit Semantic Classes**: Built-in, accessible CSS utility classes (`ts-container`, `ts-button`, `ts-card`, `ts-badge`, `ts-alert`, `ts-grid`, `ts-section`).
- 📄 **Front Matter & Layout Templates**: Parse YAML/TOML front-matter metadata and inject rendered pages into full HTML5 layouts (`layouts/base.html`).
- 🔄 **Live Development Server**: `tessera dev` serves your static site locally with automatic file watching across `content/`, `layouts/`, `styles/`, and `tessera.toml`.
- 🩺 **Project Diagnostics**: `tessera doctor` checks token validity, configuration health, content counts, and template engine status.
- 📦 **Asset Pipeline**: Automatically copies static assets from `public/` into `dist/`.
- 🧹 **Clean & Configurable**: Custom port flag (`-p` / `--port`), artifact cleaner (`tessera clean`), and structured logging support via `env_logger`.

---

## 🚀 Installation & Build

Build Tessera from source using Cargo:

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

## 📂 Project Structure

When you create a project with `tessera new <name>`, Tessera generates:

```text
my-site/
├── content/           # 📝 Markdown pages with front-matter
│   └── index.md
├── layouts/           # 🖼️ HTML layout templates
│   └── base.html
├── styles/            # 🎨 Custom CSS/SCSS stylesheets
│   └── main.css
├── public/            # 📁 Static assets copied unchanged into dist/
├── templates/         # 📑 Page templates and component stubs
├── dist/              # 🌐 Generated static output (ignored by git)
└── tessera.toml       # ⚙️ Project metadata and design tokens
```

---

## 🏁 Quickstart

### 1. Initialize a Project
```bash
tessera new my-portfolio
cd my-portfolio
```

### 2. Verify Health
```bash
tessera doctor
```
```text
[doctor] ✓ tessera.toml parsed successfully
[doctor]   Tokens defined: 3
[doctor] ✓ content/ exists with 1 markdown file(s)
[doctor] ✓ Site engine instantiated
```

### 3. Add Pages with Front Matter
Pages in `content/` support front matter between `---` delimiters:

```markdown
---
title: "About Me"
description: "My personal developer portfolio"
layout: base
---

# Hello, World! 👋

Welcome to my static site powered by **Tessera**.

<a class="ts-button" href="#contact">Contact Me</a>
```

You can scaffold new pages quickly:
```bash
tessera add-page contact
```

### 4. Customize Design Tokens
Update theme tokens in `tessera.toml` or via CLI:
```bash
tessera theme-set primary "#2563eb"
tessera theme-set secondary "#7c3aed"
```

### 5. Build the Site
Compile markdown, layouts, tokens, and assets into `dist/`:
```bash
tessera build
```

### 6. Run the Live Development Server
Start the local server with automatic file watching:
```bash
# Default port (8000)
tessera dev

# Or with a custom port
tessera dev --port 3000
```
Open [http://localhost:8000](http://localhost:8000). Any changes to `content/`, `layouts/`, `styles/`, or `tessera.toml` will instantly trigger a rebuild!

### 7. Clean Build Artifacts
```bash
tessera clean
```

---

## 📖 CLI Reference

| Command | Arguments / Flags | Description |
|---|---|---|
| `new` | `<name>` | 🏗️ Scaffolds a new project with templates, layouts, and tokens |
| `add-page` | `<name>` | 📄 Creates a new markdown page stub in `content/<name>.md` |
| `add-component`| `<name>` | 🧩 Generates a component placeholder in `src/components/<name>.rs` |
| `theme-set` | `<key> <value>` | 🎨 Sets or updates a token under `[theme.tokens]` in `tessera.toml` |
| `dev` | `-p, --port <PORT>` | 🔄 Starts the live development server with file watching |
| `doctor` | — | 🩺 Runs diagnostic checks on tokens, content, and the engine |
| `build` | — | 🔨 Compiles markdown sources, layouts, and styles into `dist/` |
| `clean` | — | 🧹 Deletes the generated `dist/` directory |

---

## 🎨 Design Tokens & Tessera Kit

### Configuration (`tessera.toml`)
```toml
name = "my-site"
version = "0.1.0"

[theme.tokens]
primary = "#0066ff"
secondary = "#ff6600"
font-main = "system-ui, -apple-system, sans-serif"
```

Tessera compiles these tokens into CSS custom properties:
```css
:root {
  --ts-primary: #0066ff;
  --ts-secondary: #ff6600;
  --ts-font-main: system-ui, -apple-system, sans-serif;
}
```

### Built-in Semantic Classes
Tessera Kit provides ready-to-use semantic classes out-of-the-box:
- **Layout:** `.ts-container`, `.ts-section`, `.ts-grid`
- **Buttons:** `.ts-button`, `.ts-button--secondary`
- **Cards:** `.ts-card`
- **Feedback & Tags:** `.ts-badge`, `.ts-alert`

---

## 🪵 Logging & Verbose Diagnostics

Tessera uses `env_logger`. To view detailed diagnostic and server logs:

```bash
RUST_LOG=info tessera dev
```

---

## 📜 License

Dual-licensed under either:
- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [MIT license](http://opensource.org/licenses/MIT)

at your option.
