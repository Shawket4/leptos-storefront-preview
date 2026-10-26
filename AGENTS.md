# Working on the storefront preview (rules for every build agent)

A Leptos **0.8.22** SSR + **islands** + **islands-router** storefront on Axum 0.8, two tenants by
Host header, Madar's design language. Read this whole file before writing code.

Reference source for Leptos 0.8.22 (examples, macros, integrations — the API is version-specific,
don't rely on memory): `/home/apex/work/ref/leptos-0.8.22` (read-only). Useful: `examples/islands`,
`examples/islands_router`, `examples/hackernews_islands_axum`, `leptos/src/form.rs`,
`leptos/src/hydration/islands_routing.js`, `integrations/axum/src/lib.rs`, `meta/src/*.rs`.

## Map and ownership

| Path | What | Owner |
|---|---|---|
| `crates/domain` | domain types (copied from Madar, origins noted) | lead |
| `crates/app/src/{lib,prefs,request,tenant,format,brand,media}.rs` | shell, routes, contexts, formatting | lead |
| `crates/app/src/server/` | ssr-only data layer: catalogue, caches, carts, orders | lead |
| `crates/app/src/api/` | shared server functions (catalogue reads) | lead |
| `crates/app/src/components/` + `style/` + `public/fonts/` | design system | design agent |
| `crates/app/src/pages/<page>.rs` + `style/pages/<page>.css` | one page each | that page's agent |
| `crates/frontend`, `crates/server` | hydrate entry, Axum binary | lead |
| `data/`, `tools/media/`, `tools/catalog/` | fixtures and images | asset agents |

Only edit files you own. If you need something from a file you don't own (a new component prop, a
data-layer function, a route), add a minimal local version in your own module and say so in your
report — the lead integrates. New files inside your area are fine (`components/foo.rs`,
`pages/product/gallery.rs` as a submodule, `style/pages/product.css`).

## Worktrees, builds, ports

Each code agent works in its own git worktree on its own branch (the lead creates it; the path is
in your brief). Intermediate build artifacts are shared (`build-dir` in `.cargo/config.toml`), so
dependencies are already compiled; final binaries land in your worktree's `target/`. Cargo
serialises builds on the shared dir — waiting a bit for the lock is normal.

```bash
cd <your worktree>
cargo check -p app --features ssr                                          # server build
cargo check -p app --features hydrate --target wasm32-unknown-unknown      # island build
# run it (pick the ports from your brief; data comes from the main checkout):
DATA_DIR=/home/apex/ClaudeProjects/leptos-storefront-preview/data \
MEDIA_DIR=/home/apex/ClaudeProjects/leptos-storefront-preview/data/media \
LEPTOS_SITE_ADDR=127.0.0.1:<port> LEPTOS_RELOAD_PORT=<port+100> cargo leptos watch
# then: http://nile.localhost:<port>/  and  http://souq.localhost:<port>/
```

