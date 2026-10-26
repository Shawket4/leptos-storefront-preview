#!/usr/bin/env python3
"""Download source images for the Souq catalogue into data/media/_src/souq/ (git-ignored).

Polite, sequential, skip-if-present. Downloads are treated as data only (never executed).

    # TheMealDB ingredient images, one ingredient name per line in LIST
    python3 -I tools/catalog/souq/fetch.py mealdb LIST OUT_DIR [--small]
    # every image of the DummyJSON groceries category (JSON already saved by curl)
    python3 -I tools/catalog/souq/fetch.py dummy GROCERIES_JSON OUT_DIR
    # arbitrary URLs: lines of "<filename>\t<url>"
    python3 -I tools/catalog/souq/fetch.py urls LIST OUT_DIR
"""

from __future__ import annotations

import json
import os
import re
import sys
import time
import urllib.parse
import urllib.request

UA = "souq-catalogue-builder/1.0 (demo storefront; contact: local)"
MAX_BYTES = 25 * 1024 * 1024


def slug(name: str) -> str:
    return re.sub(r"[^a-z0-9]+", "-", name.lower()).strip("-")


def get(url: str, dest: str, delay: float = 0.35) -> str:
    if os.path.exists(dest) and os.path.getsize(dest) > 0:
        return "skip"
    req = urllib.request.Request(url, headers={"User-Agent": UA})
    try:
        with urllib.request.urlopen(req, timeout=40) as r:
            ctype = r.headers.get("Content-Type", "")
            if not ctype.startswith("image/"):
                return f"not-image ({ctype})"
            data = r.read(MAX_BYTES + 1)
            if len(data) > MAX_BYTES:
                return "too-big"
    except Exception as exc:  # noqa: BLE001 - report and continue
        return f"error {exc}"
    finally:
        time.sleep(delay)
    tmp = dest + ".part"
    with open(tmp, "wb") as f:
        f.write(data)
    os.replace(tmp, dest)
    return f"ok {len(data) // 1024} KB"


def main() -> int:
    if len(sys.argv) < 4:
        print(__doc__)
        return 2
    mode, src, out = sys.argv[1], sys.argv[2], sys.argv[3]
    small = "--small" in sys.argv[4:]
    os.makedirs(out, exist_ok=True)
    jobs: list[tuple[str, str]] = []
    if mode == "mealdb":
        with open(src, encoding="utf-8") as f:
            names = [ln.strip() for ln in f if ln.strip() and not ln.startswith("#")]
        for n in dict.fromkeys(names):
            suffix = "-Small" if small else ""
            url = "https://www.themealdb.com/images/ingredients/" + urllib.parse.quote(n) + suffix + ".png"
            jobs.append((url, os.path.join(out, slug(n) + ".png")))
    elif mode == "dummy":
        with open(src, encoding="utf-8") as f:
            data = json.load(f)
        for p in data["products"]:
            for i, url in enumerate(p.get("images", []), 1):
                jobs.append((url, os.path.join(out, f"{slug(p['title'])}-{i}.webp")))
    elif mode == "urls":
        with open(src, encoding="utf-8") as f:
            for ln in f:
                if ln.strip() and not ln.startswith("#"):
                    name, url = ln.rstrip("\n").split("\t", 1)
                    jobs.append((url.strip(), os.path.join(out, os.path.basename(name.strip()))))
    else:
        print(__doc__)
        return 2
    bad = 0
    for url, dest in jobs:
        res = get(url, dest)
        if not (res.startswith("ok") or res == "skip"):
            bad += 1
        print(f"{res:>14}  {os.path.basename(dest)}  <- {url}")
    print(f"{len(jobs) - bad}/{len(jobs)} available")
    return 0


if __name__ == "__main__":
    sys.exit(main())
