# On your phone (Tailscale on)

Swap `nile` → `souq` in any URL for the Arabic-first shop.

| What | URL |
|---|---|
| **The storefront** (last integrated build — still the WASM-islands version until the rebuild merges) | http://nile.100-123-21-13.sslip.io:3000 |
| Core agent — router, page cache, prefetch, offline, design system | http://nile.100-123-21-13.sslip.io:3101 |
| Cart agent — cart drawer, quick add, /cart, checkout, tracking | http://souq.100-123-21-13.sslip.io:3102 |
| Browse agent — home, listing, search, product page | http://nile.100-123-21-13.sslip.io:3103 |
| Showcase agent — /showcase, /ui, error pages | http://nile.100-123-21-13.sslip.io:3104/showcase |

The agent servers are the new architecture (Leptos SSR + TypeScript, no WASM) as each agent builds
it; a refused connection means that agent is rebuilding. Each shows only its own part working —
the pieces come together on :3000 when the lead merges them.

Handy links: light theme `/__prefs?theme=light&next=/` · visible streaming `/__sim?ms=350&next=/`
(`/__sim?ms=&next=/` turns it off) · a section failing on purpose `/?fail=deals`.
