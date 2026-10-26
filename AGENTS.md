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

## Design rules (Madar's language)

- Tokens only (CSS custom properties from `style/tokens.css`); no raw hex in components.
- Borders, not shadows: white cards on the paper ground, 1px hairline, radius 16. Brand colour
  (`var(--brand)`) is the one primary per screen; the rest is ink/paper/slate.
- IBM Plex Sans Arabic for words (both scripts), IBM Plex Mono with tabular figures for prices,
  quantities, order codes.
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
