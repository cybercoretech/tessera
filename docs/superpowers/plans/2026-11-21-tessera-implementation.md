# Tessera Implementation Plan

> **For agentic workers:** Use subagent-driven-development or execute each task sequentially. Track completed steps with checkboxes.

**Plan ID:** `tessera-2026-11-21-v1`  
**Plan status:** active  
**Plan readiness:** structured  
**Spec:** `docs/superpowers/specs/2026-11-21-tessera-design.md`

**Goal:** Build a standalone Rust CLI that scaffolds and builds static websites with an independently designed, ambitious Tessera Kit, native CSS/SCSS support, optional Tailwind and UnoCSS adapters, an opt-in JavaScript/TypeScript asset pipeline, and an optional NovaStack bridge.

**Architecture:** A Cargo workspace isolates configuration, site rendering, Kit assets, style adapters, optional script bundling, CLI orchestration, and bridges. Sites retain editable source files. Builds produce deterministic static output in `dist/`.

**Tech stack:** Rust 2021, MSRV 1.88, Clap, Serde/TOML, MiniJinja, RustEmbed, pulldown-cmark, Grass, Notify, Axum/Tokio, tempfile, assert_cmd, and insta. Dart Sass, Tailwind v4, UnoCSS, esbuild, and the TypeScript compiler are opt-in external tools; none are silently installed. The TypeScript compiler is used for type checking, while esbuild produces browser assets.

**Implementation note:** The current repository keeps the active CLI in the root `tessera` package. The configuration and style/script toolchain integrations are currently implemented in `crates/tessera-config/` and `src/site/{style,toolchain}.rs`; extracting these into the proposed crates remains a later architecture task and is not required for their current behavior.

## Global constraints

- Preserve the approved four-layer design.
- Prioritize a clear, learnable API over maximizing the number of classes, variants, or compatibility promises.
- Give each layout, utility, state, and component concern one canonical Tessera API. Add a variant only for a distinct, documented use case.
- Tessera owns its `ts-` component classes, markup contracts, token model, styles, and behavior. Reference frameworks inform product research; they are not compatibility targets or sources for wholesale code reuse.
- SCSS and CSS modes must build without Node.js or network access.
- Do not add a CSS-like language or a VexLang runtime dependency.
- Use `ts-` as the component-class namespace.
- Use explicit anchors for mutations; refuse unsafe edits.
- Never silently install or download Tailwind.
- Keep `tessera-novastack` optional and separately versioned.
- Generated sites include no JavaScript by default. Optional interactions use small, dependency-free plain-JavaScript enhancements with usable HTML/CSS fallbacks.
- Tailwind is an optional adapter, never a Tessera Kit dependency. Do not alias `ts-*` classes to Tailwind utilities, generate component-class mappings, require Tailwind to build the Kit, or promise Tailwind-class compatibility.
- UnoCSS is an optional utility adapter implemented after the shared style and utility-engine interfaces stabilize; it remains absent from the default build.
- CSS and SCSS source selection is independent of the selected Sass compiler and optional utility engine. Grass is the default Sass compiler; Dart Sass is an explicit compatibility option.
- At most one utility engine (`none`, Tailwind, or UnoCSS) is active for a build. Tailwind and UnoCSS remain adapters around the Tessera-owned Kit and cannot redefine its API.
- Script processing is disabled by default. When enabled, esbuild bundles JavaScript or TypeScript; TypeScript type checking is a separate opt-in check using `tsc --noEmit`.
- Tessera's own documentation and Kit site should be built with Tessera from its catalog, guides, and showcase. Cloudflare Pages (for example, `tessera.pages.dev`) is a later hosting target; deployment remains explicit and is not part of implementation builds.
- Materialize is an optional theme direction, not Tessera's default identity or a separate component API.
- UIkit and Semantic UI are pattern references only. Adopt an idea only for a clear use case, then implement it using Tessera's API and support contracts.
- Do not model the core API on Bootstrap or adopt a fixed 12-column grid contract.

## Product direction and reference boundaries

The Kit may grow across foundations, layout, controls, content and data display, website sections, feedback, and a small optional interaction package. An ambitious catalog does not require duplicate helpers or compatibility with every reference framework.

