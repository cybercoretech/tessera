# Tessera Design

**Status:** approved by the project owner on 2026-11-21

## Intent

Tessera is a standalone Rust CLI for creating, developing, and building static
websites. It generates user-owned HTML, Markdown, CSS/SCSS, assets, and
deployable static output. It is not a backend generator and does not require
NovaStack, Node.js, or a JavaScript framework for its default path.

## Product model

```text
Tessera CLI
├── Site engine       pages, navigation, layouts, Markdown, assets, build/dev
├── Tessera Kit       tokens, utilities, semantic components, themes
├── Style adapters    native SCSS, Tailwind, plain CSS
└── Optional bridges  NovaStack first; others later
```

The default style mode is a hybrid: Tessera Kit supplies accessible semantic
classes such as `ts-button` and `ts-card`, while Tailwind projects may add
Tailwind utilities for local composition. Tessera designs its API and source
independently; it does not fork DaisyUI or Bulma.

## Source and output contract

```text
tessera.toml
content/             Markdown pages with front matter
pages/               HTML page templates
layouts/             editable base and page layouts
components/          editable semantic components
styles/              CSS or Sass source and generated token inputs
assets/              images, fonts, SVG, and processed assets
public/              copied unchanged
dist/                generated static output, ignored by version control
```

`tessera.toml` is the source of truth for project metadata, navigation, output
options, and design tokens. Tessera emits CSS custom properties, SCSS
variables/maps, and a Tailwind v4 CSS theme input from the same token model.

## Commands

```text
tessera new <name> --starter <starter> --style <scss|css|tailwind>
tessera add page <name>
tessera add section <name>
tessera generate component <name>
tessera theme set <token> <value>
tessera build [--release]
tessera dev
tessera list --json
tessera doctor
tessera bridge novastack --project <path>
```

`dev` watches site source, rebuilds `dist/`, serves it locally, and requests a
browser refresh. `build --release` creates deterministic static output and runs
site checks. Commands must not overwrite user-owned files outside documented
generated files or explicit anchor regions.

## Styling decisions

- Native SCSS is the default and uses the Rust-native `grass` compiler.
- Plain CSS preserves the semantic component API without Sass or Node.
- Tailwind is first-class but opt-in. Tessera generates Tailwind v4 input,
  detects the Tailwind CLI, and supervises it during `dev`.
- Dart Sass is an explicit compatibility mode for unsupported advanced Sass
  module behavior; Tessera must not silently switch compilers.
- Tessera does not create a CSS-like language. A declarative token schema plus
  CSS, Sass, and Tailwind retain browser and editor tooling support.

## Tessera Kit v1

1. Foundations: reset, typography, colors, spaces, dark mode, and a11y helpers.
2. Layout: containers, stack/cluster primitives, responsive grid, columns, sections.
3. Controls: buttons, forms, selects, toggles, tabs, badges, and alerts.
4. Content: cards, tables, avatars, breadcrumbs, pagination, and media objects.
5. Website sections: navigation, hero, features, pricing, testimonials, FAQ, and footer.
6. Optional interaction package: mobile menu, dropdown, modal, toast, and accordion.

The base Kit is CSS-first. JavaScript behavior is an opt-in progressive
enhancement package with keyboard and focus-management tests.

## NovaStack bridge

The bridge is an optional, separately versioned `tessera-novastack` package.
Its first target is a NovaStack HTMX/Askama project. It validates the NovaStack
manifest, template, and version before creating Tessera source, emitting assets
to the configured static target, and patching only known anchors. Unknown
project shapes are errors, never guessed edits.

After the bridge contract is stable, NovaStack may expose it through
`novastack add tessera`. That command is out of Tessera core v1.

## Explicit exclusions for v1

- No database, auth, server actions, SPA runtime, visual editor, marketplace,
  or CSS-like programming language.
- No automatic remote downloads or cloud deployment.
- No code copied wholesale from DaisyUI or Bulma. Retain attribution and license
  notices if any MIT-licensed source is later reused.
- No VexLang runtime dependency. A data-only Vex build-hook remains a future option.

## Quality and safety requirements

- Generated sites work without network access in `scss` and `css` modes.
- Templates, page paths, navigation targets, and assets are validated before
  output replacement.
- Release builds check duplicate routes, broken internal links, page metadata,
  and missing image alternative text.
- Component examples exercise keyboard focus and responsive/dark-theme modes.
- Every adapter and bridge has fixture-based integration coverage.
