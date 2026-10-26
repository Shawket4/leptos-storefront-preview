#!/usr/bin/env python3
"""Print the writing brief for a range of Nile products and draw their contact sheet.

    python3 -I tools/catalog/nile/brief.py 1001-1016,1101-1108 [SHEET.png]
"""

from __future__ import annotations

import importlib.util
import json
import os
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(os.path.dirname(HERE)))
SRC = os.path.join(ROOT, "data", "media", "_src", "nile")


def load(path, name):
    spec = importlib.util.spec_from_file_location(name, path)
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def main() -> int:
    ranges = sys.argv[1]
    sheet = sys.argv[2] if len(sys.argv) > 2 else None
    want = set()
    for r in ranges.split(","):
        a, _, b = r.partition("-")
        want.update(range(int(a), int(b or a) + 1))
    skel = load(os.path.join(HERE, "skeleton.py"), "sk")
    names = load(os.path.join(HERE, "names_ar.py"), "na").NAMES
    with open(os.path.join(SRC, "dummyjson-products.json"), encoding="utf-8") as f:
        dj = {p["id"]: p for p in json.load(f)["products"]}
    with open(os.path.join(SRC, "dj", "index.json"), encoding="utf-8") as f:
        index = json.load(f)
    cats = {c[0]: c for c in skel.CATEGORIES}
    cells = []
    for pid in sorted(want):
        if pid not in skel.PRODUCTS:
            continue
        cat, img, brand, working, tags, notes = skel.PRODUCTS[pid]
        files = []
        if img[0] == "dj":
            for d in img[1:]:
                n = 1
                while f"{d}-{n}.webp" in index:
                    files.append(os.path.join(SRC, "dj", f"{d}-{n}.webp"))
                    n += 1
        print(f"=== {pid}  [{cats[cat][2][0]}]  brand={brand} {skel.BRANDS.get(brand, '')}")
        print(f"    working name: {working}   |   first-pass ar: {names.get(pid, '')}")
        print(f"    tags: {tags}   images: {len(files)}{'  SOLD OUT' if pid in skel.SOLD_OUT else ''}")
        print(f"    notes: {notes}")
        for d in img[1:]:
            if img[0] == "dj":
                p = dj[d]
                print(f"    dummyjson {d}: {p['title']} — {p['description']}")
        for i, fpath in enumerate(files):
            cells.append(f"{pid} img{i} ({os.path.basename(fpath)})|{fpath}")
    if sheet and cells:
        listing = sheet + ".txt"
        with open(listing, "w", encoding="utf-8") as f:
            f.write("\n".join(cells) + "\n")
        subprocess.run([sys.executable, "-I", os.path.join(HERE, "contact_sheet.py"), sheet, "220", "5",
                        "@" + listing], check=True)
    return 0


if __name__ == "__main__":
    sys.exit(main())
