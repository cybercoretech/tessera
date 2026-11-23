# Tessera form controls

Tessera styles native controls. Keep labels visible and use native input types, `required`, `disabled`, and browser validation where they fit the form.

## Text, select, and multiline fields

Group each field with `ts-field`. Connect help and error text through `aria-describedby`; set `aria-invalid="true"` when a field has an error and explain the correction in text.

```html
<div class="ts-field">
  <label class="ts-label" for="email">Email address</label>
  <input class="ts-input" id="email" name="email" type="email"
         autocomplete="email" required aria-describedby="email-help email-error"
         aria-invalid="true">
  <p class="ts-help" id="email-help">Use the address associated with your account.</p>
  <p class="ts-error" id="email-error">Enter a valid email address.</p>
</div>

<div class="ts-field">
  <label class="ts-label" for="project-type">Project type</label>
  <select class="ts-select" id="project-type" name="project-type">
    <option value="">Choose a type</option>
    <option value="docs">Documentation</option>
    <option value="site">Marketing site</option>
  </select>
</div>

<div class="ts-field">
  <label class="ts-label" for="project-notes">Project notes</label>
  <textarea class="ts-textarea" id="project-notes" name="project-notes"></textarea>
</div>
```

## Checkboxes and radio groups

Use `fieldset` and `legend` to name related choices. A checkbox supports independent choices; radio buttons choose one value from a group.

```html
<fieldset class="ts-choice-group">
  <legend>Updates</legend>
  <label class="ts-choice">
    <input type="checkbox" name="updates" value="release-notes">
    <span>Send release notes</span>
  </label>
</fieldset>

<fieldset class="ts-choice-group">
  <legend>Preferred contact</legend>
  <label class="ts-choice">
    <input type="radio" name="contact" value="email">
    <span>Email</span>
  </label>
  <label class="ts-choice">
    <input type="radio" name="contact" value="phone">
    <span>Phone</span>
  </label>
</fieldset>
```

Tessera keeps native checkbox and radio rendering. The class styles spacing and uses the primary token for the checked state.

## On/off settings

Use a checkbox with `role="switch"` for a binary setting. The label supplies its accessible name; the native input retains Space-key operation and checked state:

```html
<label class="ts-switch">
  <input type="checkbox" role="switch" name="email-updates" checked>
  <span>Email updates</span>
</label>
```

Use a checkbox for an independent form choice and a switch for a setting presented as on or off. Avoid adding a second control that changes the same value.
