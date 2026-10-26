# Leptos storefront preview

A two-shop e-commerce storefront built to answer one question for Madar's customer-facing web:
*is Leptos ready, and what does it cost to build with it?*

It was built **twice**:
1. **With Leptos islands.** Rust compiled to WASM ran the interactive parts in the browser, and
   the islands router handled navigation. It worked, but it shipped 380–770 KB of WASM (brotli)
   per first visit, and its hydration had races that lost toasts and cart updates on phones.
2. **With Leptos SSR and a TypeScript client.** Rust renders every page with its data, and a
   small TypeScript bundle owns the state (the cart, drawers, chosen variant, filters). The server
   is asked only for data the client doesn't have.

Both stages are documented here: what each cost, what broke, and the verdict at the end.

| Shop | Host | Catalogue | First language |
|---|---|---|---|
| **Nile** | `nile.*` | 191 products, 13 aisles: phones, laptops, audio, gaming, watches, fashion, shoes, bags, beauty, home, kitchen, sports, supermarket | English |
| **Souq** | `souq.*` | 192 products, 13 aisles of Egyptian groceries | Arabic |

One binary serves both. The shop comes from the first label of the hostname, like Madar's per-shop
subdomains. Every page works in English and Arabic (RTL), in Western or Arabic-Indic digits, in
light (the default) and dark themes, and without JavaScript.

## Run it

Requirements:
- Rust stable (built with 1.98.1), plus `rustup target add wasm32-unknown-unknown`. cargo-leptos
  still builds an empty placeholder crate for it; no page loads any WASM.
- `cargo install cargo-leptos --version 0.3.11 --locked`.
- Node.js (built with 22), plus `npm ci --prefix client` for esbuild and TypeScript (versions
  pinned).
- Read access to the private `Shawket4/madar-shared` repository. Pricing and VAT come from its
  `madar-catalog` and `madar-money` crates, a git dependency pinned to tag `v0.12.0`. Cargo
  fetches it through git (`[net] git-fetch-with-cli` in `.cargo/config.toml`), so `gh auth login`
  or any other git credential for github.com is enough.
- `.cargo/config.toml` also sets up sccache, mold and a build directory under `/home/apex/work`.
  Delete those lines on another machine.

```bash
npm ci --prefix client            # once
cargo leptos watch                # rebuilds on Rust *and* TypeScript changes
# http://nile.localhost:3000   http://souq.localhost:3000   (any *.localhost resolves to 127.0.0.1)

cargo leptos build --release
LEPTOS_OUTPUT_NAME=storefront LEPTOS_SITE_ROOT=target/site LEPTOS_SITE_PKG_DIR=pkg \
LEPTOS_SITE_ADDR=0.0.0.0:3000 LEPTOS_ENV=PROD ./target/release/server
```

The TypeScript in `client/` is type-checked by `tsc` and bundled by esbuild from
`crates/app/build.rs`. The result is an entry module plus chunks that load when a page needs them,
all named by content hash. They're embedded in the server binary and served from `/js/` with
`immutable` caching. A type error fails the Rust build.

**On a phone** (Tailscale): `http://nile.100-123-21-13.sslip.io:3000` and
`http://souq.100-123-21-13.sslip.io:3000`. sslip.io maps the name to the tailnet IP, and the shop is
still the first label. A bare IP works too: pick the shop at `/__tenant/nile`.

### Switches

| Switch | Default | What it does |
|---|---|---|
| `SIM_LATENCY_MS` (env) | `0` (off) | Pretend catalogue backend round trip, so loading states are visible. Each section waits a different multiple of it. |
| `/__sim?ms=350&next=/` | — | The same, per visitor (cookie). `/__sim?ms=&next=/` clears it. `/showcase` has buttons for it. |
| `?fail=deals` (or `fail` cookie) | — | Make one section's read fail, to see its error state and Retry. Comma lists work, as does `all`. |
| `CACHE_MISS_MS` (env) | `0` | Pretend database time on a read-cache miss, to make the cache visible. |
| `CACHE_TTL_SECS` (env) | `120` | Read-cache entry lifetime. |
| `/__prefs?lang=ar&num=arab&theme=dark&next=/` | `en`/`ar` per shop, `latn`, `light` | Display preferences (cookies). The header's language and theme controls use it. |
| `DATA_DIR`, `MEDIA_DIR` (env) | `data`, `data/media` | Catalogue JSON, images and search synonyms. Edits are picked up live. |

