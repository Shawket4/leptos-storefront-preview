#!/usr/bin/env python3
"""Hero and promo photos for Souq: derive sources, then write the process.py job list.

    python3 -I tools/catalog/souq/web_images.py      # → data/media/_src/souq/jobs-web.jsonl
    python3 -I tools/media/process.py --batch data/media/_src/souq/jobs-web.jsonl

Art direction (lead, 2026-10-26): bright, appetising food photos with one clear subject, natural or
soft light, uncluttered, and a calm area where the copy card overlaps the lower part (phones) or the
lower inline-start corner (desktop). Souq defaults to Arabic (RTL), so the card sits lower-RIGHT on
desktop: two photos are mirrored so their main subject sits on the left. No Egypt postcard clichés.
"""

from __future__ import annotations

import json
import os

from PIL import Image, ImageOps

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(os.path.dirname(HERE)))
WEB = os.path.join(ROOT, "data", "media", "_src", "souq", "web")
OUT = os.path.join(ROOT, "data", "media", "_src", "souq", "jobs-web.jsonl")

BY_SA = "CC BY-SA 4.0 (https://creativecommons.org/licenses/by-sa/4.0/)"
CC0 = "CC0 1.0 (https://creativecommons.org/publicdomain/zero/1.0/)"

# Every web image: where it came from and how we use it. `title`/`author`/`landing` feed CREDITS.md.
PHOTOS = {
    "oranges": {
        "file": "oranges-3840.jpg", "flip": False,
        "title": "Oranges - whole-halved-segment", "author": "Ivar Leidus",
        "license": BY_SA, "landing": "https://commons.wikimedia.org/w/index.php?curid=98469414",
        "source": "https://upload.wikimedia.org/wikipedia/commons/e/e3/Oranges_-_whole-halved-segment.jpg",
        "changes": "cropped and resized",
    },
    "pomegranate": {
        "file": "pomegranate-3840.jpg", "flip": True,
        "title": "Pomegranate fruit - whole and piece with arils", "author": "Ivar Leidus",
        "license": BY_SA, "landing": "https://commons.wikimedia.org/w/index.php?curid=99248987",
        "source": "https://upload.wikimedia.org/wikipedia/commons/f/fa/Pomegranate_fruit_-_whole_and_piece_with_arils.jpg",
        "changes": "mirrored, cropped and resized",
    },
    "coffee": {
        "file": "coffee-beans.jpg", "flip": True,
        "title": "Coffee Bean with bags", "author": "Shusei Toda",
        "license": CC0, "landing": "https://wordpress.org/photos/photo/203627f31f/",
        "source": "https://pd.w.org/2022/05/203627f31f8770f03.61535278-2048x1366.jpg",
        "changes": "mirrored, cropped and resized",
    },
    "bread": {
        "file": "bread-basket.jpg", "flip": False,
        "title": "Bread Basket", "author": "JESHOOTS.com",
        "license": CC0, "landing": "https://stocksnap.io/photo/bread-basket-912FD6384D",
        "source": "https://cdn.stocksnap.io/img-thumbs/960w/912FD6384D.jpg",
        "changes": "cropped and resized (960 px preview via the Openverse API)",
    },
    "basket": {
        "file": "basket-groceries.jpg", "flip": False,
        "title": "Basket Groceries", "author": "leonie wise",
        "license": CC0, "landing": "https://stocksnap.io/photo/basket-groceries-CUPJ7AU7JD",
        "source": "https://cdn.stocksnap.io/img-thumbs/960w/CUPJ7AU7JD.jpg",
        "changes": "cropped and resized (960 px preview via the Openverse API)",
    },
}

ALT = {
    "oranges": ("Whole and halved oranges with a peeled segment on a pale grey surface",
                "برتقالة كاملة ونص برتقالة وفص مقشّر على سطح رمادي فاتح"),
    "pomegranate": ("A whole pomegranate beside an opened one full of ruby-red seeds",
                    "رمانة كاملة جنب رمانة مفتوحة مليانة حبات حمرا"),
    "coffee": ("Roasted coffee beans spilling across a warm wooden table",
               "حبوب بن محمّصة متناثرة على ترابيزة خشب"),
    "bread": ("Sliced bread in a basket lined with a red-checked cloth",
              "عيش متقطع في سبت مفروش بفوطة كاروهات حمرا"),
    "basket": ("A woven basket of fresh vegetables: tomatoes, peppers, asparagus and avocados",
               "سبت خوص مليان خضار طازة: طماطم وفلفل وهليون وأفوكادو"),
}

# key → (photo, kind, aspect, focus)
JOBS = [
    ("souq/hero-1", "oranges", "hero", "21:9", "0.5,0.2"),
    ("souq/hero-1-m", "oranges", "hero", "4:5", "0.35,0.5"),
    ("souq/hero-2", "pomegranate", "hero", "21:9", "0.5,0.42"),
    ("souq/hero-2-m", "pomegranate", "hero", "4:5", "0.38,0.5"),
    ("souq/hero-3", "coffee", "hero", "21:9", "0.5,0.0"),
    ("souq/hero-3-m", "coffee", "hero", "4:5", "0.25,0.5"),
    ("souq/promo-1", "bread", "tile", "4:3", "0.5,0.5"),
    ("souq/promo-2", "basket", "tile", "4:3", "0.5,0.5"),
]


def derived_source(name: str) -> str:
    p = PHOTOS[name]
    src = os.path.join(WEB, p["file"])
    if not p["flip"]:
        return src
    out = os.path.join(WEB, os.path.splitext(p["file"])[0] + "-mirrored.jpg")
    if not os.path.exists(out) or os.path.getmtime(out) < os.path.getmtime(src):
        im = Image.open(src)
        im.load()
        ImageOps.mirror(im.convert("RGB")).save(out, "JPEG", quality=95)
    return out


def main() -> None:
    with open(OUT, "w", encoding="utf-8") as f:
        for key, name, kind, aspect, focus in JOBS:
            p = PHOTOS[name]
            job = {
                "key": key, "kind": kind, "src": derived_source(name), "aspect": aspect, "focus": focus,
                "alt": {"en": ALT[name][0], "ar": ALT[name][1]},
                "source": p["source"], "license": p["license"],
                "credit": f"“{p['title']}” by {p['author']} ({p['landing']}), {p['changes']}",
            }
            f.write(json.dumps(job, ensure_ascii=False) + "\n")
    print(f"{len(JOBS)} jobs → {OUT}")


if __name__ == "__main__":
    main()
