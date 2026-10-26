#!/usr/bin/env python3
"""Build data/catalog/souq.json (and the media job list) from the data modules next to this file.

    python3 -I tools/catalog/souq/build.py            # write souq.json + _src/souq/jobs-products.jsonl
    python3 -I tools/catalog/souq/build.py --all-jobs # job list includes images already processed

Product ids are explicit in the data modules (stable across passes); each category owns a block
of 50 ids. Media keys are souq/p<id>-<n>. Then process the job list with tools/media/process.py.
"""

from __future__ import annotations

import importlib
import json
import os
import random
import re
import sys
import urllib.parse

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(os.path.dirname(HERE)))
sys.dont_write_bytecode = True  # keep tools/ free of __pycache__
sys.path.insert(0, HERE)  # our own data modules (trusted, written by hand)

from common import L, piastres  # noqa: E402
import d_overrides as OV  # noqa: E402

SRC = os.path.join(ROOT, "data", "media", "_src", "souq")
MANIFEST = os.path.join(ROOT, "data", "media", "souq", "manifest.json")
OUT = os.path.join(ROOT, "data", "catalog", "souq.json")

MODULES = [
    "d_produce", "d_bakery", "d_dairy", "d_cheese", "d_meat", "d_fish", "d_grains", "d_spices",
    "d_oils", "d_sweets", "d_coffee", "d_drinks", "d_nuts", "d_pantry",
]

LIC_MEALDB = "TheMealDB ingredient image — placeholder, replace with tenant photography"
LIC_DUMMY = "DummyJSON demo asset — placeholder, replace with tenant photography"

STOCK_RANGE = {  # by category id: typical units on hand in the dark store
    10: (25, 160), 20: (10, 60), 30: (12, 90), 40: (6, 40), 50: (4, 35), 60: (3, 24),
    70: (20, 200), 80: (15, 120), 90: (10, 90), 100: (8, 70), 110: (10, 80), 120: (24, 240),
    130: (10, 90), 140: (15, 150),
}


def slugify(name: str) -> str:
    return re.sub(r"[^a-z0-9]+", "-", name.lower()).strip("-")


def dummy_index() -> dict:
    with open(os.path.join(SRC, "meta", "dummy_groceries.json"), encoding="utf-8") as f:
        data = json.load(f)
    out = {}
    for p in data["products"]:
        for i, url in enumerate(p.get("images", []), 1):
            out[f"{slugify(p['title'])}-{i}"] = url
    return out


def mealdb_urls() -> dict:
    with open(os.path.join(SRC, "meta", "mealdb_ingredients.json"), encoding="utf-8") as f:
        data = json.load(f)
    return {m["strIngredient"]: m.get("strThumb") for m in data["meals"]}


def resolve_image(ref, dummies: dict, thumbs: dict) -> dict:
    """ref 'm:<TheMealDB ingredient>' or 'd:<dummyjson file stem>' → src path + credits."""
    kind, name = ref.split(":", 1)
    if kind == "m":
        src = os.path.join(SRC, "mealdb", slugify(name) + ".png")
        url = "https://www.themealdb.com/images/ingredients/" + urllib.parse.quote(name) + ".png"
        return {"src": src, "source": url, "license": LIC_MEALDB,
                "credit": f"TheMealDB (themealdb.com), ingredient “{name}”", "title": name}
    if kind == "d":
        src = os.path.join(SRC, "dummy", name + ".webp")
        return {"src": src, "source": dummies[name], "license": LIC_DUMMY,
                "credit": "DummyJSON (dummyjson.com), groceries category", "title": name}
    raise SystemExit(f"bad image ref {ref!r}")


def build_product(p: dict, cat_id: int, dummies: dict, thumbs: dict, jobs: list) -> dict:
    pid = p["id"]
    out = {"id": pid, "slug": p["slug"], "category": cat_id}
    if p.get("brand"):
        out["brand"] = p["brand"]
    out["name"] = L(*p["name"])
    out["subtitle"] = L(*p["sub"])
    out["description"] = L(*p["desc"])
    out["highlights"] = [L(*h) for h in p.get("hl", [])]
    out["specs"] = p.get("specs", [])

    keys = []
    for n, img in enumerate(p["img"], 1):
        ref, caption, extra = img, None, {}
        if isinstance(img, tuple):
            ref = img[0]
            caption = img[1] if len(img) > 1 else None
            extra = img[2] if len(img) > 2 else {}
        key = f"souq/p{pid}-{n}"
        keys.append(key)
        info = resolve_image(ref, dummies, thumbs)
        alt_en, alt_ar = p["name"][0], p["name"][1]
        if caption:
            alt_en = f"{alt_en} — {caption['en']}"
            alt_ar = f"{alt_ar} — {caption['ar']}"
        job = {"key": key, "kind": "product", "src": info["src"], "alt": {"en": alt_en, "ar": alt_ar},
               "source": info["source"], "license": info["license"], "credit": info["credit"]}
        job.update(extra)
        jobs.append(job)
    out["images"] = keys
    out["rating"], out["reviews"] = p["r"]
    tags = p.get("tags", "").split()
    tags += [t for t in OV.TAG_ADD.get(pid, "").split() if t not in tags]
    if pid in OV.EXPRESS_ADD and "express" not in tags:
        tags.append("express")
    tags = [t for t in tags if t not in OV.TAG_REMOVE.get(pid, "").split()]
    order = ["featured", "deal", "new", "bestseller", "express"]
    out["tags"] = sorted(tags, key=order.index)

    options = p.get("options", [])
    out["options"] = options
    variants = []
    lo, hi = STOCK_RANGE[cat_id]
    for vid, v in enumerate(p["v"], 1):
        opt = v["opt"]
        opts = [] if opt is None else (list(opt) if isinstance(opt, tuple) else [opt])
        if len(opts) != len(options):
            raise SystemExit(f"product {pid}: variant {vid} opts {opts} vs {len(options)} axes")
        code = v.get("sku") or "-".join(o.upper() for o in opts)
        sku = f"SOQ-{pid}" + (f"-{code}" if code else "")
        rnd = random.Random(pid * 100 + vid)
        stock = v["stock"] if v["stock"] is not None else rnd.randint(lo, hi)
        if p.get("soldout") or pid in OV.SOLDOUT or (pid, vid) in OV.OOS:
            stock = 0
        var = {"id": vid, "sku": sku, "opts": opts, "price": piastres(v["price"])}
        was = v["was"] if v["was"] is not None else OV.WAS.get((pid, vid))
        if was is not None:
            var["compare_at"] = piastres(was)
        var["stock"] = stock
        if v["img"] is not None:
            var["image"] = v["img"]
        variants.append(var)
    out["variants"] = variants
    if p.get("mods") and pid in OV.MODS_KEEP:
        out["modifier_groups"] = p["mods"]
    return out


