#!/usr/bin/env python3
"""Build the storefront's brand kit: SVG sources in assets-src/brand/, WebP sets in data/media/brand/.

    python3 -I tools/brand/build.py                 # write SVGs, render, run tools/media/process.py
    python3 -I tools/brand/build.py --svg-only      # only (re)write the generated SVG sources
    python3 -I tools/brand/build.py --only nile-mark,og-souq-ar
    python3 -I tools/brand/build.py --sheet /tmp/brand-sheet.png   # contact sheet for review

What is generated and what is hand-drawn:
  * marks, lockups, icons, OG images, the pattern tile: generated from tools/brand/geometry.py
    (the shapes) and this file (layout, colour). Edit those, not the SVGs.
  * empty-state illustrations: hand-drawn `<name>-light.svg`; `<name>-dark.svg` is derived here by
    a role-for-role colour swap (ART_DARK).
  * madar-mark(-dark).svg: verbatim copies of Madar's own symbol-primary / symbol-reversed.

Rules (owner, 2026-10-26): one global Porcelain + graphite palette for every shop, so logos are
monochrome and the shops differ by mark and wordmark, not colour; light first; app icons are
squircles (continuous-corner superellipses); no gradients anywhere; lettering in Cairo (the OG
taglines; the wordmarks are drawn, not typeset). Needs `resvg` on PATH and the Cairo TTFs (see
compose.FONTS). Run from the project root.
"""

from __future__ import annotations

import argparse
import json
import math
import os
import re
import subprocess
import sys
import tempfile

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import geometry as G  # noqa: E402
from compose import ROOT, doc, measure, resvg, shape_svg  # noqa: E402
from curves import catmull_rom  # noqa: E402
from geometry import f  # noqa: E402

SRC = os.path.join(ROOT, "assets-src", "brand")
PROCESS = os.path.join(ROOT, "tools", "media", "process.py")

# The global palette (style/tokens.css): Porcelain neutrals, graphite as the one accent.
GRAPHITE = "#111111"  # --text / --brand (light)
GRAPHITE_DARK = "#F4F4F5"  # --text / --brand (dark)
PORCELAIN = "#FAFAF9"  # --page (light); --on-ink
PAGE_DARK = "#0B0B0C"  # --page (dark)
WHITE = "#FFFFFF"

TENANTS = {
    "nile": {"name": {"en": "Nile", "ar": "نايل"},
             "tagline": {"en": ["Everything, delivered", "across Egypt"], "ar": ["كل حاجة توصلك", "لحد البيت"]}},
    "souq": {"name": {"en": "Souq", "ar": "سوق"},
             "tagline": {"en": ["Your neighbourhood", "market, online"], "ar": ["سوق الحتة...", "أونلاين"]}},
}

# Empty-state art: light colour -> dark colour, one role each (the storefront's tokens).
ART_DARK = {
    "#52525B": "#B4B4BB",  # line         (--text-2)
    "#F1F1F0": "#232326",  # fill         (--sunk)
    "#E4E4E7": "#2C2C30",  # fill-2       (--sunk-2 in dark)
    "#FFFFFF": "#18181B",  # surface      (--surface)
    "#FDFDFC": "#36363B",  # lit face     (iso tops stay the lightest face)
    "#111111": "#F4F4F5",  # accent: graphite -> dark-mode graphite
}
ART = ["empty-cart", "empty-search", "not-found", "error", "order-placed", "delivery"]

LOCKUP_K = 1.55  # mark height = K x the wordmark's x-height band (stroke included)
LOCKUP_GAP = 0.30  # gap between mark and word = this x mark height
PAD = 3.0  # transparent margin around logos, in wordmark units


def rtl(text: str) -> str:
    """Wrap an Arabic line in RLE...PDF: resvg lays text out on an LTR paragraph, which would push a
    trailing neutral (the "..." in Souq's tagline) to the wrong end."""
    return "&#x202B;" + text + "&#x202C;"  # as references, so the controls stay visible in the source


# ------------------------------------------------------------------------------------------------
# Logos
# ------------------------------------------------------------------------------------------------
def colours(tenant: str, dark: bool) -> tuple[str, str]:
    """(mark, word) colours. Monochrome by owner decision: every shop shares one palette, so a
    logo is graphite on light surfaces and near-white on dark ones."""
    c = GRAPHITE_DARK if dark else GRAPHITE
    return c, c


