---
title: Tessera · static sites from source you keep
description: Tessera is a Rust CLI that builds static websites from Markdown, HTML layouts, your CSS, and design tokens in tessera.toml.
layout: base
---

<section class="site-file site-hero" id="top" aria-labelledby="hero-title">
<div class="site-tab"><span class="site-tab__file"><span class="site-ico site-ico--md" aria-hidden="true">M↓</span>README.md</span><span class="site-tab__kicker">// 01 · overview</span></div>
<div class="site-hero__grid">
<div class="site-hero__copy">
<p class="site-eyebrow"><span class="site-dot" aria-hidden="true"></span>cybercore / tessera · static-site cli in rust</p>
<h1 id="hero-title" data-scramble>Build the site. <em>Keep the source.</em></h1>
<p class="site-lede">Tessera turns Markdown, HTML layouts and your own CSS into a static site in <code>dist/</code>. Design tokens live in <code>tessera.toml</code>, the Kit gives you plain-CSS <code>ts-*</code> building blocks, and every file stays yours to edit.</p>
<div class="site-cta">
<a class="site-btn site-btn--primary" href="#start">▸ get started</a>
<a class="site-btn site-btn--ghost" href="https://github.com/cybercoretech/tessera" target="_blank" rel="noopener">$ view source <span aria-hidden="true">↗</span><span class="site-sr"> (opens in a new tab)</span></a>
</div>
<ul class="site-facts" aria-label="Tessera defaults">
<li><b>static</b> html + css out</li>
<li><b>no node</b> on the css path</li>
<li><b>opt-in</b> sass, tailwind, scripts</li>
</ul>
</div>
<div class="site-term" aria-label="Output of tessera --help">
<div class="site-term__bar"><i></i><i></i><i></i><span>zsh — tessera --help</span></div>
<pre class="site-term__body" data-type><span class="site-tl"><b class="site-p">❯</b> tessera --help</span>
<span class="site-tl site-dim">Static‑site scaffolding CLI</span>
<span class="site-tl"><span class="site-dim">Usage:</span> tessera &lt;COMMAND&gt;</span>
<span class="site-tl"><b class="site-k">new</b>            <s>create a project with scaffolding</s></span>
<span class="site-tl"><b class="site-k">add-page</b>       <s>add a Markdown page under content/</s></span>
<span class="site-tl"><b class="site-k">add-component</b>  <s>add a component stub</s></span>
<span class="site-tl"><b class="site-k">theme-set</b>      <s>set a design token in tessera.toml</s></span>
<span class="site-tl"><b class="site-k">dev</b>            <s>serve, watch and live-reload</s></span>
<span class="site-tl"><b class="site-k">doctor</b>         <s>run project health checks</s></span>
<span class="site-tl"><b class="site-k">build</b>          <s>build the static site into dist/</s></span>
<span class="site-tl"><b class="site-k">clean</b>          <s>remove dist/</s></span>
<span class="site-tl"><b class="site-p">❯</b> <span class="site-caret"></span></span></pre>
</div>
</div>
</section>

