#!/usr/bin/env python3
"""Turn source images into the storefront's responsive WebP set.

For every image key (e.g. ``nile/p1001-1``) this writes ``data/media/<key>-<w>.webp`` for each
width in the kind's ladder (never upscaling past the source), plus a manifest entry with a tiny
blurred placeholder (LQIP), the dominant colour, alt text and licence/credit, in
``data/media/<tenant>/manifest.json`` (tenant = the key's first path segment).

Usage (one image):
    python3 -I tools/media/process.py --key nile/p1001-1 --kind product --src /path/a.webp \
        --alt-en "Galaxy S24, front" --alt-ar "جالاكسي إس 24 من الأمام" \
        --source https://... --license "DummyJSON demo asset" --credit "dummyjson.com"

Usage (many; one JSON object per line with the same field names, alt as {"en","ar"}):
    python3 -I tools/media/process.py --batch jobs.jsonl

Kinds:
    product   square canvas; transparent margins trimmed, content scaled to --fill of the side,
              alpha kept (the UI supplies the backdrop so it adapts to dark mode). Photos without
              alpha are centre-cropped to square instead.
    hero      photo, cropped to --aspect (default 21:9). Use a second key + --aspect 4:5 for the
              mobile art-directed crop.
    tile      photo, cropped to --aspect (default 4:3), for category/collection tiles.
    logo      kept as-is (alpha), widths for 1x/2x/3x header use.
    art       illustration / OG image, kept as-is, full ladder.

Run with ``python3 -I`` (isolated mode) from the project root.
"""

from __future__ import annotations

import argparse
import base64
import fcntl
import io
import json
import os
import sys

from PIL import Image

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
MEDIA = os.path.join(ROOT, "data", "media")

LADDERS = {
    "product": [240, 400, 640, 960],
    "hero": [640, 960, 1280, 1920],
    "tile": [240, 400, 640, 960],
    "logo": [64, 128, 192, 256, 384, 512],
    "art": [240, 400, 640, 960, 1280],
}
QUALITY = {"product": 80, "hero": 74, "tile": 76, "logo": 90, "art": 82}
DEFAULT_ASPECT = {"hero": (21, 9), "tile": (4, 3)}


def parse_aspect(s: str | None, kind: str):
    if s:
        w, h = s.split(":")
        return int(w), int(h)
    return DEFAULT_ASPECT.get(kind)


def has_alpha(im: Image.Image) -> bool:
    if im.mode in ("RGBA", "LA") or (im.mode == "P" and "transparency" in im.info):
        alpha = im.convert("RGBA").getchannel("A")
        return alpha.getextrema()[0] < 250
    return False


def crop_to_aspect(im: Image.Image, aw: int, ah: int, focus=(0.5, 0.5)) -> Image.Image:
    w, h = im.size
    target = aw / ah
    if w / h > target:  # too wide
        nw = round(h * target)
        x = round((w - nw) * focus[0])
        return im.crop((x, 0, x + nw, h))
    nh = round(w / target)
    y = round((h - nh) * focus[1])
    return im.crop((0, y, w, y + nh))


