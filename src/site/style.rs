use std::fs;
use std::path::Path;
use crate::token::TokenSet;

/// Compiles CSS by bundling design token variables, Tessera Kit base styles,
/// and any custom stylesheets located in `styles/`.
pub struct StyleCompiler;

impl StyleCompiler {
    /// Compiles all styles into a single CSS string.
    pub fn compile(tokens: &TokenSet, root: &Path) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let mut css = String::new();

        // 1. Emit compiled design token variables
        css.push_str(&tokens.compile_css());
        css.push('\n');

        // 2. Emit Tessera Kit base semantic styles
        css.push_str(Self::tessera_kit_base());
        css.push('\n');

        // 3. Append custom project stylesheet if present (styles/main.css or styles/styles.css)
        let custom_css_paths = [
            root.join("styles").join("main.css"),
            root.join("styles").join("style.css"),
            root.join("styles").join("styles.css"),
        ];

        for path in &custom_css_paths {
            if path.is_file() {
                let user_css = fs::read_to_string(path)?;
                css.push_str("/* Custom Project Styles */\n");
                css.push_str(&user_css);
                css.push('\n');
                break;
            }
        }

        Ok(css)
    }

    /// Default base CSS containing resets, typography, and foundational `ts-*` component classes.
    fn tessera_kit_base() -> &'static str {
        r#"/* Tessera Kit Base v1 */
*, *::before, *::after {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}