## Deploy

One Docker image, built like MadarRust's (`Dockerfile`):
- cargo-chef caches the dependencies in their own layer;
- the release profile is fat LTO with one codegen unit, and CI targets the VPS's CPU (`x86-64-v4`);
- the private madar-shared dependency is fetched with a BuildKit secret that never reaches a layer.

The image holds the server, the site assets and the catalogue (JSON, WebP images, synonyms). It
needs no database, `.env` or volume: carts live in the shopper's cookie. It is about 326 MB to
pull, 300 MB of which is catalogue images.

`.github/workflows/deploy.yaml` runs on a version tag (`v*.*.*`). It builds the image and pushes
it to GHCR. On the VPS it starts the image with `docker compose` behind nginx and waits for
`/health`. Then it checks the public URL and publishes a GitHub Release. One-time VPS setup is in
`deploy/SETUP.md`; `deploy/vps-agent-prompt.md` is the brief for the agent that prepares the box.

## The pages, and what each one demonstrates

Every page is rendered completely by the server, in order, so it works without JavaScript. The
TypeScript makes it instant:
- in-app navigation with a page cache and prefetch on intent;
- local state changes;
- the server asked only for missing data.

| Route | Server renders | TypeScript does |
|---|---|---|
| `/` | Category rail and the Deals, Picked for you, Express and Bestsellers rails, each with its own empty and error state (`?fail=deals`) | Deals countdown; quick add on cards |
| `/c/:slug` | Listing with facets, chips, sort, result count and server-side pages, all from the URL (`ListingQuery`) | Filter sheet; a filter or sort change becomes a debounced in-app navigation that keeps the scroll position and dims the old results while the new ones load |
| `/search?q=` | Bilingual search results (below) with "did you mean" | Instant search in the header: an ARIA combobox fed by `/api/search/suggest`, cached for 60 s |
| `/p/:slug` | Product page, its SEO head (OpenGraph, JSON-LD) and a real 404 with suggestions | Variant picker (price, stock and photo switch with **no request**), add-ons, quantity, add to cart; gallery swipe, zoom and fullscreen on springs |
| `/cart` | Priced lines from the `cart` cookie; forms that work without JS | The same line renderer as the drawer. Quantity, remove and Undo are local: lines, totals, savings and delivery are computed in the browser, with no request |
| `/checkout` | Four steps, each a real POST validated by the server. Placing an order re-prices, re-checks stock and clears the cart | Inline validation messages, pending state, double-submit guard |
| `/orders`, `/orders/:id` | Order lookup and the status timeline | Live status polling with backoff and Retry |
| `/showcase` | A guided tour of this architecture: Leptos's server features and the TypeScript client, with live demos | Spring lab, cache inspector, toast stack |
| `/ui` | The design-system kit: every component in every state | The live drawer, stepper and toasts |

Every page also handles:
- offline: a banner, with retries when the connection returns;
- failed navigations, with Retry;
- slow navigations: a progress bar after 150 ms;
- prefetch on hover after 65 ms or on finger-down, following Madar's gesture model and limited by
  a budget.

### Search, in both languages

Each product is indexed once with both languages, so either language finds it from either UI.
Text and queries are folded to one form:
- diacritics and tatweel removed;
- أ إ آ ٱ → ا, ة → ه, ى → ي, ؤ → و, ئ → ي;
- Arabic-Indic digits → 0–9;
- Latin lower-cased.

