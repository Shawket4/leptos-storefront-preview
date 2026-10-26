"""Shared helpers for the Souq catalogue data modules (see build.py)."""

from __future__ import annotations

from decimal import Decimal


def L(en: str, ar: str) -> dict:
    return {"en": en, "ar": ar}


def piastres(egp) -> int:
    return int((Decimal(str(egp)) * 100).to_integral_value())


# ---------------------------------------------------------------------------------------------
# Spec keys
SK = {
    "weight": L("Net weight", "الوزن الصافي"),
    "volume": L("Volume", "الحجم"),
    "pack": L("Pack", "العبوة"),
    "count": L("Count", "العدد"),
    "origin": L("Origin", "المنشأ"),
    "storage": L("Storage", "طريقة الحفظ"),
    "shelf": L("Shelf life", "مدة الصلاحية"),
    "ingredients": L("Ingredients", "المكونات"),
    "sold": L("Sold by", "طريقة البيع"),
    "fat": L("Fat", "الدسم"),
    "cut": L("Cut", "القطعية"),
    "variety": L("Variety", "الصنف"),
    "grade": L("Grade", "الدرجة"),
    "roast": L("Roast", "التحميص"),
    "grind": L("Grind", "الطحنة"),
    "type": L("Type", "النوع"),
    "use": L("Best for", "مناسب لـ"),
    "season": L("Season", "الموسم"),
    "cooking": L("Cooking time", "مدة الطهي"),
    "brew": L("How to brew", "طريقة التحضير"),
    "serves": L("Serves", "تكفي"),
    "form": L("Form", "الشكل"),
    "catch": L("Catch", "المصدر"),
    "prep": L("Preparation", "التجهيز"),
    "halal": L("Slaughter", "الذبح"),
    "baked": L("Baked", "الخَبز"),
    "sugar": L("Sugar", "السكر"),
    "milk": L("Milk", "اللبن"),
    "flour": L("Flour", "الدقيق"),
}

# Frequent spec values
EGYPT = L("Egypt", "مصر")
FRIDGE = L("Refrigerate at 0–4 °C", "في الثلاجة من 0 لـ 4 درجات")
FREEZER = L("Frozen at −18 °C", "في الفريزر على −18 درجة")
DRY = L("Cool, dry place away from sunlight", "مكان جاف وبارد بعيد عن الشمس")
AIRTIGHT = L("Airtight jar, cool and dry", "برطمان محكم في مكان جاف وبارد")
HALAL = L("Halal, slaughtered locally", "ذبح حلال محلي")


def S(key: str, en: str | dict, ar: str | None = None) -> dict:
    v = en if isinstance(en, dict) else L(en, ar)
    return {"k": SK[key], "v": v}


# ---------------------------------------------------------------------------------------------
# Option axes
AXIS_NAMES = {
    "weight": L("Weight", "الوزن"),
    "size": L("Size", "الحجم"),
    "pack": L("Pack", "العبوة"),
    "flavour": L("Flavour", "الطعم"),
    "type": L("Type", "النوع"),
    "colour": L("Colour", "اللون"),
    "form": L("Form", "الشكل"),
    "count": L("Count", "العدد"),
}

# Packaged goods: "500 جم"; loose goods (produce, meat, nuts by weight): "نص كيلو"
PACKED = {
    "50g": L("50 g", "50 جم"),
    "70g": L("70 g", "70 جم"),
    "80g": L("80 g", "80 جم"),
    "100g": L("100 g", "100 جم"),
    "125g": L("125 g", "125 جم"),
    "150g": L("150 g", "150 جم"),
    "200g": L("200 g", "200 جم"),
    "250g": L("250 g", "250 جم"),
    "300g": L("300 g", "300 جم"),
    "350g": L("350 g", "350 جم"),
    "400g": L("400 g", "400 جم"),
    "450g": L("450 g", "450 جم"),
    "500g": L("500 g", "500 جم"),
    "700g": L("700 g", "700 جم"),
    "800g": L("800 g", "800 جم"),
    "900g": L("900 g", "900 جم"),
    "1kg": L("1 kg", "1 كجم"),
    "2kg": L("2 kg", "2 كجم"),
    "5kg": L("5 kg", "5 كجم"),
}
LOOSE = {
    "250g": L("250 g", "ربع كيلو"),
    "500g": L("500 g", "نص كيلو"),
    "1kg": L("1 kg", "كيلو"),
    "1500g": L("1.5 kg", "كيلو ونص"),
    "2kg": L("2 kg", "2 كيلو"),
    "3kg": L("3 kg", "3 كيلو"),
}


def axis(axis_id: str, values: list, kind: str = "chip", name: dict | None = None) -> dict:
    """values: list of (id, L) or (id, L, swatch)."""
    out = []
    for v in values:
        d = {"id": v[0], "label": v[1]}
        if kind == "swatch":
            d["swatch"] = v[2]
        out.append(d)
    return {"id": axis_id, "name": name or AXIS_NAMES[axis_id], "kind": kind, "values": out}


def weights(*ids: str, loose: bool = False) -> dict:
    table = LOOSE if loose else PACKED
    return axis("weight", [(i, table[i]) for i in ids])


def V(opt, price, was=None, stock=None, img=None, sku=None) -> dict:
    """One variant: opt is a value id (1 axis), a tuple (2 axes) or None (no options)."""
    return {"opt": opt, "price": price, "was": was, "stock": stock, "img": img, "sku": sku}


# ---------------------------------------------------------------------------------------------
# Modifier groups (Madar style)
def mo(oid: str, en: str, ar: str, egp=0) -> dict:
    return {"id": oid, "name": L(en, ar), "price": piastres(egp)}


def mg(gid: str, en: str, ar: str, options: list, selection: str = "single", required=False,
       min_=None, max_=None) -> dict:
    if min_ is None:
        min_ = 1 if required else 0
    if max_ is None and selection == "single":
        max_ = 1
    g = {"id": gid, "name": L(en, ar), "selection": selection, "min": min_, "required": required,
         "options": options}
    if max_ is not None:
        g["max"] = max_
    return g


def P(**kw) -> dict:
    return kw
