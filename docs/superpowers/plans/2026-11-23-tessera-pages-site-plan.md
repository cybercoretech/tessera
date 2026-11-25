# Tessera project website plan

**Status:** Redesign approved 2026-11-25. Initial (editorial) site still live.
**Target:** [https://tessera-8gd.pages.dev](https://tessera-8gd.pages.dev/) on Cloudflare Pages
**Implementation:** The site is built with Tessera itself, from `website/`. Deployment is manual and explicit. Cloudflare assigned the `-8gd` suffix; Pages `pages.dev` subdomains cannot be changed after project creation.

## Direction change (2026-11-25)

The first version used an "editorial workbench" look: serif display type, warm
paper surfaces, a deliberately non-Cybercore identity. The owner reviewed it next
to [dwcode.pages.dev](https://dwcode.pages.dev/) and asked for the opposite: the
Tessera site should look and behave like the Cybercore family sites. This plan
replaces the editorial direction.

Approved scope, in order:

| # | Idea | Phase |
| --- | --- | --- |
| 1 | Editor-shell layout | Now |
| 2 | Live build hero | Now |
| 3 | Token playground | Now |
| 4 | Kit explorer tree with detail sheets | Now |
| 7 | Standard Cybercore chrome | Now |
| 5 | Command cards with detail sheets | Later |
| 6 | Toolchain matrix | Later |
| 8 | "Built with Tessera" build footer | Later |

## Purpose (unchanged)

A visitor should quickly learn:

1. What Tessera is and what it generates.
2. What it can do today, and what is optional or planned.
3. How to install it and start a project.
4. Where the Kit guides, source and releases are.

The site is also living proof of the generator: it is an ordinary Tessera project.

## Design direction: the dwcode style

Reuse the style that landed on dwcode.pages.dev:

- **Type:** JetBrains Mono everywhere at small, code-like sizes (body about
  14–15px, headings set by weight and colour, not by huge display sizes).
  Drop Iowan Old Style and Trebuchet MS. Self-host the font in `public/fonts/`
  or load it from the same source other Cybercore sites use. Keep a system
  monospace fallback.
- **Kickers:** `// 01 · workflow` style labels; file names in window bars.
- **Backdrop:** a static, dim code-editor scene (split panes, line numbers,
  syntax-coloured token bars, minimap) under low cyan/magenta glows, generated
  once as an image in `public/` (dwcode's `tools/make-site-background.py` is the
  reference). Decorative only: `aria-hidden`, no layout impact, clipped so it
  can never widen the page on phones.
- **Colour:** default theme `cybercore-tech` from the shared chooser. The
  local fallback palette moves from storm blue to the Cybercore dark/cyan/magenta
  roles so the page still looks right when the shared kit cannot load.
- **Motion:** light touches only: typed terminal lines, a heading scramble,
  staggered reveals. All disabled under `prefers-reduced-motion`.
- **Density:** shorter page than the current ~6,400px. Fewer, denser sections;
  no full-viewport serif headlines.

## 1. Editor-shell layout

The page is framed as the Tessera project open in an editor.

- **Window bar** at the top: `~/cybercore/tessera ▸ main`, logo, the
  hamburger button and the theme chooser.
- **File tree** (left rail on desktop, inside the drawer on phones) listing the
  sections as project files:

  ```text
  tessera/
  ├── README.md            → hero
  ├── content/index.md     → live build
  ├── tessera.toml         → token playground
  ├── kit/                 → Kit explorer
  └── dist/                → get started
  ```

  Each entry is a normal in-page anchor link. The active file is highlighted
  while scrolling (IntersectionObserver), but links work without JavaScript.
- **Section headers** look like editor tabs with the file name and a
  `// 0N · topic` kicker.
- **Drawer:** a hamburger drawer driven by one `NAV` list (the same list feeds the
  file tree). It replaces the current BUILD/KIT hover dropdowns. GitHub and
  external links open in new tabs. Escape closes it, focus returns to the button,
  and with JavaScript off the drawer falls back to a native `<details>` panel.

## 2. Live build hero

Replace the serif hero with:

- A short mono headline: `Build the site. Keep the source.` plus one sentence
  saying what Tessera is (Rust CLI, Markdown + layouts + your CSS → static
  `dist/`).
- **Split view:** left pane types in a small `content/index.md` and
  `tessera.toml` fragment; right pane shows the rendered `dist/index.html`
  preview built from those same lines.
- **Terminal strip** below that replays the output of a real `tessera build` run.
  The output is captured from the actual CLI into a committed fixture
  (`website/fixtures/build-output.txt` or inline in the page) and is labelled as
  a recording, not a live compiler.
- Actions: `▸ get started` (to `#start`) and `$ view source` (GitHub).
- No JavaScript: the full text and preview render statically; typing is an
  enhancement only.

## 3. Token playground

An interactive `tessera.toml` pane beside a small Kit preview (button, card,
badge, alert, code block).

- Editable values for `primary`, `secondary`, `color_bg`, `color_text`,
  `color_card` (colour inputs plus text). Changes update the preview's `--ts-*`
  custom properties live, scoped to the preview only.
- A generated-CSS pane shows the `:root { --ts-primary: …; }` lines the build
  would emit, matching the real token compiler's naming (`token.rs`).
- A "reset to theme" button reads the current chooser theme, so picking any of
  the shared themes also restyles the demo.
- Clearly marked as a browser preview; the real path is `tessera theme-set
  <key> <value>` or editing `tessera.toml` and rebuilding.
- No JavaScript: shows the static example config and its CSS output.

## 4. Kit explorer tree

Present the Kit as an explorer tree, like dwcode's theme explorer, grouped by
the existing guides:

```text
kit/
├── foundations/   container · stack · cluster · grid · section · prose
├── controls/      button · input · select · checkbox · switch
├── content/       card · badge · table · code
├── feedback/      alert · toast · progress
└── disclosure/    accordion · tabs
```

- The final list is taken from `src/kit/catalog.rs` and `docs/kit/*.md`, not
  invented. Anything listed but not shipped is marked planned.
- Selecting an entry opens a **detail sheet**: live rendered example using the
  real Kit CSS, the markup, the class names, and a link to its guide section on
  GitHub.
- Sheets are dialogs with focus trapping and Escape to close; without
  JavaScript each entry links to its guide.

## 7. Standard Cybercore chrome

- **Theme chooser** in the navbar beside the hamburger, never inside the
  drawer; square corners; open menu never covered. Keep the existing
  `window.SITE.themeVars` mapping, updated for the new site variables
  (including the cyan/magenta roles currently mapped to `null`).
- **Contact band:** keep `<cybercore-contact repo="tessera">` with the email
  fallback.
- **Scrollbars:** themed page and inner-pane scrollbars with
  `scrollbar-color` / `scrollbar-width` and `::-webkit-scrollbar*`, using site
  tokens.
- **Mobile first:** no horizontal overflow at any width; text at least 11px and
  tap targets at least 24px on phones; file tree collapses into the drawer.
- **Accessibility:** skip link, landmarks, visible focus, heading order,
  reduced motion, contrast checked against the default theme.

## Content and implementation constraints

- Build only with shipped Tessera features. Site behaviour goes in one plain
  script, `website/public/site.js` (copied by the `public/` pipeline), so the
  site needs no Node toolchain.
- Reuse canonical `ts-*` classes and tokens; site-only styling stays in
  `website/styles/main.css` under `site-*` names. Do not add Kit classes for the
  site's sake.
- Never copy theme data or chooser code into the site; reference the shared kit.
- Copy stays accurate to the CLI: commands are `new`, `add-page`,
  `add-component`, `theme-set`, `dev`, `doctor`, `build`, `clean`. No
  performance, adoption or certification claims.

## Files

| File | Change |
| --- | --- |
| `website/layouts/base.html` | Window bar, drawer, file tree, chooser placement, font and script tags |
| `website/content/index.md` | New section structure: hero, playground, Kit explorer, get started, FAQ |
| `website/styles/main.css` | Mono type, editor shell, panes, sheets, scrollbars, mobile rules |
| `website/tessera.toml` | Fallback tokens moved to the Cybercore palette |
| `website/public/site.js` | New: drawer, active file, typing, playground, sheets |
| `website/public/backdrop.webp` | New: generated editor backdrop |
| `website/tools/make-backdrop.py` | New: backdrop generator (adapted from dwcode) |
| `website/README.md` | Note the new structure and script |

## Verification

1. `cargo run --manifest-path ../Cargo.toml -- build` from `website/` succeeds.
2. Serve `website/dist/` locally and screenshot desktop (1568px), tablet and
   phone widths at 2x device pixel ratio; check no horizontal overflow.
3. Click through the drawer, file tree, playground, every Kit sheet and the
   chooser; switch a few themes and confirm the page and playground recolour.
4. Repeat with JavaScript disabled and with reduced motion on.
5. Keyboard-only pass: Tab order, Escape on drawer and sheets, visible focus.

## Release path

1. Branch, commit, open a PR against `main`; ordinary PRs and CI never publish.
2. After merge and explicit owner approval, deploy manually from `website/`:
   `wrangler pages deploy dist --project-name=tessera --branch=main`.
3. Verify the live page (HTTP 200, assets, chooser, contact band) before
   announcing.

No Cloudflare credentials in the repository.

## Deferred (next round)

- **5. Command cards:** one card per CLI command, opening a sheet with flags,
  example output and status (shipped / opt-in / planned). A `bridge` command
  does not exist yet and must show as planned.
- **6. Toolchain matrix:** CSS · Grass · Dart Sass · Tailwind · UnoCSS ·
  esbuild, showing default vs opt-in and whether Node is needed.
- **8. Build footer:** `built with tessera vX · dist/ N files · size`, using
  numbers from the real build.

## Risks

- **Feature drift:** keep claims tied to the CLI and `catalog.rs`; mark planned
  items.
- **Shared kit outage:** the local fallback palette and email fallback keep the
  page usable.
- **Decorative overflow on phones:** the backdrop and glows must be clipped
  (`overflow-x: clip` on a positioned wrapper), not just hidden on `html`.
- **Deployment coupling:** CI and PRs never deploy.