def lockup_parts(tenant: str, lang: str, mark_col: str, word_col: str, dot_col: str) -> tuple[str, tuple, tuple]:
    """Mark + wordmark in wordmark units; returns (svg body, bbox of everything, bbox of the mark).
    Arabic puts the mark on the right (it comes first in reading order); the mark itself is never
    mirrored."""
    mark = G.MARKS[tenant]()
    word_fn, (ref_top, ref_bot) = G.WORDS[tenant][lang]
    word = word_fn()
    mb = measure(shape_svg(mark, mark_col, G.MARK_SW, {"mark": mark_col}))
    wb = measure(shape_svg(word, word_col, G.WORD_SW, {"accent": dot_col, "ink": word_col}))
    target = LOCKUP_K * (42 + G.WORD_SW)
    ms = target / (mb[3] - mb[1])
    ty = (ref_top + ref_bot) / 2 - (mb[1] + mb[3]) / 2 * ms
    gap = LOCKUP_GAP * target
    tx = (wb[0] - gap - mb[2] * ms) if lang == "en" else (wb[2] + gap - mb[0] * ms)
    body = (
        shape_svg(mark, mark_col, G.MARK_SW, {"mark": mark_col}, f"translate({f(tx)} {f(ty)}) scale({ms:.4f})")
        + shape_svg(word, word_col, G.WORD_SW, {"accent": dot_col, "ink": word_col})
    )
    mark_box = (tx + mb[0] * ms, ty + mb[1] * ms, tx + mb[2] * ms, ty + mb[3] * ms)
    return body, measure(body), mark_box


def lockup_body(tenant: str, lang: str, mark_col: str, word_col: str, dot_col: str) -> tuple[str, tuple]:
    body, bb, _ = lockup_parts(tenant, lang, mark_col, word_col, dot_col)
    return body, bb


_BLEED: float | None = None


def lockup_bleed() -> float:
    """How far any word reaches above or below its mark, across all four lockups. Every lockup
    canvas is the mark's height plus this (and PAD) on both sides, centred on the mark, so the same
    CSS height gives the same mark size for both tenants in both languages."""
    global _BLEED
    if _BLEED is None:
        worst = 0.0
        for tenant in TENANTS:
            for lang in ("en", "ar"):
                _, bb, mk = lockup_parts(tenant, lang, "#000", "#000", "#000")
                worst = max(worst, mk[1] - bb[1], bb[3] - mk[3])
        _BLEED = worst + PAD
    return _BLEED


def lockup_svg(tenant: str, lang: str, dark: bool) -> str:
    mark_c, word_c = colours(tenant, dark)
    body, bb, mk = lockup_parts(tenant, lang, mark_c, word_c, mark_c)
    bleed = lockup_bleed()
    top = mk[1] - bleed
    height = (mk[3] - mk[1]) + 2 * bleed
    vb = (bb[0] - PAD, top, bb[2] - bb[0] + 2 * PAD, height)
    name = TENANTS[tenant]["name"]
    surface = "dark" if dark else "light"
    return doc(body, vb, title=f"{name['en']} · {name['ar']}",
               desc=f"{name['en']} logo, {'Arabic' if lang == 'ar' else 'Latin'} wordmark, for {surface} surfaces. Generated by tools/brand/build.py.")


def mark_svg(tenant: str, dark: bool) -> str:
    mark_c, _ = colours(tenant, dark)
    body = shape_svg(G.MARKS[tenant](), mark_c, G.MARK_SW, {"mark": mark_c})
    bb = measure(body)
    side = max(bb[2] - bb[0], bb[3] - bb[1]) + 2 * PAD
    cx, cy = (bb[0] + bb[2]) / 2, (bb[1] + bb[3]) / 2
    name = TENANTS[tenant]["name"]
    return doc(body, (cx - side / 2, cy - side / 2, side, side), title=f"{name['en']} · {name['ar']}",
               desc=f"{name['en']} mark for {'dark' if dark else 'light'} surfaces. Generated by tools/brand/build.py.")