<section class="site-file" id="build" aria-labelledby="build-title">
<div class="site-tab"><span class="site-tab__file"><span class="site-ico site-ico--md" aria-hidden="true">M↓</span>content/index.md</span><span class="site-tab__kicker">// 02 · live build</span></div>
<div class="site-head">
<h2 id="build-title" data-scramble>Two files in. <em>A page out.</em></h2>
<p>A page is Markdown with front matter. Tokens set the look. <code>tessera build</code> renders both into plain HTML and CSS. This is a real project from <a href="https://github.com/cybercoretech/tessera/tree/main/website/fixtures/night-shift" target="_blank" rel="noopener">website/fixtures/night-shift<span class="site-sr"> (opens in a new tab)</span></a>.</p>
</div>
<div class="site-split" data-build>
<div class="site-split__src">
<div class="site-pane">
<div class="site-pane__bar"><span>content/index.md</span><span>markdown</span></div>
<pre class="site-pane__code" data-type><span class="site-tl"><span class="site-c">---</span></span>
<span class="site-tl"><span class="site-k">title</span>: Night shift</span>
<span class="site-tl"><span class="site-k">description</span>: Notes from the late build</span>
<span class="site-tl"><span class="site-k">layout</span>: base</span>
<span class="site-tl"><span class="site-c">---</span></span>
<span class="site-tl"><span class="site-c"></span></span>
<span class="site-tl"><b class="site-p"># Night shift</b></span>
<span class="site-tl"><span class="site-c"></span></span>
<span class="site-tl">Small tools, built after dark.</span>
<span class="site-tl"><span class="site-c"></span></span>
<span class="site-tl"><span class="site-c">&lt;a</span> <span class="site-k">class</span>=<span class="site-s">"ts-button"</span> <span class="site-k">href</span>=<span class="site-s">"/log.html"</span><span class="site-c">&gt;</span>Read the log<span class="site-c">&lt;/a&gt;</span></span></pre>
</div>
<div class="site-pane">
<div class="site-pane__bar"><span>tessera.toml</span><span>excerpt</span></div>
<pre class="site-pane__code" data-type><span class="site-tl"><span class="site-c">[theme.tokens]</span></span>
<span class="site-tl"><span class="site-k">primary</span> = <span class="site-s">"#7c3aed"</span></span>
<span class="site-tl"><span class="site-k">secondary</span> = <span class="site-s">"#00e5ff"</span></span>
<span class="site-tl"><span class="site-k">color_bg</span> = <span class="site-s">"#0a0d14"</span></span>
<span class="site-tl"><span class="site-k">color_text</span> = <span class="site-s">"#c9d4df"</span></span>
<span class="site-tl"><span class="site-k">color_heading</span> = <span class="site-s">"#f2f6fa"</span></span></pre>
</div>
</div>
<div class="site-split__out">
<div class="site-browser">
<div class="site-browser__bar"><i></i><i></i><i></i><span>dist/index.html</span></div>
<div class="site-browser__page site-demo-page" data-reveal>
<h1>Night shift</h1>
<p>Small tools, built after dark.</p>
<p><a class="ts-button" href="#build" tabindex="-1">Read the log</a></p>
</div>
</div>
<div class="site-term site-term--compact" aria-label="Recorded output of tessera build">
<div class="site-term__bar"><i></i><i></i><i></i><span>recording · tessera 0.1.0</span></div>
<pre class="site-term__body" data-type><span class="site-tl"><b class="site-p">❯</b> tessera build</span>
<span class="site-tl">Primary color from Kit: <b class="site-k">#7c3aed</b></span>
<span class="site-tl"><b class="site-ok">✓</b> Built 1 page(s) into dist/</span>
<span class="site-tl site-dim">Defined design tokens:</span>
<span class="site-tl">  primary = #7c3aed</span>
<span class="site-tl">  secondary = #00e5ff</span>
<span class="site-tl">  color_bg = #0a0d14</span>
<span class="site-tl">  color_text = #c9d4df</span>
<span class="site-tl">  color_heading = #f2f6fa</span></pre>
</div>
<p class="site-note">Recorded from a real run; the CLI prints token lines in no fixed order, shown here in file order. The preview uses this page's copy of the Kit CSS.</p>
</div>
</div>
</section>

