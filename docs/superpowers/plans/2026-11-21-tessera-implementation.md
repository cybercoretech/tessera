# Tessera Implementation Plan

> **For agentic workers:** Use subagent-driven-development or execute each task sequentially. Track completed steps with checkboxes.

**Plan ID:** `tessera-2026-11-21-v1`  
**Plan status:** active  
**Plan readiness:** structured  
**Spec:** `docs/superpowers/specs/2026-11-21-tessera-design.md`

**Goal:** Build a standalone Rust CLI that scaffolds and builds static websites with an independently designed, ambitious Tessera Kit, native SCSS/CSS, optional Tailwind integration, and an optional NovaStack bridge.

**Architecture:** A Cargo workspace isolates configuration, site rendering, Kit assets, style adapters, CLI orchestration, and bridges. Sites retain editable source files. Builds produce deterministic static output in `dist/`.

**Tech stack:** Rust 2021, MSRV 1.88, Clap, Serde/TOML, MiniJinja, RustEmbed, pulldown-cmark, Grass, Notify, Axum/Tokio, tempfile, assert_cmd, and insta. Tailwind v4 is external and only required in Tailwind mode.

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
- UnoCSS is deferred as a possible later optional adapter, after core interfaces stabilize and user demand is demonstrated.
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
| UnoCSS | Potential on-demand utility adapter | Defer until core style interfaces are stable and there is demonstrated demand. |
| Zola | None for the current product direction | Out of current scope; the approved site-engine plan remains authoritative. |
| Pure.css, Pico.css, HTML5 Boilerplate, Metro UI, Bootflat, Spectre.css, Bootstrap | None for the current roadmap | Excluded from active reference scope. Metro UI may be reconsidered only if Tessera later becomes an app framework. |

### Tailwind adapter contract

The initial Tailwind integration is an explicit, optional adapter. It may emit Tailwind v4 theme input from Tessera's token model and coordinate scanning/build output for supported project sources. Users install and update Tailwind themselves; Tessera reports a missing or unsupported CLI and never downloads it.

The adapter boundary is one-way: Tessera tokens may feed Tailwind theme input, and Tailwind may scan site source, but the Tessera Kit builds and works without Tailwind. `ts-*` components retain their own CSS, semantics, and behavior. Do not translate `ts-*` into Tailwind utilities, publish aliases, or make component behavior depend on Tailwind. Keep CLI invocation, version checks, source discovery, diagnostics, and process shutdown inside the adapter so they can evolve without changing the Kit API.

### JavaScript and interaction contract

Generated sites ship no JavaScript by default. The optional interaction package begins with a small number of high-value enhancements, implemented in dependency-free plain JavaScript and included only when selected. Prefer native HTML and CSS; document keyboard, focus, reduced-motion, responsive, and no-JavaScript behavior for each enhancement. TypeScript may be considered for authoring Tessera-maintained scripts if their complexity justifies a compile step, but generated sites receive browser-ready JavaScript and require no TypeScript toolchain.

## Workspace layout

```text
Cargo.toml
crates/tessera-config/       manifest, tokens, diagnostics
crates/tessera-site/         content, routes, render, assets, checks
crates/tessera-kit/          semantic CSS/SCSS and component catalogue
crates/tessera-style/        CSS/SCSS compilation contract
crates/tessera-tailwind/     Tailwind adapter and process supervision
crates/tessera-cli/          public CLI (extract the existing root CLI in T-06)
crates/tessera-novastack/    optional bridge
templates/                   embedded site and generator templates
tests/fixtures/              minimal test sites and host projects
```

## Dependency model

```text
T-01 config/workspace ─┬─> T-02 Kit/tokens ─┬─> T-04 build ─> T-06 dev
                       │                     └─> T-07 Tailwind
                       └─> T-03 starters ───┘
T-04 ─> T-05 safe generators ─> T-08 catalogue/docs
T-01 + T-04 + T-05 ─> T-09 NovaStack bridge
T-02..T-09 ─> T-10 quality/release gates
```

## Tasks

### T-01: Establish the workspace and manifest contract