Every query word must be found in the product. A word matches when it is:
- the **start** of a word ("sams" → Samsung);
- the same word **without the Arabic article and its clitics** (الرز, بالتمر, والطحينة). A word
  written with a hamza on its alef (ألبان, ألعاب) keeps its letters, so «ألبان» doesn't find
  Ray-Ban (راي بان);
- the same **light stem** (سماعات ↔ سماعة, phones ↔ phone);
- in the **synonym table** (below).

Names weigh most, then brand, aisle, subtitle and variant labels ("256 GB"). Highlights and specs
count least and only on whole words. Descriptions aren't indexed: here they're recipe prose
("stuffed zucchini with rice"), and indexing them made "rice" match 31 products instead of 14.
Matches are highlighted on the catalogue's own spelling. A search takes about 2–3 ms and results
are cached.

**The synonym table is data.** `data/search/synonyms.txt` has 461 groups shared by both shops,
and `synonyms.<shop>.txt` adds per-shop groups: at Nile "apple" is the brand, at Souq it's تفاح.
Each line is one group, and any word in it finds the others:

```
bread, خبز, عيش
laptop, laptops, notebook, لابتوب, لاب توب, كمبيوتر محمول
soft drinks, fizzy drinks, cola, colas, كولا, مياه غازية, مشروبات غازية, حاجة ساقعة
```

- The table mixes Egyptian, standard Arabic and English: «قوطة» finds tomatoes, «حاجة ساقعة»
  finds Coca-Cola, «نضارة شمس» finds sunglasses.
- Multi-word entries are matched as one word, so «عيش الغراب» finds mushrooms, not bread.
- Edits apply live, with the caches dropped.
- The file's header lists the traps found while building it: ساعة alone means "hours" in specs,
  قدر is a pot but also "amount", and "soda" is also baking soda.

**What new product data needs.** Search indexes each language separately, so a product with only
English is still found:
- in English;
- in Arabic, through the synonym table («أرز» finds an English-only "rice");
- by brand and model names typed in Latin, and by digits in either script.

The pages are stricter: `L10n::pick` doesn't fall back, so an empty Arabic name shows blank on the
Arabic store, and `tools/catalog/validate.py` rejects it. Brand and aisle names matter most in
Arabic script, because that's how Arabic shoppers type them («سامسونج», not "Samsung").

## Architecture

```
 browser                                        server (Rust, Leptos SSR on Axum)
 ───────                                        ─────────────────────────────────
 HTML, complete, per page   ◀── GET /c/phones ──  pages: #[component]s rendered in order
 /js/app-<hash>.js + chunks ◀── once, cached ────  build.rs: tsc + esbuild → embedded
   router + page cache ───────── GET (prefetch) ─▶ same pages (X-Router: 1)
   stores: cart, toasts                           data endpoints (server functions, fixed URLs):
   cart cookie + snapshots ─── GET /api/cart/quote ▶  price lines the browser lacks
   search box ─────────── GET /api/search/suggest ▶  bilingual search
   order page ─────────── GET /api/orders/track ──▶  order status
   checkout (plain forms) ─ POST server functions ▶  re-price (madar-shared), re-check stock, order
                                                  data layer: in-memory catalogue + moka cache
```

**Principles.** These are in `AGENTS.md`, which every contributor reads first:
1. **The server renders data; the client owns state.** TypeScript changes the cart, drawers,
   variant choice and filters locally and instantly. It calls the server only for data it lacks.
2. **One source for everything:**
   - one store per concern (`client/src/stores/`), never copied by a feature;
   - one path per action (the cart changes only through `stores/cart.ts`);
   - one network layer, one data cache and one router;
   - one drawer, one stepper and one toast queue;
   - one string catalogue (`crates/app/src/strings/`).
3. **Markup lives in Rust, once.** TypeScript never builds HTML. When it shows data (a cart line, a
   toast, a search suggestion) it fills a `<template>` the server rendered. Money and dates arrive
   formatted by the server, with one exception: the cart, which the browser computes in integer
   piastres and formats with `core/money.ts`, an exact port of the server's formatter.