| Reference | Allowed influence | Boundary |
| --- | --- | --- |
| Bulma | Modular organization, Sass configuration, CSS-variable theming, responsive behavior | No copied class vocabulary or parallel helper families. Define shared base and theme dependencies for modules. |
| Tailwind CSS | Optional utility-first authoring through a separate adapter | No Tailwind requirement in core, `ts-*` aliases or mappings, utility-parity promise, or silent installation. |
| Foundation | Responsive layout and accessibility practices | Use CSS Grid, Flexbox, intrinsic sizing, and container queries; no fixed 12-column public API. |
| DaisyUI | Semantic component discoverability, theme roles, and theme-authoring workflow | Tessera designs its own markup, class names, CSS, tokens, and behavior; no Tailwind coupling or source copying. |
| UIkit | Selected component and interaction patterns | Reference only; no dependency or compatibility promise. Each interaction needs a documented behavior and accessibility contract. |
| Semantic UI | Readable component concepts | Reference only; use concise Tessera names and Rust-native orchestration, without inherited naming grammar or legacy runtime conventions. |
| Materialize | Optional Material-inspired theme direction | Theme only; component APIs and functionality remain independent of this visual style. |
| Power CSS and Cardinal CSS | Compatible CSS architecture and composition ideas | Pattern-level influence only; no compatibility promise. |
| UnoCSS | Optional on-demand utility adapter | Implement as an independently selected adapter using Tessera tokens and configuration; never require it for the Kit or run it alongside Tailwind in one build. |
| Zola | None for the current product direction | Out of current scope; the approved site-engine plan remains authoritative. |
| Pure.css, Pico.css, HTML5 Boilerplate, Metro UI, Bootflat, Spectre.css, Bootstrap | None for the current roadmap | Excluded from active reference scope. Metro UI may be reconsidered only if Tessera later becomes an app framework. |

### Tailwind adapter contract

The initial Tailwind integration is an explicit, optional adapter. It may emit Tailwind v4 theme input from Tessera's token model and coordinate scanning/build output for supported project sources. Users install and update Tailwind themselves; Tessera reports a missing or unsupported CLI and never downloads it.

The adapter boundary is one-way: Tessera tokens may feed Tailwind theme input, and Tailwind may scan site source, but the Tessera Kit builds and works without Tailwind. `ts-*` components retain their own CSS, semantics, and behavior. Do not translate `ts-*` into Tailwind utilities, publish aliases, or make component behavior depend on Tailwind. Keep CLI invocation, version checks, source discovery, diagnostics, and process shutdown inside the adapter so they can evolve without changing the Kit API.

### JavaScript and interaction contract

Generated sites ship no JavaScript by default. The optional interaction package begins with a small number of high-value enhancements and is included only when selected. Prefer native HTML and CSS; document keyboard, focus, reduced-motion, responsive, and no-JavaScript behavior for each enhancement. Tessera-maintained scripts may use TypeScript when useful; esbuild emits browser-ready JavaScript, while type checking remains a separate opt-in `tsc --noEmit` step. Generated sites do not need a TypeScript runtime.

## Workspace layout

```text
Cargo.toml
crates/tessera-config/       manifest, tokens, diagnostics
crates/tessera-site/         content, routes, render, assets, checks
crates/tessera-kit/          semantic CSS/SCSS and component catalogue
crates/tessera-style/        CSS/SCSS compilation contract and compiler selection
crates/tessera-tailwind/     Tailwind adapter and process supervision
crates/tessera-unocss/       optional UnoCSS adapter and process supervision
crates/tessera-scripts/      opt-in JavaScript/TypeScript asset pipeline
crates/tessera-cli/          public CLI (extract the existing root CLI in T-06)
crates/tessera-novastack/    optional bridge
templates/                   embedded site and generator templates
tests/fixtures/              minimal test sites and host projects
```

## Dependency model

```text
T-01 config/workspace ─┬─> T-02 Kit/tokens ─┬─> T-04 build ─> T-06 dev
                       │                     ├─> T-07 Tailwind
                       │                     ├─> T-11 UnoCSS
                       │                     └─> T-12 scripts
                       └─> T-03 starters ───┘
T-04 ─> T-05 safe generators ─> T-08 catalogue/docs
T-01 + T-04 + T-05 ─> T-09 NovaStack bridge
T-02..T-09 + T-11..T-12 ─> T-10 quality/release gates
```

