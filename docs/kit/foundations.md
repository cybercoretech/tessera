# Tessera Kit foundations

Tessera's core Kit is plain CSS. It does not require Tailwind, a JavaScript runtime, or a component framework. Use native HTML for behavior and semantics; `ts-*` classes provide the shared visual and layout layer.

## Tokens

Token keys become `--ts-*` CSS custom properties. Underscores in token keys become hyphens:

```toml
[theme.tokens]
primary = "#1e5949"
color_bg = "#f6f4ee"
color_text = "#263a34"
font_display = "Georgia, serif"
```

The CSS output contains `--ts-primary`, `--ts-color-bg`, `--ts-color-text`, and `--ts-font-display`. `TokenSet::compile_scss()` emits corresponding `$ts-*` variables and a `$ts-tokens` map for projects that build SCSS separately.

The core stylesheet reads semantic roles such as `color-bg`, `color-text`, `color-heading`, `color-card`, `border-color`, `color-focus`, `font-main`, `font-display`, `container-max`, and `grid-gap`. Values remain ordinary CSS values; Tessera does not impose a color or scale vocabulary.

## Canonical classes

| Class | Use |
| --- | --- |
| `ts-container` | Center a page region with a responsive gutter and maximum width. |
| `ts-section` | Add responsive vertical section spacing. |
| `ts-grid` | Lay out items in an auto-fitting grid. |
| `ts-stack` | Arrange related content vertically. |
| `ts-cluster` | Group inline items that wrap. |
| `ts-sidebar-layout` | Pair a supporting column with a flexible content region; it collapses at the container breakpoint. |
| `ts-prose` | Set a readable measure for long-form content. |
| `ts-card` | Group related content on a surfaced panel. |
| `ts-button` | Style a native button or action link. |
| `ts-button--secondary` | Style a secondary action with readable text contrast. |
| `ts-field`, `ts-label`, `ts-input`, `ts-select`, `ts-textarea`, `ts-help`, `ts-error`, `ts-switch` | Style native form fields, descriptions, errors, and a checkbox presented as a switch. |
| `ts-choice-group`, `ts-choice` | Group native checkbox or radio choices with their labels. |
| `ts-badge` | Show a short label or status. |
| `ts-alert` | Present an in-flow notice; use an appropriate status role when its meaning requires one. |
| `ts-accordion` | Group expandable content with native `details` and `summary` behavior. |
| `ts-breadcrumbs` | Show hierarchy through a labelled navigation landmark and ordered list. |
| `ts-avatar` | Show a person's image with alt text or labelled initials. |
| `ts-media` | Pair a visual with related text, such as an avatar and attribution. |
| `ts-table-wrap`, `ts-table` | Keep a semantic data table locally scrollable on narrow screens. |
| `ts-pagination` | Present page links in a labelled navigation landmark. |
| `ts-sr-only` | Provide text for assistive technology without visual duplication. |

The [form guide](forms.md) and [content patterns](content-patterns.md) explain accessible markup for form controls, breadcrumbs, avatars, tables, pagination, and disclosure.
The [theme guide](theming.md) covers token roles and light/dark mode, while [sections and feedback](sections-feedback.md) shows how to compose site sections and alerts.

Use `ts-stack` for vertical rhythm and `ts-cluster` for wrapping inline groups. These are separate patterns; avoid adding one-off spacing or alignment classes when composition already expresses the layout.

## Accessibility defaults

- Native elements retain their roles and keyboard behavior. Associate every input with a visible `label`.
- Keyboard focus has a visible, token-driven outline. High-contrast mode receives a system focus color.
- Dark colors adapt to the operating-system preference unless the page sets `data-theme="light"`; `data-theme="dark"` enables the dark tokens explicitly.
- Reduced-motion preferences disable smooth scrolling and minimize animation and transition durations.
- Layouts use intrinsic sizing, container queries, and wrapping instead of a fixed column count.

The [Kit showcase](../../examples/kit-showcase/content/index.md) demonstrates the classes in an editable static site.
