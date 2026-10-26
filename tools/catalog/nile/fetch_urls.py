#!/usr/bin/env python3
"""Polite downloader for a short list of web images (Openverse / Wikimedia Commons originals).

    python3 -I tools/catalog/nile/fetch_urls.py OUT_DIR list.tsv
        list.tsv: "<name>\t<url>" per line; saved as OUT_DIR/<name> (extension from the URL).

Waits between requests and identifies itself (Wikimedia's User-Agent policy). Files are only decoded
with Pillow to report size/alpha, never executed. Existing files are kept.
"""

from __future__ import annotations

import os
import sys
import time
import urllib.parse
import urllib.request

from PIL import Image

Image.MAX_IMAGE_PIXELS = 400_000_000
UA = "NileStorefrontPreview/1.0 (offline demo catalogue build; low-volume one-off fetch of CC-licensed images) Python-urllib/3.10"


def main() -> int:
    out, listing = sys.argv[1], sys.argv[2]
    os.makedirs(out, exist_ok=True)
    with open(listing, encoding="utf-8") as f:
        rows = [line.rstrip("\n").split("\t") for line in f if line.strip() and not line.startswith("#")]
    for name, url in rows:
        ext = os.path.splitext(urllib.parse.urlparse(url).path)[1].lower() or ".img"
        dest = os.path.join(out, name + ext)
        if not os.path.exists(dest):
            req = urllib.request.Request(url, headers={"User-Agent": UA})
            try:
                with urllib.request.urlopen(req, timeout=120) as r:
                    data = r.read()
                if len(data) > 60_000_000:
                    raise ValueError("too large")
                with open(dest + ".part", "wb") as fh:
                    fh.write(data)
                os.replace(dest + ".part", dest)
            except Exception as exc:
                print(f"FAIL {name}: {exc}")
                time.sleep(3)
                continue
            time.sleep(float(os.environ.get("FETCH_DELAY", "1.5")))
        try:
            with Image.open(dest) as im:
                im.load()
                alpha = im.mode in ("RGBA", "LA", "P") and im.convert("RGBA").getchannel("A").getextrema()[0] < 250
                print(f"ok   {name}{ext}  {im.size}  {im.mode}  alpha={alpha}  {os.path.getsize(dest) // 1024} KB")
        except Exception as exc:
            print(f"BAD  {name}: {exc}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