body {
  font-family: var(--ts-font-main, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif);
  line-height: 1.6;
  color: var(--ts-color-text, #1a202c);
  background-color: var(--ts-color-bg, #ffffff);
  -webkit-font-smoothing: antialiased;
}

a {
  color: var(--ts-primary, #0066ff);
  text-decoration: none;
}
a:hover {
  text-decoration: underline;
}

h1, h2, h3, h4, h5, h6 {
  line-height: 1.25;
  margin-top: 1.5rem;
  margin-bottom: 0.75rem;
  color: var(--ts-color-heading, #111827);
  font-weight: 700;
}

h1 { font-size: 2.25rem; }
h2 { font-size: 1.75rem; }
h3 { font-size: 1.35rem; }

p, ul, ol, blockquote, pre, table {
  margin-bottom: 1.25rem;
}

ul, ol {
  padding-left: 1.5rem;
}

code {
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
  font-size: 0.9em;
  padding: 0.15rem 0.35rem;
  border-radius: 4px;
  background-color: var(--ts-color-code-bg, #f3f4f6);
}

pre {
  position: relative;
  padding: 1.25rem 1rem 1rem 1rem;
  border-radius: 8px;
  background-color: var(--ts-color-code-block, #1f2937);
  color: #f9fafb;
  overflow-x: auto;
}
pre code {
  padding: 0;
  background-color: transparent;
  color: inherit;
}

blockquote {
  border-left: 4px solid var(--ts-primary, #0066ff);
  padding-left: 1rem;
  color: var(--ts-color-muted, #4b5563);
  font-style: italic;
}

/* Tessera Semantic Layout & Components */
.ts-container {
  width: 100%;
  max-width: var(--ts-container-max, 1024px);
  margin-left: auto;
  margin-right: auto;
  padding-left: 1.5rem;
  padding-right: 1.5rem;
}

.ts-section {
  padding-top: 3rem;
  padding-bottom: 3rem;
}

.ts-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  gap: var(--ts-grid-gap, 1.5rem);
}

.ts-button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  padding: 0.6rem 1.25rem;
  font-size: 1rem;
  font-weight: 600;
  border-radius: 6px;
  border: 1px solid transparent;
  cursor: pointer;
  background-color: var(--ts-primary, #0066ff);
  color: #ffffff;
  transition: opacity 0.15s ease;
}
.ts-button:hover {
  opacity: 0.9;
  text-decoration: none;
}
.ts-button--secondary {
  background-color: var(--ts-secondary, #ff6600);
}

.ts-card {
  background-color: var(--ts-color-card, #ffffff);
  border: 1px solid var(--ts-border-color, #e5e7eb);
  border-radius: 8px;
  padding: 1.5rem;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
}

.ts-badge {
  display: inline-block;
  padding: 0.2rem 0.5rem;
  font-size: 0.75rem;
  font-weight: 700;
  border-radius: 9999px;
  background-color: var(--ts-color-code-bg, #e5e7eb);
  color: var(--ts-color-text, #374151);
  text-transform: uppercase;
}

.ts-alert {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 1rem 1.25rem;
  border-radius: 6px;
  border-left: 4px solid var(--ts-primary, #0066ff);
  background-color: var(--ts-color-alert-bg, #eff6ff);
  color: var(--ts-color-alert-text, #1e3a8a);
  margin-bottom: 1.25rem;
}

.ts-alert__dismiss {
  background: transparent;
  border: none;
  font-size: 1.25rem;
  cursor: pointer;
  color: inherit;
  opacity: 0.65;
  margin-left: 1rem;
}
.ts-alert__dismiss:hover {
  opacity: 1;
}

/* Navigation & Interactive Header */
.ts-nav {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding-top: 1rem;
  padding-bottom: 1rem;
}

.ts-nav__links {
  display: flex;
  align-items: center;
  gap: 1.5rem;
  list-style: none;
  margin: 0;
  padding: 0;
}

.ts-nav__toggle {
  display: none;
  background: transparent;
  border: 1px solid var(--ts-border-color, #e5e7eb);
  border-radius: 6px;
  padding: 0.4rem 0.75rem;
  font-size: 1rem;
  cursor: pointer;
  color: inherit;
}

.ts-theme-toggle {
  background: transparent;
  border: 1px solid var(--ts-border-color, #e5e7eb);
  border-radius: 6px;
  padding: 0.4rem 0.75rem;
  cursor: pointer;
  font-size: 0.9rem;
  color: inherit;
  transition: background-color 0.15s ease;
}
.ts-theme-toggle:hover {
  background-color: var(--ts-color-code-bg, #f3f4f6);
}

/* Mobile Drawer (Progressive Enhancement) */
@media (max-width: 768px) {
  .ts-nav__toggle {
    display: inline-flex;
  }
  .ts-nav__links {
    display: none;
  }
  .ts-drawer {
    display: none;
    position: fixed;
    top: 0;
    right: 0;
    width: 280px;
    height: 100vh;
    background-color: var(--ts-color-card, #ffffff);
    border-left: 1px solid var(--ts-border-color, #e5e7eb);
    padding: 2rem 1.5rem;
    box-shadow: -4px 0 16px rgba(0, 0, 0, 0.1);
    z-index: 1000;
  }
  .ts-drawer.ts-open {
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
  }
}

/* Dark Theme Overrides */
@media (prefers-color-scheme: dark) {
  :root:not([data-theme="light"]) {
    --ts-color-bg: #0f172a;
    --ts-color-text: #f1f5f9;
    --ts-color-heading: #ffffff;
    --ts-color-card: #1e293b;
    --ts-border-color: #334155;
    --ts-color-code-bg: #1e293b;
    --ts-color-code-block: #0b0f19;
    --ts-color-alert-bg: #1e293b;
    --ts-color-alert-text: #93c5fd;
  }
}

[data-theme="dark"] {
  --ts-color-bg: #0f172a;
  --ts-color-text: #f1f5f9;
  --ts-color-heading: #ffffff;
  --ts-color-card: #1e293b;
  --ts-border-color: #334155;
  --ts-color-code-bg: #1e293b;
  --ts-color-code-block: #0b0f19;
  --ts-color-alert-bg: #1e293b;
  --ts-color-alert-text: #93c5fd;
}

/* 1. Code Copy Button */
.ts-code-copy {
  position: absolute;
  top: 0.5rem;
  right: 0.5rem;
  padding: 0.25rem 0.6rem;
  font-size: 0.75rem;
  font-weight: 600;
  border-radius: 4px;
  border: 1px solid rgba(255, 255, 255, 0.2);
  background: rgba(0, 0, 0, 0.4);
  color: #f9fafb;
  cursor: pointer;
  transition: all 0.15s ease;
  backdrop-filter: blur(4px);
}
.ts-code-copy:hover {
  background: rgba(0, 0, 0, 0.7);
  border-color: rgba(255, 255, 255, 0.4);
}
.ts-code-copy.ts-copied {
  background: #10b981;
  border-color: #10b981;
}

/* 2. Reading Progress Indicator */
.ts-reading-progress {
  position: fixed;
  top: 0;
  left: 0;
  height: 3px;
  width: 0%;
  background: var(--ts-primary, #0066ff);
  z-index: 9999;
  transition: width 0.1s ease-out;
}

/* 3. Accessible Tabs */
.ts-tabs {
  border-bottom: 2px solid var(--ts-border-color, #e5e7eb);
  margin-bottom: 1.5rem;
}
.ts-tab-list {
  display: flex;
  gap: 1rem;
  list-style: none;
  margin: 0;
  padding: 0;
}
.ts-tab {
  background: transparent;
  border: none;
  padding: 0.75rem 1rem;
  font-size: 0.95rem;
  font-weight: 600;
  color: var(--ts-color-muted, #6b7280);
  cursor: pointer;
  border-bottom: 2px solid transparent;
  margin-bottom: -2px;
  transition: all 0.15s ease;
}
.ts-tab:hover {
  color: var(--ts-color-text, #111827);
}
.ts-tab--active, .ts-tab[aria-selected="true"] {
  color: var(--ts-primary, #0066ff);
  border-bottom-color: var(--ts-primary, #0066ff);
}
.ts-tabpanel[hidden] {
  display: none;
}

/* 4. Instant Search Trigger & Modal */
.ts-search-btn {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  background: transparent;
  border: 1px solid var(--ts-border-color, #e5e7eb);
  border-radius: 6px;
  padding: 0.35rem 0.75rem;
  font-size: 0.85rem;
  color: inherit;
  cursor: pointer;
}
.ts-search-btn__hint {
  font-size: 0.7rem;
  padding: 0.1rem 0.3rem;
  border-radius: 4px;
  background-color: var(--ts-color-code-bg, #e5e7eb);
  color: inherit;
}
.ts-search-dialog {
  border: none;
  border-radius: 12px;
  background-color: var(--ts-color-card, #ffffff);
  color: var(--ts-color-text, #111827);
  box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.2), 0 10px 10px -5px rgba(0, 0, 0, 0.1);
  width: 90vw;
  max-width: 580px;
  margin: 10vh auto auto auto;
  padding: 0;
  overflow: hidden;
}
.ts-search-dialog::backdrop {
  background: rgba(0, 0, 0, 0.5);
  backdrop-filter: blur(4px);
}
.ts-search-input {
  width: 100%;
  border: none;
  border-bottom: 1px solid var(--ts-border-color, #e5e7eb);
  padding: 1rem 1.25rem;
  font-size: 1.1rem;
  outline: none;
  background: transparent;
  color: inherit;
}
.ts-search-results {
  max-height: 340px;
  overflow-y: auto;
  padding: 0.5rem;
}
.ts-search-item {
  display: block;
  padding: 0.75rem 1rem;
  border-radius: 8px;
  color: inherit;
  text-decoration: none;
  transition: background-color 0.1s ease;
}
.ts-search-item:hover {
  background-color: var(--ts-color-code-bg, #f3f4f6);
  text-decoration: none;
}
.ts-search-item__title {
  font-weight: 600;
  color: var(--ts-primary, #0066ff);
}
.ts-search-item__desc {
  font-size: 0.85rem;
  color: var(--ts-color-muted, #6b7280);
  margin-top: 0.15rem;
}
.ts-search-hint, .ts-search-empty {
  padding: 2rem 1rem;
  text-align: center;
  color: var(--ts-color-muted, #6b7280);
  font-size: 0.9rem;
}

/* 5. Image Lightbox */
.ts-lightbox {
  display: none;
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.88);
  backdrop-filter: blur(6px);
  z-index: 10000;
  align-items: center;
  justify-content: center;
  padding: 1.5rem;
  cursor: zoom-out;
}
.ts-lightbox.ts-open {
  display: flex;
}
.ts-lightbox__img {
  max-width: 90vw;
  max-height: 90vh;
  object-fit: contain;
  border-radius: 8px;
  box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
  cursor: default;
}
.ts-lightbox__close {
  position: absolute;
  top: 1.5rem;
  right: 1.5rem;
  background: transparent;
  border: none;
  font-size: 2rem;
  color: #ffffff;
  cursor: pointer;
  line-height: 1;
}


"#
    }
}
