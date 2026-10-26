# Data contract: catalogues and media

Everything the storefront shows comes from two kinds of files, loaded by the server at startup:

| File | What | Rust type |
|---|---|---|
| `data/catalog/<tenant>.json` | brands, categories, products | `domain::Catalog` (`crates/domain/src/catalog.rs`) |
| `data/media/<tenant>/manifest.json` | one entry per image key | `app::media::MediaEntry` |
| `data/media/<key>-<width>.webp` | the image files (WebP only) | served at `/media/<key>-<width>.webp` |

Tenants: `nile` (general marketplace, English default, LTR) and `souq` (Egyptian grocery & pantry,
Arabic default, RTL). Shared platform/brand art uses the pseudo-tenant `brand`.

Validate before handing off:

```bash
python3 -I tools/catalog/validate.py nile      # or souq
```

## Catalogue JSON (`data/catalog/<tenant>.json`)

Field names are exactly the serde names of the Rust types. Every user-facing string is an `L10n`
object `{"en": "...", "ar": "..."}` and both sides must be filled. Money is **integer piastres**
(EGP × 100), like Madar: `EGP 1,299.00` → `129900`.

```jsonc
{
  "tenant": "nile",
  "brands": [
    { "id": "samsung", "name": { "en": "Samsung", "ar": "سامسونج" } }
  ],
  "categories": [
    {
      "id": 10,                                   // unique small int
      "slug": "phones",                           // unique, [a-z0-9-]
      "name":  { "en": "Phones", "ar": "موبايلات" },
      "blurb": { "en": "Flagships to everyday phones", "ar": "من الفلاجشيب للموبايل اليومي" },
      "icon": "smartphone",                       // from the icon list below
      "image": "nile/p1001-1",                    // any media key (a product packshot is fine)
      "display_order": 1
    }
  ],
  "products": [
    {
      "id": 1001,                                 // unique int, 4 digits
      "slug": "galaxy-s25-ultra",                 // unique, [a-z0-9-]
      "category": 10,
      "brand": "samsung",                         // brand id or omit
      "name":     { "en": "Galaxy S25 Ultra", "ar": "جالاكسي S25 ألترا" },
      "subtitle": { "en": "AI phone · 200 MP camera", "ar": "موبايل بالذكاء الاصطناعي · كاميرا 200 ميجا" },
      "description": { "en": "2–4 sentences.", "ar": "٢–٤ جمل." },
      "highlights": [ { "en": "…", "ar": "…" } ],             // 2–4 bullets
      "specs": [ { "k": { "en": "Display", "ar": "الشاشة" }, "v": { "en": "6.9\" AMOLED 120 Hz", "ar": "6.9 بوصة AMOLED بتردد 120 هرتز" } } ],
      "images": ["nile/p1001-1", "nile/p1001-2", "nile/p1001-3"],   // first = primary
      "rating": 4.6,                              // 0–5, one decimal
      "reviews": 312,
      "tags": ["featured", "deal", "new", "bestseller", "express"], // any subset
      "options": [                                // 0, 1 or 2 axes
        { "id": "color", "name": { "en": "Colour", "ar": "اللون" }, "kind": "swatch",
          "values": [
            { "id": "black", "label": { "en": "Titanium Black", "ar": "أسود تيتانيوم" }, "swatch": "#2B2D30" },
            { "id": "silver", "label": { "en": "Titanium Silver", "ar": "فضي تيتانيوم" }, "swatch": "#C9CCD1" }
          ] },
        { "id": "storage", "name": { "en": "Storage", "ar": "المساحة" }, "kind": "chip",
          "values": [
            { "id": "256", "label": { "en": "256 GB", "ar": "256 جيجا" } },
            { "id": "512", "label": { "en": "512 GB", "ar": "512 جيجا" } }
          ] }
      ],
      "variants": [                               // ≥ 1; opts = one value id per axis, in axis order
        { "id": 1, "sku": "NIL-1001-BLK-256", "opts": ["black", "256"], "price": 6499900, "compare_at": 7199900, "stock": 14, "image": 0 },
        { "id": 2, "sku": "NIL-1001-BLK-512", "opts": ["black", "512"], "price": 7299900, "stock": 3, "image": 0 },
        { "id": 3, "sku": "NIL-1001-SLV-256", "opts": ["silver", "256"], "price": 6499900, "stock": 0, "image": 1 }
      ],
      "modifier_groups": [                        // optional add-ons (Madar modifier groups)
        { "id": "protect", "name": { "en": "Protection", "ar": "حماية" }, "selection": "single",
          "min": 0, "max": 1, "required": false,
          "options": [ { "id": "care1", "name": { "en": "1-year Care+", "ar": "Care+ لمدة سنة" }, "price": 249900 } ] }
      ]
    }
  ]
}
```

