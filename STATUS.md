# Status — 2026-10-26, 20:15

- **Live build of `main`:** http://nile.100-123-21-13.sslip.io:3000 · http://souq.100-123-21-13.sslip.io:3000
  (Tailscale on).
- **Repo:** https://github.com/Shawket4/leptos-storefront-preview (private), `main` pushed.
- **Deploy:** a version tag runs `.github/workflows/deploy.yaml`: build, GHCR, the VPS, a GitHub
  Release. The secrets, variables and `production` environment are set.

## Done
- **Stage 2 architecture:** Leptos SSR (every route in order, works without JS) plus a TypeScript
  client with no framework and no WASM. Features load on demand, and the first load is one shared
  chunk.
- **The cart is computed in the browser** (cookie plus priced snapshots, piastres): editing makes
  no request, and checkout re-prices on the server with madar-shared v0.12.0.
- **Editing a cart line** (variant, add-ons, quantity): each line with choices has Edit. With JS it
  opens a sheet over the drawer or /cart with the product page's own picker, the line's new total
  on the button, and Update with Undo; the editor's markup is the one request. Without JS it is
  the page `/cart/edit?key=…`, posting `POST /cart/update`.
- **Design system:** 15 new shared components, adopted by every page, with specimens on `/ui`.
  `tools/ui-lint.mjs` is clean.
- **No Madar branding** on any page. The footer badge is gone; pricing still uses madar-shared on
  the server.
- **README:** pages, the size report for both stages, what came from Madar, rough edges, verdict.
- **Phone walk** (`tools/shots/walk.mjs`, throttled to 200 ms and 3 Mbit/s): 13/13 on nile and
  souq, editing a line included.

## Sizes (`node tools/size-budget.mjs`)
| | gzip | brotli | Budget (gzip) |
|---|---|---|---|
| JS first load (every page; 2 files) | 12.4 KB | 11.2 KB | 15 KB |
| JS heaviest page (product + every extra, the line editor included) | 29.9 KB | 26.9 KB | 30 KB |
| CSS | 29.7 KB | 25.3 KB | 35 KB |

The heaviest page is now about 50 bytes under its budget: the next feature on every page needs
room made first.

## Open, for a later round (not started)
- **Drawer footer on short phones:** at 360×800 the footer (free-delivery note, totals, two
  buttons) fills about half the drawer, so only about two lines show before scrolling.
- **One more component:** the listing's min–max price boxes are still page markup (`RangeFields`).
- **Offline photos:** a cart line's photo shows as broken offline if it was never cached.