## Tasks

### T-01: Establish the workspace and manifest contract

**Lifecycle:** ready  
**Dependencies:** none  
**Repository setup:** At the start of T-01, create `https://github.com/cybercoretech/tessera` as a public repository with no contents. Do not initialize it with a README, license, `.gitignore`, or other files. Add the local GitHub Actions workflow files during the first implementation work, before pushing any project source. Do not push source content until the workflow files are present and ready.
**Files:** Create root workspace files, `.github/workflows/`, `crates/tessera-config/`, and config tests. Keep the existing root `tessera` binary as the active CLI until T-06 extracts it into `crates/tessera-cli/`.

**Interfaces:** `ProjectConfig::load(root: &Path) -> Result<ProjectConfig, ConfigError>`; typed `ThemeTokens`; `StyleSource` (`Css` or `Scss`), `SassCompiler` (`Grass` by default or explicitly selected `DartSass`), `UtilityEngine` (`None`, `Tailwind`, or `UnoCss`, mutually exclusive), and an opt-in script pipeline configuration.

**Acceptance:** valid manifests parse. Invalid style sources, compiler/utility selections, tokens, metadata, and unknown keys give field-specific errors without panics. Rust 1.88 is enforced. Configuration provides clear extension points for Kit modules, optional adapters, interaction assets, script processing, and theme selection without creating ambiguous or duplicate class APIs. Only one utility engine can be selected at a time.

**Repository/workflow acceptance:** the GitHub repository is public and empty before local implementation files are created; the initial local workflow files are added and reviewed before the first source push. The first push targets the already-created empty repository and includes the project source together with those workflow files.

- [ ] Create the empty public GitHub repository at `https://github.com/cybercoretech/tessera`.
- [ ] Add the initial local GitHub Actions workflow file(s) under `.github/workflows/` before pushing project source; keep the remote repository empty until the workflows are ready.
- [ ] Write failing valid/invalid manifest tests.
- [ ] Run `cargo test -p tessera-config --test config`; expect missing imports.
- [ ] Implement the TOML schema, validation, and diagnostics.
- [ ] Run `cargo fmt --check`, crate-scoped Clippy with warnings denied, and the targeted tests.
- [ ] Commit `feat: establish Tessera workspace and manifest contract` with body `Project contributor`.

### T-02: Implement tokens and Tessera Kit foundations

**Lifecycle:** in progress; foundation styles, the showcase, exact CSS snapshot, and class-prefix checks are implemented in the current root-package layout
**Dependencies:** T-01  
**Files:** `src/token.rs`, `src/kit/{foundations.rs,layout.rs,controls.rs,catalog.rs,foundations.css,layout.css,controls.css,content.css,disclosure.css,feedback.css}`, `src/site/style.rs`, `docs/kit/`, and `examples/kit-showcase/`. A separate `crates/tessera-kit/` extraction remains deferred.

**Interfaces:** `TokenSet::compile_css()` emits token custom properties; `crate::kit::STYLES` embeds the core CSS foundations and layout primitives in site builds.

**Acceptance:** the complete Kit scope remains reset, typography, color roles, light/dark theme foundations, containers, CSS Grid/Flexbox/intrinsic layout primitives, container-query patterns, curated utilities, visible focus states, and reduced-motion defaults. Delivered slices cover theme-aware foundations, responsive layout, forms, content and disclosure patterns, feedback tones, and a static showcase. CSS/SCSS token emitters and canonical catalog metadata are implemented; the generated CSS snapshot and class-prefix regression checks pass. Generated catalog docs and broader component coverage remain open. Every Tessera Kit class starts with `ts-`; each concern has one documented canonical API; there is no fixed 12-column contract or duplicate helper family. Core Kit output has no Tailwind or JavaScript dependency.

- [x] Snapshot generated Kit CSS and check catalog/CSS class selectors use the `ts-` prefix.
- [x] Embed core Kit styles into the CSS-first site build and add the first responsive layout primitives.
- [x] Add an editable static showcase demonstrating tokens, typography, layouts, controls, and focus styling.
- [x] Implement SCSS token output and canonical component catalogue metadata.
- [x] Run formatting and Clippy with warnings denied.
- [x] Add and inspect the Kit CSS snapshot and class-prefix regression checks.
- [ ] Commit `feat: add token-driven Tessera Kit foundation` with body `Project contributor`.

