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
- 🧵 **Optional Style Toolchains**: Compile SCSS with Grass by default or an explicitly selected Dart Sass CLI. Tailwind CSS and UnoCSS are optional, mutually exclusive utility adapters.
- 🧰 **Optional Script Build**: Bundle explicitly selected JavaScript or TypeScript entry points with esbuild. Type checking is a separate opt-in `tsc --noEmit` step.
- 🧹 **Clean & Configurable**: Custom port flag (`-p` / `--port`), artifact cleaner (`tessera clean`), and structured logging support via `env_logger`.

---

## 🚀 Installation & Build

Build Tessera from source using Cargo:

```bash
# Clone the repository
git clone https://github.com/cybercoretech/tessera.git
cd tessera

# Install the `tessera` command into ~/.cargo/bin (on your PATH with a standard Rust setup)
cargo install --path .
tessera --help

# Or run it without installing (works whatever CARGO_TARGET_DIR is set to)
cargo run --release -- --help
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
├── styles/            # 🎨 Custom CSS or SCSS stylesheets
│   └── main.css
├── scripts/           # 🧩 Optional JavaScript/TypeScript entry points
│   └── main.ts
├── public/            # 📁 Static assets copied unchanged into dist/
├── templates/         # 📑 Page templates used by `tessera add-page`
│   └── page.md
├── src/components/    # 🧩 Component stubs from `tessera add-component`
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

# Let other devices on your network (a phone, say) reach it
tessera dev --host 0.0.0.0
```
Run it from your project folder and open [http://localhost:8000](http://localhost:8000). Changes to `content/`, `layouts/`, `templates/`, `styles/`, `scripts/`, `public/`, or `tessera.toml` trigger a rebuild, and open pages reload themselves once it succeeds. The reload script is added only to pages served by `tessera dev`; it is never written into `dist/`.

By default the server listens on `127.0.0.1`, so only your own machine can reach it. It serves files from `dist/` only, with the right `Content-Type` for each file.

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
| `dev` | `-p, --port <PORT>`, `--host <ADDR>` | 🔄 Starts the development server with file watching and live reload (`127.0.0.1` by default) |
| `doctor` | — | 🩺 Runs diagnostic checks on tokens, content, and the engine |
| `build` | — | 🔨 Compiles markdown sources, layouts, and styles into `dist/` |
| `clean` | — | 🧹 Deletes the generated `dist/` directory |

---

## 🎨 Styles, utility adapters, and scripts

Generated projects default to CSS, with all external toolchains disabled. Tessera does not install npm packages or download compiler binaries for you.

```toml
[style]
source = "scss"             # css or scss
sass_compiler = "grass"     # grass or dart-sass
utility = "none"            # none, tailwind, or uno-css

[scripts]
enabled = false
entry_points = ["scripts/main.ts"]
typecheck = false
interactions = false
```

Grass compiles SCSS in-process. Selecting `sass_compiler = "dart-sass"` requires the `sass` executable on `PATH`. Tailwind requires its v4 `tailwindcss` CLI, while UnoCSS requires `unocss` and either a root `uno.config.*` file or an explicit preset:

```toml
[style]
utility = "uno-css"

[style.unocss]
preset = "wind4"
```

Select only one utility engine per build. Both adapters are independent of Tessera component classes and never translate `ts-*` classes.

To bundle scripts, enable the pipeline. The esbuild executable is required; TypeScript checking separately requires `tsc`:

```toml
[scripts]
enabled = true
entry_points = ["scripts/main.ts"]
typecheck = true
```

When scripts are disabled, Tessera emits no JavaScript by default. Set `interactions = true` to include Tessera's optional plain JavaScript enhancements.

## 🧱 Tessera Kit showcase

The editable [Kit showcase](examples/kit-showcase/) demonstrates the current foundations, layouts, controls, content patterns, and native disclosure component. From its directory, build the static page with:

```sh
cargo run --manifest-path ../../Cargo.toml -- build
```

The generated page is written to `examples/kit-showcase/dist/`.

Kit references: [foundations](docs/kit/foundations.md), [forms](docs/kit/forms.md), [themes](docs/kit/theming.md), [content patterns](docs/kit/content-patterns.md), and [sections and feedback](docs/kit/sections-feedback.md).

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
- **Navigation:** `.ts-nav`, `.ts-nav__links`, `.ts-nav__toggle`, `.ts-drawer`
- **Buttons:** `.ts-button`, `.ts-button--secondary`, `.ts-theme-toggle`
- **Cards:** `.ts-card`
- **Feedback & Tags:** `.ts-badge`, `.ts-alert`, `.ts-alert__dismiss`

---

## ⚡ Optional Progressive Enhancement & Interactivity

Tessera can include its zero-dependency client script (`dist/tessera.js`) when `[scripts].interactions = true`. Generated sites omit it by default. Keep native HTML and CSS fallbacks usable when interactions are disabled.

Components declare interactive behavior via simple `data-ts-*` attributes:

### 🌓 Dark / Light Theme Switcher
```html
<button class="ts-theme-toggle" data-ts-theme-toggle aria-label="Toggle theme">
  🌓 Toggle Theme
</button>
```
- Automatically follows user OS preferences (`@media (prefers-color-scheme: dark)`).
- Remembers user choice across page visits via `localStorage`.

### 📱 Responsive Mobile Drawer
```html
<button class="ts-nav__toggle" data-ts-toggle="mobile-drawer" aria-expanded="false">
  ☰ Menu
</button>

<nav id="mobile-drawer" class="ts-drawer">
  <a href="/">Home</a>
  <a href="/about.html">About</a>
</nav>
```
- Accessible toggle: updates `aria-expanded`.
- Keyboard friendly: closes automatically on `Escape` key and returns focus.
- Closes on outside clicks.

### 🔔 Dismissible Alerts
```html
<div class="ts-alert">
  <span>Notice: Site deployment succeeded!</span>
  <button class="ts-alert__dismiss" data-ts-dismiss="alert" aria-label="Dismiss">&times;</button>
</div>
```
- Smoothly fades out and unmounts the alert on click.

### 📋 Code Block Copy Buttons
All Markdown code blocks (`<pre><code>`) automatically receive an accessible **Copy** button:
- Clicking it copies the snippet to clipboard via the Clipboard API.
- Changes button label to `Copied! ✓` for 2 seconds with visual confirmation.

### 📑 Accessible Tabs (`data-ts-tabs`)
```html
<div data-ts-tabs>
  <div class="ts-tabs">
    <div class="ts-tab-list" role="tablist">
      <button class="ts-tab ts-tab--active" data-ts-tab="tab-rust">Rust</button>
      <button class="ts-tab" data-ts-tab="tab-js">JavaScript</button>
    </div>
  </div>
  <div class="ts-tabpanel ts-tabpanel--active" data-ts-tabpanel="tab-rust">
    <p>Rust code instructions...</p>
  </div>
  <div class="ts-tabpanel" data-ts-tabpanel="tab-js" hidden>
    <p>JavaScript code instructions...</p>
  </div>
</div>
```
- Full WAI-ARIA keyboard navigation (`ArrowRight`, `ArrowLeft`, `Home`, `End`).

### 🔍 Zero-Backend Client Search (Ctrl+K)
Tessera generates a lightweight `dist/search-index.json` during build:
- Trigger with the search button: `<button data-ts-search-trigger>🔍 Search</button>`
- Or press **`Ctrl+K`** (or `Cmd+K`) anywhere on the page to open the search modal.
- Live client-side filtering by page title and description.

### 🧭 Scroll Reading Progress Indicator
Add a progress bar at the top of the viewport:
```html
<div class="ts-reading-progress" data-ts-progress></div>
```
- Automatically tracks scroll percentage down the document with fluid CSS width transitions.

### 🖼️ Zoomable Image Lightbox
All images in `<main>` or tagged with `data-ts-lightbox` automatically become zoomable:
- Clicking an image opens a centered, fullscreen overlay.
- Closes with `Escape` or by clicking the darkened backdrop.

### 🎨 Client-Side Syntax Highlighting
Code blocks in Markdown (`language-rust`, `language-js`, `language-python`, etc.) are automatically tokenized and highlighted with language-aware colors in both light and dark themes—zero external dependencies or heavy scripts required.

### 📊 Interactive Table Sorting
All HTML/Markdown tables are automatically sortable:
- Click any table header (`<th>`) to sort columns ascending or descending.
- Intelligently parses numbers, currency, and text with directional indicators (`▲` / `▼`).

### 🍞 Toast Notifications API
Programmatically display toast alerts from anywhere in your site:
```javascript
window.tessera.toast('Changes saved successfully!', 'success');
window.tessera.toast('Failed to reach server', 'error');
window.tessera.toast('New post published', 'info');
```

### ⚡ Instant Page Prefetching
Hovering over any internal link automatically injects a `<link rel="prefetch">` tag, providing near-instantaneous page transitions with 0ms perceived latency.

### 🔗 Section Anchor Links
All `h2`, `h3`, and `h4` headings automatically gain a hoverable `#` anchor link. Clicking it updates the URL and copies the direct link to the clipboard with visual confirmation.

---

## 📁 User Custom Scripts Pipeline

You can write your own custom client-side JavaScript in `scripts/`:

```text
my-site/
└── scripts/
    └── main.js       # Your custom JavaScript logic
```

During `tessera build` and `tessera dev`:
- All `.js` files from `scripts/` are copied directly into `dist/`.
- `<script src="/main.js" defer></script>` is automatically injected into `<head>`.
- Any changes in `scripts/` immediately trigger a live browser reload.

---

## 📈 Privacy-First Analytics

Configure cookieless, lightweight privacy analytics directly in `tessera.toml`:

### Plausible Analytics
```toml
[analytics]
provider = "plausible"
domain = "yourdomain.com"
```

### Umami Analytics
```toml
[analytics]
provider = "umami"
website_id = "your-website-id-uuid"
```

### Cloudflare Web Analytics
```toml
[analytics]
provider = "cloudflare"
token = "your-cloudflare-beacon-token"
```

Tessera automatically injects the proper asynchronous snippet into `<head>` upon building.


---

## 🪵 Logging & Verbose Diagnostics

Tessera uses `env_logger`. To view detailed diagnostic and server logs:

```bash
RUST_LOG=info tessera dev
```

---

## 🧑‍💻 Development Checks

The repository is a Cargo workspace. Run the same checks used by GitHub Actions before opening a pull request:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

---

## 📜 License

Dual-licensed under either:
- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [MIT license](http://opensource.org/licenses/MIT)

at your option.