**Lifecycle:** ready  
**Dependencies:** none  
**Repository setup:** At the start of T-01, create `https://github.com/cybercoretech/tessera` as a public repository with no contents. Do not initialize it with a README, license, `.gitignore`, or other files. Add the local GitHub Actions workflow files during the first implementation work, before pushing any project source. Do not push source content until the workflow files are present and ready.
**Files:** Create root workspace files, `.github/workflows/`, `crates/tessera-config/`, and config tests. Keep the existing root `tessera` binary as the active CLI until T-06 extracts it into `crates/tessera-cli/`.

**Interfaces:** `ProjectConfig::load(root: &Path) -> Result<ProjectConfig, ConfigError>`; typed `ThemeTokens`; `StyleMode` with `Scss`, `Css`, and `Tailwind`.

**Acceptance:** valid manifests parse. Invalid style modes, tokens, metadata, and unknown keys give field-specific errors without panics. Rust 1.88 is enforced. Configuration provides clear extension points for Kit modules, optional adapters, interaction assets, and theme selection without creating ambiguous or duplicate class APIs.

**Repository/workflow acceptance:** the GitHub repository is public and empty before local implementation files are created; the initial local workflow files are added and reviewed before the first source push. The first push targets the already-created empty repository and includes the project source together with those workflow files.

- [ ] Create the empty public GitHub repository at `https://github.com/cybercoretech/tessera`.
- [ ] Add the initial local GitHub Actions workflow file(s) under `.github/workflows/` before pushing project source; keep the remote repository empty until the workflows are ready.
- [ ] Write failing valid/invalid manifest tests.
- [ ] Run `cargo test -p tessera-config --test config`; expect missing imports.
- [ ] Implement the TOML schema, validation, and diagnostics.
- [ ] Run `cargo fmt --check`, crate-scoped Clippy with warnings denied, and the targeted tests.
- [ ] Commit `feat: establish Tessera workspace and manifest contract` with body `Project contributor`.

### T-02: Implement tokens and Tessera Kit foundations

**Lifecycle:** proposed  
**Dependencies:** T-01  
**Files:** Create `crates/tessera-kit/`, `tests/kit.rs`, and shared style templates.

**Interfaces:** `render_tokens(tokens: &ThemeTokens, mode: StyleMode) -> Result<String, KitError>` and `KitArtifact { css, component_catalog }`.

**Acceptance:** the token model emits CSS variables and SCSS variables/maps. The Kit includes reset, typography, color roles, light/dark theme foundations, containers, CSS Grid/Flexbox/intrinsic layout primitives, container-query patterns, curated utilities, visible focus states, and reduced-motion defaults. Every Tessera class starts with `ts-`. Each concern has one documented canonical API; there is no fixed 12-column contract or duplicate helper family. Core Kit output has no Tailwind or JavaScript dependency.

- [ ] Write output snapshots and a class-prefix regression test.
- [ ] Run `cargo test -p tessera-kit --test kit`; expect failures.
- [ ] Implement embedded Kit assets, token rendering, and catalogue metadata.
- [ ] Run formatting, Clippy, and snapshot tests; inspect snapshots before accepting them.
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

**Interfaces:** `build(project: &ProjectConfig, options: BuildOptions) -> Result<BuildReport, BuildError>`. Builds stage output and replace `dist/` only after validation succeeds.

**Acceptance:** HTML and Markdown render through layouts. Navigation, `public/`, and assets emit correctly. Duplicate routes, broken internal links, absent metadata, and missing image alt text fail. Output is deterministic. Failed builds keep the prior `dist/`.

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

**Lifecycle:** proposed  
**Dependencies:** T-02, T-04, T-06  
**Files:** Create `crates/tessera-tailwind/{Cargo.toml,src/lib.rs,src/detect.rs,src/generate.rs,src/process.rs}`, `templates/shared/styles/tailwind.css.jinja`, and `tests/tailwind.rs`; modify CLI style-mode wiring.

**Interfaces:** `TailwindAdapter::check(&ProjectConfig) -> AdapterStatus` and `TailwindAdapter::build(&ProjectConfig, BuildMode) -> Result<StyleArtifact, TailwindError>`.

**Acceptance:** Tailwind is explicitly opt-in. Tailwind v4 input may be generated from Tessera tokens and the adapter may scan supported project sources. `doctor` reports missing or incompatible user-installed executables. SCSS/CSS modes never spawn Tailwind. Tessera components retain independent CSS and work without Tailwind; tests prove there are no `ts-*` aliases or generated component mappings. A fixture shows user-authored Tailwind utilities can coexist with Tessera components without defining or replacing the Kit API. `dev` stops supervised child processes. Tessera never silently installs or downloads Tailwind.