### T-03: Scaffold editable starter projects

**Lifecycle:** proposed  
**Dependencies:** T-01  
**Files:** Create `crates/tessera-site/src/starter.rs`, `templates/{shared,agency,portfolio,business,docs}/`, and `tests/starter.rs`.

**Interfaces:** `StarterKind`, `StarterSpec`, and `scaffold(root: &Path, spec: &StarterSpec) -> Result<(), ScaffoldError>`.

**Acceptance:** Agency, portfolio, business, and docs starters create the approved source layout and `tessera.toml`. The command refuses nonempty targets without modification.

- [ ] Write temp-directory tests for every starter/style combination and nonempty-target refusal.
- [ ] Run `cargo test -p tessera-site --test starter`; expect failures.
- [ ] Implement embedded starter rendering and atomic destination creation.
- [ ] Run targeted tests and inspect the generated source boundaries.
- [ ] Commit `feat: scaffold Tessera website starters` with body `Project contributor`.

### T-04: Build the static-site engine

**Lifecycle:** proposed  
**Dependencies:** T-01, T-02, T-03  
**Files:** Create `crates/tessera-site/src/{content,navigation,render,assets,check,build}.rs`, `tests/build.rs`, and `tests/fixtures/basic-site/`.

**Interfaces:** `build(project: &ProjectConfig, options: BuildOptions) -> Result<BuildReport, BuildError>`. The style pipeline accepts CSS or SCSS; Grass is the default SCSS compiler, and Dart Sass may be explicitly selected as a separately installed compatible compiler. Utility adapters are invoked only when selected. Builds stage output and replace `dist/` only after validation succeeds.

**Acceptance:** HTML and Markdown render through layouts. Navigation, `public/`, styles, and assets emit correctly. CSS and default Grass-based SCSS builds do not require Node.js or network access. Selecting Dart Sass, Tailwind, or UnoCSS requires the user-installed corresponding executable and produces a clear diagnostic if unavailable. Duplicate routes, broken internal links, absent metadata, and missing image alt text fail. Output is deterministic. Failed builds keep the prior `dist/`.

- [ ] Write failing fixtures for routing, Markdown, links, metadata, alt text, and preserved output.
- [ ] Run `cargo test -p tessera-site --test build`; expect failures.
- [ ] Implement discovery, rendering, staging, assets, and checks.
- [ ] Run `cargo test -p tessera-site`; expect deterministic success.
- [ ] Commit `feat: build validated static Tessera sites` with body `Project contributor`.

### T-05: Add safe generators and theme mutation

**Lifecycle:** proposed  
**Dependencies:** T-03, T-04  
**Files:** Create `crates/tessera-site/src/{mutate,anchors}.rs`, `templates/generate/`, and `tests/mutate.rs`.

**Interfaces:** `add_page`, `add_section`, `generate_component`, and `set_theme_token` return `MutationReport`. Anchor edits require unique `TESSERA:` markers.

**Acceptance:** Generated files register correctly. Duplicate names, reserved paths, traversal attempts, and invalid anchors fail atomically. User content outside anchors is unchanged.

- [ ] Write failing mutation and untouched-region tests.
- [ ] Run `cargo test -p tessera-site --test mutate`; expect failures.
- [ ] Implement validated naming, anchors, staging, and reports.
- [ ] Run crate tests and inspect mutation reports.
- [ ] Commit `feat: add safe Tessera project generators` with body `Project contributor`.

### T-06: Expose CLI, server, watcher, and live reload

**Lifecycle:** proposed  
**Dependencies:** T-03, T-04, T-05  
**Files:** Create `crates/tessera-cli/` by extracting the existing root `tessera` binary into `src/{commands,doctor,dev,serve}.rs` and `src/main.rs`; add `tests/cli.rs` and update the workspace/root package entry point as needed.

**Acceptance:** `new`, `add`, `generate`, `theme`, `build`, `dev`, `list --json`, and `doctor` validate arguments and provide human/JSON diagnostics. The server exposes only `dist/`. One source change triggers one rebuild. Shutdown releases the watcher and port.

