// Tessera project site: progressive enhancement only. Every feature here has a
// no-JavaScript path in the markup (anchor links, native <details>, static text).
window.siteReady = true;

const $ = (sel, root = document) => root.querySelector(sel);
const $$ = (sel, root = document) => [...root.querySelectorAll(sel)];
const reduce = matchMedia("(prefers-reduced-motion: reduce)").matches;
const escapeHtml = (s) => s.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;").replace(/"/g, "&quot;");

// ---------------------------------------------------------------- drawer
// Below 1024px the explorer rail is an off-canvas drawer. Without JS the
// burger is a plain #files link and CSS :target opens it.
{
  const burger = $(".site-burger");
  const rail = $("#files");
  const scrim = $(".site-scrim");
  const closeBtn = $(".site-rail__close");
  const mobile = matchMedia("(max-width: 1023px)");
  let open = false;

  const set = (next, { focus = true } = {}) => {
    open = next;
    rail.classList.toggle("is-open", next);
    burger.setAttribute("aria-expanded", String(next));
    burger.setAttribute("aria-label", next ? "Close file explorer" : "Open file explorer");
    scrim.hidden = !next;
    if (next && focus) $("a[data-file]", rail)?.focus();
    if (!next && focus) burger.focus();
  };

  burger.addEventListener("click", (e) => { e.preventDefault(); set(!open); });
  burger.addEventListener("keydown", (e) => { if (e.key === " ") { e.preventDefault(); set(!open); } });
  closeBtn.addEventListener("click", (e) => { e.preventDefault(); set(false); });
  scrim.addEventListener("click", () => set(false));
  rail.addEventListener("click", (e) => { if (open && e.target.closest("a:not(.site-rail__close)")) set(false, { focus: false }); });
  document.addEventListener("keydown", (e) => { if (e.key === "Escape" && open) set(false); });
  mobile.addEventListener("change", () => { if (!mobile.matches && open) set(false, { focus: false }); });
  // Arrived via the no-JS #files link: drop the hash so :target cannot stick.
  if (location.hash === "#files") history.replaceState(null, "", location.pathname);
}

// ---------------------------------------------------------------- active file
{
  const links = new Map($$(".site-rail a[data-file]").map((a) => [a.getAttribute("href").slice(1), a]));
  const visible = new Map();
  const spy = new IntersectionObserver((entries) => {
    for (const e of entries) visible.set(e.target.id, e.isIntersecting ? e.intersectionRatio : 0);
    let best = null, ratio = 0;
    for (const [id, r] of visible) if (r > ratio) { best = id; ratio = r; }
    if (!best) return;
    for (const [id, a] of links) {
      if (id === best) a.setAttribute("aria-current", "true");
      else a.removeAttribute("aria-current");
    }
  }, { rootMargin: "-52px 0px -45% 0px", threshold: [0, .1, .25, .5, .75] });
  for (const id of links.keys()) { const s = document.getElementById(id); if (s) spy.observe(s); }
}

// ---------------------------------------------------------------- headings + typing
const GLYPHS = "01<>/{}[]=;#$_";
const scramble = (el) => {
  const nodes = [];
  const walk = (n) => n.nodeType === 3 ? nodes.push(n) : n.childNodes.forEach(walk);
  walk(el);
  const original = nodes.map((n) => n.textContent);
  const total = original.join("").length;
  el.setAttribute("aria-label", el.textContent);
  const start = performance.now();
  const duration = Math.min(900, 280 + total * 14);
  const frame = (now) => {
    const shown = Math.floor(((now - start) / duration) * total);
    let offset = 0;
    nodes.forEach((n, i) => {
      const text = original[i];
      n.textContent = [...text].map((ch, j) => (offset + j < shown || ch === " ") ? ch : GLYPHS[(Math.random() * GLYPHS.length) | 0]).join("");
      offset += text.length;
    });
    if (shown < total) requestAnimationFrame(frame);
    else { nodes.forEach((n, i) => { n.textContent = original[i]; }); el.removeAttribute("aria-label"); }
  };
  requestAnimationFrame(frame);
};

const type = (block) => new Promise((resolve) => {
  const lines = $$(".site-tl", block);
  let i = 0;
  const next = () => {
    if (i >= lines.length) { block.classList.add("is-done"); resolve(); return; }
    const line = lines[i++];
    line.classList.add("is-typed");
    setTimeout(next, 45 + Math.min(130, line.textContent.length * 4));
  };
  next();
});

const finishAll = () => {
  $$("[data-type], [data-reveal]").forEach((el) => el.classList.add("is-done"));
};

