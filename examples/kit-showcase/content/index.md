---
title: A considered starting point
description: Explore Tessera's token-driven CSS foundations and responsive layout patterns.
layout: base
---

<section class="showcase-hero ts-container ts-section" aria-labelledby="hero-title">
  <div class="showcase-hero__copy ts-stack">
    <p class="showcase-eyebrow"><span class="showcase-eyebrow__line" aria-hidden="true"></span> An independent CSS kit</p>
    <h1 id="hero-title">Good structure.<br><em>Room to make it yours.</em></h1>
    <p class="showcase-hero__lede">Tessera gives thoughtful defaults for color, type, focus, and layout. Start with a sturdy foundation, then build a visual language that belongs to your project.</p>
    <div class="ts-cluster">
      <a class="ts-button" href="#foundations">Explore the foundations <span aria-hidden="true">↓</span></a>
      <a class="showcase-text-link" href="#components">See the patterns <span aria-hidden="true">↗</span></a>
    </div>
    <p class="showcase-note">No framework runtime · no required utility engine</p>
  </div>
  <aside class="showcase-specimen ts-card" aria-label="Tessera design tokens preview">
    <div class="showcase-specimen__top ts-cluster">
      <span class="ts-badge">Token study 01</span>
      <span class="showcase-specimen__index">/ 03</span>
    </div>
    <div class="showcase-orbit" aria-hidden="true">
      <span class="showcase-orbit__ring showcase-orbit__ring--outer"></span>
      <span class="showcase-orbit__ring showcase-orbit__ring--inner"></span>
      <span class="showcase-orbit__core">T</span>
      <span class="showcase-orbit__dot showcase-orbit__dot--one"></span>
      <span class="showcase-orbit__dot showcase-orbit__dot--two"></span>
    </div>
    <p class="showcase-specimen__caption">One token model.<br><strong>Many directions.</strong></p>
    <ul class="showcase-specimen__swatches" aria-label="Theme color samples">
      <li><span class="showcase-swatch showcase-swatch--primary" aria-hidden="true"></span><span class="ts-sr-only">Primary color</span></li>
      <li><span class="showcase-swatch showcase-swatch--secondary" aria-hidden="true"></span><span class="ts-sr-only">Accent color</span></li>
      <li><span class="showcase-swatch showcase-swatch--surface" aria-hidden="true"></span><span class="ts-sr-only">Surface color</span></li>
      <li><span class="showcase-swatch showcase-swatch--ink" aria-hidden="true"></span><span class="ts-sr-only">Text color</span></li>
    </ul>
  </aside>
</section>

<section class="showcase-ribbon" aria-label="Tessera Kit principles">
  <div class="ts-container showcase-ribbon__inner">
    <p><span>01</span> Semantic tokens</p>
    <p><span>02</span> Responsive by default</p>
    <p><span>03</span> Accessible states</p>
    <p><span>04</span> CSS owned by you</p>
  </div>
</section>

<section id="foundations" class="ts-container ts-section showcase-section" aria-labelledby="foundations-title">
  <div class="showcase-section__heading">
    <p class="showcase-eyebrow">The starting materials <span>01 — Foundations</span></p>
    <h2 id="foundations-title">Quietly doing<br>the important work.</h2>
    <p>Tokens set the tone. Sensible type and visible focus keep the experience clear for everyone.</p>
  </div>
  <div class="showcase-foundations ts-grid">
    <article class="ts-card showcase-foundation-card">
      <div class="ts-cluster showcase-card-heading">
        <span class="showcase-card-number">A</span>
        <h3>Color with roles</h3>
      </div>
      <p>Use names that explain a color’s job, so a theme can change without rewriting components.</p>
      <div class="showcase-palette">
        <div><span class="showcase-color showcase-color--brand" aria-hidden="true"></span><code>primary</code></div>
        <div><span class="showcase-color showcase-color--accent" aria-hidden="true"></span><code>secondary</code></div>
        <div><span class="showcase-color showcase-color--surface" aria-hidden="true"></span><code>surface</code></div>
        <div><span class="showcase-color showcase-color--ink" aria-hidden="true"></span><code>text</code></div>
      </div>
    </article>
    <article class="ts-card showcase-foundation-card showcase-type-card">
      <div class="ts-cluster showcase-card-heading">
        <span class="showcase-card-number">B</span>
        <h3>Type with rhythm</h3>
      </div>
      <p>Fluid display sizing, a readable measure, and familiar document elements make a strong base.</p>
      <div class="showcase-type-sample">
        <span class="showcase-type-sample__label">Display / h2</span>
        <strong>A clear point of view.</strong>
        <span class="showcase-type-sample__label">Body / 1rem · 1.65</span>
        <span>Comfortable reading at every screen size.</span>
      </div>
    </article>
    <article class="ts-card showcase-foundation-card showcase-focus-card">
      <div class="ts-cluster showcase-card-heading">
        <span class="showcase-card-number">C</span>
        <h3>Focus you can find</h3>
      </div>
      <p>Use your keyboard to move through the links and controls below. A high-contrast ring follows focus.</p>
      <a class="showcase-focus-demo" href="#layouts">Tab here to preview the focus style <span aria-hidden="true">↗</span></a>
      <span class="showcase-focus-hint">Focus visibility is part of the foundation.</span>
    </article>
  </div>
