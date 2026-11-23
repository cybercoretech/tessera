# Tessera content patterns

These patterns add a visual layer to semantic HTML. Use the native element contract shown here so the meaning remains available without the Kit CSS.

## Breadcrumbs

Use a labelled navigation landmark, an ordered list, and `aria-current="page"` for the current location:

```html
<nav class="ts-breadcrumbs" aria-label="Breadcrumb">
  <ol class="ts-breadcrumbs__list">
    <li><a href="/">Home</a></li>
    <li><a href="/guides.html">Guides</a></li>
    <li><span aria-current="page">Foundations</span></li>
  </ol>
</nav>
```

## Avatars

For a photograph, provide useful alternative text. For initials, give the element an accessible name:

```html
<img class="ts-avatar" src="mara.jpg" alt="Mara Chen">
<span class="ts-avatar" role="img" aria-label="Mara Chen">MC</span>
```

## Media object

Pair a visual and a text region with `ts-media`. Use `figure` and `figcaption` when the content is an attributed quotation or image:

```html
<figure class="ts-media">
  <span class="ts-avatar" role="img" aria-label="Mara Chen">MC</span>
  <figcaption class="ts-media__body">
    <blockquote><p>Clear foundations made the site easier to adapt.</p></blockquote>
    <p>Mara Chen, product designer</p>
  </figcaption>
</figure>
```

## Tables

Wrap wide tables so they scroll locally on narrow screens. Keep a caption and use scoped column headers:

```html
<div class="ts-table-wrap" role="region" aria-labelledby="build-status-caption" tabindex="0">
  <table class="ts-table">
    <caption id="build-status-caption">Build status by package</caption>
    <thead>
      <tr><th scope="col">Package</th><th scope="col">Status</th></tr>
    </thead>
    <tbody>
      <tr><th scope="row">Tessera Kit</th><td>In progress</td></tr>
    </tbody>
  </table>
</div>
```

The named, focusable wrapper lets keyboard users reach and scroll a wide table without moving the whole page.

## Pagination

Pagination is a navigation landmark with a useful label. Mark the current page and give symbol-only links accessible names:

```html
<nav class="ts-pagination" aria-label="Search results pages">
  <ul class="ts-pagination__list">
    <li><a href="?page=1" aria-label="Page 1">1</a></li>
    <li><a href="?page=2" aria-current="page" aria-label="Page 2, current page">2</a></li>
    <li><a href="?page=3" aria-label="Page 3">3</a></li>
  </ul>
</nav>
```

All pagination items remain ordinary links. Tessera does not add client-side pagination behavior.

## Accordion and disclosure

Use native `details` and `summary` for expandable content. The browser supplies keyboard behavior and expanded state; no Tessera script is required:

```html
<div class="ts-accordion">
  <details class="ts-accordion__item">
    <summary class="ts-accordion__trigger">Does this require JavaScript?</summary>
    <div class="ts-accordion__panel">
      <p>No. Native disclosure works without a runtime.</p>
    </div>
  </details>
</div>
```

Keep the summary concise and make each answer understandable when read out of context.
