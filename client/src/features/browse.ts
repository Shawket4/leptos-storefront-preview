// Browse = three features, one file each (each keeps its own `data-ts` names):
//   search.ts  — the header's instant search (every page)
//   listing.ts — category and /search results (filters, sort, chips, pager)
//   home.ts    — the deals countdown
// This barrel keeps main.ts's import working until it loads listing and home on demand (`lazy()`);
// then main.ts imports search.ts directly and this file goes.

import "./search";
import "./listing";
import "./home";