if (reduce) {
  finishAll();
} else {
  const seen = new IntersectionObserver((entries) => entries.forEach(async (e) => {
    if (!e.isIntersecting) return;
    seen.unobserve(e.target);
    const el = e.target;
    if (el.matches("[data-scramble]")) scramble(el);
    if (el.matches("[data-build]")) {
      for (const block of $$("[data-type]", el)) await type(block);
      $$("[data-reveal]", el).forEach((r) => r.classList.add("is-done"));
    } else if (el.matches("[data-type]")) {
      type(el);
    }
  }), { threshold: .25 });
  $$("[data-scramble]").forEach((el) => seen.observe(el));
  $$("[data-build]").forEach((el) => seen.observe(el));
  $$("[data-type]").filter((el) => !el.closest("[data-build]")).forEach((el) => seen.observe(el));
}

// ---------------------------------------------------------------- colour helpers
const probeCanvas = document.createElement("canvas");
probeCanvas.width = probeCanvas.height = 1;
const probeCtx = probeCanvas.getContext("2d", { willReadFrequently: true });
const toHex = (cssColor) => {
  probeCtx.clearRect(0, 0, 1, 1);
  probeCtx.fillStyle = "#000";
  probeCtx.fillStyle = cssColor;
  probeCtx.fillRect(0, 0, 1, 1);
  const [r, g, b] = probeCtx.getImageData(0, 0, 1, 1).data;
  return "#" + [r, g, b].map((v) => v.toString(16).padStart(2, "0")).join("");
};
// Resolve a custom property (which may hold var() or color-mix()) to a hex colour.
const resolveVar = (name) => {
  const probe = document.createElement("span");
  probe.style.color = `var(${name})`;
  probe.hidden = true;
  document.body.append(probe);
  const value = getComputedStyle(probe).color;
  probe.remove();
  return toHex(value);
};
const luminance = (hex) => {
  const [r, g, b] = [1, 3, 5].map((i) => parseInt(hex.slice(i, i + 2), 16) / 255)
    .map((c) => (c <= .03928 ? c / 12.92 : ((c + .055) / 1.055) ** 2.4));
  return .2126 * r + .7152 * g + .0722 * b;
};
const contrast = (a, b) => {
  const [hi, lo] = [luminance(a), luminance(b)].sort((x, y) => y - x);
  return (hi + .05) / (lo + .05);
};

// ---------------------------------------------------------------- token playground
{
  const root = $("[data-playground]");
  if (root) {
    const preview = $("[data-play-preview]", root);
    const cssOut = $("[data-play-css]", root);
    const cliOut = $("[data-play-cli]", root);
    const status = $("[data-play-status]", root);
    const contrastOut = $("[data-play-contrast]", root);
    const texts = $$("input[data-token]", root);
    const picks = new Map($$("input[data-pick]", root).map((p) => [p.dataset.pick, p]));
    const example = Object.fromEntries(texts.map((t) => [t.dataset.token, t.value]));
    const HEX = /^#([0-9a-f]{3}|[0-9a-f]{6})$/i;
    const expand = (v) => v.length === 4 ? "#" + [...v.slice(1)].map((c) => c + c).join("") : v.toLowerCase();
    const cssName = (key) => "--ts-" + key.replace(/_/g, "-");
    const KIT_SECONDARY_CONTRAST = "#172b25"; // Kit default for ts-button--secondary text

    const render = (lastKey) => {
      const values = Object.fromEntries(texts.filter((t) => HEX.test(t.value.trim())).map((t) => [t.dataset.token, expand(t.value.trim())]));
      for (const [key, value] of Object.entries(values)) preview.style.setProperty(cssName(key), value);
      cssOut.innerHTML = '<span class="site-c">:root {</span>\n' +
        texts.map((t) => `  <span class="site-k">${cssName(t.dataset.token)}</span>: ${escapeHtml(values[t.dataset.token] ?? example[t.dataset.token])};`).join("\n") +
        '\n<span class="site-c">}</span>';
      if (lastKey && values[lastKey]) {
        cliOut.innerHTML = `<span class="site-p">❯</span> tessera theme-set ${lastKey} "${escapeHtml(values[lastKey])}"`;
      }
      const primary = contrast("#ffffff", values.primary ?? example.primary);
      const secondary = contrast(KIT_SECONDARY_CONTRAST, values.secondary ?? example.secondary);
      const fmt = (r) => `<b class="${r < 4.5 ? "is-low" : ""}">${r.toFixed(1)}:1${r < 4.5 ? " · below 4.5" : ""}</b>`;
      contrastOut.innerHTML = `Button text contrast: primary ${fmt(primary)} · secondary ${fmt(secondary)}`;
    };

    const setAll = (values, label) => {
      for (const t of texts) {
        const v = values[t.dataset.token];
        if (!v) continue;
        t.value = v;
        t.removeAttribute("aria-invalid");
        picks.get(t.dataset.token).value = expand(v);
      }
      status.textContent = label;
      render();
    };

    texts.forEach((t) => t.addEventListener("input", () => {
      const v = t.value.trim();
      if (HEX.test(v)) {
        t.removeAttribute("aria-invalid");
        picks.get(t.dataset.token).value = expand(v);
      } else {
        t.setAttribute("aria-invalid", "true");
      }
      status.textContent = "edited";
      render(t.dataset.token);
    }));
    picks.forEach((p, key) => p.addEventListener("input", () => {
      const t = texts.find((x) => x.dataset.token === key);
      t.value = p.value;
      t.removeAttribute("aria-invalid");
      status.textContent = "edited";
      render(key);
    }));
    $("[data-play-reset]", root).addEventListener("click", () => setAll(example, "example"));
    $("[data-play-theme]", root).addEventListener("click", () => {
      // The page's own --ts-* roles follow the chosen Cybercore theme.
      setAll(Object.fromEntries(texts.map((t) => [t.dataset.token, resolveVar(cssName(t.dataset.token))])), "from theme");
    });
    $(".site-play__form", root).addEventListener("submit", (e) => e.preventDefault());
    render();
  }
}