def main() -> int:
    all_jobs = "--all-jobs" in sys.argv[1:]
    cats_mod = importlib.import_module("d_categories")
    brands_mod = importlib.import_module("d_brands")
    dummies, thumbs = dummy_index(), mealdb_urls()
    cats = {c["id"]: c for c in cats_mod.CATEGORIES}
    products, jobs, seen = [], [], set()
    for mod_name in MODULES:
        try:
            mod = importlib.import_module(mod_name)
        except ModuleNotFoundError:
            continue
        cat_id = mod.CATEGORY
        for p in mod.PRODUCTS:
            # ids are stable once published; a product keeps its id if it later moves category
            if not (2001 <= p["id"] <= 2699):
                raise SystemExit(f"product {p['id']} outside the souq id range 2001–2699")
            if p["id"] in seen:
                raise SystemExit(f"duplicate id {p['id']}")
            seen.add(p["id"])
            products.append(build_product(p, cat_id, dummies, thumbs, jobs))
    products.sort(key=lambda p: p["id"])
    used_brands = {p.get("brand") for p in products if p.get("brand")}
    brands = [b for b in brands_mod.BRANDS if b["id"] in used_brands]
    missing = used_brands - {b["id"] for b in brands}
    if missing:
        raise SystemExit(f"brands not defined: {sorted(missing)}")
    used_cats = {p["category"] for p in products}
    categories = []
    for c in cats_mod.CATEGORIES:
        if c["id"] not in used_cats:
            continue
        c = dict(c)
        if c.get("image_of"):
            c["image"] = f"souq/p{c.pop('image_of')}-1"
        categories.append(c)
    catalog = {"tenant": "souq", "brands": brands, "categories": categories, "products": products}
    manifest = {}
    if os.path.exists(MANIFEST):
        with open(MANIFEST, encoding="utf-8") as f:
            manifest = json.load(f)
    # Publish only a catalogue whose images all exist, so data/catalog/souq.json on disk always
    # validates (page agents read it live). Otherwise stage it and publish after processing.
    unprocessed = [j["key"] for j in jobs if j["key"] not in manifest]
    out = OUT if not unprocessed else os.path.join(SRC, "souq.pending.json")
    os.makedirs(os.path.dirname(out), exist_ok=True)
    tmp = out + ".tmp"
    with open(tmp, "w", encoding="utf-8") as f:
        json.dump(catalog, f, ensure_ascii=False, indent=1)
        f.write("\n")
    os.replace(tmp, out)
    if unprocessed:
        print(f"STAGED (not published): {len(unprocessed)} images not processed yet → {out}")
    else:
        print(f"published {OUT}")

    # media jobs: only images that are new or whose source changed (unless --all-jobs)
    todo = [j for j in jobs if all_jobs or manifest.get(j["key"], {}).get("source") != j["source"]
            or manifest.get(j["key"], {}).get("alt") != j["alt"]]
    missing_src = [j["src"] for j in todo if not os.path.exists(j["src"])]
    with open(os.path.join(SRC, "jobs-products.jsonl"), "w", encoding="utf-8") as f:
        for j in todo:
            if os.path.exists(j["src"]):
                f.write(json.dumps(j, ensure_ascii=False) + "\n")
    with open(os.path.join(SRC, "jobs-all-products.json"), "w", encoding="utf-8") as f:
        json.dump(jobs, f, ensure_ascii=False, indent=0)
    nvar = sum(len(p["variants"]) for p in products)
    print(f"souq.json: {len(categories)} categories, {len(brands)} brands, {len(products)} products, "
          f"{nvar} variants, {len(jobs)} images ({len(todo)} to process)")
    for s in missing_src:
        print("MISSING SOURCE:", s)
    return 1 if missing_src else 0


if __name__ == "__main__":
    sys.exit(main())
