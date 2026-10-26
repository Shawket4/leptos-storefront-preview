# Souq catalogue — notes

Egyptian neighbourhood grocery & pantry (Arabic default, RTL; full English). Generated from the data
modules in `tools/catalog/souq/` — edit those, not `souq.json`.

**Totals:** 13 categories · 192 products · 324 variants · 36 brands · 209 product images + 3 heroes
(desktop + mobile crops) + 2 promo tiles. `python3 -I tools/catalog/validate.py souq` → OK.

## Categories

| id | slug | name (en / ar) | icon | products | variants |
|---|---|---|---|---|---|
| 10 | `fruits-vegetables` | Fruits & vegetables / خضار وفاكهة | `carrot` | 49 | 85 |
| 20 | `bakery` | Bakery / مخبوزات | `croissant` | 13 | 16 |
| 30 | `dairy-eggs` | Dairy & eggs / ألبان وبيض | `milk` | 8 | 15 |
| 40 | `cheese` | Cheese & olives / جبن وزيتون | `refrigerator` | 8 | 16 |
| 50 | `meat-poultry` | Meat & poultry / لحوم وفراخ | `beef` | 18 | 36 |
| 60 | `fish-seafood` | Fish & seafood / أسماك ومأكولات بحرية | `fish` | 14 | 27 |
| 70 | `rice-grains-pulses` | Rice, grains & pulses / أرز وبقول وحبوب | `wheat` | 8 | 12 |
| 80 | `spices-herbs` | Spices & herbs / بهارات وعطارة | `leaf` | 13 | 21 |
| 100 | `dates-sweets` | Dates & sweets / بلح وحلويات | `candy` | 9 | 14 |
| 110 | `coffee-tea` | Coffee & tea / بن وشاي | `coffee` | 8 | 15 |
| 120 | `drinks-juices` | Drinks & juices / مشروبات وعصائر | `cup-soda` | 8 | 12 |
| 130 | `nuts-snacks` | Nuts & snacks / مكسرات وتسالي | `nut` | 13 | 25 |
| 140 | `breakfast-pantry` | Breakfast & pantry / فطار ومونة البيت | `shopping-basket` | 23 | 30 |

- **Ids are stable.** Products live in 2001–2699; each category was given a block of 50 ids
  (10 → 2001…, 20 → 2051…, … 140 → 2651…). Slugs are stable too. The first pass had a 14th
  category (90 `oils-tahini-honey`); with no usable tahini/ghee/molasses images it was folded into
  breakfast & pantry — its products kept ids 2401–2404. No product id or slug has changed since pass 1.
- Merchandising: featured 11%, deal 15% (every deal has a `compare_at`), new 9%, bestseller 11%,
  express 39% (dark-store stock is the norm here). 26 of 324 variants (8%) are out of stock; four
  products are fully sold out: 2035 green fava beans and 2049 fresh figs (out of season), 2264 rainbow
  trout and 2459 Cadbury Royal Dark (stock-outs).
- Options: 126 products have one axis (weight, pack, size, colour swatch, type), 66 have none.
  Loose goods label weights the Egyptian way (نص كيلو، كيلو، 2 كيلو); packaged goods use جم/كجم.
- Madar-style `modifier_groups` on 17 products: butcher (2201 cut incl. mince +15 / kofta +35 and
  packing, 2202 fat level + hawawshi seasoning, 2205 chicken cutting/deboning + skin, 2206 pané /
  shish / shish-tawook prep, 2216 rotisserie-chicken sides), fishmonger (2251, 2252 cleaning +
  ready-to-cook bran/frying/sangari; 2255 shrimp peeling), coffee (2501 roast + grind + cardamom
  "mahwag" +EGP 10 / mastic +15; 2502 grind), and produce/deli touches (2005 peeled garlic, 2009
  zucchini cored for mahshi, 2020 pomegranate seeded, 2021 molokhia picked/minced, 2042 avocado
  ripeness, 2151 romy slicing/grating, 2451 date gift box). Copy on other products never promises
  an option that isn't offered.

## Rebuilding

```bash
python3 -I tools/catalog/souq/build.py          # writes souq.json (only if every image is processed,
                                                #   otherwise stages _src/souq/souq.pending.json) + job list
python3 -I tools/media/process.py --batch data/media/_src/souq/jobs-products.jsonl
python3 -I tools/catalog/souq/build.py          # publish
python3 -I tools/catalog/souq/web_images.py     # hero/promo jobs → _src/souq/jobs-web.jsonl
python3 -I tools/catalog/souq/credits.py        # data/media/souq/CREDITS.md + orphan check
python3 -I tools/catalog/validate.py souq
```

`d_overrides.py` holds the tag mix, deals, stock-outs and the modifier allow-list. `fetch.py` downloads
sources into `data/media/_src/souq/` (git-ignored); `sheet.py` makes review contact sheets.

## Images

- **Product packshots (209):** 189 TheMealDB ingredient PNGs, 20 DummyJSON grocery renders, all
  processed as `kind: product` (transparent, trimmed, centred, 0.82 fill). Every one was reviewed on
  contact sheets before and after processing.