4. **Progressive enhancement:** every page and form works without JS.
5. **Fail loudly in development, gracefully for shoppers.**
6. **Small:** the JavaScript of the first load and of each page has a budget (below).

**The cart.** The browser owns it, in a cookie: `product:variant:qty[:addons]` lines joined by `|`.
The format has two identical implementations, `crates/domain/src/cart.rs` and
`client/src/stores/cart.ts`.
- Each line also keeps a **snapshot** from the card or product page it was added from: name,
  image, variant, add-ons, and prices in piastres (unit, was, add-ons, stock cap). Snapshots are
  kept in `localStorage`, per shop and language.
- Adding, removing and changing quantities write the cookie. The browser computes line totals,
  subtotal, savings and the delivery rule from the snapshots: instant, with no request.
- The server is asked only for what the browser lacks. `/api/cart/quote` fills lines that have no
  snapshot (added without JS, or storage cleared).
- `/cart` and checkout render the server's prices from the cookie. Checkout re-prices with
  madar-shared and re-checks stock, like Madar at order intake, and says so if a price moved.
- Without JS, plain forms rewrite the cookie on the server, and nothing else changes.

**Caching:**
- **Server:** catalogue reads go through a moka cache keyed `tenant:generation:kind:key` (the shape
  of Madar's `MenuCache`; bumping the generation invalidates a shop). Catalogue pages send
  `CDN-Cache-Control` and `Cache-Tag` for edge caching and targeted purges. Personal responses are
  `private, no-store`.
- **Client:** `core/cache.ts` keeps pages and API reads with a fresh window (TTL), then serves them
  stale while revalidating. Entries are invalidated by TTL, by a refresh (pull-to-refresh), and
  when the server's catalogue generation changes (`x-catalog-gen`). Static assets are
  content-hashed and `immutable`.

**Crates and folders:**

```
crates/domain    domain types (copied from Madar, origins noted) and the cart cookie format
crates/app       pages, components, server functions, data layer (ssr), strings, build.rs
crates/server    Axum: tenancy, prefs/sim endpoints, media and the TS bundle with caching
crates/frontend  an empty placeholder cargo-leptos requires (no WASM is loaded)
client/          TypeScript: core/ (router, cache, net, dom, mount, motion, drawer, stepper),
                 stores/ (cart, toasts), features/ (one file per area), api/types.ts
data/            catalogue JSON, WebP image ladders, search synonyms
```

## Size report

### Stage 2: SSR + TypeScript (what ships)

⟨TBD after integration: the client bundle (raw · gzip · brotli), HTML per page, what each page
downloads⟩

### Stage 1: the islands build (measured before the rebuild)

These are release builds of the islands version (2026-10-26). Brotli is what browsers download.

| Build | Main module: raw · gzip · **brotli** | What a page downloads |
|---|---|---|
| **One bundle** (`--release`) | 2,787 KB · 1,085 KB · **769 KB** | 769 KB on the first visit, in one file; then cached |
| **Split** (`--split --lib-features split`) | 1,235 KB · 498 KB · **379 KB** | 379 KB plus that page's chunks: 29 lazy islands, plus 217 small chunks of code they share |

Per page with the split build, measured in Chromium against the release server (bytes as sent):

| Page | WASM files | WASM | JS | CSS | HTML |
|---|---:|---:|---:|---:|---:|
| Home | 1 | 379 KB | 13 KB | 28 KB | 34–37 KB |
| Cart, checkout | 1 | 379 KB | 13 KB | 28 KB | 19–21 KB |
| Listing, search | 83 | 578 KB | 13 KB | 28 KB | 28–29 KB |
| Product | 107 | 581 KB | 13 KB | 28 KB | 29 KB |
| `/ui` | 37 | 431 KB | 13 KB | 28 KB | 41 KB |
| `/showcase` | 210 | 713 KB | 13 KB | 28 KB | 64 KB |

The largest lazy chunks (brotli) are:
- listing explorer: 121 KB;
- purchase panel: 90 KB;
- checkout address form: 60 KB;
- gallery: 31 KB;
- order tracking: 21 KB.

The 379 KB main module splits roughly in half:
- **About 190 KB is Leptos's floor:** the reactive runtime, hydration, serde (island props and
  server-function responses), the server-function client, instant search and toasts. This was
  measured before the cart merged.