<section class="site-file" id="tokens" aria-labelledby="tokens-title">
<div class="site-tab"><span class="site-tab__file"><span class="site-ico site-ico--toml" aria-hidden="true">≡</span>tessera.toml</span><span class="site-tab__kicker">// 03 · token playground</span></div>
<div class="site-head">
<h2 id="tokens-title" data-scramble>One token. <em>Every component.</em></h2>
<p>Each key in <code>[theme.tokens]</code> becomes a <code>--ts-*</code> custom property, and the Kit reads only those. Change a value and watch the preview follow. This runs in your browser; in a project you edit <code>tessera.toml</code> or run <code>tessera theme-set</code>, then rebuild.</p>
</div>
<div class="site-play" data-playground>
<div class="site-pane site-play__config">
<div class="site-pane__bar"><span>tessera.toml</span><span data-play-status>example</span></div>
<form class="site-play__form" aria-label="Theme tokens">
<p class="site-c">[theme.tokens]</p>
<div class="site-play__row"><label for="tok-primary">primary</label><input type="color" id="tok-primary-pick" data-pick="primary" value="#7c3aed" aria-label="primary colour picker"><input type="text" id="tok-primary" data-token="primary" value="#7c3aed" spellcheck="false" autocomplete="off"></div>
<div class="site-play__row"><label for="tok-secondary">secondary</label><input type="color" id="tok-secondary-pick" data-pick="secondary" value="#00e5ff" aria-label="secondary colour picker"><input type="text" id="tok-secondary" data-token="secondary" value="#00e5ff" spellcheck="false" autocomplete="off"></div>
<div class="site-play__row"><label for="tok-color_bg">color_bg</label><input type="color" id="tok-color_bg-pick" data-pick="color_bg" value="#0a0d14" aria-label="color_bg colour picker"><input type="text" id="tok-color_bg" data-token="color_bg" value="#0a0d14" spellcheck="false" autocomplete="off"></div>
<div class="site-play__row"><label for="tok-color_card">color_card</label><input type="color" id="tok-color_card-pick" data-pick="color_card" value="#121826" aria-label="color_card colour picker"><input type="text" id="tok-color_card" data-token="color_card" value="#121826" spellcheck="false" autocomplete="off"></div>
<div class="site-play__row"><label for="tok-color_text">color_text</label><input type="color" id="tok-color_text-pick" data-pick="color_text" value="#c9d4df" aria-label="color_text colour picker"><input type="text" id="tok-color_text" data-token="color_text" value="#c9d4df" spellcheck="false" autocomplete="off"></div>
<div class="site-play__row"><label for="tok-color_heading">color_heading</label><input type="color" id="tok-color_heading-pick" data-pick="color_heading" value="#f2f6fa" aria-label="color_heading colour picker"><input type="text" id="tok-color_heading" data-token="color_heading" value="#f2f6fa" spellcheck="false" autocomplete="off"></div>
<div class="site-play__row"><label for="tok-border_color">border_color</label><input type="color" id="tok-border_color-pick" data-pick="border_color" value="#26324a" aria-label="border_color colour picker"><input type="text" id="tok-border_color" data-token="border_color" value="#26324a" spellcheck="false" autocomplete="off"></div>
<div class="site-play__row"><label for="tok-color_code_bg">color_code_bg</label><input type="color" id="tok-color_code_bg-pick" data-pick="color_code_bg" value="#1b2438" aria-label="color_code_bg colour picker"><input type="text" id="tok-color_code_bg" data-token="color_code_bg" value="#1b2438" spellcheck="false" autocomplete="off"></div>
<div class="site-play__row"><label for="tok-color_alert_bg">color_alert_bg</label><input type="color" id="tok-color_alert_bg-pick" data-pick="color_alert_bg" value="#1a1433" aria-label="color_alert_bg colour picker"><input type="text" id="tok-color_alert_bg" data-token="color_alert_bg" value="#1a1433" spellcheck="false" autocomplete="off"></div>
<div class="site-play__row"><label for="tok-color_alert_text">color_alert_text</label><input type="color" id="tok-color_alert_text-pick" data-pick="color_alert_text" value="#d9ccff" aria-label="color_alert_text colour picker"><input type="text" id="tok-color_alert_text" data-token="color_alert_text" value="#d9ccff" spellcheck="false" autocomplete="off"></div>
<div class="site-play__actions">
<button class="site-btn site-btn--ghost" type="button" data-play-theme>⟲ from current theme</button>
<button class="site-btn site-btn--ghost" type="button" data-play-reset>reset example</button>
</div>
</form>
</div>
<div class="site-play__preview-wrap">
<div class="site-browser">
<div class="site-browser__bar"><i></i><i></i><i></i><span>preview · real ts-* classes</span></div>
<div class="site-browser__page site-play__preview" data-play-preview>
<article class="ts-card">
<p><span class="ts-badge">release</span></p>
<h3>Night shift 0.2</h3>
<p>New layouts and a quieter palette. <code>tessera build</code> to try it.</p>
<p class="site-play__buttons"><a class="ts-button" href="#tokens" tabindex="-1">Read the log</a> <a class="ts-button ts-button--secondary" href="#tokens" tabindex="-1">Changelog</a></p>
</article>
<div class="ts-alert" role="note">Tokens changed. Components did not.</div>
</div>
</div>
<p class="site-play__contrast" data-play-contrast aria-live="polite">Button text contrast: 5.7:1</p>
</div>
<div class="site-pane site-play__css">
<div class="site-pane__bar"><span>styles.css · generated</span><span>css</span></div>
<pre class="site-pane__code" data-play-css><span class="site-c">:root {</span>
  <span class="site-k">--ts-primary</span>: #7c3aed;
  <span class="site-k">--ts-secondary</span>: #00e5ff;
  <span class="site-k">--ts-color-bg</span>: #0a0d14;
  <span class="site-k">--ts-color-card</span>: #121826;
  <span class="site-k">--ts-color-text</span>: #c9d4df;
  <span class="site-k">--ts-color-heading</span>: #f2f6fa;
  <span class="site-k">--ts-border-color</span>: #26324a;
  <span class="site-k">--ts-color-code-bg</span>: #1b2438;
  <span class="site-k">--ts-color-alert-bg</span>: #1a1433;
  <span class="site-k">--ts-color-alert-text</span>: #d9ccff;
<span class="site-c">}</span></pre>
<p class="site-play__cli" data-play-cli><span class="site-p">❯</span> tessera theme-set primary "#7c3aed"</p>
</div>
</div>
</section>

