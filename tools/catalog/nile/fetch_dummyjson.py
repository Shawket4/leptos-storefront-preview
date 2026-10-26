#!/usr/bin/env python3
"""Download every DummyJSON product image (except vehicle/motorcycle) for the Nile catalogue.

    python3 -I tools/catalog/nile/fetch_dummyjson.py data/media/_src/nile/dummyjson-products.json \
        data/media/_src/nile/dj

Writes <out>/<djId>-<n>.webp (n from 1, in the API's order) and <out>/index.json with the size and
alpha of each file. Files already on disk are kept (re-runs only fetch what is missing). The
downloads are treated as untrusted data: only decoded with Pillow, never executed.
"""

from __future__ import annotations

import json
import os
import sys
import time
import urllib.request

from PIL import Image

SKIP = {"vehicle", "motorcycle"}
UA = "nile-storefront-preview/1.0 (demo catalogue build)"


def fetch(url: str, dest: str) -> None:
    req = urllib.request.Request(url, headers={"User-Agent": UA})
    with urllib.request.urlopen(req, timeout=60) as r:
        data = r.read()
    if len(data) > 15_000_000:
        raise ValueError("file too large")
    tmp = dest + ".part"
    with open(tmp, "wb") as f:
        f.write(data)
    os.replace(tmp, dest)


def main() -> int:
    src, out = sys.argv[1], sys.argv[2]
    os.makedirs(out, exist_ok=True)
    with open(src, encoding="utf-8") as f:
        products = json.load(f)["products"]
    index = {}
    for p in products:
        if p["category"] in SKIP:
            continue
        for n, url in enumerate(p["images"], start=1):
            name = f"{p['id']}-{n}.webp"
            dest = os.path.join(out, name)
            if not os.path.exists(dest):
                for attempt in range(3):
                    try:
                        fetch(url, dest)
                        break
                    except Exception as exc:  # retry politely
                        print(f"retry {name}: {exc}", file=sys.stderr)
                        time.sleep(2 + attempt * 3)
                time.sleep(0.12)
            try:
                with Image.open(dest) as im:
                    im.load()
                    alpha = im.mode in ("RGBA", "LA") and im.getchannel("A").getextrema()[0] < 250
                    index[name] = {"url": url, "size": im.size, "mode": im.mode, "alpha": alpha,
                                   "bytes": os.path.getsize(dest)}
            except Exception as exc:
                print(f"BAD {name}: {exc}", file=sys.stderr)
    with open(os.path.join(out, "index.json"), "w", encoding="utf-8") as f:
        json.dump(index, f, indent=1)
    print(f"{len(index)} images ok")
    return 0


if __name__ == "__main__":
    sys.exit(main())
