// Entry point. Module scripts run after the document is parsed, so the page is already there.
// Boot order: page data → stores → features → router. After every client-side navigation the
// same happens for the new regions (the router calls `mountAll` on what it swapped in).

import { readPage } from "./core/page";
import { mountAll } from "./core/mount";
import { onNavigate, startRouter } from "./core/router";
import { syncFromCookie } from "./stores/cart";
import { mountToaster } from "./stores/toasts";

// Features register their `data-ts` names on import. One file per area, one owner per file.
import "./features/core";
import "./features/cart";
import "./features/checkout";
import "./features/browse";
import "./features/product";
import "./features/showcase";

readPage();
syncFromCookie();
mountToaster();
mountAll(document);
startRouter();

onNavigate(() => {
  readPage();
  syncFromCookie();
  mountAll(document);
});

// Coming back to the tab or through the back-forward cache: the cookie may have changed elsewhere.
addEventListener("pageshow", () => syncFromCookie());
document.addEventListener("visibilitychange", () => document.hidden || syncFromCookie());