- [ ] Write failing command tests for errors, JSON catalog shape, build exits, and serve-root containment.
- [ ] Run `cargo test -p tessera-cli --test cli`; expect failures.
- [ ] Implement dispatch, watcher debounce, local serving, and live-reload events.
- [ ] Run targeted tests and manually smoke-test a fixture site.
- [ ] Commit `feat: add Tessera build and development commands` with body `Project contributor`.

### T-07: Add first-class Tailwind v4 support

**Lifecycle:** implemented in the current root-package layout; crate extraction is deferred to T-06
**Dependencies:** T-01, T-02, T-04
**Files:** `src/site/toolchain.rs`, `src/site/mod.rs`, `src/doctor.rs`, `crates/tessera-config/`; optional future extraction to `crates/tessera-tailwind/`.

**Interfaces:** `TailwindAdapter::check(&ProjectConfig) -> AdapterStatus` and `TailwindAdapter::build(&ProjectConfig, BuildMode) -> Result<StyleArtifact, TailwindError>`.

**Acceptance:** Tailwind is explicitly opt-in through utility-engine selection. Tailwind v4 input may be generated from Tessera tokens and the adapter may scan supported project sources. `doctor` reports missing or incompatible user-installed executables. Selecting CSS or SCSS alone never spawns Tailwind; it runs only when explicitly selected as the utility engine. Tessera components retain independent CSS and work without Tailwind; tests prove there are no `ts-*` aliases or generated component mappings. A fixture shows user-authored Tailwind utilities can coexist with Tessera components without defining or replacing the Kit API. `dev` stops supervised child processes. Tessera never silently installs or downloads Tailwind.

- [x] Add fake-executable coverage for missing, failing, unsupported-version, and successful Tailwind CLI paths.
- [x] Implement Tessera-token theme input, CLI invocation, and diagnostics without class translation or Kit dependencies.
- [x] Run workspace tests and Clippy on stable and Rust 1.88.
- [ ] Run an opt-in real-CLI test when Tailwind is already installed.
- [ ] Commit `feat: add first-class Tailwind style adapter` with body `Project contributor`.

### T-11: Add optional UnoCSS support

**Lifecycle:** implemented in the current root-package layout; crate extraction is deferred to T-06
**Dependencies:** T-01, T-02, T-04
**Files:** `src/site/toolchain.rs`, `src/site/mod.rs`, `src/doctor.rs`, `crates/tessera-config/`; optional future extraction to `crates/tessera-unocss/`.

**Interfaces:** `UnoCssAdapter::check(&ProjectConfig) -> AdapterStatus` and `UnoCssAdapter::build(&ProjectConfig, BuildMode) -> Result<StyleArtifact, UnoCssError>`.

**Acceptance:** UnoCSS is explicitly opt-in and mutually exclusive with Tailwind for a build. Users provide or select an explicit UnoCSS preset/configuration; Tessera does not assume a default preset, translate `ts-*` classes, or redefine Kit components. `doctor` reports missing or unsupported user-installed tooling. Tessera never installs or downloads UnoCSS. `dev` stops supervised child processes. The default CSS/SCSS build has no UnoCSS dependency.

- [x] Add fake-executable coverage for missing, failing, and successful UnoCSS paths, explicit preset/configuration requirements, and mutual-exclusion validation.
- [x] Implement source selection, explicit configuration/preset handling, CLI invocation, and diagnostics.
- [x] Keep UnoCSS optional and independent of the Tessera Kit build.
- [x] Run workspace tests and Clippy on stable and Rust 1.88.
- [ ] Commit `feat: add optional UnoCSS style adapter` with body `Project contributor`.

### T-12: Add opt-in JavaScript and TypeScript asset processing

**Lifecycle:** implemented in the current root-package layout; crate extraction is deferred to T-06
**Dependencies:** T-01, T-04
**Files:** `src/site/toolchain.rs`, `src/site/mod.rs`, `src/doctor.rs`, `crates/tessera-config/`; optional future extraction to `crates/tessera-scripts/`.

**Interfaces:** A script build configuration with explicit entry points, output behavior, and an opt-in type-check setting; a script pipeline that returns browser-ready assets or a structured error.