<section class="site-file" id="kit" aria-labelledby="kit-title">
<div class="site-tab"><span class="site-tab__file"><span class="site-ico site-ico--dir" aria-hidden="true">▸</span>kit/</span><span class="site-tab__kicker">// 04 · kit explorer</span></div>
<div class="site-head">
<h2 id="kit-title" data-scramble>A plain-CSS kit. <em>Open any file.</em></h2>
<p>Every class in the Kit catalogue, grouped by role. Open one to see it rendered with this page's tokens, the exact markup, and its guide. The Kit is plain CSS with semantic <code>ts-*</code> classes; it needs no Tailwind and no browser runtime.</p>
</div>
<div class="site-kit">
<nav class="site-kit__tree" aria-label="Kit classes">
<details open><summary>layout/</summary><ul>
<li><a href="https://github.com/cybercoretech/tessera/blob/main/docs/kit/foundations.md#canonical-classes" data-kit="ts-container" data-purpose="Center content with a responsive maximum width and page gutter." data-element="Any page section or layout wrapper">ts-container</a></li>
<li><a href="https://github.com/cybercoretech/tessera/blob/main/docs/kit/foundations.md#canonical-classes" data-kit="ts-section" data-purpose="Apply responsive block spacing to a page section." data-element="section or main content region">ts-section</a></li>
<li><a href="https://github.com/cybercoretech/tessera/blob/main/docs/kit/foundations.md#canonical-classes" data-kit="ts-grid" data-purpose="Create an auto-fitting grid that adapts to available width." data-element="A wrapper around related items">ts-grid</a></li>
<li><a href="https://github.com/cybercoretech/tessera/blob/main/docs/kit/foundations.md#canonical-classes" data-kit="ts-stack" data-purpose="Arrange related content vertically with a consistent gap." data-element="A wrapper around related content">ts-stack</a></li>
<li><a href="https://github.com/cybercoretech/tessera/blob/main/docs/kit/foundations.md#canonical-classes" data-kit="ts-cluster" data-purpose="Group inline items that wrap naturally when space is limited." data-element="A wrapper around related actions or labels">ts-cluster</a></li>
<li><a href="https://github.com/cybercoretech/tessera/blob/main/docs/kit/foundations.md#canonical-classes" data-kit="ts-sidebar-layout" data-purpose="Pair a narrow supporting region with a flexible main region." data-element="A wrapper containing complementary and main content">ts-sidebar-layout</a></li>
</ul></details>
<details open><summary>foundation/</summary><ul>
<li><a href="https://github.com/cybercoretech/tessera/blob/main/docs/kit/foundations.md#canonical-classes" data-kit="ts-prose" data-purpose="Constrain a reading column and trim its outer margins." data-element="article, section, or other content container">ts-prose</a></li>
<li><a href="https://github.com/cybercoretech/tessera/blob/main/docs/kit/foundations.md#canonical-classes" data-kit="ts-sr-only" data-purpose="Keep supporting text available to assistive technology." data-element="Any element containing supporting text">ts-sr-only</a></li>
</ul></details>
<details open><summary>controls/</summary><ul>
<li><a href="https://github.com/cybercoretech/tessera/blob/main/docs/kit/foundations.md#canonical-classes" data-kit="ts-button" data-purpose="Style a native button or link used for an action. Add ts-button--secondary for a secondary action." data-element="button or a[href]">ts-button</a></li>
<li><a href="https://github.com/cybercoretech/tessera/blob/main/docs/kit/forms.md#text-select-and-multiline-fields" data-kit="ts-field" data-purpose="Group a label, control, help text, and optional error message (ts-label, ts-input, ts-help, ts-error)." data-element="div or field container with associated labels and descriptions">ts-field</a></li>
<li><a href="https://github.com/cybercoretech/tessera/blob/main/docs/kit/forms.md#text-select-and-multiline-fields" data-kit="ts-select" data-purpose="Style a native option selector while retaining its platform behavior. ts-textarea styles multiline text." data-element="select with a visible label">ts-select</a></li>
<li><a href="https://github.com/cybercoretech/tessera/blob/main/docs/kit/forms.md#checkboxes-and-radio-groups" data-kit="ts-choice-group" data-purpose="Group related native checkboxes or radio buttons under a legend, one ts-choice per option." data-element="fieldset with legend and labelled checkbox/radio inputs">ts-choice-group</a></li>
<li><a href="https://github.com/cybercoretech/tessera/blob/main/docs/kit/forms.md#onoff-settings" data-kit="ts-switch" data-purpose="Style a native checkbox as an on/off setting control." data-element="label containing input type=checkbox with role=switch">ts-switch</a></li>
</ul></details>
<details open><summary>content/</summary><ul>
<li><a href="https://github.com/cybercoretech/tessera/blob/main/docs/kit/sections-feedback.md#feature-and-pricing-sections" data-kit="ts-card" data-purpose="Group related content on a distinct surface." data-element="article, section, or div as appropriate to the content">ts-card</a></li>
<li><a href="https://github.com/cybercoretech/tessera/blob/main/docs/kit/foundations.md#canonical-classes" data-kit="ts-badge" data-purpose="Present a short status or category label." data-element="span">ts-badge</a></li>
<li><a href="https://github.com/cybercoretech/tessera/blob/main/docs/kit/content-patterns.md#avatars" data-kit="ts-avatar" data-purpose="Show a person's image or labelled initials in a consistent circle." data-element="img with alt text, or span role=img with aria-label">ts-avatar</a></li>
<li><a href="https://github.com/cybercoretech/tessera/blob/main/docs/kit/content-patterns.md#media-object" data-kit="ts-media" data-purpose="Pair a visual or avatar with flexible descriptive content." data-element="figure with figcaption">ts-media</a></li>
<li><a href="https://github.com/cybercoretech/tessera/blob/main/docs/kit/content-patterns.md#tables" data-kit="ts-table" data-purpose="Style a data table while retaining native row and header semantics; ts-table-wrap keeps it scrollable." data-element="table inside a labelled, focusable region">ts-table</a></li>
</ul></details>
<details open><summary>feedback/</summary><ul>
<li><a href="https://github.com/cybercoretech/tessera/blob/main/docs/kit/sections-feedback.md#alerts" data-kit="ts-alert" data-purpose="Present a concise status or notice within the page flow. Tones: success, warning, error." data-element="div, with role=status or role=alert when appropriate">ts-alert</a></li>
</ul></details>
<details open><summary>navigation/</summary><ul>
<li><a href="https://github.com/cybercoretech/tessera/blob/main/docs/kit/content-patterns.md#breadcrumbs" data-kit="ts-breadcrumbs" data-purpose="Show the current page's location within a hierarchy." data-element="nav with aria-label and an ordered list">ts-breadcrumbs</a></li>
<li><a href="https://github.com/cybercoretech/tessera/blob/main/docs/kit/content-patterns.md#pagination" data-kit="ts-pagination" data-purpose="Navigate between related pages or result sets." data-element="nav with aria-label and a list of links">ts-pagination</a></li>
<li><a href="https://github.com/cybercoretech/tessera/blob/main/docs/kit/foundations.md#canonical-classes" data-kit="ts-nav" data-purpose="Lay out a navigation region's brand and links; ts-nav__links wraps the link list." data-element="nav">ts-nav</a></li>
</ul></details>
<details open><summary>disclosure/</summary><ul>
<li><a href="https://github.com/cybercoretech/tessera/blob/main/docs/kit/content-patterns.md#accordion-and-disclosure" data-kit="ts-accordion" data-purpose="Group expandable answers using the browser's native disclosure behavior." data-element="details and summary">ts-accordion</a></li>
</ul></details>
</nav>
<div class="site-kit__hint">
<p class="site-c">// select a class</p>
<p>These entries cover all 32 classes in the Kit catalogue (<code>src/kit/catalog.rs</code>); related classes such as <code>ts-label</code> and <code>ts-input</code> appear inside their parent's example. Each entry opens a sheet with a live render. Without JavaScript, each link goes to its guide.</p>
<p class="site-kit__count"><b>23</b> entries · <b>32</b> classes · <b>7</b> folders</p>
</div>
</div>
<template data-kit-example="ts-container"><div class="ts-container site-ex-outline"><p>Centered, with a page gutter.</p></div></template>
<template data-kit-example="ts-section"><section class="ts-section site-ex-outline"><p>A section with responsive block spacing.</p></section></template>
<template data-kit-example="ts-grid"><div class="ts-grid"><div class="ts-card">One</div><div class="ts-card">Two</div><div class="ts-card">Three</div></div></template>
<template data-kit-example="ts-stack"><div class="ts-stack"><div class="ts-card">First</div><div class="ts-card">Second</div><div class="ts-card">Third</div></div></template>
<template data-kit-example="ts-cluster"><div class="ts-cluster"><span class="ts-badge">rust</span><span class="ts-badge">static</span><span class="ts-badge">css-first</span><span class="ts-badge">tokens</span></div></template>
<template data-kit-example="ts-sidebar-layout"><div class="ts-sidebar-layout"><aside class="ts-card">Sidebar</aside><div class="ts-card">Main content takes the remaining width.</div></div></template>
<template data-kit-example="ts-prose"><article class="ts-prose"><h3>Release notes</h3><p>A readable measure keeps long lines comfortable, and the outer margins are trimmed so the column sits flush in its container.</p></article></template>
<template data-kit-example="ts-sr-only"><button class="ts-button" type="button">✕<span class="ts-sr-only">Close dialog</span></button></template>
<template data-kit-example="ts-button"><div class="ts-cluster"><button class="ts-button" type="button">Build site</button><a class="ts-button ts-button--secondary" href="#kit">Read the guide</a></div></template>
<template data-kit-example="ts-field"><div class="ts-field"><label class="ts-label" for="ex-email">Email address</label><input class="ts-input" id="ex-email" name="email" type="email" autocomplete="email" aria-describedby="ex-email-help ex-email-error" aria-invalid="true"><p class="ts-help" id="ex-email-help">Use the address associated with your account.</p><p class="ts-error" id="ex-email-error">Enter a valid email address.</p></div></template>
<template data-kit-example="ts-select"><div class="ts-field"><label class="ts-label" for="ex-type">Project type</label><select class="ts-select" id="ex-type" name="project-type"><option value="">Choose a type</option><option value="docs">Documentation</option><option value="site">Marketing site</option></select></div><div class="ts-field"><label class="ts-label" for="ex-notes">Project notes</label><textarea class="ts-textarea" id="ex-notes" name="project-notes"></textarea></div></template>
<template data-kit-example="ts-choice-group"><fieldset class="ts-choice-group"><legend>Preferred contact</legend><label class="ts-choice"><input type="radio" name="ex-contact" value="email" checked><span>Email</span></label><label class="ts-choice"><input type="radio" name="ex-contact" value="phone"><span>Phone</span></label></fieldset></template>
<template data-kit-example="ts-switch"><label class="ts-switch"><input type="checkbox" role="switch" name="ex-updates" checked><span>Email updates</span></label></template>
<template data-kit-example="ts-card"><article class="ts-card"><h3>Starter</h3><p>Related content on its own surface.</p></article></template>
<template data-kit-example="ts-badge"><p><span class="ts-badge">stable</span> <span class="ts-badge">opt-in</span></p></template>
<template data-kit-example="ts-avatar"><span class="ts-avatar" role="img" aria-label="Mara Chen">MC</span></template>
<template data-kit-example="ts-media"><figure class="ts-media"><span class="ts-avatar" role="img" aria-label="Mara Chen">MC</span><figcaption class="ts-media__body"><blockquote><p>Clear foundations made the site easier to adapt.</p></blockquote><p>Mara Chen, product designer</p></figcaption></figure></template>
<template data-kit-example="ts-table"><div class="ts-table-wrap" role="region" aria-labelledby="ex-caption" tabindex="0"><table class="ts-table"><caption id="ex-caption">Build status by package</caption><thead><tr><th scope="col">Package</th><th scope="col">Status</th></tr></thead><tbody><tr><th scope="row">Tessera Kit</th><td>In progress</td></tr><tr><th scope="row">Site engine</th><td>Shipped</td></tr></tbody></table></div></template>
<template data-kit-example="ts-alert"><div class="ts-stack"><div class="ts-alert" role="status">Build finished.</div><div class="ts-alert" data-tone="warning" role="status">Tailwind is selected but not installed.</div></div></template>
<template data-kit-example="ts-breadcrumbs"><nav class="ts-breadcrumbs" aria-label="Breadcrumb"><ol class="ts-breadcrumbs__list"><li><a href="#kit">Home</a></li><li><a href="#kit">Guides</a></li><li><span aria-current="page">Foundations</span></li></ol></nav></template>
<template data-kit-example="ts-pagination"><nav class="ts-pagination" aria-label="Example pages"><ul class="ts-pagination__list"><li><a href="#kit" aria-label="Page 1">1</a></li><li><a href="#kit" aria-current="page" aria-label="Page 2, current page">2</a></li><li><a href="#kit" aria-label="Page 3">3</a></li></ul></nav></template>
<template data-kit-example="ts-nav"><nav class="ts-nav" aria-label="Example"><a href="#kit"><strong>night shift</strong></a><ul class="ts-nav__links"><li><a href="#kit">log</a></li><li><a href="#kit">tools</a></li><li><a href="#kit">about</a></li></ul></nav></template>
<template data-kit-example="ts-accordion"><div class="ts-accordion"><details class="ts-accordion__item"><summary class="ts-accordion__trigger">Does this require JavaScript?</summary><div class="ts-accordion__panel"><p>No. Native disclosure works without a runtime.</p></div></details></div></template>
<dialog class="site-sheet" data-kit-sheet aria-labelledby="sheet-title">
<div class="site-sheet__bar"><span class="site-sheet__file" id="sheet-title">kit/</span><form method="dialog"><button class="site-sheet__close" type="submit" aria-label="Close">✕</button></form></div>
<div class="site-sheet__body">
<p class="site-sheet__purpose" data-sheet-purpose></p>
<p class="site-sheet__element"><span class="site-c">// element</span> <span data-sheet-element></span></p>
<div class="site-sheet__render" data-sheet-render></div>
<div class="site-pane"><div class="site-pane__bar"><span>markup</span><span>html</span></div><pre class="site-pane__code" data-sheet-code></pre></div>
<p><a class="site-btn site-btn--ghost" data-sheet-guide href="https://github.com/cybercoretech/tessera/tree/main/docs/kit" target="_blank" rel="noopener">§ open the guide <span aria-hidden="true">↗</span><span class="site-sr"> (opens in a new tab)</span></a></p>
</div>
</dialog>
</section>

