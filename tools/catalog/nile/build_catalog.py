#!/usr/bin/env python3
"""Assemble data/catalog/nile.json and the image job list from the skeleton and the content parts.

    python3 -I tools/catalog/nile/build_catalog.py                 # write nile.json, print a summary
    python3 -I tools/catalog/nile/build_catalog.py --jobs FILE      # also write process.py jobs (JSONL)
    python3 -I tools/catalog/nile/build_catalog.py --dry-run        # summary only

Inputs (all in tools/catalog/nile/):
    skeleton.py        categories, brands, product ids, image sources, tags, price anchors (global)
    names_ar.py        rough Arabic names used until a part supplies the real copy (first pass)
    parts/*.py         content per product: PRODUCTS = [dict(id=…, name=(en, ar), …)]; a part may also
                       extend the skeleton with SKELETON = {pid: (cat, img, brand, name, tags, notes)}
                       and BRANDS = {id: (en, ar)}
    commons_credits.py licence metadata for Wikimedia Commons originals

Slugs are derived from the skeleton's working English name and never from part copy, so product
URLs stay stable between passes. Stock, ratings fallback and the out-of-stock picks are
deterministic (hash-based), so re-runs don't reshuffle them.
"""

from __future__ import annotations

import glob
import hashlib
import importlib.util
import json
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(os.path.dirname(HERE)))
SRC = os.path.join(ROOT, "data", "media", "_src", "nile")
OUT = os.path.join(ROOT, "data", "catalog", "nile.json")
DJ_LICENSE = "DummyJSON demo asset — placeholder, replace with tenant photography"
OOS_TARGET = 0.08


def load(path: str, name: str):
    spec = importlib.util.spec_from_file_location(name, path)
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def h(*parts) -> int:
    return int(hashlib.sha256("|".join(map(str, parts)).encode()).hexdigest()[:12], 16)


def L(pair) -> dict:
    if isinstance(pair, dict):
        return {"en": pair["en"], "ar": pair["ar"]}
    en, ar = pair
    return {"en": en, "ar": ar}


def slugify(s: str) -> str:
    s = s.lower().replace("&", " and ").replace("'", "").replace("’", "").replace('"', "")
    s = s.replace("ş", "s").replace("ç", "c").replace("é", "e")
    s = re.sub(r"[^a-z0-9]+", "-", s).strip("-")
    return re.sub(r"-+", "-", s)


def egp(n) -> int:
    """Part prices are whole EGP; the catalogue stores piastres."""
    if isinstance(n, float) and not n.is_integer():
        return round(n * 100)
    return int(n) * 100


COLOR_CODES = {
    "black": "BLK", "white": "WHT", "silver": "SLV", "grey": "GRY", "gray": "GRY", "gold": "GLD",
    "red": "RED", "blue": "BLU", "green": "GRN", "pink": "PNK", "purple": "PUR", "yellow": "YLW",
    "brown": "BRN", "navy": "NVY", "beige": "BGE", "orange": "ORG", "space-grey": "SGR",
    "starlight": "STL", "midnight": "MID",
}


def code_for(value: dict) -> str:
    if value.get("code"):
        return str(value["code"]).upper()
    vid = str(value["id"]).lower()
    if vid in COLOR_CODES:
        return COLOR_CODES[vid]
    words = [w for w in re.split(r"[^a-z0-9]+", vid) if w]
    if len(words) > 1 and not any(w.isdigit() for w in words):
        return "".join(w[0] for w in words).upper()[:4]
    s = "".join(words).upper()
    if s.isalpha():
        return s[:3]
    return s[:6]


def stock_for(pid: int, vid: int, price_egp: float, cat: int) -> int:
    r = h("stock", pid, vid)
    if cat == 22:
        lo, hi = 20, 300
    elif price_egp >= 300_000:
        lo, hi = 1, 2
    elif price_egp >= 50_000:
        lo, hi = 1, 6
    elif price_egp >= 10_000:
        lo, hi = 2, 18
    elif price_egp >= 1_000:
        lo, hi = 4, 45
    else:
        lo, hi = 12, 240
    return lo + r % (hi - lo + 1)


