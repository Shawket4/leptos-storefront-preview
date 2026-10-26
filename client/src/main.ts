// Entry point. Module scripts run after the document is parsed, so the page is already there.
// Boot order: page data → stores → features → router. After every client-side navigation the
// same happens for the new regions (the router calls `mountAll` on what it swapped in).

import { readPage } from "./core/page";
import { lazy, lazyOnIntent, mountAll } from "./core/mount";
import { onNavigate, startRouter } from "./core/router";
import { syncFromCookie } from "./stores/cart";
import { mountToaster } from "./stores/toasts";

// Features register their `data-ts` names on import. One file per area, one owner per file.
// With the page, on every page: core (drawer, stepper, offline, nav feedback…) and the cart (badge,
// drawer, quick add).
import "./features/core";
import "./features/cart";
// The instant search (the header's box is on every page) loads when the box is reached for, or
// when the page has settled; at once on /search, or when its field already has focus or a value.
lazyOnIntent(["search-box"], () => import("./features/search"), {
  now: (el) =>
    location.pathname === "/search" ||
    el.contains(document.activeElement) ||
    !!el.querySelector<HTMLInputElement>("input")?.value,
  idle: true,
});
// On demand: a separate chunk, loaded the first time a page has one of its `data-ts` names
// (shared code lands in shared chunks, so nothing loads twice).
lazy(["listing"], () => import("./features/listing"));
lazy(["countdown"], () => import("./features/home"));
lazy(["variant-picker", "gallery"], () => import("./features/product"));
lazy(["checkout-form", "checkout", "order-track"], () => import("./features/checkout"));
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