Rules the validator enforces:

- ids and slugs unique; every `category`/`brand` reference exists; every image key exists in the
  tenant manifest (or `brand/…`).
- With no `options`, exactly one variant with `"opts": []`.
- With options, every variant's `opts` has one value per axis, value ids exist, and no two
  variants share the same combination. Missing combinations are fine (shown as unavailable).
- `compare_at`, when present, is greater than `price`. A product tagged `deal` has at least one
  variant with a `compare_at`.
- `image` (variant) is a valid index into `images`.
- Prices are positive integers. Stock is an integer ≥ 0 (make ~8% of variants `0`, and some
  products entirely out of stock, so the UI can show it).

Content guidance:

- **Realistic for Egypt, 2026.** Plausible EGP retail prices (end in 9/99 like local shops),
  Egyptian brands where they exist (Juhayna, Domty, El Rashidi El Mizan, Abu Auf, Isis, El Arosa,
  Bisco Misr, Edita, Tornado, Fresh, Eva, Cottonil …) and international brands sold locally.
- **Arabic is first-class, not machine-literal.** Egyptian colloquial is fine for blurbs and
  subtitles (Madar's shop pages use it); product names and specs in clear Modern Standard Arabic.
  Keep Latin model names/units where Egyptians write them in Latin (`iPhone 16`, `256 GB` is fine
  as `256 جيجا`).
- Tags: ~12% `featured`, ~15% `deal`, ~10% `new`, ~12% `bestseller`, ~20% `express`.
- 8–14 categories per tenant, each with ≥ 8 products.

## Media keys and files

| Kind | Key pattern | Shape | Widths |
|---|---|---|---|
| product packshot | `<tenant>/p<productId>-<n>` (n from 1) | square, transparent background kept | 240 400 640 960 |
| hero (wide) | `<tenant>/hero-<n>` | 21:9 photo | 640 960 1280 1920 |
| hero (mobile crop) | `<tenant>/hero-<n>-m` | 4:5 photo | same ladder, capped at source |
| promo / collection tile | `<tenant>/promo-<n>` / `<tenant>/tile-<name>` | 4:3 or 3:2 photo | 240 400 640 960 |
| logos, marks | `brand/<name>` | as drawn, transparent | 64 … 512 |
| illustrations, OG images | `brand/<name>` | as drawn | 240 … 1280 |

Make every file with `tools/media/process.py` (it writes the widths, the manifest entry, a 20 px
blurred placeholder and the dominant colour). Keep downloaded originals in `data/media/_src/`
(git-ignored). Every manifest entry from the web must carry `source` (original URL), `license` and
`credit`; also list them in `data/media/<tenant>/CREDITS.md`.

## Icon names for categories

Lucide names the UI ships (category `icon` must be one of these):

`smartphone laptop tablet headphones watch tv camera gamepad-2 speaker monitor keyboard mouse
battery-charging cable shirt footprints gem glasses sparkles spray-can brush sofa lamp
cooking-pot refrigerator washing-machine baby dumbbell bike book-open gift apple carrot milk egg
beef fish drumstick croissant coffee cup-soda candy wheat leaf flame cookie soup ice-cream-cone
citrus cherry grape salad nut bean shopping-basket store package tag sun-snow heart-pulse pill
paw-print`
