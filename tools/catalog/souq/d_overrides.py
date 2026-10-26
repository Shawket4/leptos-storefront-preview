"""Merchandising overrides applied on top of the data modules by build.py.

Kept separate so the tag mix (~12% featured, ~15% deal, ~10% new, ~12% bestseller, express common)
and the out-of-stock share (~8% of variants, a few products fully sold out) can be tuned in one place.
Variant ids are 1-based, in the order the variants are listed in the data module.
"""

TAG_ADD = {
    # featured
    2216: "featured", 2456: "featured",
    # deals (each has a compare_at below)
    2002: "deal", 2015: "deal", 2105: "deal", 2151: "deal", 2202: "deal", 2252: "deal", 2304: "deal",
    2307: "deal", 2403: "deal", 2454: "deal", 2503: "deal", 2553: "deal", 2609: "deal", 2655: "deal",
    # new arrivals
    2156: "new", 2306: "new", 2063: "new", 2108: "new", 2667: "new", 2507: "new",
}

EXPRESS_ADD = [2014, 2022, 2026, 2036, 2038, 2041, 2043, 2104, 2105, 2106, 2151, 2207, 2208, 2302, 2305,
               2351, 2454, 2457, 2504, 2609, 2655, 2656]

TAG_REMOVE = {2027: "bestseller", 2158: "bestseller", 2403: "bestseller", 2609: "bestseller"}

# (product id, variant id) → compare-at price in EGP
WAS = {
    (2002, 1): 34.99, (2015, 1): 44.99, (2105, 1): 169.99, (2151, 2): 189.99, (2202, 2): 499.99,
    (2252, 1): 359.99, (2304, 1): 27.99, (2307, 1): 69.99, (2403, 2): 379.99, (2454, 2): 399.99,
    (2503, 2): 154.99, (2553, 2): 54.99, (2609, 3): 184.99, (2655, 1): 134.99,
}

# whole products sold out (in addition to `soldout=True` in the data modules)
SOLDOUT = {2264, 2459}

# single variants out of stock (bigger packs and premium cuts that sell through first)
OOS = {
    (2006, 3), (2010, 2), (2041, 2), (2051, 2), (2054, 2), (2105, 2), (2152, 2), (2201, 3), (2211, 2),
    (2218, 2), (2252, 2), (2259, 2), (2354, 2), (2451, 2), (2502, 2), (2605, 2),
}

# Madar-style modifier groups are emitted only for these products (~15, per the brief); the data
# modules may describe more, but their copy must not promise options that are not offered.
MODS_KEEP = {2005, 2009, 2020, 2021, 2042, 2151, 2201, 2202, 2205, 2206, 2216, 2251, 2252, 2255, 2451,
             2501, 2502}