# ------------------------------------------------------------------------------------------------
# Squircles: continuous-curvature shapes (Apple's app-icon and continuous-corner look)
# ------------------------------------------------------------------------------------------------
def _resample(pts: list[tuple[float, float]], m: int, closed: bool) -> list[tuple[float, float]]:
    """m points evenly spaced by arc length along a dense polyline."""
    seq = pts + ([pts[0]] if closed else [])
    acc = [0.0]
    for a, b in zip(seq, seq[1:]):
        acc.append(acc[-1] + math.dist(a, b))
    total = acc[-1]
    out, j = [], 0
    count = m if closed else m - 1
    for k in range(m):
        s = total * k / count
        while j < len(acc) - 2 and acc[j + 1] < s:
            j += 1
        a, b = seq[j], seq[j + 1]
        seg = acc[j + 1] - acc[j]
        t = (s - acc[j]) / seg if seg else 0.0
        out.append((a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t))
    return out


def _se(cx: float, cy: float, r: float, n: float, t: float) -> tuple[float, float]:
    c, s = math.cos(t), math.sin(t)
    return cx + r * math.copysign(abs(c) ** (2 / n), c), cy + r * math.copysign(abs(s) ** (2 / n), s)


def superellipse(cx: float, cy: float, r: float, n: float = 5.0, m: int = 96) -> str:
    """|x|^n + |y|^n = r^n, the classic app-icon squircle (n = 5), as one smooth closed path."""
    dense = [_se(cx, cy, r, n, 2 * math.pi * i / 4096) for i in range(4096)]
    return catmull_rom(_resample(dense, m, closed=True), closed=True) + "Z"


def squircle_rect(x: float, y: float, w: float, h: float, r: float, n: float = 5.0) -> str:
    """Rectangle whose corners are superellipse quarters of extent r: curvature runs continuously
    into the straight edges (no tangent-only joint like a plain rx corner)."""
    corners = [  # (centre, start angle) clockwise from the top-right
        ((x + w - r, y + r), -math.pi / 2),
        ((x + w - r, y + h - r), 0.0),
        ((x + r, y + h - r), math.pi / 2),
        ((x + r, y + r), math.pi),
    ]
    out = []
    for k, ((cx, cy), a0) in enumerate(corners):
        dense = [_se(cx, cy, r, n, a0 + (math.pi / 2) * i / 512) for i in range(513)]
        seg = catmull_rom(_resample(dense, 16, closed=False))
        out.append(seg if k == 0 else "L" + seg[1:])
    return "".join(out) + "Z"


def icon_svg(tenant: str, dark: bool) -> str:
    """App icon / favicon, 512 base: a squircle, graphite mark on porcelain; the dark one inverted."""
    ground, ink = (GRAPHITE, PORCELAIN) if dark else (PORCELAIN, GRAPHITE)
    mark = G.MARKS[tenant]()
    body = shape_svg(mark, ink, G.MARK_SW, {"mark": ink})
    bb = measure(body)
    glyph = 0.56 * 512
    s = glyph / max(bb[2] - bb[0], bb[3] - bb[1])
    cx, cy = (bb[0] + bb[2]) / 2, (bb[1] + bb[3]) / 2
    tile = f'<path d="{superellipse(256, 256, 256)}" fill="{ground}"/>'
    glyph_svg = shape_svg(mark, ink, G.MARK_SW, {"mark": ink}, f"translate({f(256 - cx * s)} {f(256 - cy * s)}) scale({s:.4f})")
    name = TENANTS[tenant]["name"]
    return doc(tile + glyph_svg, (0, 0, 512, 512), title=f"{name['en']} · {name['ar']}",
               desc=f"{name['en']} app icon{' (inverted)' if dark else ''}: superellipse (n = 5) squircle. Generated by tools/brand/build.py.")


# ------------------------------------------------------------------------------------------------
# Pattern: 8-point star-and-cross (khatam) lattice with an inner star in each rosette
# ------------------------------------------------------------------------------------------------
def star(cx: float, cy: float, r: float) -> str:
    """{8/2} star outline (two interlaced squares): tips at r, notches on the octagon in between."""
    rin = r * math.cos(math.radians(45)) / math.cos(math.radians(22.5))
    pts = []
    for k in range(16):
        rad = r if k % 2 == 0 else rin
        a = math.radians(k * 22.5)
        pts.append(f"{f(cx + rad * math.cos(a))} {f(cy + rad * math.sin(a))}")
    return "M" + "L".join(pts) + "Z"


