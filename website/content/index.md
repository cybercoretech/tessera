---
title: Tessera — make the web your own
description: Build a static site with Tessera: a small Rust CLI, editable source files, and a CSS-first Kit.
layout: base
---

<section class="site-hero ts-container ts-section" aria-labelledby="hero-title">
  <div class="site-hero__copy">
    <p class="site-kicker"><span class="site-kicker__dot" aria-hidden="true"></span> A static-site toolkit in Rust</p>
    <h1 id="hero-title">Build the site.<br><em>Keep the source.</em></h1>
    <p class="site-hero__lead">Tessera turns Markdown, HTML layouts, and your styles into a fast static site. Start with a working project, shape its design with tokens, and keep every file yours.</p>
    <div class="ts-cluster site-hero__actions">
      <a class="ts-button site-button" href="#start">Make your first site <span aria-hidden="true">↗</span></a>
      <a class="site-text-link" href="#how-it-works">See how it works <span aria-hidden="true">↓</span></a>
    </div>
    <ul class="site-hero__facts" aria-label="Tessera defaults">
      <li><span>01</span> Static HTML</li><li><span>02</span> Your CSS</li><li><span>03</span> No required runtime</li>
    </ul>
  </div>
  <div class="site-hero__visual" role="img" aria-label="Tessera project files building into a finished static website">
    <div class="site-orbit site-orbit--one" aria-hidden="true"></div><div class="site-orbit site-orbit--two" aria-hidden="true"></div>
    <div class="site-build-card">
      <div class="site-build-card__top"><span class="site-window-dots" aria-hidden="true"><i></i><i></i><i></i></span><span>my-site / build</span><span class="site-build-status"><i aria-hidden="true"></i> ready</span></div>
      <div class="site-build-card__body">
        <div class="site-file-list" aria-label="Project inputs">
          <p class="site-micro-label">FILES YOU EDIT</p>
          <div><span class="site-file-icon site-file-icon--content" aria-hidden="true">▧</span><span>content/</span><span class="site-file-note">Markdown</span></div>
          <div><span class="site-file-icon site-file-icon--layout" aria-hidden="true">◫</span><span>layouts/</span><span class="site-file-note">HTML</span></div>
          <div><span class="site-file-icon site-file-icon--style" aria-hidden="true">◈</span><span>styles/</span><span class="site-file-note">CSS or SCSS</span></div>
          <div><span class="site-file-icon site-file-icon--config" aria-hidden="true">≡</span><span>tessera.toml</span><span class="site-file-note">tokens</span></div>
        </div>
        <div class="site-build-arrow" aria-hidden="true">↓</div><div class="site-build-command"><span class="site-command-prompt" aria-hidden="true">$</span> tessera build</div>
        <div class="site-output-preview"><div class="site-output-preview__bar"><span></span><span></span><span></span><b>dist/index.html</b></div><div class="site-output-preview__page"><span class="site-output-preview__wordmark">your site<span>.</span></span><div class="site-output-preview__lines" aria-hidden="true"><i></i><i></i><i></i></div><div class="site-output-preview__tiles" aria-hidden="true"><i></i><i></i><i></i></div></div></div>
      </div>
      <div class="site-build-card__foot"><span>source files</span><span>→</span><strong>portable site</strong></div>
    </div>
    <p class="site-visual-caption"><span aria-hidden="true">·</span> Build once. Host almost anywhere.</p>
  </div>
</section>

<section class="site-proof-band" aria-label="Tessera principles"><div class="ts-container site-proof-band__inner"><p><span>01</span> Write in Markdown</p><p><span>02</span> Design with tokens</p><p><span>03</span> Keep your HTML</p><p><span>04</span> Ship static files</p></div></section>

<section id="how-it-works" class="ts-container ts-section site-section" aria-labelledby="workflow-title">
  <div class="site-section-heading"><p class="site-kicker">A small, understandable workflow <span>01 / How it works</span></p><h2 id="workflow-title">From a blank folder<br>to a real website.</h2><p>Tessera gives you a complete project to edit. Your content, page structure, styles, and settings stay in ordinary files, and the build puts the finished site in <code>dist/</code>.</p></div>
  <div class="ts-grid site-workflow-grid">
    <article class="site-workflow-step"><span class="site-step-number">01 / CREATE</span><h3>Start with a project</h3><p>Scaffold the folders and starter files. The defaults use CSS; scripts are present as an example but stay disabled.</p><code>tessera new my-site</code></article>
    <article class="site-workflow-step"><span class="site-step-number">02 / WRITE</span><h3>Make it yours</h3><p>Write a page in Markdown, add its title and description, then edit the HTML layout and CSS to fit your idea.</p><code>content/index.md</code></article>
    <article class="site-workflow-step"><span class="site-step-number">03 / PREVIEW</span><h3>See changes locally</h3><p>Run the development server while you work. It watches your source files and rebuilds when they change.</p><code>tessera dev</code></article>
    <article class="site-workflow-step"><span class="site-step-number">04 / PUBLISH</span><h3>Build portable output</h3><p>Compile pages, styles, and public assets into static files. Upload <code>dist/</code> to the host you choose.</p><code>tessera build</code></article>
  </div>
