#!/usr/bin/env python3
"""Validate data/catalog/<tenant>.json against data/SCHEMA.md and the tenant's media manifest.

    python3 -I tools/catalog/validate.py nile
"""

from __future__ import annotations

import json
import os
import re
import sys
from collections import Counter

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
SLUG = re.compile(r"^[a-z0-9]+(?:-[a-z0-9]+)*$")
TAGS = {"featured", "deal", "new", "bestseller", "express"}
ICONS = set("""smartphone laptop tablet headphones watch tv camera gamepad-2 speaker monitor keyboard mouse
battery-charging cable shirt footprints gem glasses sparkles spray-can brush sofa lamp
cooking-pot refrigerator washing-machine baby dumbbell bike book-open gift apple carrot milk egg
beef fish drumstick croissant coffee cup-soda candy wheat leaf flame cookie soup ice-cream-cone
citrus cherry grape salad nut bean shopping-basket store package tag sun-snow heart-pulse pill
paw-print""".split())
ARABIC = re.compile(r"[؀-ۿ]")


def load_manifest(tenant: str) -> dict:
    keys = {}
    for t in (tenant, "brand"):
        path = os.path.join(ROOT, "data", "media", t, "manifest.json")
        if os.path.exists(path):
            with open(path, encoding="utf-8") as f:
                keys.update(json.load(f))
    return keys