**Acceptance:** Script processing is disabled unless explicitly enabled. When enabled, esbuild bundles JavaScript or TypeScript into browser-ready output. TypeScript checking is separate and runs only when requested, using `tsc --noEmit`; esbuild success alone is not reported as type correctness. Missing user-installed tools produce actionable diagnostics, and Tessera never installs them. No JavaScript is added to generated sites by default. The optional interaction package keeps no-JavaScript fallbacks and includes only selected assets. Builds remain staged and preserve the prior `dist/` on script-processing failure.

- [x] Add fake-executable tests for disabled mode, JS/TS bundling, separate type-checking, tool failures, and preserving output on failure.
- [x] Implement entry-point validation, esbuild bundling, optional `tsc --noEmit`, and actionable tool diagnostics.
- [x] Document runtime output, type-check separation, explicit tool installation, and the no-JavaScript default.
- [x] Run workspace tests and Clippy on stable and Rust 1.88.
- [ ] Commit `feat: add opt-in JavaScript and TypeScript build pipeline` with body `Project contributor`.

### T-08: Complete the Kit, catalogue, and documentation

**Lifecycle:** in progress; foundations, controls, themes, content, disclosure, and feedback slices are in the current root-package layout
**Dependencies:** T-02, T-04, T-05, T-07, T-11, T-12
**Files:** `src/kit/{catalog.rs,foundations.css,layout.css,controls.css,content.css,disclosure.css,feedback.css}`, `docs/kit/`, and `examples/kit-showcase/`; a separate `crates/tessera-kit/` extraction remains deferred. Tessera's future project site will consume this catalog and documentation as a Tessera-built static site, with Cloudflare Pages hosting considered separately.

**Acceptance:** Controls, content/data display, website sections, feedback, and optional interactions complete the ambitious Kit families. `list` and documentation use one generated catalogue. Each component documents semantic markup, variants, theme states, responsive behavior, and relevant accessibility expectations. Each interaction has a written keyboard/focus contract plus responsive, dark-mode, reduced-motion, and no-JavaScript fallback evidence. The default Kit remains CSS-first; opt-in interactions use small, dependency-free plain JavaScript. Reference patterns are implemented under Tessera's own API, without compatibility naming.

- [x] Establish canonical names and semantic markup contracts for foundation, control, navigation, data, and disclosure patterns.
- [x] Document forms, feedback tones, theme behavior, composable website sections, and content/accessibility contracts.
- [x] Add light/dark theme previews to the showcase without changing component markup.
- [x] Expand form examples with selects, textareas, radio choices, and a native switch; add a media-object example.
- [ ] Establish complete component support criteria before expanding the remaining catalog: visual states, keyboard behavior where relevant, responsive behavior, theme behavior, fallbacks, and documentation.
- [x] Treat Materialize as an optional theme direction only; demonstrate theme changes do not change component APIs.
- [ ] Keep UIkit and Semantic UI as research references and require a documented user need before adopting a pattern.

- [ ] Add component/accessibility fixtures for each family.
- [ ] Implement the remaining families in independently reviewable slices.
- [ ] Generate catalogue documentation from the canonical registry to prevent duplicated component lists.
- [ ] Run component fixtures and manual keyboard checks for interactions.
- [ ] Commit each family as `feat: add Tessera Kit <family>` with body `Project contributor`.

### T-09: Deliver the optional NovaStack bridge

**Lifecycle:** proposed  
**Dependencies:** T-04, T-05, T-06  
**Files:** Create `crates/tessera-novastack/{Cargo.toml,src/lib.rs,src/validate.rs,src/bridge.rs}`, `templates/bridge/novastack/`, `tests/novastack_bridge.rs`, and `tests/fixtures/novastack-htmx/`; modify CLI command registration.

**Interfaces:** `bridge(project: &Path, options: NovaStackBridgeOptions) -> Result<BridgeReport, BridgeError>`. It validates `novastack.toml`, supported version/template, and anchors before staging writes.

**Acceptance:** Only supported HTMX/Askama fixtures integrate. The bridge reports changed paths, preserves non-anchor edits, rejects invalid projects atomically, and emits a host build target for Tessera assets.

