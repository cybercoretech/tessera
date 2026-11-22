# AGENT_HANDOFF: Tessera

**Updated:** 2026-11-21  
**Scope:** Design and implementation planning only. This is a new directory with no `.git` repository and no implementation source yet.

## Objective

Build Tessera as a standalone Rust CLI for scaffolding, developing, and building
static websites. It combines a site engine, an independently designed semantic
component kit, SCSS/CSS/Tailwind adapters, and optional ecosystem bridges.

## State now (verified 2026-11-21)

- `/home/raven/Devspace/Cybercore/tooling/tessera` has no `.git` directory.
- No code, manifests, dependencies, CI, background processes, commits, pull
  requests, or releases exist.
- Two planning artifacts exist and are the authoritative product/design records:
  - [Approved design](docs/superpowers/specs/2026-11-21-tessera-design.md)
  - [Implementation plan](docs/superpowers/plans/2026-11-21-tessera-implementation.md)
- The plan is **active** and **structured**. Only T-01 is technically ready.
- Implementation execution remains subject to explicit user authorization.

## Approved product model

```text
Tessera CLI
├── Site engine       pages, navigation, layouts, Markdown, assets, build/dev
├── Tessera Kit       tokens, utilities, semantic components, themes
├── Style adapters    native SCSS, Tailwind, plain CSS
└── Optional bridges  NovaStack first; others later
```

Tessera is not a NovaStack template or a Rust backend generator. NovaStack is
only a quality and CLI-ergonomics reference. Generated sites remain editable,
portable static projects.

## User decisions

| Decision | By | Why | Date |
| --- | --- | --- | --- |
| Product name: Tessera | User | Approved as the standalone identity. Recheck public-name availability before publishing. | 2026-11-21 |
| Use a Rust workspace with four layers | User | Keeps site engine, Kit, adapters, and bridges independently maintainable. | 2026-11-21 |
| Default hybrid API | User | Tessera semantic classes (`ts-*`) plus optional Tailwind utilities balances concise markup with flexibility. | 2026-11-21 |
| Token configuration | User | `tessera.toml` is the declarative source of theme tokens and site configuration. | 2026-11-21 |
| Styling modes | User | Native SCSS, plain CSS, and first-class opt-in Tailwind are required. | 2026-11-21 |
| Component strategy | User | Design independently; borrow product ideas, behavior, and accessibility expectations from DaisyUI/Bulma, not their source wholesale. | 2026-11-21 |
| No custom CSS language | User | Use CSS/Sass/Tailwind and declarative tokens; do not make VexLang a runtime dependency. | 2026-11-21 |
| Initial Kit | User | Foundations, layout, controls, content, website sections, and opt-in interactions are in scope. | 2026-11-21 |
| NovaStack integration | User | Optional `tessera bridge novastack --project <path>` first; later `novastack add tessera`; keep bridge in `tessera-novastack`. | 2026-11-21 |

## Implementation plan at a glance

1. **T-01:** Create the workspace and typed `tessera.toml` contract.
2. **T-02:** Implement tokens and Tessera Kit foundations.
3. **T-03:** Create Agency, Portfolio, Business, and Docs starters.
4. **T-04:** Implement rendering, assets, validation, and atomic static builds.
5. **T-05:** Add anchor-safe page, section, component, and theme generators.
6. **T-06:** Add CLI commands, local development server, watcher, and live reload.
7. **T-07:** Add first-class Tailwind v4 adapter and process supervision.
8. **T-08:** Complete component families, catalogue, examples, and accessibility docs.
9. **T-09:** Add the optional NovaStack HTMX/Askama bridge with fixture tests.
10. **T-10:** Add CI, packaging, release checks, license notices, and documentation.

Read the linked implementation plan for exact files, interfaces, dependencies,
acceptance criteria, verification commands, risks, and commit format.

## Technical calls and constraints

- Default native Sass compiler: `grass`; Dart Sass is an explicit compatibility
  mode for advanced unsupported Sass module behavior.
- Tailwind v4 is external and opt-in. Tessera must not install/download it
  silently; `doctor` detects and reports the executable.
- The default Kit is CSS-first. Interactive behavior is optional progressive
  enhancement with keyboard/focus and no-JavaScript fallback coverage.
- Builds stage output and replace `dist/` only when all checks pass.
- Generators patch only unique `TESSERA:` anchors and reject unknown layouts.
- NovaStack bridge supports HTMX/Askama first, validates its manifest/version,
  and never guesses anchors or edits arbitrary host projects.
- Default CSS/SCSS modes must not depend on Node.js or network access.

## Open questions and release gates

1. **Public availability:** Recheck `tessera` against crates.io, GitHub, AUR,
   domains, and relevant trademarks before any public repository or release.
2. **Public license:** Recommended: MIT OR Apache-2.0 for ecosystem consistency.
   The user has not selected a final public license.
3. **Implementation authorization:** The user approved the design and plan, but
   code changes should begin only after they explicitly request implementation.

## Source material

- [NovaStack source](https://github.com/cybercoretech/NovaStack) and local reference: `/home/raven/Devspace/Cybercore/tooling/novastack`
- [NovaStack site](https://novastack-rs.pages.dev/)
- [Zola](https://www.getzola.org/) — single-binary static-site product model.
- [RsHtml](https://rshtml.github.io/) and [repository](https://github.com/rshtml/rshtml) — possible future Rust SSR adapter; not Tessera core.
- [mdBook](https://github.com/rust-lang/mdBook) and [preprocessor model](https://rust-lang.github.io/mdBook/for_developers/preprocessors) — future extension-contract inspiration.
- [Grass](https://docs.rs/grass/latest/grass/) — native Sass/SCSS compilation.
- [Tailwind CLI](https://tailwindcss.com/docs/installation/tailwind-cli?web=1) — Tailwind v4 integration contract.
- [DaisyUI](https://daisyui.com/) and [repository](https://github.com/saadeghi/daisyui) — semantic Tailwind component model and themes.
- [Bulma](https://bulma.io/) and [repository](https://github.com/jgthms/bulma) — modular Sass, CSS-first components, and optional JavaScript model.
- [VexLang](https://github.com/cybercoretech/vexlang) and local reference: `/home/raven/Devspace/Cybercore/tooling/vexlang` — useful future build-hook candidate, not a v1 dependency.

## How to resume

1. Read this file, then the approved design and implementation plan.
2. Verify current state before trusting this handoff:

   ```sh
   cd /home/raven/Devspace/Cybercore/tooling/tessera
   find . -maxdepth 3 -type f -not -path './.git/*' | sort
   test -d .git && git status --short || true
   ```

3. Confirm the user has authorized implementation before creating workspace or
   code files.
4. Start with T-01. Do not start later tasks until their dependencies and
   interfaces are accepted.