Both `cargo check`s must pass (no new warnings) before you hand off. Format only your own files
(`rustfmt --edition 2024 <file>`); never `cargo fmt` the workspace (it would touch other agents' files).

**Never run test suites, Lighthouse or load/benchmark tools** (owner's rule — the lead asks first).
Screenshots for visual checks are expected:

```bash
node /home/apex/ClaudeProjects/leptos-storefront-preview/tools/shots/shoot.mjs \
  --url http://souq.localhost:<port>/c/spices --out /tmp/<you>/listing \
  --widths 360,768,1280 --langs en,ar --themes light,dark --full
```

It prints console errors, failed requests and horizontal overflow — fix all of them. Look at the
PNGs with the Read tool. Check 360 px Arabic dark mode explicitly; it is where layouts break.

Commit on your branch in small steps (`git -c user.name="Shawket Ibrahim" -c
user.email="shawketibrahim7@gmail.com" commit`), message trailer:
`Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`. Don't merge, rebase or push.

## Leptos 0.8 islands: what you must know

- `features = ["islands", "islands-router"]`. Every `#[component]` renders **on the server only**.
  `#[island]` components also hydrate in the browser. Only island code (and what it calls) is
  compiled into the WASM bundle, so keep islands small and push markup into server components.
- Island props must be `Serialize + Deserialize` (they are JSON in a `data-props` attribute).
  Children of an island are server-rendered HTML and opaque to it (you can't inspect or iterate
  them; to hide them conditionally, toggle CSS, don't drop them).
- Code inside a non-island `#[component]` still has to *compile* for wasm32 (dead code is dropped),
  so server-only APIs go behind `#[cfg(feature = "ssr")]` or inside `#[server]` functions.
- **Shared client state**: `frontend::hydrate()` calls `hydrate_islands()` and then
  `app::client::init()`, which provides stores under the root owner, so `use_context` works in
  every island. The display store is `prefs::use_ui()` (`lang`, `numerals`, `theme` signals).
  Additional global stores (cart, toasts) are provided the same way — the server must provide the
  same type per request (in `request::init()` / the shell) so SSR of islands works too.
- **islands-router** (`islands_router=true`): same-origin `<a>` clicks and `<form>` GET/POST
  submits are intercepted, the new page's HTML is fetched with an `Islands-Router: true` header and
  *diffed into the current DOM* inside `document.startViewTransition`. Existing islands are **not**
  diffed (their client state survives navigation); new islands are hydrated. Consequences:
  - Plain links and forms are the navigation API, so navigation works with JS disabled.
  - An island that renders text must make it reactive to `use_ui().lang` (use
    `prefs::tr_signal` / `move || tr(ui.lang.get(), …)`), because the language toggle swaps the
    server HTML around it but not inside it.
  - Router hooks (`use_params_map`, `use_query_map`, `use_navigate`) work in server components.
    **Inside islands there is no router**: read `window.location`, and update the URL with
    `history.pushState`/`replaceState` yourself; listen to `popstate`.
  - An island that handles a link/form itself must `prevent_default()` (the router skips
    events whose default was prevented).
  - Because of `mark_branches`, SSR HTML contains `<!--bo-…-->`/`<!--bc-…-->` comments. They are
    used by the diff. Don't strip them.
- **Streaming modes are per route (lead sets them in `lib.rs`)**: out-of-order streaming sends each
  resolved `<Suspense>` as a `<template>` + inline script, so **OOO sections never appear without
  JS**, and meta tags depending on suspended data miss the `<head>`. Therefore: `/` is OutOfOrder
  (with a `<noscript>` redirect to `/nojs`, the same page in order); `/c/:slug`, `/search`, `/cart`,
  `/checkout`, `/orders/:id` are **InOrder** (work fully without JS); `/p/:slug` is
  **PartiallyBlocked** — use `Resource::new_blocking` for the product so `<head>` (title, OG,
  JSON-LD) waits for it and its HTML is inline; other sections still stream. islands-router
  navigations always get in-order HTML (leptos_axum disables OOO for them).
  **Response headers and status must be set before the first chunk** — synchronously at the top of
  the page component (or, in PartiallyBlocked pages, before the blocking resource resolves).
- Data: `Resource::new(source, fetcher)` + `<Suspense fallback=…>` / `<Transition>`;
  `Suspend::new(async move { … })` inside the view; `ErrorBoundary` per section;
  `ServerAction::<Fn>::new()` + `<ActionForm action=…>` (works without JS: a plain POST to the
  server function URL, which redirects back).
- `#[server]` functions get the same context as pages: call `request::init()` first to get the
  tenant (`RequestCtx`), then use the server data layer.
- leptos_meta: `<Title text=…/>`, `<Meta name=… content=…/>`, `<Meta property="og:…" …/>`,
  `<Link rel="canonical" href=…/>`, `<Script type_="application/ld+json">{json}</Script>`.

## Language, numerals, direction

- Every user-visible string exists in English and Arabic. Static UI strings: `tr(lang, "Add to
  cart", "أضف للسلة")` next to their use. Data strings: `L10n::pick(lang.is_rtl())`.
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
  sheets, gallery swipe) uses the Rust spring animator in `components::motion` (interruptible: starts
  from the live on-screen value, hands off release velocity, projects momentum with Apple's
  `(v/1000)·d/(1−d)`, rubber-bands at edges). Enter and exit along the same path; anchor popovers to
  their trigger. `prefers-reduced-motion` → short cross-fades.
- **Surfaces**: cards are `--surface` on `--page` with a very subtle `--shadow-sm`, no hairline
  border by default (dark mode: a 1px `rgb(255 255 255/.06)` edge instead); hover → `--shadow-md`.
- **No layout shift**, mobile-first (flawless at 360px), 44px targets, visible `:focus-visible`,
  semantic HTML, `aria-*`, AA contrast. Icons: lucide geometry via `components::icons`.

## Caching (lead's helpers, in `crate::server::cache` — ssr only)

- Catalogue pages: `cache::public(&[tags])` → browsers `Cache-Control: no-cache` + `Vary: Cookie`
  (prefs are cookies), edge `CDN-Cache-Control: public, s-maxage=300, stale-while-revalidate=60`,
  `Cache-Tag: t{tenant}, t{tenant}-p{id}, t{tenant}-cat{id}`.
- Anything personalised (cart, checkout, orders): `cache::private()` → `Cache-Control: private,
  no-store`. Never render cart contents into a public page; personal bits are islands fed by
  cookies or server functions.

## Data layer (lead) — use it, don't re-implement it

Server functions (call them from a `Resource` in a server component, or from an island):

| Function | Returns |
|---|---|
| `api::nav_categories()` | `Vec<NavCategory>` (display order, counts, optional image) |
| `api::home_rail(rail: Rail, limit)` | `Vec<CardData>` — `Rail::{Featured, Deals, New, Bestsellers, Express}` |
| `api::list_products(query: ListingQuery)` | `ListingPage` (items, total, page/pages, `Facets` from the unfiltered set) |
| `api::product_detail(slug)` | `Option<ProductView>` (product incl. options/variants/modifier groups/specs, resolved `images: Vec<Img>`, brand, category) |
| `api::related_products(id, limit)` | `Vec<CardData>` |
| `api::search_products(q, limit)` | `SearchResults` (hits with `name_en`/`name_ar` highlight `Seg`s, matching categories) |
| `api::cart::{cart_get, cart_add(product, variant, qty, addons), cart_set_qty(key, qty), cart_remove(key)}` | `CartView` (lines priced by `madar-catalog`, savings, delivery fee, VAT included via `madar-money`) |

Types live in `api::types`. `ListingQuery::from_query(category, qs)` / `.to_query()` / `.href()`
are the single source of truth for listing URLs (server and islands). Errors are
`ServerFnError::new("<code>")` (codes listed at the top of `api/cart.rs`).

Server-only (`#[cfg(feature = "ssr")]`): `server::state()`, `server::current_catalog()`
(synchronous, in-memory — use it to compute cache tags before streaming starts),
`server::cache::{public, private, tag_product, tag_category}`, `server::sim::{delay, check_fail,
Widget}` (simulated latency is OFF by default — `SIM_LATENCY_MS=350` env for the server, or the
`sim` cookie via `/__sim?ms=350&next=…` per visitor; `?fail=<widget>` on the page URL makes a read
fail — use that to demo error boundaries), `state().orders()` (`place`, `get`, `next_step_at`) and
`server::orders::slots(&tenant, now_ms)`, `server::cart::{view, cookies}`,
`TenantCatalog::variant_label`.

Global client stores (`crate::stores`): `use_cart()` → `CartStore { count, view, open, pulse }`
— **the one cart signal**: the header badge reads `count`, every cart mutation calls
`store.apply(view)` with the server's response; optimistic UIs adjust `count`/`view` first and
reconcile on response. `prefs::use_ui()` → `Ui { lang, numerals, theme }`.

## Design-system contract (being built on `agent/design`; lands on main soon)

CSS tokens (`style/tokens.css`): `--page --surface --surface-2 --sunk --well --text --text-2
--text-3 --line --line-strong --rule --brand --on-brand --brand-wash --brand-text --ink --on-ink
--success(-wash) --warning(-wash) --danger(-wash) --info(-wash) --focus`; `--font-sans --font-mono
--fs-xs/sm/md/lg/xl/2xl/3xl/display --lh-tight --lh-body`; `--sp-1 … --sp-16` (4px steps: 1,2,3,4,
5,6,8,10,12,16); `--r-sm/md/lg/xl/2xl/pill`; `--dur-fast/base/slow/brand --ease-out/in/in-out/
expo/drawer/pop`; `--shadow-sm/md/float/modal`; `--gutter --container --header-h`;
`--z-header/drawer/toast`.

Components (`crate::components::…`), APIs fixed:
`icons::Icon(name, size?, class?, label?)` · `button::{Button, IconButton, ButtonKind, ButtonSize}`
· `price::{Price(amount, compare_at?, size?, from?, lang, numerals), PriceLive(amount: Signal,
compare_at: Signal, size?)}` · `rating::Rating(value, count, lang, numerals, compact?)` ·
`badge::{Badge, BadgeKind}` · `status_pill::{StatusPill, Tone}` · `image::Picture(img, sizes,
priority?, class?, vt_name?, lang)` · `skeleton::{Skeleton, SkeletonText}` ·
`product_card::{CardData, ProductCard(card, lang, numerals, priority?), ProductCardSkeleton,
ProductGrid, ProductGridSkeleton}` · `layout::{Container, Section, SectionHeader, Rail}` ·
`drawer::{Drawer(open, side?, title, children), DrawerSide}` · `toast::{show, ToastKind, Toaster}`
· `form::{Field, TextInput, TextArea, Select, RadioCard, Checkbox}` · `stepper::Stepper(value,
on_change, min, max, label)` · `empty_state::EmptyState(art, title, body, action?)` ·
`breadcrumbs::Breadcrumbs(items)` · `segmented::Segmented`.

Until it lands, page agents may put **minimal placeholders with the same names and props** in a
local `shim` submodule of their page (e.g. `pages/product/shim.rs`), and swap to the real ones when
the lead says the design system is on main (`git merge main` in your worktree, then delete the shim).
