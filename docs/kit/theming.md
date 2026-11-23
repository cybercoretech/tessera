# Tessera themes

Tessera themes use the same token names and component classes. Projects provide values in `tessera.toml`; the Kit consumes semantic roles for surfaces, text, borders, focus, and feedback.

```toml
[theme.tokens]
primary = "#1e5949"
secondary = "#df8750"
color_bg = "#f6f4ee"
color_text = "#263a34"
color_heading = "#172b25"
color_card = "#fffefa"
border_color = "#d9ded6"
color_focus = "#155b4c"
```

Token keys become CSS custom properties such as `--ts-color-bg` and `--ts-color-focus`. Components keep the same markup across visual themes.

## Light and dark mode

With no `data-theme` attribute, Tessera follows `prefers-color-scheme`. Set `data-theme="light"` on the document root to force light tokens, or `data-theme="dark"` to force dark tokens:

```html
<html lang="en" data-theme="dark">
```

The optional interaction script can persist a user's choice, but the CSS theme works without JavaScript. If an application provides a theme toggle, use a named native button, update its accessible state and label, and preserve visible focus.

The showcase includes side-by-side light and dark previews using the same `ts-card`, `ts-badge`, and `ts-button` patterns.

## Optional Material-inspired direction

Materialize is a visual reference, not a compatibility mode. A project can create a warmer, more elevated theme by changing its surface, radius, shadow, and accent tokens in its own stylesheet. Keep the same component names and markup; do not make the theme a second component API.