<section class="site-file" id="start" aria-labelledby="start-title">
<div class="site-tab"><span class="site-tab__file"><span class="site-ico site-ico--dir" aria-hidden="true">▸</span>dist/</span><span class="site-tab__kicker">// 05 · get started</span></div>
<div class="site-start">
<div class="site-head">
<h2 id="start-title" data-scramble>Four commands. <em>One folder to ship.</em></h2>
<p>Install from source with Cargo, create a project, preview it locally, then build. Upload <code>dist/</code> to any static host; this site runs on Cloudflare Pages.</p>
<p class="site-cta"><a class="site-btn site-btn--primary" href="https://github.com/cybercoretech/tessera#-installation--build" target="_blank" rel="noopener">▸ installation guide <span aria-hidden="true">↗</span><span class="site-sr"> (opens in a new tab)</span></a></p>
</div>
<div class="site-term" aria-label="Commands to install Tessera and build a first site">
<div class="site-term__bar"><i></i><i></i><i></i><span>zsh — first site</span></div>
<pre class="site-term__body" data-type><span class="site-tl site-dim"># install the tessera command</span>
<span class="site-tl"><b class="site-p">❯</b> git clone https://github.com/cybercoretech/tessera.git</span>
<span class="site-tl"><b class="site-p">❯</b> cargo install --path tessera</span>
<span class="site-tl site-dim"># create, check and preview</span>
<span class="site-tl"><b class="site-p">❯</b> tessera new my-site &amp;&amp; cd my-site</span>
<span class="site-tl"><b class="site-p">❯</b> tessera doctor</span>
<span class="site-tl"><b class="site-p">❯</b> tessera dev</span>
<span class="site-tl site-dim"># static output for your host</span>
<span class="site-tl"><b class="site-p">❯</b> tessera build</span>
<span class="site-tl"><b class="site-ok">✓</b> Built 1 page(s) into dist/</span></pre>
</div>
</div>
</section>

