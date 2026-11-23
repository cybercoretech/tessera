# Tessera sections and feedback

Tessera keeps larger page sections composable. Build hero, feature, pricing, testimonial, and footer layouts from the same container, section, grid, stack, cluster, and card primitives rather than introducing a second section-specific utility vocabulary.

## Feature and pricing sections

```html
<section class="ts-container ts-section" aria-labelledby="features-title">
  <div class="ts-prose">
    <p>What you get</p>
    <h2 id="features-title">A clear place to start.</h2>
  </div>
  <div class="ts-grid">
    <article class="ts-card">
      <h3>Useful defaults</h3>
      <p>Start with accessible typography, focus, and layout rules.</p>
    </article>
    <article class="ts-card">
      <h3>Room to adapt</h3>
      <p>Use project tokens and your own stylesheet to set the visual direction.</p>
    </article>
  </div>
</section>
```

Pricing plans can use the same `ts-grid` and `ts-card`; distinguish the recommended plan with a visible text label, not color alone:

```html
<section class="ts-container ts-section" aria-labelledby="plans-title">
  <h2 id="plans-title">Choose a plan</h2>
  <div class="ts-grid">
    <article class="ts-card">
      <span class="ts-badge">Most popular</span>
      <h3>Studio</h3>
      <p><strong>$24</strong> per month</p>
      <ul><li>Three projects</li><li>All core Kit patterns</li></ul>
      <a class="ts-button" href="/signup.html">Choose Studio</a>
    </article>
  </div>
</section>
```

Testimonials should use `figure`, `blockquote`, and `figcaption`. Site footers remain semantic `footer` landmarks.

## Alerts

Use one `.ts-alert` pattern with a tone describing the message. Include text that explains the state; color is supplementary. Use `role="status"` for a non-urgent update and `role="alert"` only when the message needs an immediate announcement.

```html
<p class="ts-alert" data-tone="success" role="status">
  Preferences saved.
</p>

<p class="ts-alert" data-tone="error" role="alert">
  The form could not be saved. Check the highlighted fields and try again.
</p>
```

Supported tones are `success`, `warning`, and `error`. A neutral notice uses `.ts-alert` without `data-tone`.