</section>

<section class="site-example-band" aria-labelledby="example-title"><div class="ts-container ts-section site-example-layout">
  <div class="site-example-copy"><p class="site-kicker">A page is just a file <span>02 / In practice</span></p><h2 id="example-title">Write content<br>without a new language.</h2><p>Front matter gives each page its metadata. Markdown holds the words. Add regular HTML when you need a specific element, then use your own classes or Tessera's Kit.</p><a class="site-light-link" href="https://github.com/cybercoretech/tessera#quickstart">Read the full quickstart <span aria-hidden="true">↗</span></a></div>
  <div class="site-code-window"><div class="site-code-window__head"><span>content/about.md</span><span>MARKDOWN</span></div><pre><code>---
title: About the studio
description: What we make and why
layout: base
---

# Built with care.

We make useful things for the open web.

&lt;a class="ts-button" href="/work.html"&gt;See our work&lt;/a&gt;</code></pre><div class="site-code-window__result"><span class="site-code-window__label">TESSERA BUILDS</span><strong>About the studio</strong><span>Rendered page · dist/about.html</span></div></div>
</div></section>

<section class="site-kit-band" id="kit" aria-labelledby="kit-title"><div class="ts-container ts-section site-kit-layout">
  <div class="site-kit-copy"><p class="site-kicker">The Tessera Kit <span>03 / Design foundations</span></p><h2 id="kit-title">Useful defaults.<br><em>Your own identity.</em></h2><p>Use a consistent set of responsive layout patterns, controls, content styles, and theme tokens. The Kit is plain CSS with semantic <code>ts-*</code> classes; it works without Tailwind or a browser runtime.</p><a class="site-light-link" href="https://github.com/cybercoretech/tessera/tree/main/docs/kit">Read the Kit guides <span aria-hidden="true">↗</span></a></div>
  <div class="site-kit-board"><div class="site-kit-board__top"><span>THE KIT / COMPOSED, NOT LOCKED</span><span>PLAIN CSS</span></div><div class="site-kit-board__tiles">
    <article class="site-kit-tile site-kit-tile--type"><span>LAYOUT / 01</span><strong>Room to<br>read and think.</strong><small>.ts-prose</small></article>
    <article class="site-kit-tile site-kit-tile--control"><span>ACTION / 02</span><a class="ts-button site-kit-button" href="#start">Start a project <span aria-hidden="true">↗</span></a><small>.ts-button</small></article>
    <article class="site-kit-tile site-kit-tile--layout"><span>GRID / 03</span><div class="site-kit-mini-grid" aria-hidden="true"><i></i><i></i><i></i></div><small>.ts-grid</small></article>
    <article class="site-kit-tile site-kit-tile--theme"><span>TOKENS / 04</span><div class="site-swatch-row" aria-label="Storm blue, rain silver, and charcoal theme colors"><i></i><i></i><i></i></div><small>tokens → CSS variables</small></article>
  </div><div class="site-kit-board__bottom"><span>semantic markup</span><span>shared tokens</span><span>your stylesheet</span></div></div>
</div></section>

<section class="ts-container ts-section site-section" aria-labelledby="tokens-title"><div class="site-section-heading site-section-heading--split"><div><p class="site-kicker">One token, a site-wide change <span>04 / Make it yours</span></p><h2 id="tokens-title">Set the tone<br>in one place.</h2></div><p>Token keys become CSS custom properties with the <code>--ts-</code> prefix. Change a value in <code>tessera.toml</code>, rebuild, and keep your component markup intact.</p></div>
  <div class="site-token-demo"><div class="site-token-demo__code"><span class="site-code-window__label">tessera.toml</span><pre><code>[theme.tokens]
primary = "#456a83"
secondary = "#b8c9d2"
color_bg = "#0b141c"
color_text = "#d9e3e8"
font_display = "Georgia, serif"</code></pre></div><div class="site-token-demo__arrow" aria-hidden="true">→</div><div class="site-token-demo__result"><span class="site-code-window__label">YOUR CSS</span><code>--ts-primary: #456a83;</code><code>--ts-color-bg: #0b141c;</code><code>--ts-font-display: Georgia, serif;</code><p>Same classes. A visual direction that belongs to you.</p></div></div>
</section>

