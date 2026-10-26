// Entry point. Module scripts run after the document is parsed, so the page is already there.
// Boot order: page data → stores → features → router. After every client-side navigation the
// same happens for the new regions (the router calls `mountAll` on what it swapped in).

import { readPage } from "./core/page";
import { lazy, lazyOnIntent, mountAll } from "./core/mount";
import { onNavigate, startRouter } from "./core/router";
import { syncFromCookie } from "./stores/cart";
import { mountToaster } from "./stores/toasts";

// Features register their `data-ts` names on import. One file per area, one owner per file.
// With the page, on every page: core (drawer, stepper, offline, nav feedback…), the cart (badge,
// drawer, quick add) and the header search.
import "./features/core";
import "./features/cart";
import "./features/search";
// On demand: a separate chunk, loaded the first time a page has one of its `data-ts` names
// (shared code lands in shared chunks, so nothing loads twice).
lazy(["listing"], () => import("./features/listing"));
lazy(["countdown"], () => import("./features/home"));
lazy(["variant-picker", "gallery"], () => import("./features/product"));
lazy(["checkout-form", "checkout", "order-track"], () => import("./features/checkout"));
// The cart's renderer: on /cart, and for the drawer's content on the intent to open the cart.
lazy(["cart-page"], () => import("./features/cart-view"));
lazyOnIntent(["cart-drawer"], () => import("./features/cart-view"), { triggers: '[data-drawer-open="cart-drawer"]' });
lazy(["sc-*", "kit-*"], () => import("./features/showcase"));

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
