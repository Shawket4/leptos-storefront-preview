#!/usr/bin/env python3
"""Check one content part file before it is merged into the Nile catalogue.

    python3 -I tools/catalog/nile/check_part.py tools/catalog/nile/parts/phones.py

Part format (prices in whole EGP; the build converts to piastres; L10n is an (en, ar) tuple):

    PRODUCTS = [
        dict(
            id=1001,                                   # must exist in skeleton.py (or the part's SKELETON)
            name=("…", "…"),
            subtitle=("…", "…"),                       # short card line; Arabic may be Egyptian colloquial
            description=("…", "…"),                    # 2–4 sentences
            highlights=[("…", "…"), …],                # 2–4
            specs=[(("Display", "الشاشة"), ("6.1\" OLED", "6.1 بوصة OLED")), …],   # 3–6
            options=[dict(id="storage", name=("Storage", "السعة"), kind="chip",
                          values=[dict(id="128", label=("128 GB", "128 جيجابايت")), …]), …],  # 0–2 axes
            variants=[dict(opts=["128"], price=23999, compare_at=26999, image=0), …],
            modifier_groups=[dict(id="protect", name=("…", "…"), selection="multi", min=0, max=2,
                                  required=False, options=[dict(id="glass", name=("…", "…"), price=249)])],
            rating=4.5, reviews=312,
            alt=("Apple iPhone 13 Pro in Sierra Blue", "آيفون 13 برو باللون الأزرق سييرا"),
            views=[("back", "من الخلف"), …],          # one per image, in image order
        ),
    ]
"""

from __future__ import annotations

import importlib.util
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ARABIC = re.compile(r"[؀-ۿ]")
EASTERN_DIGITS = re.compile(r"[٠-٩۰-۹]")
ID = re.compile(r"^[a-z0-9]+(?:-[a-z0-9]+)*$")
HEX = re.compile(r"^#[0-9A-Fa-f]{6}$")