- **About 190 KB is the cart:** drawer, quick add, and the optimistic update queue. Every page pays
  for it, because the cart button lives in the header.

Two more things counted against it:
- **Splitting needs HTTP/2/3.** Over plain HTTP/1.1 (this preview's tailnet link, through a
  ~200 ms relay), the product page's 107 chunk requests cost more than they saved.
- **The build settings were already tight.** The release profile was already `opt-level = "z"`,
  LTO, one codegen unit, `panic = "abort"` and `strip`. The weight came from rendering big UIs in
  the browser, not from settings.

**What we did about it:** the rebuild. The browser keeps state and runs behaviour in TypeScript;
Rust renders everything on the server. See Stage 2.

## What came from Madar

| From Madar | Where it lives here | How |
|---|---|---|
| Menu/catalogue types: `DeliveryMenuCategory`, `DeliveryMenuItem`, `DeliveryModifierGroup`, `DeliveryModifierOption` (MadarRust `src/delivery/public.rs`) | `crates/domain/src/catalog.rs` | Copied and trimmed, with the origin noted on each type. Added for retail: slug, SKU, stock, options/variants. MadarRust itself pulls in actix + sqlx and can't build for WASM. |
| Money in integer piastres; `name_translations {en, ar}` | `domain::money`, `domain::text::L10n` | Copied. |
| Order statuses (`delivery_order_status`) and their en/ar labels (MadarDashboard i18n) | `domain::order` | Copied. |
| `CartLineInput`, `DeliveryTracking` shapes | `domain::order` | Copied and trimmed. |
| Pricing (`madar_catalog::price_line`) and VAT (`madar_money::tax::compute`) | server, `app::server::pricing` | **Used as is**: a git dependency on `madar-shared`, pinned to tag `v0.12.0`. The browser computes the cart's running totals from snapshot prices; the server's pricing decides at checkout. |
| Money and time display (MadarDashboard `format.ts` / madar-core `display.rs`): Western digits by default, bidi isolation of figures, 12-hour Cairo time, Egyptian month names | `app::format`; for the cart, `client/src/core/money.ts` | Ported. The TypeScript port gives the same output for the same input. |
| Per-shop subdomains (`hostSlug`, `tenant_shell.rs`) | `app::tenant` | Same rule: the tenant is the first hostname label. |
| Read-cache key shape (`MenuCache` `"{org}:{version}:{variant}"`) | `app::server::cache::ReadCache` | Same shape and invalidation by version bump. |
| Data caching on the client: TTL, stale-while-revalidate, refresh (the Madar app's data layer) | `client/src/core/cache.ts` | Same model, applied to pages and API reads. |
| Re-pricing at order intake | checkout (`pages/checkout.rs`) | Same rule: the client's cart is never trusted for prices; the server re-prices and re-checks stock. |
| Prefetch gesture model (MadarDashboard `src/lib/prefetch/intent.ts`) | `client/src/core/router.ts` | Copied exactly: hover 65 ms, finger-down, de-dupe, budget, and skipping Save-Data/2G. |
| Arabic register: Egyptian colloquial on shop pages, Modern Standard Arabic for errors | all UI copy | Same convention. |
| WebP-only media | `tools/media/process.py` | Same choice, with a width ladder and LQIP. |

**Deliberately not taken:** Madar's design tokens. This preview uses its own "Porcelain" palette
with a graphite accent, Apple-style motion, and squircle corners. Madar's UI components (React /
Flutter) aren't portable to Leptos; the client is plain TypeScript with no framework.

## Rough edges

These are ordered by how much they cost us. Everything here was hit while building this preview and
checked against the running server, with Leptos 0.8.22, leptos_axum 0.8.10 and cargo-leptos 0.3.11.

