# Status — 2026-10-26, 15:54 (live)

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
| cart | adopting Notice, Totals, ItemRow, KeyValueList, StepIndicator, Timeline, LiveStatus and the form gaps; deleting its page CSS |
| browse | adopting LinkChips, FilterChips, Menu, Pager, FacetGroup, ChoiceChips, ChoiceList, EmptyState suggestion, Notice; deleting its page CSS |
| showcase | /ui specimens for the 15 new components; fault toggles on link chips |
| core | the Arabic font off English pages (−31 KB per first visit), scratch route out of release, `form::Radio`, no compression for fonts |
| lead | final release image build and per-page measurement once adoption lands; README sizes and verdict |

## Sizes (gzip, `node tools/size-budget.mjs`)
| | Now | Budget |
|---|---|---|
| JS first load (every page; 2 files) | 12.1 KB (10.9 KB brotli) | 15 KB |
| JS heaviest page (product + every extra) | 27.4 KB | 30 KB |
| CSS | 33.4 KB: the new components landed; drops as pages adopt them | 35 KB |
| Strings per page (`#app-strings`) | 1.4 KB, was 2.1 KB | — |

Server render time (release image): 2.1–6.3 ms median per page, compression included.