</section>

<section id="themes" class="ts-container ts-section showcase-section" aria-labelledby="themes-title">
  <div class="showcase-section__heading">
    <p class="showcase-eyebrow">One API, different moods <span>02 — Themes</span></p>
    <h2 id="themes-title">Color changes.<br>The structure stays.</h2>
    <p>Theme roles let a project change its surfaces, text, and focus color while keeping the same component markup.</p>
  </div>
  <div class="ts-grid showcase-theme-grid">
    <article class="ts-card showcase-theme-preview" data-theme="light">
      <div class="ts-cluster showcase-theme-preview__top"><span class="ts-badge">Light theme</span><span class="showcase-theme-dot" aria-hidden="true"></span></div>
      <h3>Open and sunlit</h3>
      <p>Warm paper, deep green text, and a clear focus ring.</p>
      <a class="ts-button" href="#components">Primary action</a>
    </article>
    <article class="ts-card showcase-theme-preview" data-theme="dark">
      <div class="ts-cluster showcase-theme-preview__top"><span class="ts-badge">Dark theme</span><span class="showcase-theme-dot" aria-hidden="true"></span></div>
      <h3>Quiet after hours</h3>
      <p>The same roles shift to dark surfaces and lighter text.</p>
      <a class="ts-button" href="#components">Primary action</a>
    </article>
    <article class="ts-card showcase-theme-preview showcase-theme-preview--material" data-theme="material">
      <div class="ts-cluster showcase-theme-preview__top"><span class="ts-badge">Material-inspired</span><span class="showcase-theme-dot" aria-hidden="true"></span></div>
      <h3>Clay and shadow</h3>
      <p>A warmer surface and softer elevation, with the same Kit classes.</p>
      <a class="ts-button" href="#components">Primary action</a>
    </article>
  </div>
  <p class="showcase-theme-caption">The page follows your system setting. Set <code>data-theme="light"</code> or <code>data-theme="dark"</code> on the document root to choose explicitly.</p>
</section>

<section id="layouts" class="showcase-layout-band" aria-labelledby="layouts-title">
  <div class="ts-container ts-section showcase-section">
    <div class="showcase-section__heading showcase-section__heading--wide">
    <p class="showcase-eyebrow">Space that adapts <span>03 — Layout</span></p>
      <h2 id="layouts-title">A few useful ways<br>to bring things together.</h2>
      <p>Compose with intrinsic grid, stack, and cluster patterns. They wrap with the available space instead of relying on a fixed column system.</p>
    </div>
    <div class="ts-grid showcase-layout-demos">
      <article class="showcase-layout-demo">
        <div class="showcase-layout-demo__top"><span>01 / Auto-fit grid</span><code>.ts-grid</code></div>
        <div class="showcase-mini-grid" aria-hidden="true"><span></span><span></span><span></span></div>
        <h3>Let the content set the columns.</h3>
        <p>Cards take the space they need; columns reflow as the container narrows.</p>
      </article>
      <article class="showcase-layout-demo">
        <div class="showcase-layout-demo__top"><span>02 / Vertical rhythm</span><code>.ts-stack</code></div>
        <div class="showcase-mini-stack" aria-hidden="true"><span></span><span></span><span></span></div>
        <h3>Keep a steady reading flow.</h3>
        <p>Stack related content with one consistent gap and no spacing helpers to memorize.</p>
      </article>
      <article class="showcase-layout-demo">
        <div class="showcase-layout-demo__top"><span>03 / Wrapping group</span><code>.ts-cluster</code></div>
        <div class="showcase-mini-cluster" aria-hidden="true"><span></span><span></span><span></span><span></span></div>
        <h3>Let controls wrap naturally.</h3>
        <p>Cluster tags, actions, or navigation with a gap that stays useful at every width.</p>
      </article>
    </div>
  </div>