Sections 1, 4, 5 and 6 are about **islands**. They are why the client moved to TypeScript, and they
no longer apply to what ships. Sections 2, 3, 7, 8 and 9 still apply to Leptos on the server.
Sections 10 and 11 are what the SSR + TypeScript architecture costs.

### 1. The islands router is a work in progress, and it needs a client shim
The router's own README calls it "Work in Progress". It fetches the next page and diffs it into the
DOM. Each of these needed code in `shell.js`:
- **`<head>` drift.** The diff walks `<head>` in lockstep. When two pages have different numbers of
  head tags (home has about 10 more OG/JSON-LD tags), the walkers drift, the next page's body lands
  inside the old `<head>`, and the screen keeps showing the previous page. Fix: align the heads
  before diffing.
- **`<noscript>` goes live.** Fetched HTML is parsed with `DOMParser` (scripting off), so
  `<noscript>` contents become real elements. A `<noscript><meta http-equiv=refresh>` sent JS users
  to the no-JS page. Fix: empty `<noscript>` before diffing.
- **HTTP cache.** Router fetches use the browser cache, so with cookie-based preferences a page came
  back in the old language. Fix: `cache: "no-cache"`.
- **Forms lose the pressed button.** It posts `new FormData(form)` without the submitter, so
  `name/value` on submit buttons only works without JS. Fix: forward the submitter.
- **Scroll.** It scrolled to the top after *every* POST to another path, including every `/api/…`
  server-function post. Fix: scroll only when the URL you land on changes.
- **Back.** It always refetches the page and, about 0.4 s later, rewrites the URL. An island that
  owns URL state (filters, `?v=`) must handle `popstate` itself, in the capture phase.
- **No router inside islands.** `use_query_map` and `use_navigate` are server-component tools.
  Islands talk to `history` directly.
- **Islands aren't diffed.** This keeps their state, but it means text inside an island doesn't
  follow a server-side language switch. Every island renders its strings from a shared signal.
- **Weight.** `mark_branches` adds `<!--bo-…-->` markers around every view branch: 992 markers,
  21 KB of the 199 KB release home page, and 57% of the debug one. They compress well, but the
  server builds every byte.

### 2. Streaming modes are per route, and the defaults bite
- **Out-of-order** (the default) sends each resolved `<Suspense>` as a `<template>` plus an inline
  script. Without JS those sections stay skeletons forever, and meta tags that depend on async data
  miss the already-sent `<head>`. Hence in-order rendering on every page that must work without JS,
  and a `/nojs` twin of the home page.
- **`PartiallyBlocked` doesn't block** in leptos_axum 0.8. A `Resource::new_blocking` only delays
  the first byte; its `<Suspense>` still streams as a `<template>`, and `<Title>/<Meta>` inside it
  never reach `<head>`. The product page builds its head synchronously from an in-memory read.
- **An `<ErrorBoundary>` containing a `<Suspense>`** holds back everything after it in the stream
  until it resolves (the rest of a page at about 917 ms instead of 18 ms). Keep the Suspense
  outside and the boundary inside, per section.
- **A keyed `<For>` item** renders only after everything inside it resolves. A Suspense nested in
  it froze on its fallback.
- **Headers and status leave with the first chunk.** Cache headers and 404s are decided
  synchronously at the top of each page.
- **Islands-router navigations always get in-order HTML**, so a client-side visit to a streaming
  page waits for its slowest section.

### 3. Two server-side lifecycle traps
- **Thread-bound values.** `StoredValue::new_local`, `Action::new_local`, `LocalResource` and our
  spring animation all wrap values in a `SendWrapper`. Create one in a component or island body,
  and on a multi-threaded tokio server it can be created on one worker and dropped on another when
  the stream ends. The server then panics ("Dropped SendWrapper… from a different thread") and the
  HTML is truncated, *intermittently*. Three different parts of this app hit it.
  `Action::new_unsync` doesn't help: in 0.8.22 it goes through the same `SendWrapper`. Rule:
  browser-only state is created only in the browser build or inside effects; server-visible
  actions use `Action::new` with a `Send` future.