def lattice(x0, y0, x1, y1, period, ox=0.0, oy=0.0) -> str:
    """Stars of radius period/2 on a square lattice (they touch tip to tip; the gaps are the
    crosses), each with an inner star of radius period/4, covering [x0,x1] x [y0,y1]."""
    d = []
    for i in range(math.floor((x0 - ox) / period) - 1, math.ceil((x1 - ox) / period) + 2):
        for j in range(math.floor((y0 - oy) / period) - 1, math.ceil((y1 - oy) / period) + 2):
            cx, cy = ox + i * period, oy + j * period
            d.append(star(cx, cy, period / 2))
            d.append(star(cx, cy, period / 4))
    return " ".join(d)


PATTERN_P = 320  # one period = the whole tile, so the tile is seamless at any size
PATTERN_INK = "#80808A"  # between --text-3 light (#71717A) and dark (#8E8E96): one tile for both themes
PATTERN_ALPHA = 0.20


def pattern_svg() -> str:
    P = PATTERN_P
    path = lattice(0, 0, P, P, P)
    body = (f'<path d="{path}" fill="none" stroke="{PATTERN_INK}" stroke-opacity="{PATTERN_ALPHA}" '
            f'stroke-width="{f(P * 0.009)}" stroke-linejoin="round"/>')
    return doc(body, (0, 0, P, P), title="Pattern",
               desc="Seamless tile: 8-point star-and-cross lattice, transparent, one period. Generated by tools/brand/build.py.")


# ------------------------------------------------------------------------------------------------
# OpenGraph images (1200 x 630), monochrome: porcelain ground, lockup + tagline on the start side,
# a solid graphite panel (continuous corners) with the lattice and the porcelain mark on the end
# side. Solid fills only.
# ------------------------------------------------------------------------------------------------
def og_svg(tenant: str, lang: str) -> str:
    t = TENANTS[tenant]
    W, H, M = 1200, 630, 48
    pw = 400
    is_ar = lang == "ar"
    px = M if is_ar else W - M - pw
    panel = squircle_rect(px, M, pw, H - 2 * M, 60)
    out = [f'<rect width="{W}" height="{H}" fill="{PORCELAIN}"/>']
    out.append(f'<defs><clipPath id="panel"><path d="{panel}"/></clipPath></defs>')
    out.append(f'<path d="{panel}" fill="{GRAPHITE}"/>')
    pcx, pcy = px + pw / 2, H / 2
    period = 132
    out.append(f'<g clip-path="url(#panel)"><path d="{lattice(px, M, px + pw, H - M, period, pcx, pcy)}" fill="none" '
               f'stroke="{PORCELAIN}" stroke-opacity="0.10" stroke-width="2" stroke-linejoin="round"/></g>')
    mark = G.MARKS[tenant]()
    mb = measure(shape_svg(mark, PORCELAIN, G.MARK_SW, {"mark": PORCELAIN}))
    s = 230 / (mb[3] - mb[1])
    out.append(shape_svg(mark, PORCELAIN, G.MARK_SW, {"mark": PORCELAIN},
                         f"translate({f(pcx - (mb[0] + mb[2]) / 2 * s)} {f(pcy - (mb[1] + mb[3]) / 2 * s)}) scale({s:.4f})"))
    # text column
    c0 = (px + pw + 48) if is_ar else (M + 48)
    c1 = (W - M - 48) if is_ar else (px - 48)
    body, bb = lockup_body(tenant, lang, GRAPHITE, GRAPHITE, GRAPHITE)
    ls = 1.25
    lw = (bb[2] - bb[0]) * ls
    lx = (c1 - lw) if is_ar else c0
    out.append(f'<g transform="translate({f(lx - bb[0] * ls)} {f(M + 48 - bb[1] * ls)}) scale({ls})">{body}</g>')
    fs = 66 if is_ar else 64
    anchor, tx = ("end", c1) if is_ar else ("start", c0)
    base = H - M - 62
    lines = t["tagline"][lang]
    for k, line in enumerate(reversed(lines)):
        text = rtl(line) if is_ar else line
        out.append(f'<text x="{f(tx)}" y="{f(base - k * fs * 1.18)}" font-family="Cairo" font-weight="700" '
                   f'font-size="{fs}" fill="{GRAPHITE}" text-anchor="{anchor}">{text}</text>')
    name = t["name"]
    return doc("".join(out), (0, 0, W, H), title=f"{name['en']} · {name['ar']}",
               desc=f"Open Graph image, {'Arabic' if is_ar else 'English'}. Tagline set in Cairo Bold. Generated by tools/brand/build.py.")


