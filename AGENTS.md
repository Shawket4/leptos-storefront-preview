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
  - Plain links and forms are the navigation API. Everything works with JS disabled.
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
- **Streaming**: routes use out-of-order streaming; each `<Suspense>` streams in when its resource
  resolves. **Response headers and status must be set synchronously, before any `Suspense`
  resolves** (headers go out with the first chunk). Use the cache helpers below at the top of the
  page component.
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

## Design rules — the storefront's own look (owner, 2026-10-26: *not* Madar's tokens, no gradients)

- **Not Madar's look.** Madar provided the domain model, money format and quality bar; the visual
  language is the storefront's own: warm **linen/sand neutrals** (page `#F6F4EF`, surface `#FFFFFF`,
  warm ink `#1A1814`, stone greys; dark mode warm charcoal `#121110` / `#1C1B19`), the tenant accent
  (`var(--brand)`) as the single primary per screen.
- **Light by default** (owner): the page renders `data-theme="light"` unless the visitor picked
  Dark or System in the theme control — even on a dark-mode phone. Dark mode must still be complete
  and polished for those who choose it.
- **No gradients.** None in backgrounds, buttons, scrims or skeletons (a *very* faint one is tolerated
  only where it is functionally needed, e.g. a rail's edge fade). Text never sits on a photo behind a
  dark scrim — put copy on solid surfaces (split layouts, solid chips/cards overlapping an image).
  Skeletons **pulse** (opacity), they don't shimmer.
- **Surfaces:** softer and rounder than Madar: radii `--r-sm 8 · --r-md 12 (controls) · --r-lg 16 ·
  --r-xl 20 (cards) · --r-2xl 28 (sheets, hero) · --r-pill`. Cards are `--surface` on the linen
  `--page` with a very subtle `--shadow-sm` and **no hairline border** by default (hairlines only to
  separate content inside a surface); hover lifts to `--shadow-md`.
- **Type:** **Cairo** (variable, Arabic + Latin) for all UI text — confident weights (600–800) for
  headings, tight Latin tracking, no tracking in Arabic; **IBM Plex Mono** with tabular figures for
  prices, quantities, order codes.
- Motion: tokens `--dur-fast 160ms`, `--dur-base 220ms`, `--dur-slow 320ms`, `--dur-brand 500ms`,
  `--ease-out cubic-bezier(.22,1,.36,1)`, `--ease-drawer cubic-bezier(.32,.72,0,1)`,
  `--ease-expo cubic-bezier(.16,1,.3,1)`, `--ease-pop cubic-bezier(.34,1.56,.64,1)`. Animate
  transform/opacity only. Press feedback scale .97. Everything respects `prefers-reduced-motion`.
- **No layout shift**: every image box has its aspect ratio reserved; every skeleton has the exact
  dimensions of what replaces it; fonts use `font-display: swap` with metric-compatible fallbacks.
- Mobile-first, flawless at 360 px; 44 px minimum tap targets; visible `:focus-visible` rings;
  semantic HTML; `aria-*` on custom controls; AA contrast.
- Icons: lucide geometry (24 grid, 2px stroke, round caps/joins) via `components::icons`.

## Caching (lead's helpers, in `crate::server::cache` — ssr only)

- Catalogue pages: `cache::public(&[tags])` → `Cache-Control: public, max-age=0, s-maxage=300,
  stale-while-revalidate=60` and `Cache-Tag: t{tenant}, t{tenant}-p{id}, t{tenant}-cat{id}`.
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
Widget}` (every read already waits its simulated latency; `?fail=<widget>` on the page URL makes it
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
