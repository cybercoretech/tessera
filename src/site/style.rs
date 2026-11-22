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
  padding: 1rem;
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

"#
    }
}