def load(path, name):
    spec = importlib.util.spec_from_file_location(name, path)
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def main() -> int:
    path = sys.argv[1]
    skel = load(os.path.join(HERE, "skeleton.py"), "sk")
    import json
    with open(os.path.join(os.path.dirname(os.path.dirname(os.path.dirname(HERE))),
                           "data", "media", "_src", "nile", "dj", "index.json"), encoding="utf-8") as f:
        dj_index = json.load(f)
    dj_count = {}
    for n in dj_index:
        dj_count[int(n.split("-")[0])] = dj_count.get(int(n.split("-")[0]), 0) + 1
    part = load(path, "part")
    skeleton = dict(skel.PRODUCTS)
    skeleton.update(getattr(part, "SKELETON", {}))
    errs, warns = [], []

    def l10n(x, where, ar_needed=True):
        if not (isinstance(x, tuple) and len(x) == 2 and all(isinstance(s, str) for s in x)):
            errs.append(f"{where}: must be an (en, ar) tuple of str, got {x!r}"[:160])
            return
        en, ar = x
        if not en.strip() or not ar.strip():
            errs.append(f"{where}: empty en/ar")
        if ar_needed and not ARABIC.search(ar):
            errs.append(f"{where}: ar has no Arabic letters: {ar!r}"[:160])
        if EASTERN_DIGITS.search(ar):
            warns.append(f"{where}: use Western digits 0-9 in Arabic text: {ar!r}"[:160])
        if ARABIC.search(en):
            errs.append(f"{where}: en contains Arabic: {en!r}"[:160])

    seen = set()
    for p in part.PRODUCTS:
        pid = p.get("id")
        w = f"product {pid}"
        if pid in seen:
            errs.append(f"{w}: duplicated")
        seen.add(pid)
        if pid not in skeleton:
            errs.append(f"{w}: not in skeleton")
            continue
        cat, img, brand, working, tags, notes = skeleton[pid]
        if img[0] == "dj":
            n_img = sum(dj_count[d] for d in img[1:])
        else:
            n_img = len(img) - 1
        for k in ("name", "subtitle", "description"):
            l10n(p.get(k), f"{w}.{k}")
        hl = p.get("highlights", [])
        if not 2 <= len(hl) <= 4:
            warns.append(f"{w}: {len(hl)} highlights (want 2–4)")
        for i, x in enumerate(hl):
            l10n(x, f"{w}.highlights[{i}]")
        sp = p.get("specs", [])
        if not 3 <= len(sp) <= 6:
            warns.append(f"{w}: {len(sp)} specs (want 3–6)")
        for i, s in enumerate(sp):
            if not (isinstance(s, tuple) and len(s) == 2):
                errs.append(f"{w}.specs[{i}]: must be ((k_en, k_ar), (v_en, v_ar))")
                continue
            l10n(s[0], f"{w}.specs[{i}].k")
            l10n(s[1], f"{w}.specs[{i}].v", ar_needed=False)
        l10n(p.get("alt"), f"{w}.alt")
        views = p.get("views", [])
        if n_img > 1 and len(views) != n_img:
            errs.append(f"{w}: views has {len(views)} entries, product has {n_img} images")
        for i, v in enumerate(views):
            l10n(v, f"{w}.views[{i}]")
        r = p.get("rating")
        if not (isinstance(r, (int, float)) and 0 <= r <= 5 and round(r, 1) == r):
            errs.append(f"{w}: rating must be 0–5 with one decimal")
        if not (isinstance(p.get("reviews"), int) and p["reviews"] >= 0):
            errs.append(f"{w}: reviews must be an int ≥ 0")
        options = p.get("options", [])
        if len(options) > 2:
            errs.append(f"{w}: more than 2 option axes")
        axis_vals = []
        for o in options:
            if not ID.match(o.get("id", "")):
                errs.append(f"{w}: option id {o.get('id')!r} must be [a-z0-9-]")
            l10n(o.get("name"), f"{w}.option {o.get('id')}.name")
            if o.get("kind") not in ("swatch", "chip"):
                errs.append(f"{w}: option kind {o.get('kind')!r}")
            vals = []
            for v in o.get("values", []):
                if not ID.match(str(v.get("id", ""))):
                    errs.append(f"{w}: value id {v.get('id')!r} must be [a-z0-9-]")
                if v["id"] in vals:
                    errs.append(f"{w}: value {v['id']} duplicated")
                vals.append(v["id"])
                l10n(v.get("label"), f"{w}.{o.get('id')}.{v.get('id')}", ar_needed=False)
                if o.get("kind") == "swatch" and not HEX.match(v.get("swatch") or ""):
                    errs.append(f"{w}: swatch value {v['id']} needs swatch='#RRGGBB'")
            axis_vals.append(set(vals))
        variants = p.get("variants", [])
        if not variants:
            errs.append(f"{w}: no variants")
        if not options and (len(variants) != 1 or variants[0].get("opts", []) != []):
            errs.append(f"{w}: no options → exactly one variant with opts=[]")
        combos = set()
        has_compare = False
        for i, v in enumerate(variants):
            vw = f"{w}.variants[{i}]"
            opts = v.get("opts", [])
            if options:
                if len(opts) != len(options):
                    errs.append(f"{vw}: opts needs {len(options)} values")
                else:
                    for ax, val in enumerate(opts):
                        if val not in axis_vals[ax]:
                            errs.append(f"{vw}: {val!r} not a value of {options[ax]['id']}")
                if tuple(opts) in combos:
                    errs.append(f"{vw}: duplicate combination {opts}")
                combos.add(tuple(opts))
            price = v.get("price")
            if not (isinstance(price, int) and price > 0):
                errs.append(f"{vw}: price must be a positive int in EGP")
            elif price < 100000 and price > 20 and str(price)[-1] != "9":
                warns.append(f"{vw}: price {price} doesn't end in 9")
            ca = v.get("compare_at")
            if ca is not None:
                has_compare = True
                if not (isinstance(ca, int) and isinstance(price, int) and ca > price):
                    errs.append(f"{vw}: compare_at must be an int > price")
                elif not 1.05 <= ca / price <= 1.6:
                    warns.append(f"{vw}: compare_at is {ca / price - 1:.0%} above price")
            im = v.get("image")
            if im is not None and not (isinstance(im, int) and 0 <= im < n_img):
                errs.append(f"{vw}: image index {im} out of range (0..{n_img - 1})")
        if "deal" in tags and not has_compare:
            errs.append(f"{w}: tagged deal but no variant has compare_at")
        for g in p.get("modifier_groups", []):
            l10n(g.get("name"), f"{w}.modifier {g.get('id')}")
            if g.get("selection") not in ("single", "multi"):
                errs.append(f"{w}: modifier selection must be single|multi")
            for o in g.get("options", []):
                l10n(o.get("name"), f"{w}.modifier option {o.get('id')}")
                if not (isinstance(o.get("price"), int) and o["price"] >= 0):
                    errs.append(f"{w}: modifier option price must be int EGP ≥ 0")
    for x in warns[:60]:
        print("warn:", x)
    for x in errs[:120]:
        print("ERROR:", x)
    print(f"{len(seen)} products, {len(errs)} errors, {len(warns)} warnings")
    return 1 if errs else 0


if __name__ == "__main__":
    sys.exit(main())