def parse_anchor(notes: str):
    m = re.search(r"EGP ([\d,]+)", notes)
    price = int(m.group(1).replace(",", "")) if m else 999
    c = re.search(r"compare ([\d,]+)", notes)
    compare = int(c.group(1).replace(",", "")) if c else None
    return price, compare


def round9(x: float) -> int:
    """Round up to a price ending in 9 (or 99 above EGP 1,000)."""
    x = int(x)
    if x >= 1000:
        return (x // 100) * 100 + 99
    return (x // 10) * 10 + 9


def main() -> int:
    args = sys.argv[1:]
    jobs_path = args[args.index("--jobs") + 1] if "--jobs" in args else None
    dry = "--dry-run" in args

    skel = load(os.path.join(HERE, "skeleton.py"), "nile_skeleton")
    credits = load(os.path.join(HERE, "commons_credits.py"), "nile_credits")
    names_ar = {}
    if os.path.exists(os.path.join(HERE, "names_ar.py")):
        names_ar = load(os.path.join(HERE, "names_ar.py"), "nile_names_ar").NAMES

    skeleton = dict(skel.PRODUCTS)
    brands = dict(skel.BRANDS)
    content: dict[int, dict] = {}
    for path in sorted(glob.glob(os.path.join(HERE, "parts", "*.py"))):
        mod = load(path, "part_" + os.path.basename(path)[:-3].replace("-", "_"))
        skeleton.update(getattr(mod, "SKELETON", {}))
        brands.update(getattr(mod, "BRANDS", {}))
        for p in getattr(mod, "PRODUCTS", []):
            if p["id"] in content:
                raise SystemExit(f"product {p['id']} defined twice (second in {path})")
            content[p["id"]] = p

    with open(os.path.join(SRC, "dj", "index.json"), encoding="utf-8") as f:
        dj_index = json.load(f)
    dj_imgs: dict[int, list[tuple[str, str]]] = {}
    for name in sorted(dj_index, key=lambda n: (int(n.split("-")[0]), int(n.split("-")[1].split(".")[0]))):
        djid = int(name.split("-")[0])
        dj_imgs.setdefault(djid, []).append((os.path.join(SRC, "dj", name), dj_index[name]["url"]))

    cats = {c[0]: c for c in skel.CATEGORIES}
    products = []
    jobs = []
    used_brands = set()
    slugs = set()

    for pid in sorted(skeleton):
        cat, img, brand, working_name, tags, notes = skeleton[pid]
        c = content.get(pid, {})
        slug = slugify(working_name)
        if slug in slugs:
            raise SystemExit(f"duplicate slug {slug}")
        slugs.add(slug)

        # ---- images
        sources = []  # (src path, source url, license, credit)
        if img[0] == "dj":
            for djid in img[1:]:
                for path, url in dj_imgs[djid]:
                    sources.append((path, url, DJ_LICENSE, "dummyjson.com"))
        elif img[0] == "commons":
            for stem in img[1:]:
                meta = credits.FILES[stem]
                path = glob.glob(os.path.join(SRC, "commons", stem + ".*"))
                path = [p for p in path if not p.endswith(".part")]
                if not path:
                    raise SystemExit(f"{pid}: missing commons original {stem}")
                lic = f"{credits.license_label(meta)} ({credits.license_url(meta)})"
                sources.append((path[0], meta["url"], lic, f"{meta['creator']} / Wikimedia Commons"))
        else:
            raise SystemExit(f"{pid}: unknown image source {img[0]}")
        keys = [f"nile/p{pid}-{n}" for n in range(1, len(sources) + 1)]

        name = c.get("name") or (working_name, names_ar.get(pid) or working_name)
        name = L(name)
        alt = L(c["alt"]) if c.get("alt") else name
        views = c.get("views") or []
        for n, (key, (path, url, lic, credit)) in enumerate(zip(keys, sources)):
            if n < len(views) and views[n]:
                v = L(views[n])
                a = {"en": f"{alt['en']}, {v['en']}", "ar": f"{alt['ar']}، {v['ar']}"}
            elif n == 0:
                a = dict(alt)
            else:
                a = {"en": f"{alt['en']}, photo {n + 1}", "ar": f"{alt['ar']}، صورة {n + 1}"}
            job = {"key": key, "kind": "product", "src": path, "alt": a, "source": url,
                   "license": lic, "credit": credit}
            if c.get("fill"):
                job["fill"] = str(c["fill"])
            jobs.append(job)

        # ---- copy
        cat_row = cats[cat]
        brand_l = L(brands[brand]) if brand else None
        if brand:
            used_brands.add(brand)
        subtitle = L(c["subtitle"]) if c.get("subtitle") else (
            {"en": f"{brand_l['en']} · {cat_row[2][0]}", "ar": f"{brand_l['ar']} · {cat_row[2][1]}"}
            if brand_l else {"en": cat_row[2][0], "ar": cat_row[2][1]})
        description = L(c["description"]) if c.get("description") else {
            "en": f"{name['en']}. Sold and shipped by Nile: pay cash on delivery, by card or in instalments, "
                  f"with easy 14-day returns.",
            "ar": f"{name['ar']}. بيتباع ويتشحن من نايل: ادفع كاش عند الاستلام أو بالكارت أو بالتقسيط، "
                  f"واسترجاع سهل خلال 14 يوم.",
        }
        highlights = [L(x) for x in c.get("highlights", [])] or [
            {"en": "Genuine product with warranty", "ar": "منتج أصلي بضمان"},
            {"en": "Delivery to every governorate in Egypt", "ar": "توصيل لكل محافظات مصر"},
        ]
        specs = [{"k": L(k), "v": L(v)} for k, v in c.get("specs", [])] or (
            [{"k": {"en": "Brand", "ar": "الماركة"}, "v": brand_l}] if brand_l else []) + [
            {"k": {"en": "Category", "ar": "القسم"}, "v": {"en": cat_row[2][0], "ar": cat_row[2][1]}}]

        # ---- options & variants
        options = []
        for o in c.get("options", []):
            opt = {"id": o["id"], "name": L(o["name"]), "kind": o["kind"], "values": []}
            for v in o["values"]:
                val = {"id": v["id"], "label": L(v["label"])}
                if o["kind"] == "swatch":
                    val["swatch"] = v["swatch"]
                opt["values"].append(val)
            options.append(opt)
        value_by_id = [{v["id"]: v for v in o["values"]} for o in c.get("options", [])]
        raw_variants = c.get("variants")
        if not raw_variants:
            price, compare = parse_anchor(notes)
            if "deal" in tags and not compare:
                compare = round9(price * 1.22)
            raw_variants = [dict(opts=[], price=price, compare_at=compare, image=0)]
        variants = []
        for i, rv in enumerate(raw_variants, start=1):
            opts = list(rv.get("opts", []))
            codes = [code_for(value_by_id[ax][val]) for ax, val in enumerate(opts)]
            sku = "-".join(["NIL", str(pid)] + codes)
            var = {"id": i, "sku": sku, "opts": opts, "price": egp(rv["price"])}
            if rv.get("compare_at"):
                var["compare_at"] = egp(rv["compare_at"])
            var["stock"] = None  # filled below
            if rv.get("image") is not None:
                var["image"] = rv["image"]
            var["_egp"] = rv["price"]
            var["_stock"] = rv.get("stock")
            variants.append(var)

        groups = []
        for g in c.get("modifier_groups", []):
            groups.append({
                "id": g["id"], "name": L(g["name"]), "selection": g.get("selection", "multi"),
                "min": g.get("min", 0), "max": g.get("max"), "required": g.get("required", False),
                "options": [{"id": o["id"], "name": L(o["name"]), "price": egp(o["price"])} for o in g["options"]],
            })
            if groups[-1]["max"] is None:
                del groups[-1]["max"]

        rating = c.get("rating")
        reviews = c.get("reviews")
        if rating is None:
            rating = round(3.9 + (h("rating", pid) % 10) / 10, 1)
        if reviews is None:
            base = {True: 900, False: 40}["bestseller" in tags]
            reviews = base + h("reviews", pid) % (base * 3 + 60)
            if "new" in tags:
                reviews = 2 + h("reviews", pid) % 40

        products.append({
            "id": pid, "slug": slug, "category": cat, **({"brand": brand} if brand else {}),
            "name": name, "subtitle": subtitle, "description": description, "highlights": highlights,
            "specs": specs, "images": keys, "rating": rating, "reviews": reviews, "tags": tags,
            "options": options, "variants": variants, "modifier_groups": groups,
        })

    # ---- stock: sold-out products, then ~8 % of all variants at zero overall
    sold_out = set(skel.SOLD_OUT)
    for p in products:
        for v in p["variants"]:
            if p["id"] in sold_out:
                v["stock"] = 0
            elif v["_stock"] is not None:
                v["stock"] = int(v["_stock"])
            else:
                v["stock"] = stock_for(p["id"], v["id"], v["_egp"], p["category"])
    total = sum(len(p["variants"]) for p in products)
    zero = sum(1 for p in products for v in p["variants"] if v["stock"] == 0)
    need = max(0, round(OOS_TARGET * total) - zero)
    cands = sorted(
        ((h("oos", p["id"], v["id"]), p, v) for p in products if p["id"] not in sold_out and len(p["variants"]) > 1
         for v in p["variants"] if v["_stock"] is None and v["stock"] > 0),
        key=lambda t: t[0])
    for _, p, v in cands:
        if need <= 0:
            break
        if sum(1 for x in p["variants"] if x["stock"] > 0) <= 1:
            continue
        v["stock"] = 0
        need -= 1
    for p in products:
        for v in p["variants"]:
            del v["_egp"], v["_stock"]
            v.pop("image", None) if v.get("image") is None else None

    categories = []
    for cid, slug, nm, blurb, icon, order, rep in skel.CATEGORIES:
        categories.append({"id": cid, "slug": slug, "name": L(nm), "blurb": L(blurb), "icon": icon,
                           "image": f"nile/p{rep}-1" if rep in skeleton else None, "display_order": order})
        if categories[-1]["image"] is None:
            del categories[-1]["image"]

    catalog = {
        "tenant": "nile",
        "brands": [{"id": b, "name": L(brands[b])} for b in sorted(used_brands)],
        "categories": categories,
        "products": products,
    }

    zero = sum(1 for p in products for v in p["variants"] if v["stock"] == 0)
    per_cat = {}
    for p in products:
        per_cat[p["category"]] = per_cat.get(p["category"], 0) + 1
    print(f"products {len(products)}  variants {total}  zero-stock {zero} ({zero * 100 / total:.1f}%)  "
          f"images {len(jobs)}  with content {sum(1 for p in products if p['id'] in content)}")
    print("per category: " + ", ".join(f"{k}:{v}" for k, v in sorted(per_cat.items())))
    if not dry:
        os.makedirs(os.path.dirname(OUT), exist_ok=True)
        tmp = OUT + ".tmp"
        with open(tmp, "w", encoding="utf-8") as f:
            json.dump(catalog, f, ensure_ascii=False, indent=1)
            f.write("\n")
        os.replace(tmp, OUT)
        print("wrote", os.path.relpath(OUT, ROOT))
    if jobs_path:
        with open(jobs_path, "w", encoding="utf-8") as f:
            for j in jobs:
                f.write(json.dumps(j, ensure_ascii=False) + "\n")
        print("wrote", len(jobs), "jobs to", jobs_path)
    return 0


if __name__ == "__main__":
    sys.exit(main())