def main() -> int:
    if len(sys.argv) != 2:
        print(__doc__)
        return 2
    tenant = sys.argv[1]
    path = os.path.join(ROOT, "data", "catalog", f"{tenant}.json")
    with open(path, encoding="utf-8") as f:
        cat = json.load(f)
    media = load_manifest(tenant)
    errors: list[str] = []
    warn: list[str] = []

    def err(msg: str):
        errors.append(msg)

    def l10n(obj, where: str, required=True):
        if not isinstance(obj, dict) or set(obj) != {"en", "ar"}:
            if required or obj is not None:
                err(f"{where}: must be {{'en','ar'}}, got {obj!r}"[:200])
            return
        if required and (not obj["en"].strip() or not obj["ar"].strip()):
            err(f"{where}: empty en/ar")
        if obj.get("ar", "").strip() and not ARABIC.search(obj["ar"]) and required:
            warn.append(f"{where}: 'ar' has no Arabic letters: {obj['ar']!r}")

    def media_key(key: str, where: str):
        if key not in media:
            err(f"{where}: media key {key!r} not in manifest")

    if cat.get("tenant") != tenant:
        err(f"tenant field is {cat.get('tenant')!r}, expected {tenant!r}")

    brands = {}
    for b in cat.get("brands", []):
        if b["id"] in brands:
            err(f"brand {b['id']} duplicated")
        brands[b["id"]] = b
        l10n(b.get("name"), f"brand {b['id']}.name")

    cats = {}
    cat_slugs = set()
    for c in cat.get("categories", []):
        cid = c.get("id")
        if cid in cats:
            err(f"category id {cid} duplicated")
        cats[cid] = c
        if not SLUG.match(c.get("slug", "")) or c["slug"] in cat_slugs:
            err(f"category {cid}: bad or duplicate slug {c.get('slug')!r}")
        cat_slugs.add(c.get("slug"))
        l10n(c.get("name"), f"category {cid}.name")
        if "blurb" in c:
            l10n(c["blurb"], f"category {cid}.blurb")
        if c.get("icon") and c["icon"] not in ICONS:
            err(f"category {cid}: icon {c['icon']!r} not in the icon list")
        if c.get("image"):
            media_key(c["image"], f"category {cid}.image")
        if not isinstance(c.get("display_order"), int):
            err(f"category {cid}: display_order must be int")

    pids = set()
    pslugs = set()
    per_cat = Counter()
    tag_count = Counter()
    variants_total = 0
    oos_variants = 0
    for p in cat.get("products", []):
        pid = p.get("id")
        where = f"product {pid}"
        if pid in pids:
            err(f"{where}: id duplicated")
        pids.add(pid)
        if not SLUG.match(p.get("slug", "")) or p["slug"] in pslugs:
            err(f"{where}: bad or duplicate slug {p.get('slug')!r}")
        pslugs.add(p.get("slug"))
        if p.get("category") not in cats:
            err(f"{where}: unknown category {p.get('category')}")
        per_cat[p.get("category")] += 1
        if p.get("brand") is not None and p["brand"] not in brands:
            err(f"{where}: unknown brand {p['brand']!r}")
        l10n(p.get("name"), f"{where}.name")
        l10n(p.get("description"), f"{where}.description")
        if "subtitle" in p:
            l10n(p["subtitle"], f"{where}.subtitle")
        for i, h in enumerate(p.get("highlights", [])):
            l10n(h, f"{where}.highlights[{i}]")
        for i, s in enumerate(p.get("specs", [])):
            l10n(s.get("k"), f"{where}.specs[{i}].k")
            l10n(s.get("v"), f"{where}.specs[{i}].v")
        images = p.get("images", [])
        if not images:
            err(f"{where}: no images")
        for k in images:
            media_key(k, f"{where}.images")
        r = p.get("rating", 0)
        if not (0 <= r <= 5):
            err(f"{where}: rating {r} out of range")
        for t in p.get("tags", []):
            if t not in TAGS:
                err(f"{where}: unknown tag {t!r}")
            tag_count[t] += 1

        options = p.get("options", [])
        if len(options) > 2:
            err(f"{where}: more than 2 option axes")
        axis_values = []
        for o in options:
            l10n(o.get("name"), f"{where}.options[{o.get('id')}].name")
            if o.get("kind") not in ("swatch", "chip"):
                err(f"{where}: option kind {o.get('kind')!r}")
            vals = set()
            for v in o.get("values", []):
                if v["id"] in vals:
                    err(f"{where}: option {o['id']} value {v['id']} duplicated")
                vals.add(v["id"])
                l10n(v.get("label"), f"{where}.options[{o['id']}].{v['id']}")
                if o.get("kind") == "swatch" and not re.match(r"^#[0-9A-Fa-f]{6}$", v.get("swatch") or ""):
                    err(f"{where}: swatch value {v['id']} needs #RRGGBB")
            axis_values.append(vals)

        variants = p.get("variants", [])
        if not variants:
            err(f"{where}: no variants")
        if not options and (len(variants) != 1 or variants[0].get("opts", []) != []):
            err(f"{where}: no options → exactly one variant with opts []")
        seen_vid = set()
        seen_combo = set()
        has_compare = False
        for v in variants:
            variants_total += 1
            vw = f"{where}.variant {v.get('id')}"
            if v.get("id") in seen_vid:
                err(f"{vw}: id duplicated")
            seen_vid.add(v.get("id"))
            if not v.get("sku"):
                err(f"{vw}: sku missing")
            opts = v.get("opts", [])
            if options:
                if len(opts) != len(options):
                    err(f"{vw}: opts must have {len(options)} values")
                else:
                    for i, val in enumerate(opts):
                        if val not in axis_values[i]:
                            err(f"{vw}: opts[{i}]={val!r} not a value of {options[i]['id']}")
                combo = tuple(opts)
                if combo in seen_combo:
                    err(f"{vw}: combination {combo} duplicated")
                seen_combo.add(combo)
            price = v.get("price")
            if not isinstance(price, int) or price <= 0:
                err(f"{vw}: price must be a positive int (piastres)")
            if v.get("compare_at") is not None:
                has_compare = True
                if not isinstance(v["compare_at"], int) or v["compare_at"] <= (price or 0):
                    err(f"{vw}: compare_at must be an int > price")
            stock = v.get("stock")
            if not isinstance(stock, int) or stock < 0:
                err(f"{vw}: stock must be int ≥ 0")
            elif stock == 0:
                oos_variants += 1
            img = v.get("image")
            if img is not None and not (0 <= img < len(images)):
                err(f"{vw}: image index {img} out of range")
        if "deal" in p.get("tags", []) and not has_compare:
            err(f"{where}: tagged deal but no variant has compare_at")
        for g in p.get("modifier_groups", []):
            l10n(g.get("name"), f"{where}.modifier_groups[{g.get('id')}].name")
            if g.get("selection") not in ("single", "multi"):
                err(f"{where}: modifier selection {g.get('selection')!r}")
            for o in g.get("options", []):
                l10n(o.get("name"), f"{where}.modifier {o.get('id')}")
                if not isinstance(o.get("price"), int) or o["price"] < 0:
                    err(f"{where}: modifier {o.get('id')} price must be int ≥ 0")

    for cid, n in per_cat.items():
        if n < 8:
            warn.append(f"category {cid} has only {n} products")

    n = len(pids)
    print(f"{tenant}: {len(cats)} categories, {len(brands)} brands, {n} products, {variants_total} variants")
    print("tags: " + ", ".join(f"{t} {c} ({c * 100 // max(n, 1)}%)" for t, c in tag_count.most_common()))
    print(f"out-of-stock variants: {oos_variants} ({oos_variants * 100 // max(variants_total, 1)}%)")
    for w in warn[:40]:
        print("warn:", w)
    if len(warn) > 40:
        print(f"warn: … {len(warn) - 40} more")
    for e in errors[:80]:
        print("ERROR:", e)
    if len(errors) > 80:
        print(f"ERROR: … {len(errors) - 80} more")
    print("OK" if not errors else f"{len(errors)} errors")
    return 0 if not errors else 1


if __name__ == "__main__":
    sys.exit(main())