- **A client that hangs up mid-stream.** Page reads run as spawned tasks and keep running after the
  client disconnects (a cancelled prefetch, a closed tab, a navigation). By then the request's
  owner is disposed, so a context lookup after an `.await` panics ("expected context … AppState"),
  and so does a stored value read by late rendering ("already disposed"). Aborting a few streamed
  pages under simulated latency produced 69 panics. Rule: take everything from context *before*
  the first `.await`, and make process-wide state reachable without context.

### 4. Hydration is fragile in specific ways
- **An island that is still pending at hydration** (a `Suspense` over a resource that isn't ready
  synchronously) hydrates on a full load. After a router navigation into that page it **panics**,
  and the panic cascades to every island hydrated after it, the header's cart badge included.
  Islands here are seeded from props only, and fetch only in response to the user.
- **A lazy island built without `--split`** makes the JS glue import `__wasm_split_placeholder__`,
  the module fails to load, and **no island on any page hydrates**. That's why lazy is behind a
  feature here.

### 5. `Action` keeps the last response to arrive, not the last one sent
Its `dispatched` counter is never advanced, so fast repeated taps can show a stale quantity. Cart
controls send through one queue that sends one change at a time, merges repeated taps, and rolls
back on failure, around the same `<ActionForm>` forms.

### 6. WASM is not small, and the levers are architectural
- With every island in one bundle it's 769 KB brotli, including 23 demo islands. Lazy page
  islands bring the main module to 379 KB, but the product page then fetches 107 chunk files (see
  the size report).
- Lazy islands load when they hydrate, not when they're first used, so they shrink the
  *first* page rather than the work done on that page.
- **Serde decoding is compiled twice:** once for island props, once for server-function
  responses.
- **A client-rendered results grid costs about 200 KB brotli** (the listing explorer). Letting the
  server render it and the router swap it would be smaller, at the cost of the
  keep-old-results-while-loading feel.

### 7. The edit–compile loop
On this machine (6 cores, sccache, mold):
- A one-line change in the app crate takes about **15 s** to rebuild the server alone, and
  **36–48 s** when the WASM rebuilds too. A cold release build takes **5 m 24 s**.
- A passing `cargo check` doesn't guarantee a passing `cargo leptos watch`: dev builds compile
  `view!` code with `--cfg erase_components`.
- Debug HTML is about three times release size, because of hot-reload markers.

### 8. Papercuts
- `#[prop(optional)] x: Option<T>` makes the prop take `T`.
- `view!` can move a value before an earlier attribute clones it, giving "borrow of moved value"
  inside the macro.
- A bare `>` or `<` in an attribute closure ends the tag: Rust source appeared on the page. Wrap
  the body in braces.
- server_fn form posts reject repeated keys, so a multi-select needs a `Vec<String>` endpoint.
  Its redirects leave a bare `?` on the URL.
- Not Leptos: the `text_direction_codepoint_in_comment` lint is deny-by-default, so bidi isolates
  in doc comments fail the build.

### 9. Version churn
leptos 0.9.0-rc0 shipped on 2026-10-15, and the islands router is still marked WIP. Adopting 0.8
now means planning a migration soon.

### 10. Leptos without hydration still assumes hydration
These apply to Stage 2, which renders on the server only:
- **Bootstrap scripts.** Every page carries two inline scripts that set up hydration state
  (`__RESOLVED_RESOURCES=[]…`, `__INCOMPLETE_CHUNKS=[]`), although nothing hydrates. They're tiny,
  but they carry a per-response nonce. A strict Content-Security-Policy therefore has to come from
  the app, which knows the nonce, not from nginx.
- **Resources ship their data.** A `Resource` read during server rendering is serialised into the
  page for hydration, even when no client will read it. Pages here read through `Suspend` with
  plain futures instead.