# ------------------------------------------------------------------------------------------------
# Illustrations: derive the dark file from the hand-drawn light one
# ------------------------------------------------------------------------------------------------
def art_dark(name: str) -> str:
    s = open(os.path.join(SRC, f"{name}-light.svg"), encoding="utf-8").read()
    s = re.sub(r"<!-- Storefront empty-state art, light version.*?-->",
               "<!-- Storefront empty-state art, dark version. Derived from "
               f"{name}-light.svg by tools/brand/build.py (ART_DARK); edit the light file. -->", s, flags=re.S)
    for light, dark in ART_DARK.items():
        s = re.sub(rf'(fill|stroke)="{light}"', rf'\1="{dark}"', s)
    left = set(re.findall(r'(?:fill|stroke)="(#[0-9A-Fa-f]{6})"', s)) - set(ART_DARK.values())
    if left:
        raise SystemExit(f"{name}: colours with no dark mapping: {sorted(left)}")
    return s


# ------------------------------------------------------------------------------------------------
# Jobs: key -> (svg file, kind, render size, alt)
# ------------------------------------------------------------------------------------------------
ART_ALT = {
    "empty-cart": ("An empty shopping cart", "عربة تسوق فارغة"),
    "empty-search": ("A magnifying glass over empty search results", "عدسة مكبرة فوق نتائج بحث فارغة"),
    "not-found": ("A parcel that lost its way to the map pin", "طرد ضلّ طريقه إلى العنوان على الخريطة"),
    "error": ("An unplugged cable", "كابل مفصول عن الكهرباء"),
    "order-placed": ("A sealed parcel with a check mark", "طرد مغلق عليه علامة صح"),
    "delivery": ("A delivery scooter on its way", "دراجة توصيل في الطريق"),
}


def jobs() -> list[dict]:
    out = []
    origin = "Original artwork for this storefront"
    credit = "Storefront brand kit (tools/brand)"
    for t, cfg in TENANTS.items():
        en, ar = cfg["name"]["en"], cfg["name"]["ar"]
        out.append(dict(key=f"brand/{t}-mark", svg=f"{t}-mark.svg", kind="logo", width=1024,
                        alt_en=f"{en} logo", alt_ar=f"شعار {ar}", gen=("mark", t, False)))
        out.append(dict(key=f"brand/{t}-mark-dark", svg=f"{t}-mark-dark.svg", kind="logo", width=1024,
                        alt_en=f"{en} logo", alt_ar=f"شعار {ar}", gen=("mark", t, True)))
        for lang in ("en", "ar"):
            for dark in (False, True):
                sfx = "-dark" if dark else ""
                out.append(dict(key=f"brand/{t}-lockup-{lang}{sfx}", svg=f"{t}-lockup-{lang}{sfx}.svg", kind="logo",
                                width=1024, alt_en=en, alt_ar=ar, gen=("lockup", t, lang, dark)))
        out.append(dict(key=f"brand/{t}-icon", svg=f"{t}-icon.svg", kind="logo", width=1024,
                        alt_en=f"{en} app icon", alt_ar=f"أيقونة تطبيق {ar}", gen=("icon", t, False)))
        out.append(dict(key=f"brand/{t}-icon-dark", svg=f"{t}-icon-dark.svg", kind="logo", width=1024,
                        alt_en=f"{en} app icon", alt_ar=f"أيقونة تطبيق {ar}", gen=("icon", t, True)))
        for lang in ("en", "ar"):
            tag = cfg["tagline"][lang]
            out.append(dict(key=f"brand/og-{t}-{lang}", svg=f"og-{t}-{lang}.svg", kind="art", width=1200,
                            alt_en=f"{en}: {' '.join(cfg['tagline']['en'])}",
                            alt_ar=f"{ar}: {' '.join(cfg['tagline']['ar'])}", gen=("og", t, lang)))
    for dark in (False, True):
        sfx = "-dark" if dark else ""
        out.append(dict(key=f"brand/madar-mark{sfx}", svg=f"madar-mark{sfx}.svg", kind="logo", width=1024,
                        alt_en="Madar", alt_ar="مدار", gen=None,
                        source=f"MadarDashboard/site/src/assets/brand/symbol-{'reversed' if dark else 'primary'}.svg",
                        license="Madar brand asset, used for the Powered by Madar badge", credit="Madar"))
    for name in ART:
        en, ar = ART_ALT[name]
        out.append(dict(key=f"brand/{name}-light", svg=f"{name}-light.svg", kind="art", width=1280,
                        alt_en=en, alt_ar=ar, gen=None))
        out.append(dict(key=f"brand/{name}-dark", svg=f"{name}-dark.svg", kind="art", width=1280,
                        alt_en=en, alt_ar=ar, gen=("art", name)))
    out.append(dict(key="brand/pattern-geo", svg="pattern-geo.svg", kind="art", width=1280,
                    alt_en="Eight-point star lattice pattern", alt_ar="نقش هندسي من نجوم ثمانية", gen=("pattern",)))
    for j in out:
        j.setdefault("source", f"assets-src/brand/{j['svg']}")
        j.setdefault("license", origin)
        j.setdefault("credit", credit)
    return out