<section class="site-file" id="faq" aria-labelledby="faq-title">
<div class="site-tab"><span class="site-tab__file"><span class="site-ico site-ico--md" aria-hidden="true">M↓</span>FAQ.md</span><span class="site-tab__kicker">// 06 · questions</span></div>
<div class="site-head">
<h2 id="faq-title" data-scramble>Good to know.</h2>
</div>
<div class="ts-accordion site-faq">
<details class="ts-accordion__item"><summary class="ts-accordion__trigger">Do I need JavaScript or Node?</summary><div class="ts-accordion__panel"><p>No. The default path builds static HTML and CSS with no Node toolchain. Script bundling with esbuild is a separate opt-in, off by default.</p></div></details>
<details class="ts-accordion__item"><summary class="ts-accordion__trigger">Can I use Sass, Tailwind or TypeScript?</summary><div class="ts-accordion__panel"><p>Yes, each as an explicit choice. SCSS compiles with Grass by default, or the Dart Sass CLI if you select it. Tailwind and UnoCSS are optional, mutually exclusive utility adapters. TypeScript type checking is its own opt-in step. Tessera uses tools already installed on your machine; it never downloads them for you.</p></div></details>
<details class="ts-accordion__item"><summary class="ts-accordion__trigger">Does Tailwind replace the Kit?</summary><div class="ts-accordion__panel"><p>No. The Kit is its own plain-CSS API with <code>ts-*</code> classes. Utility adapters sit beside it and do not translate or replace those classes.</p></div></details>
<details class="ts-accordion__item"><summary class="ts-accordion__trigger">Where can I host the output?</summary><div class="ts-accordion__panel"><p>Anywhere that serves static files. This site is built with Tessera from <a href="https://github.com/cybercoretech/tessera/tree/main/website" target="_blank" rel="noopener">website/<span class="site-sr"> (opens in a new tab)</span></a> and published on Cloudflare Pages.</p></div></details>
<details class="ts-accordion__item"><summary class="ts-accordion__trigger">Where are full Kit examples?</summary><div class="ts-accordion__panel"><p>See the <a href="https://github.com/cybercoretech/tessera/tree/main/examples/kit-showcase" target="_blank" rel="noopener">Kit showcase<span class="site-sr"> (opens in a new tab)</span></a> and the <a href="https://github.com/cybercoretech/tessera/tree/main/docs/kit" target="_blank" rel="noopener">Kit guides<span class="site-sr"> (opens in a new tab)</span></a>.</p></div></details>
</div>
</section>