- **A WASM crate nobody loads.** cargo-leptos still requires a library crate for the browser. It
  builds an empty one, downloads `wasm-opt` during the build, and writes `.wasm`, glue JS and
  `.d.ts` files into the site. The image deletes them.
- **Compile-time environment.** `leptos` and `leptos_config` read `LEPTOS_OUTPUT_NAME` with
  `option_env!`, and cargo-leptos sets it. A Docker dependency layer built without it is silently
  rebuilt in the final step, so the Dockerfile sets it for both.

### 11. Two languages share contracts by hand
The browser code is TypeScript, so everything that crosses the boundary exists twice, and nothing
but care keeps the two copies identical:
- the cart cookie format (`crates/domain/src/cart.rs`, `client/src/stores/cart.ts`);
- the money formatter (`crate::format::money`, `client/src/core/money.ts`);
- the API types (`api::types`, `client/src/api/types.ts`).

A code generator (ts-rs or specta) would remove the third. The first two are small and stable.

### Our own choices that cost time (not Leptos)
- **Display preferences in cookies:** they fight public caching. The fix is `no-cache` for
  browsers plus an edge cache key on the preference cookies. Language in the URL (`/ar/…`) would
  be simpler.
- **One cargo `build-dir` shared across git worktrees:** identical relative paths made dev servers
  silently link another worktree's code. Fix: `{workspace-path-hash}`.

## Verdict

**Leptos is ready as a server renderer. It isn't ready to run a phone-first shop in the browser.**
Use it the way Stage 2 does: Leptos SSR on Axum for every page, with a small TypeScript client.

**On the server, it's good.**
- Every page here is a Leptos component tree rendered on Axum, complete and in order, in 2–6 ms
  (median, release build, compression included).
  It works without JavaScript. `#[component]`s compose the way a React developer expects, and
  server functions with fixed URLs give typed data endpoints with almost no code.
- The strongest argument for Rust in Madar's web layer is sharing, not speed. madar-shared's
  pricing and VAT crates run unchanged at checkout, so the storefront prices exactly as Madar's
  backend does instead of re-implementing it.
- The server-side traps (rough edges 2, 3 and 10) cost time once. They're avoidable now that
  they're rules in `AGENTS.md`.

**In the browser, it isn't ready yet.**
- Stage 1 (islands) shipped 379–769 KB of WASM, brotli, to every first visit. Stage 2 does the
  same jobs with **10.9 KB** of JavaScript on the first load and 24.7 KB on the heaviest page
  (brotli). That's 35–70 times less, and every page still renders on the server first.
- The islands router is marked work in progress and needed a client shim. Hydration had races we
  never root-caused: toasts and cart updates were lost after navigation. Lazy islands traded one
  379 KB file for over 100 requests.
- The weight is architectural, from rendering large UIs in the browser in WASM, not a build
  setting. The profile was already `opt-level = "z"`, LTO and one codegen unit.

**What it costs to build with.**
- **The compile loop is the daily cost.** A one-line change takes about 15 s to show (server
  only), a cold release build 5½ minutes, and an image built without a cache about 25 minutes on
  this machine (fat LTO; cargo-leptos's own install, which CI caches, is 10 of them). JavaScript
  stacks reload in under a second.
- **Two languages in the web layer.** The client is TypeScript either way. What crosses the
  boundary (the cart cookie, the money formatter, the API types) is written twice and kept
  identical by hand.
- **Version churn.** 0.9 is at release candidate, so adopting 0.8 means planning a migration
  within months.
- **Macro papercuts** (rough edge 8) slow newcomers down more than they slow old hands.

**Recommendation.** For Madar's customer-facing web:
- Leptos SSR on Axum, rendering in order, with server functions on fixed URLs.
- A TypeScript client that owns the state, uses no framework, and has a size budget.
- Money and pricing stay in madar-shared, called from the server.
- Revisit islands or full hydration when the islands router is out of work in progress and a
  storefront's WASM is within a few times the TypeScript client's size, not 35–70 times.
