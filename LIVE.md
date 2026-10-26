# On your phone

Swap `nile` → `souq` in any URL for the Arabic-first shop.

| What | URL |
|---|---|
| **The deployed release** (the VPS, from the latest `v*` tag) | https://nile.leptos-preview.duckdns.org · https://souq.leptos-preview.duckdns.org |
| The live build of `main` on the dev machine (Tailscale on; reloads by itself on every change) | http://nile.100-123-21-13.sslip.io:3000 · http://souq.100-123-21-13.sslip.io:3000 |

Handy links (append to either host):
- light theme: `/__prefs?theme=light&next=/`;
- visible streaming: `/__sim?ms=350&next=/` (`/__sim?ms=&next=/` turns it off);
- a section failing on purpose: `/?fail=deals`;
- the guided tour: `/showcase`; the design-system kit: `/ui`.