- [ ] Write fake-executable tests for found, absent, failing, unsupported-version, and successful Tailwind paths.
- [ ] Run `cargo test -p tessera-tailwind --test tailwind`; expect failures.
- [ ] Implement token-input generation, source discovery, process supervision, and diagnostics inside the adapter; do not add class translation or Kit dependencies.
- [ ] Run adapter tests and an opt-in real-CLI test only when Tailwind is already installed.
- [ ] Commit `feat: add first-class Tailwind style adapter` with body `Project contributor`.

### T-08: Complete the Kit, catalogue, and documentation

**Lifecycle:** proposed  
**Dependencies:** T-02, T-04, T-05, T-07  
**Files:** Extend `crates/tessera-kit/assets/`; create `docs/{components,theming,tailwind,accessibility}.md`, `examples/kit-showcase/`, and component fixtures.

**Acceptance:** Controls, content/data display, website sections, feedback, and optional interactions complete the ambitious Kit families. `list` and documentation use one generated catalogue. Each component documents semantic markup, variants, theme states, responsive behavior, and relevant accessibility expectations. Each interaction has a written keyboard/focus contract plus responsive, dark-mode, reduced-motion, and no-JavaScript fallback evidence. The default Kit remains CSS-first; opt-in interactions use small, dependency-free plain JavaScript. Reference patterns are implemented under Tessera's own API, without compatibility naming.

- [ ] Establish component support criteria before expanding the catalog: markup contract, visual states, keyboard behavior where relevant, responsive behavior, theme behavior, fallback, and documentation.
- [ ] Treat Materialize as an optional theme direction only; demonstrate theme changes do not change component APIs.
- [ ] Keep UIkit and Semantic UI as research references and require a documented user need before adopting a pattern.

- [ ] Add failing component/accessibility fixtures for each family before styles or markup.
- [ ] Implement each family in an independently reviewable commit.
- [ ] Verify generated docs have no duplicated component registry.
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
**Dependencies:** T-01 through T-09  
**Files:** Complete and refine `.github/workflows/ci.yml` and any additional required workflows, `deny.toml`, `CONTRIBUTING.md`, `SECURITY.md`, `README.md`, `CHANGELOG.md`, and `scripts/check-release.sh`; modify workspace manifests and docs. The initial workflow file(s) are created in T-01 so they are ready before the first source push.

**Acceptance:** CI runs formatting, Clippy with warnings denied, tests, generated-site fixtures, and bridge fixtures. Release checks validate versions, package contents, name availability, and license notices. No publishing or deployment command runs automatically.

- [ ] Write a release-package inspection test and local CI checklist.
- [ ] Complete workflow and release checks without credentials or publish steps; keep publishing and deployment manual and explicit.
- [ ] Run quality commands only after verification authorization; stop on the first failure.
- [ ] Commit `chore: add Tessera quality and release gates` with body `Project contributor`.

## Verification, risks, and handoff

After each task, run `cargo fmt --check`, crate-scoped Clippy with warnings denied, and targeted tests. Before release, run workspace tests, workspace Clippy, generated-site and bridge fixture matrices, package inspection, and browser/keyboard/responsive/dark-mode smoke checks.

Stop and reconcile if the public name is unavailable, Grass cannot compile a required fixture, the Tailwind CLI contract changes, or NovaStack anchors cannot be proven safe. Do not publish, silently switch compilers, or patch unknown NovaStack layouts.

The plan covers every approved layer and isolates Tailwind and bridge risk. The Kit is intentionally ambitious, but its API is constrained by one canonical expression for each concern, a shared token model, and explicit component support criteria. Do not add utilities or variants solely to match another framework's feature list. Keep UnoCSS as a later decision. Do not expand the optional JavaScript package or adopt a TypeScript build step until interaction requirements show the plain-JavaScript approach is insufficient.

Public-name availability and the final public license are external release gates; they do not block T-01. Start T-01 only after implementation authorization. Once T-01 is accepted, T-02 and T-03 may proceed in parallel if the configuration interface remains unchanged.
