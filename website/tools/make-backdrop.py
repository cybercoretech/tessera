#!/usr/bin/env python3
"""Writes website/public/backdrop.svg: a static, dimmed "code editor" backdrop.

Adapted from dwcode's tools/make-site-background.py. An explorer rail, a code
pane with line numbers, syntax-coloured token bars and indent guides, and a
preview pane made of mosaic tiles (the rendered page), under faint neon glows.
Deterministic (fixed seed), so rebuilding gives the same file.

Run from anywhere: python3 website/tools/make-backdrop.py
"""
import random
from pathlib import Path

W, H = 1920, 1200
LINE = 24
TOP = 104
rng = random.Random(0x7E55E2A)

INK = {  # syntax colour -> weight
    "#8b98a8": 46,  # identifiers / punctuation
    "#2ee6f6": 12,  # types
    "#ff2e97": 11,  # keywords
    "#9b6bff": 8,   # functions
    "#6ee7a8": 7,   # strings
    "#ffb347": 4,   # numbers
}
COLORS, WEIGHTS = zip(*INK.items())

out = []
add = out.append
add(f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" preserveAspectRatio="xMidYMid slice">')
add("""<defs>
 <radialGradient id="gc" cx="0" cy="0" r="1" gradientUnits="userSpaceOnUse" gradientTransform="translate(320 160) scale(900 620)">
  <stop stop-color="#2ee6f6" stop-opacity=".15"/><stop offset="1" stop-color="#2ee6f6" stop-opacity="0"/></radialGradient>
 <radialGradient id="gm" cx="0" cy="0" r="1" gradientUnits="userSpaceOnUse" gradientTransform="translate(1680 1060) scale(1000 680)">
  <stop stop-color="#ff2e97" stop-opacity=".13"/><stop offset="1" stop-color="#ff2e97" stop-opacity="0"/></radialGradient>
 <radialGradient id="gv" cx="0" cy="0" r="1" gradientUnits="userSpaceOnUse" gradientTransform="translate(1180 440) scale(700 420)">
  <stop stop-color="#9b6bff" stop-opacity=".07"/><stop offset="1" stop-color="#9b6bff" stop-opacity="0"/></radialGradient>
 <linearGradient id="fade" x1="0" y1="0" x2="0" y2="1">
  <stop stop-color="#05070b" stop-opacity="0"/><stop offset=".75" stop-color="#05070b" stop-opacity=".35"/><stop offset="1" stop-color="#05070b" stop-opacity=".85"/></linearGradient>
 <linearGradient id="cur" x1="0" y1="0" x2="1" y2="0">
  <stop stop-color="#2ee6f6" stop-opacity=".10"/><stop offset=".6" stop-color="#2ee6f6" stop-opacity=".03"/><stop offset="1" stop-color="#2ee6f6" stop-opacity="0"/></linearGradient>
</defs>""")
add(f'<rect width="{W}" height="{H}" fill="#05070b"/>')


def explorer(x0, x1):
    """File tree rail: folder rows with chevrons and file rows indented."""
    add(f'<rect x="{x0}" y="{TOP-30}" width="{x1-x0}" height="{H-TOP+30}" fill="#060910" opacity=".9"/>')
    add(f'<rect x="{x0+14}" y="{TOP-16}" width="70" height="4" rx="2" fill="#8b98a8" opacity=".25"/>')
    y = TOP + 6
    depth = 0
    while y < H - 20:
        roll = rng.random()
        x = x0 + 18 + depth * 16
        if roll < 0.28:  # folder
            add(f'<rect x="{x}" y="{y-6}" width="6" height="6" fill="#2ee6f6" opacity=".35" transform="rotate(45 {x+3} {y-3})"/>')
            add(f'<rect x="{x+14}" y="{y-7}" width="{rng.randint(50, 110)}" height="6" rx="3" fill="#8b98a8" opacity=".32"/>')
            depth = min(depth + 1, 3)
        else:
            color = rng.choices(COLORS[1:], WEIGHTS[1:])[0]
            add(f'<rect x="{x+2}" y="{y-8}" width="8" height="8" rx="1.5" fill="{color}" opacity=".3"/>')
            add(f'<rect x="{x+16}" y="{y-7}" width="{rng.randint(40, 120)}" height="6" rx="3" fill="#8b98a8" opacity=".2"/>')
            if rng.random() < 0.3 and depth > 0:
                depth -= 1
        y += 26


def code_pane(x0, x1):
    """One editor pane between x0 and x1."""
    gutter = 58
    add(f'<rect x="{x0}" y="{TOP-30}" width="{x1-x0}" height="{H-TOP+30}" fill="#070a10" opacity=".9"/>')
    add(f'<rect x="{x0}" y="{TOP-30}" width="{x1-x0}" height="30" fill="#0a0e15"/>')
    tx = x0 + 10
    for i, wtab in enumerate(rng.sample([96, 120, 84, 132, 108], 3)):
        add(f'<rect x="{tx}" y="{TOP-24}" width="{wtab}" height="20" rx="4" fill="{"#111a26" if i == 0 else "#0c121b"}"/>')
        add(f'<rect x="{tx+10}" y="{TOP-16}" width="{wtab-34}" height="4" rx="2" fill="#8b98a8" opacity="{.45 if i == 0 else .2}"/>')
        if i == 0:
            add(f'<rect x="{tx}" y="{TOP-6}" width="{wtab}" height="2" fill="#2ee6f6" opacity=".55"/>')
        tx += wtab + 6
    code_x = x0 + gutter + 14
    right = x1 - 30
    indent = 0
    cur = rng.randint(5, 9)
    for r in range((H - TOP) // LINE):
        y = TOP + r * LINE
        if r == cur:
            add(f'<rect x="{x0}" y="{y-14}" width="{x1-x0}" height="{LINE}" fill="url(#cur)"/>')
            add(f'<rect x="{x0}" y="{y-14}" width="2" height="{LINE}" fill="#2ee6f6" opacity=".7"/>')
        add(f'<text x="{x0+gutter-8}" y="{y}" text-anchor="end" font-family="JetBrains Mono, ui-monospace, monospace" font-size="11" fill="{"#2ee6f6" if r == cur else "#3a4656"}" opacity="{.8 if r == cur else .7}">{r+1}</text>')
        roll = rng.random()
        if roll < 0.13:
            indent = max(0, indent - rng.choice([0, 1]))
            continue
        if roll < 0.22:
            wcm = rng.randint(160, 420)
            add(f'<rect x="{code_x + indent*28}" y="{y-8}" width="{min(wcm, right-code_x-indent*28)}" height="7" rx="3.5" fill="#5d6a7a" opacity=".32"/>')
            continue
        for g in range(indent):
            add(f'<rect x="{code_x + g*28 + 2}" y="{y-14}" width="1" height="{LINE}" fill="#1c2633"/>')
        x = code_x + indent * 28
        for _ in range(rng.randint(2, 7)):
            wt = rng.choice([18, 24, 30, 36, 44, 56, 72, 90, 120])
            if x + wt > right:
                break
            color = rng.choices(COLORS, WEIGHTS)[0]
            op = rng.uniform(.28, .5) if color != "#8b98a8" else rng.uniform(.18, .3)
            add(f'<rect x="{x}" y="{y-8}" width="{wt}" height="7" rx="3.5" fill="{color}" opacity="{op:.2f}"/>')
            x += wt + rng.choice([8, 8, 10, 14])
        step = rng.random()
        if step < 0.24 and indent < 5:
            indent += 1
        elif step > 0.72 and indent > 0:
            indent -= 1


def preview_pane(x0, x1):
    """The rendered page: a browser bar and a mosaic of tiles (tesserae)."""
    add(f'<rect x="{x0}" y="{TOP-30}" width="{x1-x0}" height="{H-TOP+30}" fill="#060910" opacity=".92"/>')
    add(f'<rect x="{x0+16}" y="{TOP-22}" width="{x1-x0-32}" height="16" rx="8" fill="#0c121b"/>')
    add(f'<rect x="{x0+30}" y="{TOP-16}" width="120" height="4" rx="2" fill="#8b98a8" opacity=".25"/>')
    pad, gap, cell = 28, 10, 54
    x_start, y = x0 + pad, TOP + 24
    # hero block
    add(f'<rect x="{x_start}" y="{y}" width="{x1-x0-2*pad}" height="150" rx="6" fill="#0b1119"/>')
    add(f'<rect x="{x_start+22}" y="{y+34}" width="210" height="14" rx="7" fill="#f2f6fa" opacity=".22"/>')
    add(f'<rect x="{x_start+22}" y="{y+60}" width="150" height="8" rx="4" fill="#8b98a8" opacity=".2"/>')
    add(f'<rect x="{x_start+22}" y="{y+96}" width="96" height="26" rx="5" fill="#9b6bff" opacity=".4"/>')
    y += 150 + 2 * gap
    cols = max(1, (x1 - x0 - 2 * pad + gap) // (cell + gap))
    tile_colors = ["#2ee6f6", "#ff2e97", "#9b6bff", "#6ee7a8", "#ffb347", "#8b98a8", "#8b98a8", "#8b98a8"]
    while y < H - cell:
        for c in range(cols):
            if rng.random() < 0.18:
                continue
            color = rng.choice(tile_colors)
            op = rng.uniform(.05, .16) if color == "#8b98a8" else rng.uniform(.08, .22)
            add(f'<rect x="{x_start + c*(cell+gap)}" y="{y}" width="{cell}" height="{cell}" rx="4" fill="{color}" opacity="{op:.2f}"/>')
        y += cell + gap


rail, split = 300, 1240
explorer(0, rail)
add(f'<rect x="{rail}" y="74" width="2" height="{H}" fill="#121a26"/>')
code_pane(rail + 2, split)
add(f'<rect x="{split}" y="74" width="2" height="{H}" fill="#121a26"/>')
preview_pane(split + 2, W)
add(f'<rect width="{W}" height="74" fill="#05070b"/>')
add(f'<rect y="73" width="{W}" height="1" fill="#141c28"/>')
for gid in ("gv", "gc", "gm", "fade"):
    add(f'<rect width="{W}" height="{H}" fill="url(#{gid})"/>')
add("</svg>")

dest = Path(__file__).resolve().parent.parent / "public/backdrop.svg"
dest.parent.mkdir(parents=True, exist_ok=True)
dest.write_text("\n".join(out) + "\n")
print(dest, round(dest.stat().st_size / 1024), "KiB")