// ---------------------------------------------------------------- kit sheets
{
  const sheet = $("[data-kit-sheet]");
  if (sheet && typeof sheet.showModal === "function") {
    const INLINE = new Set(["A", "SPAN", "B", "STRONG", "EM", "CODE", "OPTION", "INPUT", "LEGEND", "CAPTION", "SUMMARY", "TH", "TD", "BUTTON"]);
    const VOID = new Set(["INPUT", "BR", "IMG", "HR"]);
    const openTag = (el) => "<" + el.tagName.toLowerCase() + [...el.attributes].map((a) => a.value === "" ? ` ${a.name}` : ` ${a.name}="${a.value}"`).join("") + ">";
    // Pretty-print a template's markup: short or inline elements stay on one line.
    const format = (node, depth = 0) => {
      const pad = "  ".repeat(depth);
      if (node.nodeType === 3) { const t = node.textContent.trim(); return t ? pad + t : ""; }
      if (node.nodeType !== 1) return "";
      const html = node.outerHTML;
      if (VOID.has(node.tagName) || INLINE.has(node.tagName) || html.length <= 72 || ![...node.children].length) return pad + html;
      const kids = [...node.childNodes].map((c) => format(c, depth + 1)).filter(Boolean);
      return [pad + openTag(node), ...kids, pad + `</${node.tagName.toLowerCase()}>`].join("\n");
    };
    const highlight = (code) => escapeHtml(code)
      .replace(/&quot;([^&]*)&quot;/g, '<span class="site-s">&quot;$1&quot;</span>')
      .replace(/&lt;(\/?)([a-z0-9]+)/g, '&lt;$1<span class="site-k">$2</span>');

    const title = $("#sheet-title");
    const purpose = $("[data-sheet-purpose]", sheet);
    const element = $("[data-sheet-element]", sheet);
    const renderBox = $("[data-sheet-render]", sheet);
    const code = $("[data-sheet-code]", sheet);
    const guide = $("[data-sheet-guide]", sheet);
    let current = null;

    $$("[data-kit]").forEach((link) => link.addEventListener("click", (e) => {
      const tpl = document.querySelector(`template[data-kit-example="${link.dataset.kit}"]`);
      if (!tpl) return; // fall through to the guide link
      e.preventDefault();
      const folder = link.closest("details")?.querySelector("summary")?.textContent.trim() ?? "";
      title.textContent = `kit/${folder}${link.dataset.kit}`;
      purpose.textContent = link.dataset.purpose ?? "";
      element.textContent = link.dataset.element ?? "";
      renderBox.replaceChildren(tpl.content.cloneNode(true));
      code.innerHTML = highlight([...tpl.content.childNodes].map((n) => format(n)).filter(Boolean).join("\n"));
      guide.href = link.href;
      current?.removeAttribute("aria-current");
      current = link;
      link.setAttribute("aria-current", "true");
      sheet.showModal();
      sheet.querySelector(".site-sheet__close").focus();
    }));
    // Example links inside the render must not navigate away.
    renderBox.addEventListener("click", (e) => { if (e.target.closest("a[href]")) e.preventDefault(); });
    renderBox.addEventListener("submit", (e) => e.preventDefault());
    sheet.addEventListener("click", (e) => {
      if (e.target !== sheet) return;
      const r = sheet.getBoundingClientRect();
      if (e.clientX < r.left) sheet.close();
    });
    sheet.addEventListener("close", () => { current?.removeAttribute("aria-current"); current?.focus(); current = null; });
  }
}
