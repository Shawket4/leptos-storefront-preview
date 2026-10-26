# Working on the storefront (rules for every agent)

A **Leptos 0.8.22 SSR** storefront on Axum with a **TypeScript** client — **no WASM**. Two
tenants by Host header. Read this whole file before writing code; it was rewritten on 2026-10-26
when the project moved off WASM islands (they shipped 380–770 KB of WASM per page and needed
fragile hydration). Ask the lead before departing from anything here.

## Principles (the owner's; reviewed on every merge)

1. **The server renders data; the client owns state.** Every page is server-rendered by Leptos
   with the API data it needs (catalogue, prices, orders). The browser code changes *state*
   (cart contents, open drawers, chosen variant, quantities, filter panel, toasts) locally and
   instantly — it talks to the server **only when it needs data it doesn't have** (new listing
   results, search suggestions, the cart's prices/stock, order status).
2. **One source for everything.** One store per concern, created once (`client/src/stores/*`),
   imported by whoever needs it — no feature keeps its own copy of server data or of another
   store. One path per action (the cart is changed only through `stores/cart.ts`). One network
   layer (`core/net.ts`), one data cache (`core/cache.ts`), one router (`core/router.ts`), one
   toast queue, one string catalogue. If you need something core doesn't offer, ask the lead —
   don't build a local variant.