def generate(gen: tuple) -> str:
    kind = gen[0]
    if kind == "mark":
        return mark_svg(gen[1], gen[2])
    if kind == "lockup":
        return lockup_svg(gen[1], gen[2], gen[3])
    if kind == "icon":
        return icon_svg(gen[1], gen[2])
    if kind == "og":
        return og_svg(gen[1], gen[2])
    if kind == "art":
        return art_dark(gen[1])
    if kind == "pattern":
        return pattern_svg()
    raise ValueError(kind)


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--svg-only", action="store_true")
    ap.add_argument("--only", help="comma-separated names (key without brand/)")
    ap.add_argument("--sheet", help="also write a review contact sheet PNG here")
    a = ap.parse_args()

    todo = jobs()
    if a.only:
        want = {w.strip() for w in a.only.split(",")}
        todo = [j for j in todo if j["key"].split("/", 1)[1] in want]
    os.makedirs(SRC, exist_ok=True)
    for j in todo:
        if j["gen"]:
            with open(os.path.join(SRC, j["svg"]), "w", encoding="utf-8") as fh:
                fh.write(generate(j["gen"]))
    print(f"svg: {len(todo)} sources in {os.path.relpath(SRC, ROOT)}")
    if a.svg_only:
        return

    with tempfile.TemporaryDirectory() as tmp:
        batch = []
        for j in todo:
            png = os.path.join(tmp, j["key"].split("/", 1)[1] + ".png")
            resvg(os.path.join(SRC, j["svg"]), png, width=j["width"])
            j["png"] = png
            batch.append({"key": j["key"], "kind": j["kind"], "src": png,
                          "alt": {"en": j["alt_en"], "ar": j["alt_ar"]},
                          "source": j["source"], "license": j["license"], "credit": j["credit"]})
        if a.sheet:
            contact_sheet(todo, a.sheet)
        jl = os.path.join(tmp, "jobs.jsonl")
        with open(jl, "w", encoding="utf-8") as fh:
            for b in batch:
                fh.write(json.dumps(b, ensure_ascii=False) + "\n")
        subprocess.run([sys.executable, "-I", PROCESS, "--batch", jl], check=True, cwd=ROOT)


def contact_sheet(todo: list[dict], out: str) -> None:
    from PIL import Image

    cell = 300
    cols = 4
    rows = math.ceil(len(todo) / cols)
    sheet = Image.new("RGB", (cols * cell, rows * cell), WHITE)
    for k, j in enumerate(todo):
        dark = j["key"].endswith("-dark")
        bg = PAGE_DARK if dark else PORCELAIN
        tile = Image.new("RGBA", (cell, cell), bg)
        im = Image.open(j["png"]).convert("RGBA")
        im.thumbnail((cell - 24, cell - 24))
        tile.alpha_composite(im, ((cell - im.width) // 2, (cell - im.height) // 2))
        sheet.paste(tile.convert("RGB"), ((k % cols) * cell, (k // cols) * cell))
    sheet.save(out)
    print(f"sheet: {out}")


if __name__ == "__main__":
    main()