</section>

<section id="components" class="ts-container ts-section showcase-section" aria-labelledby="components-title">
  <div class="showcase-section__heading">
    <p class="showcase-eyebrow">Patterns, not prescriptions <span>04 — Components</span></p>
    <h2 id="components-title">A small set.<br>Ready to compose.</h2>
    <p>Semantic HTML stays in charge. Tessera classes add a consistent visual layer and remain easy to override.</p>
  </div>
  <div class="ts-sidebar-layout showcase-component-layout">
    <article class="ts-card showcase-control-panel">
      <span class="ts-badge">Control sample</span>
      <h3>Start with the element.</h3>
      <p>Labels, inputs, links, and buttons keep their native roles and keyboard behavior.</p>
      <form class="ts-stack" action="#components" method="get">
        <div class="ts-field">
          <label class="ts-label" for="email-sample">Email address</label>
          <input class="ts-input" id="email-sample" name="email" type="email" autocomplete="email" placeholder="you@example.com" aria-describedby="email-help" required>
          <p class="ts-help" id="email-help">A visible label and linked helper text.</p>
        </div>
        <div class="ts-field">
          <label class="ts-label" for="project-kind">Project type</label>
          <select class="ts-select" id="project-kind" name="project-kind">
            <option value="site">Website</option>
            <option value="docs">Documentation</option>
            <option value="portfolio">Portfolio</option>
          </select>
        </div>
        <div class="ts-field">
          <label class="ts-label" for="project-notes">A note about your project</label>
          <textarea class="ts-textarea" id="project-notes" name="project-notes" rows="3" placeholder="What are you making?"></textarea>
        </div>
        <fieldset class="ts-choice-group">
          <legend>Preferred contact</legend>
          <label class="ts-choice"><input type="radio" name="contact" value="email" checked><span>Email</span></label>
          <label class="ts-choice"><input type="radio" name="contact" value="phone"><span>Phone</span></label>
        </fieldset>
        <label class="ts-switch">
          <input type="checkbox" role="switch" name="email-updates" checked>
          <span>Email updates</span>
        </label>
        <button class="ts-button" type="submit">Preview the control</button>
      </form>
    </article>
    <div class="ts-stack showcase-component-examples">
      <div class="showcase-alert-stack">
        <p class="ts-alert" data-tone="success" role="status"><strong>Saved.</strong> Your preferences are up to date.</p>
        <p class="ts-alert" data-tone="warning" role="status"><strong>Review details.</strong> One step needs your attention.</p>
        <p class="ts-alert" data-tone="error" role="alert"><strong>Could not save.</strong> Check the fields and try again.</p>
      </div>
      <article class="ts-card showcase-note-card">
        <div class="ts-cluster showcase-note-card__top"><span class="ts-badge">Composition</span><span class="showcase-note-card__index">04 min read</span></div>
        <h3>Good defaults should get out of the way.</h3>
        <p>Start with a card, a badge, and clear type. Add a project-specific layer when your product needs it.</p>
        <figure class="ts-media">
          <span class="ts-avatar" aria-hidden="true">MC</span>
          <figcaption class="ts-media__body">
            <blockquote><p>“The best defaults make the next decision easier.”</p></blockquote>
            <p>Mara Chen, product designer</p>
          </figcaption>
        </figure>
        <a class="showcase-text-link" href="#foundations">Back to foundations <span aria-hidden="true">↑</span></a>
      </article>
      <pre class="showcase-code"><code>&lt;article class="ts-card"&gt;
  &lt;span class="ts-badge"&gt;Note&lt;/span&gt;
  &lt;h3&gt;A useful pattern&lt;/h3&gt;
  &lt;p&gt;Your content goes here.&lt;/p&gt;
