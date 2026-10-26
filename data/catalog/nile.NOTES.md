# Nile catalogue — notes

General marketplace for Egypt (EN default, full AR). **191 products · 357 variants · 13 categories ·
84 brands · 435 product images** plus home-page photos. Prices are 2026 EGP retail in piastres.
`python3 -I tools/catalog/validate.py nile` prints OK.

## Categories

| id | slug | products | variants | price range (EGP) |
|---|---|---|---|---|
| 10 | phones | 16 | 25 | 1,999 – 26,999 |
| 11 | laptops-tablets | 8 | 19 | 3,999 – 129,999 |
| 12 | audio-accessories | 13 | 14 | 499 – 29,999 |
| 13 | gaming | 8 | 11 | 3,799 – 38,999 |
| 14 | watches | 12 | 13 | 1,899 – 1,349,000 |
| 15 | fashion | 14 | 69 | 549 – 3,299 |
| 16 | shoes | 10 | 60 | 1,499 – 32,999 |
| 17 | bags-accessories | 13 | 13 | 149 – 199,000 |
| 18 | beauty-fragrance | 13 | 24 | 119 – 11,499 |
| 19 | home-furniture | 11 | 12 | 449 – 38,999 |
| 20 | kitchen-dining | 30 | 36 | 69 – 13,999 |
| 21 | sports-fitness | 18 | 25 | 249 – 7,999 |
| 22 | supermarket | 25 | 36 | 9 – 6,999 |

Product ids are `category × 100 + n` (1001…2225); category `image` is the packshot of a
representative product.

## How DummyJSON was mapped

- All 184 DummyJSON products outside vehicle/motorcycle are used exactly once, with every image in
  API order (`nile/p<id>-<n>`). 183 Nile products come from them: the red and black corset sets
  (DummyJSON 178 + 179) became one product with a colour swatch whose variants point at image 0 / 4.
- Visibly branded items keep their brand (Apple, Samsung, Rolex, Dior, Whiskas, Nescafé…). Generic
  renders got a brand that is sold in Egypt and doesn't contradict the photo (Town Team, Concrete,
  Andora, Mobaco, DeFacto, LC Waikiki, Tornado, Zahran, Koki, Crystal, Isis, Dina Farms, Siwa…), no
  brand, or the marketplace's own private label **Nile Basics** (simple kitchen tools, a football).
- Three DummyJSON pairs have identical photos; they became distinct listings: Off-White sneakers
  men's / women's sizing, Rolex Datejust 41 / 36, Rolex Cellini Moonphase new / pre-owned.
- The DummyJSON phone renders are 2016–2021 models (some have the model name baked into the
  picture), so 12 of the 16 phones are sold as **Renewed** (graded, battery ≥ 85 %, Nile warranty);
  the K1 render became the current OPPO A18. No current-flagship packshot with a usable licence was
  found; a cut-out of a Commons iPhone 16 Pro Max photo was tried and dropped as not clean enough.
- Other renames for realism: Apple Watch Series 4 → Apple Watch SE 3, AirPower → Belkin dual pad,
  iPad mini 2021 → iPad mini (A17 Pro), Tab S8+ → Tab S10+, Yoga 920 → Yoga 7i, XPS 13 9300 → 9350,
  MacBook Pro → 14" M5, Echo Plus → Echo (3rd gen).
- **Gaming** (8) has no DummyJSON source: transparent-background photos from Wikimedia Commons
  (CC BY / CC BY-SA / CC0), two of them derived by `tools/catalog/nile/cutout.py` (Series S cropped
  from an X + S photo; the white DualSense with its white backdrop removed). See CREDITS.md.

## Prices

Anchored to 2026 Egyptian retail (USD ≈ EGP 49–50 plus duties/VAT for imports), ending in 9/99:
flagship laptops EGP 75–130k, AirPods 4 7,499 / ANC 9,999, PS5 29,999, fragrances 2,999–11,499,
shirts 599–999, dresses 999–3,299, eggs 69 / tray of 30 189, Coca-Cola 1 L 29, water 9.
Outliers on purpose: Rolex/Longines/IWC at real retail (EGP 0.21–1.35 M) and the Prada Galleria
(EGP 199,000) — a marketplace can list them, but they skew "most expensive" sorts.
`compare_at` is 10–35 % above price on deal-tagged products.

## Variants, add-ons, stock, tags

- 70 products have option axes (5 with two axes): storage × condition, storage × Wi-Fi/cellular,
  sizes (S–XXL, kids' ages, EU 36–45), fragrance ml, shades (swatches), packs/weights. Seven
  products map variants to photos (iPhone SE colours, corset colours, fragrance boxes, mug tree,
  chicken cut).
- 15 products carry Madar-style `modifier_groups` (screen protector + warranty, AppleCare-style
  extensions, Microsoft 365, Apple Pencil, PS Plus / Game Pass, gift wrap + card, gift box,
  assembly, hob installation, insured delivery).
- Stock is deterministic (hash of product/variant), scaled by price; 8 % of variants are at 0 and
  five products are fully sold out: OPPO A57 renewed, video tripod, Rolex Submariner (waitlist),
  Off-White women's, black mulberries (out of season).
- Tags: featured 12 %, deal 16 %, new 9 %, bestseller 13 %, express 20 % (all Supermarket items are
  express).

## Home-page photos

All CC0 and found through Openverse; crops and focus points are in `tools/catalog/nile/hero_jobs.jsonl`.

- `hero-1` / `-m`: Grado SR80e headphones held against a soft white wall (Unsplash import on
  Commons). It replaced the blue-sky headphones shot after the switch to the porcelain/graphite
  look. The 21:9 crop is anchored low so the cups and logo stay whole; the 4:5 crop is centred.
- `hero-2` / `-m`: fashion close-up, an oversized black and beige coat with a clear tote
  (WordPress Photo Directory).
- `promo-1`: New Balance 2002R sneakers. `promo-2`: a yellow armchair against a white wall; it
  replaced a busier lounge shot.
- The pyramid photo was removed at the owner's request. There is deliberately no Egypt postcard
  imagery (pyramids, camels, feluccas).

## Odd things worth knowing

- Every packshot was re-encoded as the last step with the updated `tools/media/process.py`, which
  removes the drop shadows baked into the DummyJSON cut-outs and keeps alpha lossless.
- The validator's ~130 warnings are Latin-only Arabic values on purpose: size chips (S, M, XL, 42)
  and model references (126333, RB3025, Apple H2).
- Some spec details and service terms are plausible but invented (Nile warranty lengths, pre-owned
  watch authentication, dimensions of private-label items).
- Wikimedia throttles anonymous bulk downloads; originals were fetched one at a time, 20–60 s apart.

## Rebuilding

```bash
python3 -I tools/catalog/nile/build_catalog.py --jobs /tmp/jobs.jsonl   # nile.json + image jobs
python3 -I tools/media/process.py --batch /tmp/jobs.jsonl               # product images
python3 -I tools/media/process.py --batch tools/catalog/nile/hero_jobs.jsonl
python3 -I tools/catalog/nile/write_credits.py                           # data/media/nile/CREDITS.md
python3 -I tools/catalog/validate.py nile
```

Copy lives in `tools/catalog/nile/parts/*.py` (one file per category group); ids, slugs, brands,
tags and image sources in `skeleton.py`. Slugs come from the skeleton's working names, so editing
copy never changes a URL.