def square_packshot(im: Image.Image, fill: float) -> Image.Image:
    """Trim transparent margins, then centre on a transparent square canvas."""
    im = im.convert("RGBA")
    bbox = im.getchannel("A").point(lambda a: 255 if a > 8 else 0).getbbox()
    if bbox:
        im = im.crop(bbox)
    w, h = im.size
    side = round(max(w, h) / fill)
    canvas = Image.new("RGBA", (side, side), (0, 0, 0, 0))
    canvas.paste(im, ((side - w) // 2, (side - h) // 2), im)
    return canvas


def dominant_hex(im: Image.Image) -> str:
    rgba = im.convert("RGBA")
    bg = Image.new("RGBA", rgba.size, (255, 255, 255, 255))
    flat = Image.alpha_composite(bg, rgba).convert("RGB")
    r, g, b = flat.resize((1, 1), Image.Resampling.BOX).getpixel((0, 0))
    return f"#{r:02x}{g:02x}{b:02x}"


def lqip(im: Image.Image, alpha: bool) -> str:
    w, h = im.size
    tw = 20
    th = max(1, round(h * tw / w))
    small = im.resize((tw, th), Image.Resampling.BOX)
    buf = io.BytesIO()
    small.save(buf, "WEBP", quality=35, method=6)
    return "data:image/webp;base64," + base64.b64encode(buf.getvalue()).decode()


def save_ladder(im: Image.Image, key: str, kind: str, alpha: bool) -> list[int]:
    ladder = LADDERS[kind]
    src_w = im.size[0]
    widths = [w for w in ladder if w <= src_w] or [src_w]
    if src_w > widths[-1] and src_w < ladder[-1] and src_w - widths[-1] > 120:
        widths.append(src_w)  # keep the full source resolution as the top rung
    out_dir = os.path.join(MEDIA, os.path.dirname(key))
    os.makedirs(out_dir, exist_ok=True)
    for w in widths:
        h = round(im.size[1] * w / src_w)
        rung = im if w == src_w else im.resize((w, h), Image.Resampling.LANCZOS)
        path = os.path.join(MEDIA, f"{key}-{w}.webp")
        params = dict(quality=QUALITY[kind], method=6)
        if alpha:
            params["exact"] = False
        rung.save(path, "WEBP", **params)
    return widths


def process(job: dict) -> tuple[str, dict]:
    key = job["key"].strip("/")
    kind = job.get("kind", "product")
    if kind not in LADDERS:
        raise SystemExit(f"unknown kind {kind!r}")
    if "/" not in key:
        raise SystemExit(f"key must be <tenant>/<name>, got {key!r}")
    im = Image.open(job["src"])
    im.load()
    alpha = has_alpha(im)
    focus = tuple(float(x) for x in job.get("focus", "0.5,0.5").split(","))
    if kind == "product":
        if alpha:
            im = square_packshot(im, float(job.get("fill", 0.82)))
        else:
            im = crop_to_aspect(im.convert("RGB"), 1, 1, focus)
    elif kind in ("hero", "tile"):
        aw, ah = parse_aspect(job.get("aspect"), kind)
        im = crop_to_aspect(im.convert("RGB"), aw, ah, focus)
        alpha = False
    else:
        im = im.convert("RGBA") if alpha else im.convert("RGB")
        if job.get("aspect"):
            aw, ah = parse_aspect(job["aspect"], kind)
            im = crop_to_aspect(im, aw, ah, focus)
    widths = save_ladder(im, key, kind, alpha)
    alt = job.get("alt") or {"en": job.get("alt_en", ""), "ar": job.get("alt_ar", "")}
    entry = {
        "kind": kind,
        "widths": widths,
        "aspect": round(im.size[0] / im.size[1], 4),
        "alpha": alpha,
        "lqip": lqip(im, alpha),
        "color": dominant_hex(im),
        "alt": {"en": alt.get("en", ""), "ar": alt.get("ar", "")},
        "source": job.get("source", ""),
        "license": job.get("license", ""),
        "credit": job.get("credit", ""),
    }
    return key, entry


def write_manifest(entries: dict[str, dict]) -> None:
    by_tenant: dict[str, dict] = {}
    for key, entry in entries.items():
        by_tenant.setdefault(key.split("/", 1)[0], {})[key] = entry
    for tenant, items in by_tenant.items():
        path = os.path.join(MEDIA, tenant, "manifest.json")
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path + ".lock", "w") as lock:
            fcntl.flock(lock, fcntl.LOCK_EX)
            data = {}
            if os.path.exists(path):
                with open(path, encoding="utf-8") as f:
                    data = json.load(f)
            data.update(items)
            tmp = path + ".tmp"
            with open(tmp, "w", encoding="utf-8") as f:
                json.dump(dict(sorted(data.items())), f, ensure_ascii=False, indent=1)
            os.replace(tmp, path)


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--batch")
    ap.add_argument("--key")
    ap.add_argument("--kind", default="product")
    ap.add_argument("--src")
    ap.add_argument("--aspect")
    ap.add_argument("--focus", default="0.5,0.5")
    ap.add_argument("--fill", default="0.82")
    ap.add_argument("--alt-en", default="")
    ap.add_argument("--alt-ar", default="")
    ap.add_argument("--source", default="")
    ap.add_argument("--license", default="")
    ap.add_argument("--credit", default="")
    a = ap.parse_args()

    jobs = []
    if a.batch:
        with open(a.batch, encoding="utf-8") as f:
            jobs = [json.loads(line) for line in f if line.strip()]
    elif a.key and a.src:
        jobs = [{
            "key": a.key, "kind": a.kind, "src": a.src, "aspect": a.aspect, "focus": a.focus,
            "fill": a.fill, "alt": {"en": a.alt_en, "ar": a.alt_ar}, "source": a.source,
            "license": a.license, "credit": a.credit,
        }]
    else:
        ap.error("give --batch FILE or --key + --src")

    done = {}
    for job in jobs:
        try:
            k, e = process(job)
            done[k] = e
            print(f"ok  {k}  {e['widths']}")
        except Exception as exc:  # keep going; report at the end
            print(f"ERR {job.get('key')}: {exc}", file=sys.stderr)
    write_manifest(done)
    print(f"{len(done)}/{len(jobs)} images processed")


if __name__ == "__main__":
    main()