&lt;/article&gt;</code></pre>
    </div>
  </div>
</section>

<section id="content-patterns" class="ts-container ts-section showcase-section" aria-labelledby="content-patterns-title">
  <div class="showcase-section__heading">
    <p class="showcase-eyebrow">Useful content shapes <span>05 — Data</span></p>
    <h2 id="content-patterns-title">Keep the meaning<br>in the markup.</h2>
    <p>Breadcrumbs, tables, and pagination use native HTML semantics, with Tessera styles that adapt to a narrow container.</p>
  </div>
  <div class="ts-stack showcase-content-patterns">
    <nav class="ts-breadcrumbs" aria-label="Example breadcrumb">
      <ol class="ts-breadcrumbs__list">
        <li><a href="#content-patterns">Library</a></li>
        <li><a href="#content-patterns">Foundations</a></li>
        <li><span aria-current="page">Data patterns</span></li>
      </ol>
    </nav>
    <div class="ts-table-wrap" role="region" aria-labelledby="inventory-caption" tabindex="0">
      <table class="ts-table">
        <caption id="inventory-caption">Example component inventory</caption>
        <thead>
          <tr><th scope="col">Pattern</th><th scope="col">Role</th><th scope="col">CSS API</th></tr>
        </thead>
        <tbody>
          <tr><th scope="row">Mara Chen <span class="ts-avatar" aria-hidden="true">MC</span></th><td>Design systems</td><td><code>ts-avatar</code></td></tr>
          <tr><th scope="row">Table</th><td>Structured data</td><td><code>ts-table</code></td></tr>
          <tr><th scope="row">Pagination</th><td>Page navigation</td><td><code>ts-pagination</code></td></tr>
        </tbody>
      </table>
    </div>
    <nav class="ts-pagination" aria-label="Example inventory pages">
      <ul class="ts-pagination__list">
        <li><a href="#content-patterns" aria-label="Page 1">1</a></li>
        <li><a href="#content-patterns" aria-current="page" aria-label="Page 2, current page">2</a></li>
        <li><a href="#content-patterns" aria-label="Page 3">3</a></li>
      </ul>
    </nav>
  </div>
</section>

<section class="showcase-faq-band" aria-labelledby="faq-title">
  <div class="ts-container ts-section showcase-section">
    <div class="showcase-section__heading">
      <p class="showcase-eyebrow">Native, by design <span>06 — Disclosure</span></p>
      <h2 id="faq-title">Open up when<br>you need more.</h2>
      <p>Expandable content stays useful without a script. The browser handles keyboard interaction and state.</p>
    </div>
    <div class="ts-accordion">
      <details class="ts-accordion__item">
        <summary class="ts-accordion__trigger">Does Tessera require a JavaScript framework?</summary>
        <div class="ts-accordion__panel"><p>No. The Kit is plain CSS, and native HTML disclosure patterns work without JavaScript.</p></div>
      </details>
      <details class="ts-accordion__item">
        <summary class="ts-accordion__trigger">Can I change the visual direction?</summary>
        <div class="ts-accordion__panel"><p>Yes. Project tokens and local CSS are the source of truth. The semantic classes keep their meaning when the theme changes.</p></div>
      </details>
      <details class="ts-accordion__item">
        <summary class="ts-accordion__trigger">Do the layout patterns use a fixed grid?</summary>
        <div class="ts-accordion__panel"><p>No. They use intrinsic CSS Grid, wrapping Flexbox, and container queries to adapt to the available space.</p></div>
      </details>
    </div>
  </div>
</section>

<section class="showcase-closing">
  <div class="ts-container ts-cluster showcase-closing__inner">
    <p class="showcase-eyebrow">Built for a clear beginning</p>
    <h2>Make the next layer<br><em>your own.</em></h2>
    <a class="ts-button" href="https://github.com/cybercoretech/tessera">Read the project <span aria-hidden="true">↗</span></a>
  </div>
</section>