<section class="ts-container ts-section site-section site-tool-section" id="toolchains" aria-labelledby="toolchains-title"><div class="site-section-heading site-section-heading--split"><div><p class="site-kicker">Bring the tools you prefer <span>05 / Optional</span></p><h2 id="toolchains-title">Add complexity<br>only when useful.</h2></div><p>The plain CSS path needs no Node toolchain. If your project already uses other tools, opt in to the compiler or adapter that fits it. Tessera keeps its own Kit and token model.</p></div>
  <div class="ts-grid site-tool-grid">
    <article class="site-tool-card"><span class="site-tool-index">STYLES / 01</span><h3>CSS or Sass</h3><p>Write CSS directly or use SCSS with Grass. Choose the Dart Sass CLI for projects that need its Sass behavior.</p><span class="site-tool-status">CSS DEFAULT · COMPILERS OPTIONAL</span></article>
    <article class="site-tool-card"><span class="site-tool-index">UTILITIES / 02</span><h3>Tailwind or UnoCSS</h3><p>Enable one utility adapter if you want it. Utilities complement your project; they do not replace or translate <code>ts-*</code> classes.</p><span class="site-tool-status">OPTIONAL · SELECT ONE</span></article>
    <article class="site-tool-card"><span class="site-tool-index">SCRIPTS / 03</span><h3>JavaScript or TypeScript</h3><p>Bundle chosen entry points with esbuild. Type checking is a separate opt-in step, and browser scripts are off by default.</p><span class="site-tool-status">OPT-IN · OFF BY DEFAULT</span></article>
  </div><p class="site-tool-note"><span aria-hidden="true">↳</span> Tessera uses the tools installed in your environment; it does not fetch or install them for you.</p>
</section>

<section class="site-start-band" id="start" aria-labelledby="start-title"><div class="ts-container ts-section site-start-layout">
  <div><p class="site-kicker">Go from idea to local preview <span>06 / Start building</span></p><h2 id="start-title">Your first site<br>is four commands away.</h2><p>Install Tessera with Cargo, create a project, and start the local server. When you are ready, build the static files and publish <code>dist/</code>.</p><a class="ts-button site-button site-button--paper" href="https://github.com/cybercoretech/tessera#installation--build">Installation and quickstart <span aria-hidden="true">↗</span></a></div>
  <div class="site-terminal" aria-label="Commands to create and run a Tessera site"><div class="site-terminal__head"><span class="site-window-dots" aria-hidden="true"><i></i><i></i><i></i></span><span>TERMINAL / YOUR FIRST SITE</span></div><pre><code><span class="site-terminal__comment"># Create and enter the project</span>
<span class="site-terminal__prompt">$</span> tessera new my-site
<span class="site-terminal__prompt">$</span> cd my-site

<span class="site-terminal__comment"># Check setup and preview your work</span>
<span class="site-terminal__prompt">$</span> tessera doctor
<span class="site-terminal__prompt">$</span> tessera dev

<span class="site-terminal__comment"># Build files for your host</span>
<span class="site-terminal__prompt">$</span> tessera build</code></pre><div class="site-terminal__foot"><span>OUTPUT: ./dist/ · STATIC FILES</span><span aria-hidden="true">↗</span></div></div>
</div></section>

<section class="ts-container ts-section site-questions" aria-labelledby="questions-title"><div class="site-section-heading"><p class="site-kicker">Good to know <span>07 / Questions</span></p><h2 id="questions-title">A few things<br>before you begin.</h2></div><div class="ts-accordion site-questions__list">
  <details class="ts-accordion__item"><summary class="ts-accordion__trigger">Do I need JavaScript to use Tessera?</summary><div class="ts-accordion__panel"><p>No. Tessera builds static HTML and CSS by default. Its optional script bundling and interaction features are separate choices.</p></div></details>
  <details class="ts-accordion__item"><summary class="ts-accordion__trigger">Do I have to use Tailwind?</summary><div class="ts-accordion__panel"><p>No. Tessera's Kit is its own CSS API. Tailwind and UnoCSS are optional utility adapters and do not replace the semantic <code>ts-*</code> classes.</p></div></details>
  <details class="ts-accordion__item"><summary class="ts-accordion__trigger">Where can I host the output?</summary><div class="ts-accordion__panel"><p>Anywhere that serves static files. This Tessera site is published on Cloudflare Pages at <a href="https://tessera-8gd.pages.dev/">tessera-8gd.pages.dev</a>.</p></div></details>
  <details class="ts-accordion__item"><summary class="ts-accordion__trigger">Where can I see real Kit examples?</summary><div class="ts-accordion__panel"><p>Browse the <a href="https://github.com/cybercoretech/tessera/tree/main/examples/kit-showcase">editable Kit showcase</a> and the <a href="https://github.com/cybercoretech/tessera/tree/main/docs/kit">Kit guides</a> for working markup and patterns.</p></div></details>
</div></section>
