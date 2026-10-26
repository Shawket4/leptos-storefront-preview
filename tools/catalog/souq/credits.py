#!/usr/bin/env python3
"""Write data/media/souq/CREDITS.md from the Souq media manifest (every web image, with title,
author, licence and source URL), and report orphaned files/keys.

    python3 -I tools/catalog/souq/credits.py
"""

from __future__ import annotations

import json
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(os.path.dirname(HERE)))
sys.dont_write_bytecode = True  # keep tools/ free of __pycache__
sys.path.insert(0, HERE)

import web_images  # noqa: E402

MEDIA = os.path.join(ROOT, "data", "media")
MANIFEST = os.path.join(MEDIA, "souq", "manifest.json")
CATALOG = os.path.join(ROOT, "data", "catalog", "souq.json")
OUT = os.path.join(MEDIA, "souq", "CREDITS.md")


def main() -> int:
    with open(MANIFEST, encoding="utf-8") as f:
        man = json.load(f)
    with open(CATALOG, encoding="utf-8") as f:
        cat = json.load(f)
    names = {}
    for p in cat["products"]:
        for k in p["images"]:
            names[k] = (p["id"], p["name"]["en"])

    photos = {web_images.PHOTOS[n]["source"]: web_images.PHOTOS[n] for n in web_images.PHOTOS}
    lines = [
        "# Souq media credits",
        "",
        "Every image under `data/media/souq/` came from the web and is listed here. Product packshots",
        "are placeholders until the tenant supplies its own photography. Regenerate with",
        "`python3 -I tools/catalog/souq/credits.py`.",
        "",
        "## Hero and promo photos",
        "",
        "| Keys | Title | Author | Licence | Source | Changes |",
        "|---|---|---|---|---|---|",
    ]
    by_source: dict[str, list[str]] = {}
    for k, e in sorted(man.items()):
        if e["kind"] in ("hero", "tile"):
            by_source.setdefault(e["source"], []).append(k)
    for src, keys in by_source.items():
        p = photos[src]
        lines.append(f"| {', '.join(f'`{k}`' for k in keys)} | {p['title']} | {p['author']} | {p['license']} | "
                     f"[landing page]({p['landing']}) · [file]({src}) | {p['changes']} |")
    lines += [
        "",
        "Hero/promo licences: CC BY-SA 4.0 photos are adapted (cropped, resized, one mirrored); the",
        "adapted files are shared under the same licence. CC0 photos need no attribution but are",
        "credited anyway.",
        "",
        "## Product packshots",
        "",
        "TheMealDB ingredient images (© TheMealDB, used as demo placeholders — replace with tenant",
        "photography) and DummyJSON demo assets (dummyjson.com, placeholder renders). Each is trimmed,",
        "centred on a transparent square and resized; nothing else is altered.",
        "",
        "| Key | Product | Image title | Author | Licence | Source |",
        "|---|---|---|---|---|---|",
    ]
    for k, e in sorted(man.items()):
        if e["kind"] != "product":
            continue
        pid, pname = names.get(k, ("?", "(unused)"))
        m = re.search(r"“(.+?)”", e["credit"])
        title = m.group(1) if m else e["credit"]
        author = "TheMealDB" if "themealdb" in e["source"] else "DummyJSON"
        lic = "TheMealDB placeholder" if author == "TheMealDB" else "DummyJSON demo asset"
        lines.append(f"| `{k}` | {pid} {pname} | {title} | {author} | {lic} | {e['source']} |")
    lines.append("")
    with open(OUT, "w", encoding="utf-8") as f:
        f.write("\n".join(lines))

    # housekeeping report
    referenced = set(names) | {c.get("image") for c in cat["categories"]} | set(
        k for k, e in man.items() if e["kind"] in ("hero", "tile"))
    unused = sorted(k for k in man if k not in referenced)
    files = set(os.listdir(os.path.join(MEDIA, "souq")))
    expected = {f"{k.split('/', 1)[1]}-{w}.webp" for k, e in man.items() for w in e["widths"]}
    stray = sorted(f for f in files if f.endswith(".webp") and f not in expected)
    missing = sorted(f for f in expected if f not in files)
    print(f"CREDITS.md: {sum(1 for e in man.values() if e['kind'] == 'product')} packshots, "
          f"{len(by_source)} web photos → {OUT}")
    print(f"unused manifest keys: {unused or 'none'}; stray files: {stray or 'none'}; missing files: {missing or 'none'}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