- **Selection rule:** an Egyptian-brand product only gets a *neutral* image of its content (a plain
  milk bottle for Juhayna, a cup of Turkish coffee for Abu Auf, a cup of tea for El Arosa, an
  unbranded kraft rice bag for El Doha, a generic oil bottle for Crystal/Wadi Food, loose nuts for
  Abu Auf…). Foreign-branded packshots are used only for that same brand where it is sold in Egypt
  (Heinz, Knorr, Maggi, Hellmann's, Dr. Oetker, Barilla, Bonduelle, Alpro, Oldenburger, Anchor,
  Nutella, Oreo, Mars, Cadbury, Magnum, Kellogg's, Hershey's, Hero, Maille, Tabasco, Lea & Perrins,
  Nescafé, Coca-Cola, Schweppes). Products whose only images show an unrelated foreign brand were
  dropped (see caveats).
- Look-alike stand-ins worth knowing: beef cubes use TheMealDB "Goat Meat" chunks; chicken breast
  fillets use "Chicken Thighs" (boneless fillets); sea bass uses "barramundi"; sea bream uses
  "Snapper Fish"; hake fillet uses "Cod"; Isis dried mint uses "Chopped Parsley" (dried green herb
  flakes); baladi lemons use "Unwaxed Lime" (the small green-gold Egyptian lemon); fino bread uses
  "Cuban Bread"; breadcrumbs use "Stale bread"; Heinz tomato paste shows a catering-size Heinz can.
- **Heroes** (art direction from the lead: bright, appetising, one clear subject, calm area for the
  copy card; no pyramids/camels/feluccas): `hero-1` oranges and `hero-2` pomegranates (both Ivar
  Leidus, CC BY-SA 4.0, Wikimedia Commons; pomegranates are in season), `hero-3` roasted coffee beans
  on warm wood (Shusei Toda, CC0, WordPress Photo Directory). Each is cut twice: `hero-N` 21:9 and
  `hero-N-m` 4:5 (focus values in `web_images.py`). Because Souq defaults to Arabic, the desktop
  copy card sits lower-right, so the pomegranate and coffee photos are mirrored to keep their main
  subject on the left. Promo tiles (4:3): `promo-1` bread basket, `promo-2` basket of vegetables
  (both CC0 StockSnap, 960 px previews via the Openverse API — the StockSnap CDN refuses direct
  downloads). Full credits: `data/media/souq/CREDITS.md`.
- `du -sh data/media/souq` → 21 MB (819 WebP files + `manifest.json` + `CREDITS.md`).

## Prices (2026 EGP, piastres in JSON)

Built from late-2025 Cairo shelf prices plus ~15–25% inflation; shop-style endings (.99 / .49);
bigger packs are cheaper per unit; deals show a `compare_at` 5–15% higher.

| Anchor in the brief | Souq price |
|---|---|
| 1 kg Egyptian rice EGP 38–48 | El Doha 1 kg **42.99** (5 kg 204.99) |
| 1 L milk EGP 45–60 | Juhayna full cream **54.99** (6 × 1 L 309.99); fresh buffalo milk 59.99 |
| 1 kg chicken EGP 140–170 | whole chicken 1.1–1.3 kg **179.99** (≈ EGP 150/kg); drumsticks 149.99/kg |
| 1 kg beef EGP 480–560 | baladi kandouz **489.99**; minced 469.99; fillet 899.99 |
| 250 g Turkish coffee EGP 180–260 | Abu Auf **219.99** (+10 cardamom) |
| 500 g tahini EGP 95–140 | — (no usable image; not listed) |

Other checks: tomatoes 19.99/kg, onions 17.99/kg, baladi lemons 54.99/kg, eggs tray of 30 209.99,
shami bread 5 loaves 17.99, sunflower oil 1.6 L 154.99, sea bream 329.99/kg, medium shrimp
479.99/kg, Norwegian salmon 500 g 699.99, water 6 × 1.5 L 69.99. Imported items (Barilla, Alpro,
Maille, Tabasco, Hershey's, Lea & Perrins) carry import pricing.

## Caveats

- **Missing staples.** TheMealDB/DummyJSON have no usable image for red/brown lentils, dried fava
  (ful medames), tahini, halawa, yogurt, white/feta/istanbuli cheese, ghee, black honey, okra,
  guava, karkade (hibiscus), lupini or Egyptian biscuits/chips — every candidate showed an unrelated
  foreign brand (e.g. "Prince Tahini … from the Galilee", Lala/Yeo Valley yogurt, Athenos feta,
  TRS lentils). Following the brief ("pick products that have a matching image"), they are left out,
  and so are Domty, El Rashidi El Mizan, Lipton, Bisco Misr, Edita, Chipsy and Harvest. These are the
  first products to add once tenant photography exists.
- Dairy, cheese, grains, coffee & tea and drinks sit at the 8-product minimum for the same reason;
  fruits & vegetables (49) carries the range.
- Brand facts are plausible rather than verified (e.g. Imtenan clover honey, Isis anise and mint,
  Koki fresh chicken, Crystal pack sizes). Pack sizes on some packshots differ from the listed sizes.
- Some packshots are small at source, so their top rung is below 960 px (e.g. Maggi, coffee cup:
  640 px). `hero-3-m` tops out at 1093 px (2048 px source).
- Wikimedia rate-limits clients without contact details in the User-Agent (none was added — no
  contact address to give), so only a handful of 500 px previews and the two hero sources (standard
  3840 px renditions) were fetched from it.
- Arabic copy is Egyptian colloquial in subtitles/descriptions, clearer standard Arabic in names and
  specs; Western digits throughout.
