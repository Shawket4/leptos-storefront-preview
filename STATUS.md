# Status — 2026-10-26, 15:40 (live)

- **Live build of `main`:** http://nile.100-123-21-13.sslip.io:3000 · http://souq.100-123-21-13.sslip.io:3000
  (Tailscale on; `LIVE.md` has the agents' ports).
- **Repo:** https://github.com/Shawket4/leptos-storefront-preview (private), `main` pushed.

## On main
- **Stage 2 architecture:** Leptos SSR (every route in order, works without JS) plus a TypeScript
  client with no framework and no WASM. Features load on demand as content-hashed chunks.
- **The cart is computed in the browser:** the cookie plus a priced snapshot per line, in piastres.
  Editing makes no request; checkout re-prices on the server with madar-shared v0.12.0.
- **Design system:** shared components only; `tools/ui-lint.mjs` is clean across the repo.
- **Deploy pipeline:** Dockerfile, `.github/workflows/deploy.yaml`, compose, `deploy/SETUP.md`, and
  the VPS brief (`deploy/vps-agent-prompt.md`). **Waiting on the secrets.** No tag until the owner
  says they're set.

## In progress
| Who | Doing |
|---|---|
| core | `lazyOnIntent` / `ensure` in `core/mount.ts` (first); then Totals, ItemRow, KeyValueList, StepIndicator, Timeline, LiveStatus |
| design system | Notice, LinkChips, FilterChips, Menu, Pager, FacetGroup, ChoiceChips, ChoiceList, EmptyState suggestion; form gaps (lengths, grouped/required Select, always-present error slot) |
| cart | the cart drawer and /cart renderer out of the first load |
| browse | instant search out of the first load (on intent, then idle) |
| showcase | waiting for the component list: /ui specimens, fault toggles on link chips |
| lead | Docker image test build; README (Stage 2 sizes, verdict) |

## Sizes (gzip, `node tools/size-budget.mjs`)
| | Now | Budget |
|---|---|---|
| JS first load (every page) | 18.5 KB | 15 KB; cart + search on demand should bring it to about 14.3 KB |
| JS heaviest page (product + every extra) | 28.8 KB | 30 KB |
| CSS | 29.8 KB | 35 KB |
| Strings per page (`#app-strings`) | 1.4 KB, was 2.1 KB | — |