3. **Markup lives in Rust, once.** The client never builds HTML strings. When it must show data
   (a cart line, a toast, a search suggestion) it fills a server-rendered `<template>` via
   `fill()` (`core/dom.ts`). Money, dates and numbers arrive **formatted by the server**
   (`crate::format`, the visitor's numerals) — the client never formats or computes money.
4. **Progressive enhancement.** Pages work without JS (plain links and forms, server-side
   validation); the TS makes them instant. Forms the TS takes over keep a working `action`.
5. **Fail loudly in dev, gracefully for shoppers.** No silent fallbacks that create a second
   instance of anything. Network failures show the shared offline/retry states.
6. **Small.** The whole client stays under **30 KB gzip**; CSS under 35 KB gzip.

## Map and ownership

| Path | What | Owner |
|---|---|---|
| `crates/domain` | domain types (copied from Madar, origins noted); `cart.rs` = the cart cookie | lead |
| `crates/app/src/{lib,request,prefs,tenant,format,media,client_js}.rs`, `strings/mod.rs` | shell, routes, contexts, formatting, string catalogue | lead |
| `crates/app/src/server/{mod,catalog,search,cache,sim,pricing}.rs`, `api/{mod,types}.rs` | data layer (catalogue reads) | lead |
| `client/src/main.ts`, `client/src/api/types.ts`, `client/build.mjs`, `crates/app/build.rs` | client entry, API shapes, build | lead |
| `client/src/core/*`, `client/src/stores/{toasts,prefs}.ts`, `client/src/features/core.ts`, `client/legacy/` | router, page cache, net, motion, mount, toasts, offline, nav feedback | core agent |
| `crates/app/src/components/*` (design system, shell parts, header, toaster, offline), `style/components/*`, `style/{tokens,base,main}.css`, `crates/server/src/main.rs` | design system + static serving | core agent |
| `client/src/stores/cart.ts` (API by the lead), `client/src/features/{cart,checkout}.ts`, `crates/app/src/pages/{cart*,checkout*,order*,orders_lookup.rs}`, `api/cart.rs`, `server/{cart,orders}.rs`, `components/product_card.rs` (its quick-add part), `strings/{cart,checkout}.rs`, `style/pages/{cart,checkout,order}.css` | cart, checkout, orders | cart agent |
| `client/src/features/{browse,product}.ts`, `crates/app/src/pages/{home,category*,search*,product*}`, `strings/{browse,product}.rs`, `style/pages/{home,category,search,product}.css` | home, listing, search, product page | browse agent |
| `client/src/features/showcase.ts`, `crates/app/src/pages/{showcase*,ui_kit.rs,errors.rs}`, `strings/showcase.rs`, `style/pages/{showcase,ui-kit,errors}.css` | /showcase, /ui, error pages | showcase agent |
| `data/`, `tools/media/`, `tools/catalog/` | fixtures and images | — (don't edit) |

Only edit files you own. Need something from a file you don't own (a core API, a data-layer
function, a route, a type)? Ask the lead with SendMessage, or add a clearly marked minimal local
version and say so in your report. `client/src/api/types.ts`: you may add *your own section*.

## Builds, worktrees, ports

Each agent works in its own git worktree and branch (the path is in your brief).

```bash
cd <your worktree>
npm ci --prefix client                      # once per worktree (esbuild + TypeScript, pinned)
cargo check -p server                       # the whole app; runs tsc + esbuild through build.rs
./client/node_modules/.bin/tsc --noEmit -p client   # TS only (fast)
DATA_DIR=/home/apex/ClaudeProjects/leptos-storefront-preview/data \
MEDIA_DIR=/home/apex/ClaudeProjects/leptos-storefront-preview/data/media \
LEPTOS_SITE_ADDR=127.0.0.1:<port> LEPTOS_RELOAD_PORT=<port+100> cargo leptos watch
# then http://nile.localhost:<port>/  and  http://souq.localhost:<port>/
```

`cargo leptos watch` rebuilds on Rust *and* TS changes (`watch-additional-files`). There is no WASM
build any more (`crates/frontend` is an empty placeholder cargo-leptos requires). Before handing off:
`cargo check -p server` with **no new warnings** and `tsc` clean. Format only your own files
(`rustfmt --edition 2024 <file>`); never `cargo fmt` the workspace.

**Never run test suites, Lighthouse or load/benchmark tools** (owner's rule — the lead asks first).
Verification is done in a real browser with Playwright (`tools/shots/`):

- **Phone conditions** for anything interactive: 390×844, touch, and network emulation via CDP
  (`latency: 200`, `downloadThroughput: 3e6/8`, `uploadThroughput: 1e6/8`) — the owner tests on a
  phone over a slow relay. Tap things *during* load too.
- Screenshots at 360 and 1280, `en`/`ar`, light/dark (`node tools/shots/shoot.mjs …`); fix every
  console error, failed request and horizontal overflow it prints. Check 360 px Arabic dark.
- Walk the flows across **client-side navigations**, not only full loads.

Commit on your branch in small steps (`git -c user.name="Shawket Ibrahim" -c
user.email="shawketibrahim7@gmail.com" commit`), trailer `Co-Authored-By: Claude Opus 5.5
<noreply@anthropic.com>`. Merge `main` into your branch when the lead says so; don't rebase or push.

## Server side (Leptos SSR)

- Everything is a `#[component]` rendered on the server; there are **no islands** and no client
  Rust. Event handlers in `view!` do nothing (they never run) — delete them when you touch a file;
  interaction is TS. Code under `#[cfg(feature = "hydrate")]` is dead: delete it in your area.
- All routes are `SsrMode::InOrder` (one complete HTML document, works without JS). Response
  headers and status are set synchronously at the top of the page component.
- Hook TS behaviour onto markup with **`data-ts="<feature>"`** (space-separated if several) and
  `data-*` for its inputs (ids, limits). Pass bigger data as `<script type="application/json"
  data-ts-json>` inside the element. Templates the client fills: `<template id="tpl-<name>">`
  with `data-slot="<key>"` (text) and `data-attr-<attr>="<key>"` (attributes).
- Data endpoints for the client: server functions with **fixed** URLs, e.g.
  `#[server(endpoint = "cart/quote", input = GetUrl)]` → `/api/cart/quote`; JSON out, money
  pre-formatted; `cache::private()` for anything personal. Add the TS shape to `client/src/api/types.ts`.
- `<head>`: title/meta/OG/JSON-LD synchronously from in-memory data (no async head).

## Client code (TypeScript) — `client/src`

Bundled into one ES module by esbuild (via `crates/app/build.rs`), embedded in the server and
served at `/js/app-<hash>.js` (cached forever). `main.ts` boots: `readPage()` → stores →
`mountAll()` → `startRouter()`; after each client-side navigation it re-reads the page and
mounts the new regions.

| Module | API (stable — ask the lead before changing it) |
|---|---|
| `core/page.ts` | `cfg()` (tenant, lang, numerals, `api.*` endpoints, cart limits), `t(key, vars?)`, `readPage()` |
| `core/dom.ts` | `$`, `$$`, `on(root, type, selector, fn)` (delegated), `fill(templateName, values)`, `prefersReducedMotion()` |
| `core/mount.ts` | `register(feature, el => cleanup?)`; `mountAll(root)`, `unmountAll(root)` (router) |
| `core/store.ts` | `createStore(initial)` → `get/set/update/subscribe` — only `stores/*` create stores |
| `core/net.ts` | `request`, `getJSON<T>(url, opts)`, `NetError` (`offline/timeout/http/parse/aborted`), `isOnline`, `onOnline` |
| `core/cache.ts` | `cached(key, {ttl, swr}, fetcher, onUpdate?)`, `key(...parts)`, `peek`, `invalidate`, `noteGeneration` |
| `core/router.ts` | `startRouter`, `navigate(url, {replace, scroll: "top"|"keep"})`, `refresh()`, `prefetch(url)`, `onNavigate(fn)`, `routing` (store: `pending`, `to`) |
| `core/motion.ts` | `SpringValue`, `SPRING`, `project`, `rubberband`, `VelocityTracker` |
| `core/drawer.ts` (core agent) | **The one drawer/sheet.** Markup: `components::drawer::Drawer` (Rust) → `data-ts="drawer"` with an `id` and `data-side="end"|"start"|"bottom"`; triggers anywhere: `[data-drawer-open="<id>"]`; inside: `[data-drawer-close]`. TS: `openDrawer(id)`, `closeDrawer(id)`; the element fires `drawer:open` / `drawer:close`. Springs, swipe to dismiss, focus trap, Esc, scroll lock, `inert` background, reduced motion — features never write their own. **A drawer open when a navigation swaps it out comes back open in the new page (same `id`), instantly, with its scroll position** (the listing's filter sheet re-renders on every filter tap) |
| `core/stepper.ts` (core agent) | **The one quantity stepper.** Markup: `components::stepper::Stepper(name, value, min, max, label)` → `data-ts="stepper"` with `data-min`/`data-max` around − / `<input type=number name=… value=…>` / + (works in a plain form without JS); TS clamps and fires `change` on the input. Features listen; they don't build steppers |
| `core/cache.ts` additions (core agent) | `inspect()` → `{key, ageMs, ttl, swr}[]` and `stats()` → `{hits, misses, stale}` (read-only, for the showcase) |
| `stores/cart.ts` | `cart` (store: `lines`, `count`), `add(item, snapshot?)`, `setQty(key, n)`, `remove(key)` → undo, `clear()`, `lineKey`, `snapshot(key)`, `quote(onUpdate?)`, `syncFromCookie()` |
| `stores/toasts.ts` | `toast(msg, kind, action?)`, `dismiss(id)` |
| `features/cart.ts` (cart agent) | `addAndConfirm(item, snapshot)`: the one "add to cart" with its feedback (toast, badge pulse) — quick add and the product page both use it |

Rules: features register in their own `features/<area>.ts`; they touch the DOM only inside the
element they're mounted on (plus the page singletons: header badge, drawer, toaster); they subscribe
to stores and clean up in the returned function; strings via `t()`; no `innerHTML` with data.

## The cart (contract)

- The browser owns the cart. It lives in the **`cart` cookie** (not HttpOnly):
  `product:variant:qty[:addon+addon]` lines joined by `|` — `crates/domain/src/cart.rs` and
  `client/src/stores/cart.ts` are the two (identical) implementations. Limits: 50 lines, 99 each.
- Add / remove / change quantity = `stores/cart.ts` → cookie + store, instantly, **no request**.
- The server reads the cookie when it needs to *price* the cart: `/cart`, the drawer's quote
  (`GET /api/cart/quote` → `CartQuote`), checkout (re-prices and checks stock, like Madar at order
  intake). It writes the cookie only for no-JS forms and clears it when an order is placed.
- **No-JS cart forms** (cart agent serves them; anyone may post them; with JS the TS intercepts
  them and nothing is sent). `application/x-www-form-urlencoded` POSTs, plain HTML field names:
  | Endpoint | Fields |
  |---|---|
  | `POST /cart/add` | `product`, `variant`, `qty` (default 1), `addon` (repeated, one per chosen add-on), `next` |
  | `POST /cart/set` | `key` (a line key, `product:variant[:addons]`), `qty` (0 removes), `next` |
  | `POST /cart/remove` | `key`, `next` |

  Every form carries a hidden `next` (a same-site path; the page's own URL). The server answers
  `303` to `next` (else the Referer, else `/cart`) with `?added=<qty>` after a successful add, or
  `?cart_err=<code>` on failure — codes: `out_of_stock`, `max_qty`, `full`, `addon_required`,
  `bad_addon`, `unknown_product`, `bad_request`. The receiving page renders the notice server-side
  (strings in `strings/cart.rs`: `cart.err.<code>`, `cart.added`) and drops the param with
  `history.replaceState`.
- Personal → never in a public page's HTML: the header badge is filled by TS from the cookie.

## Strings (catalogue)

`crates/app/src/strings/<area>.rs`: `s("area.key", "English", "عربي")`. All of them reach the page as
`#app-strings`; `t("area.key", {name})` in TS, `strings::get("area.key", lang)` in Rust. Keys are
unique (checked at startup in debug). Arabic register: warm Egyptian colloquial on shop pages,
clear MSA for errors and legal text.

## Language, numerals, direction

- Every user-visible string exists in English and Arabic. Strings the browser code shows, and any
  string used in more than one place, live in the catalogue (`crates/app/src/strings/<area>.rs`,
  `t("area.key")` in TS, `strings::get(key, lang)` in Rust). One-off server labels may stay inline
  as `tr(lang, "…", "…")`. Data strings: `L10n::pick(lang.is_rtl())`.
- Arabic register: shop pages use warm Egyptian colloquial (like Madar's shop pages); errors and
  legal text in clear MSA. Read your Arabic once more as an Egyptian customer would.
- Numbers, money, dates: always through `crate::format` (`money`, `MoneyParts`, `int`, `count`,
  `dec1`, `items`, `time_cairo`, `date_cairo`) with the visitor's `Numerals`. Never `format!` a
  number into UI text directly.
- RTL is a mirror, not a flip: use **logical CSS properties only** (`margin-inline-start`,
  `inset-inline-end`, `padding-block`, `text-align: start`…), never left/right. Directional icons
  (arrows, chevrons, "back") get the `.flip-rtl` class. Figures stay LTR (`format` already isolates
  them). Don't mirror logos, product photos or media controls.


## Design rules (owner decisions, 2026-10-26 — supersede everything earlier)

Read **`.claude/skills/apple-design/SKILL.md`** (Emil Kowalski's *apple-design* skill: Apple's
WWDC design principles translated for the web) and apply it to every component and interaction.

- **One global palette for every shop — "Porcelain" + graphite.** Colours are platform-wide; there
  are no per-tenant colour tokens (tenants differ by logo, name, language, catalogue). Not Madar's
  look.
  - Light (the **default**, even on dark-mode phones): `--page #FAFAF9` · `--surface #FFFFFF` ·
    `--surface-2 #FFFFFF` · `--sunk #F1F1F0` · `--well #F4F4F3` · `--text #111111` ·
    `--text-2 #52525B` · `--text-3 #71717A` · `--line rgb(17 17 17/.08)` ·
    `--line-strong rgb(17 17 17/.16)` · `--rule #E4E4E7` · `--ink #111111` · `--on-ink #FAFAF9`.
  - Dark (opt-in via the theme control: Dark, or System on a dark device): `--page #0B0B0C` ·
    `--surface #18181B` · `--surface-2 #1F1F23` · `--sunk #232326` · `--well #2C2C30` ·
    `--text #F4F4F5` · `--text-2 #B4B4BB` · `--text-3 #8E8E96` · `--line rgb(244 244 245/.10)` ·
    `--rule #27272A` · `--ink #F4F4F5` · `--on-ink #0B0B0C`.
  - Accent = **graphite**, a plain global token in `tokens.css` (no per-tenant colour tokens — owner:
    over-engineering): `--brand #111111` / `--on-brand #FFFFFF` (dark: `#F4F4F5` / `#111111`), so
    primary buttons are near-black in light mode and near-white in dark. `--brand-wash` = `color-mix(in oklab, var(--brand) 8%, var(--surface))`.
    Colour in the UI comes from product photography and the status colours only:
    success `#15803D`/wash `#E8F5EC` (dark `#4ADE80`) · warning `#B45309`/`#FDF3E7` (`#FBBF24`) ·
    danger `#DC2626`/`#FDECEC` (`#F87171`) · info `#2563EB`/`#EAF1FE` (`#60A5FA`). Sale prices and
    discount badges use danger red.
- **No gradients** (backgrounds, buttons, scrims, skeletons). Text never on a photo scrim. Skeletons
  pulse. Translucent *material* is fine (Apple chrome: `backdrop-filter: blur(20px) saturate(180%)` on
  a semi-transparent solid) — with a `prefers-reduced-transparency` solid fallback.
- **Corners: Apple continuous corners (squircles), much less round.** Use `corner-shape: squircle`
  (progressive enhancement; plain `border-radius` elsewhere) with: `--r-xs 6` (badges) · `--r-sm 8`
  (chips, tags) · `--r-md 12` (buttons 44–48px, inputs) · `--r-lg 16` (small cards, wells) ·
  `--r-xl 20` (cards) · `--r-2xl 26` (sheets, hero, large media). **No pills** except true
  circles (round icon buttons, count badges).
- **Text must be optically centred in controls.** Cairo's vertical metrics are lopsided, so Latin
  text sits high. Fix it at the font level (`ascent-override`/`descent-override` on the Cairo
  `@font-face`, measured) and use `text-box: trim-both cap alphabetic` on buttons/chips/badges.
  Verify Latin and Arabic in 32px chips and 44–48px buttons with screenshots.
- **Type** — Cairo for all UI text, IBM Plex Mono (tabular) for prices/quantities/codes. Tracking is
  size-specific (display `-0.02…-0.03em`, body ~0, small text slightly positive; none in Arabic);
  leading tight on large text, looser on body (Arabic looser still). Spacing in rem on a 4/8px grid.
- **Motion (from the skill)** — respond on **pointer-down** (`:active` scale .97, 100ms ease-out);
  springs instead of fixed-duration curves: CSS tokens `--spring-default` (damping 1, response .35s,
  510ms), `--spring-snappy` (1, .25s, 370ms), `--spring-gentle` (1, .5s, 730ms), `--spring-bouncy`
  (0.8, .3s, 400ms — only after a flick/throw), each with a `-dur`; gesture-driven UI (drawers, bottom
  sheets, gallery swipe) uses `SpringValue` from `client/src/core/motion.ts` (interruptible: starts
  from the live on-screen value, hands off release velocity, projects momentum with Apple's
  `(v/1000)·d/(1−d)`, rubber-bands at edges). Enter and exit along the same path; anchor popovers to
  their trigger. `prefers-reduced-motion` → short cross-fades.
- **Surfaces**: cards are `--surface` on `--page` with a very subtle `--shadow-sm`, no hairline
  border by default (dark mode: a 1px `rgb(255 255 255/.06)` edge instead); hover → `--shadow-md`.
- **No layout shift**, mobile-first (flawless at 360px), 44px targets, visible `:focus-visible`,
  semantic HTML, `aria-*`, AA contrast. Icons: lucide geometry via `components::icons`.

## Caching

- **Server**: catalogue reads go through `server::cache::ReadCache` (moka, TTL + per-tenant
  generation bump = invalidate). HTTP: catalogue pages `cache::public(&[tags])` (browsers
  `no-cache` + `Vary: Cookie`; edge `CDN-Cache-Control` + `Cache-Tag`); personal (cart quote,
  checkout, orders) `cache::private()`.
- **Client**: `core/cache.ts` — pages and API reads with TTL + stale-while-revalidate, refresh,
  and invalidation when the server's `x-catalog-gen` changes. Static assets are content-hashed
  and `immutable`.

## Data layer (lead) — use it, don't re-implement it

`api::*` server functions (`nav_categories`, `home_rail`, `list_products`, `product_detail`,
`related_products`, `search_products`) and `server::current_catalog()` (synchronous, in-memory)
for server components. Types in `api::types`; `ListingQuery::{from_query,to_query,href}` is the
single source of truth for listing URLs. Simulated latency: `SIM_LATENCY_MS` / `/__sim?ms=350` /
`?fail=<widget>` (`server::sim`).