- [ ] Write fixture tests for supported, incompatible, missing-anchor, changed-anchor, and partial-write cases.
- [ ] Run `cargo test -p tessera-novastack --test novastack_bridge`; expect failures.
- [ ] Implement validation, staging, explicit patches, and compatibility reports.
- [ ] Run bridge fixtures; defer cross-repository testing until both contracts stabilize.
- [ ] Commit `feat: add optional NovaStack bridge` with body `Project contributor`.

### T-10: Add quality gates, packaging, and release policy

**Lifecycle:** proposed  
**Dependencies:** T-01 through T-09, T-11, T-12
**Files:** Complete and refine `.github/workflows/ci.yml` and any additional required workflows, `deny.toml`, `CONTRIBUTING.md`, `SECURITY.md`, `README.md`, `CHANGELOG.md`, and `scripts/check-release.sh`; modify workspace manifests and docs. The initial workflow file(s) are created in T-01 so they are ready before the first source push.

**Acceptance:** CI runs formatting, Clippy with warnings denied, tests, generated-site fixtures, bridge fixtures, and fake-executable coverage for optional external tools. Release checks validate versions, package contents, name availability, and license notices. No publishing or deployment command runs automatically.

- [ ] Write a release-package inspection test and local CI checklist.
- [ ] Complete workflow and release checks without credentials or publish steps; keep publishing and deployment manual and explicit.
- [ ] Run quality commands only after verification authorization; stop on the first failure.
- [ ] Commit `chore: add Tessera quality and release gates` with body `Project contributor`.

### T-13: Build the Tessera project website

**Lifecycle:** in progress; the first (editorial) site is live at `tessera-8gd.pages.dev`. On 2026-11-25 the owner approved a redesign in the Cybercore editor style; see the [site plan](2026-11-23-tessera-pages-site-plan.md).
**Dependencies:** T-02, T-04, T-08, T-10.
**Target:** `https://tessera-8gd.pages.dev` on Cloudflare Pages (Cloudflare assigned the `-8gd` suffix).

**Acceptance:** The project website is built with Tessera and demonstrates the real Kit. It explains the CLI, Kit, tokens, optional style adapters, and opt-in scripts without overstating unfinished features. It follows the Cybercore family site style (editor shell, mono type, shared theme chooser and contact block). It is responsive, keyboard-accessible, dark-mode aware, and usable without JavaScript. Cloudflare deployment remains an explicit manual release action.

- [x] Approve the information architecture and visual direction in the site plan.
- [x] Build the site in a dedicated self-hosted project directory using Tessera and its Kit.
- [x] Add accessible navigation, documentation pathways, real feature examples, and install instructions.
- [ ] Verify responsive, keyboard, light/dark, and no-JavaScript behavior.
- [x] Configure Cloudflare Pages and deploy the first site by hand.
- [x] Commit `feat: add the Tessera project website` with body `Project contributor`.
- [ ] Rebuild in the Cybercore editor style: editor shell, live build hero, token playground, Kit explorer, Cybercore chrome.
- [ ] Later: command cards, toolchain matrix, build footer.
- [ ] Deploy the redesign only after explicit release approval.

## Verification, risks, and handoff

After each task, run `cargo fmt --check`, crate-scoped Clippy with warnings denied, and targeted tests. Before release, run workspace tests, workspace Clippy, generated-site and bridge fixture matrices, package inspection, and browser/keyboard/responsive/dark-mode smoke checks. Exercise default CSS/Grass builds and each explicitly selected external adapter/toolchain independently.

Stop and reconcile if the public name is unavailable, Grass cannot compile a required fixture, an external compiler or utility CLI contract changes, esbuild/TypeScript behavior breaks the documented separation, or NovaStack anchors cannot be proven safe. Do not publish, silently switch compilers, silently install tools, or patch unknown NovaStack layouts.

The plan covers every approved layer and isolates style-adapter, script-toolchain, and bridge risk. The Kit is intentionally ambitious, but its API is constrained by one canonical expression for each concern, a shared token model, and explicit component support criteria. Do not add utilities or variants solely to match another framework's feature list. Keep Sass compiler choice explicit, utility engines mutually exclusive, and script processing opt-in; a TypeScript compile does not replace type checking.

Public-name availability and the final public license are external release gates; they do not block T-01. Start T-01 only after implementation authorization. Once T-01 is accepted, T-02 and T-03 may proceed in parallel if the configuration interface remains unchanged.
