// The cart's browser code that every page loads: what a tap on "+" needs — the one "add to cart"
// (`addAndConfirm`), the header badge and quick add. The drawer's content and the /cart page are
// drawn by features/cart-view.ts, a chunk loaded on demand (main.ts): on /cart (`lazy`), on the
// intent to open the cart (`lazyOnIntent`: the badge is a drawer trigger), and after an add
// ("View cart" is the likely next tap).
//
// The cart lives in the `cart` cookie (stores/cart.ts); adding sends no request.

import { $ } from "../core/dom";
import { openDrawer } from "../core/drawer";
import { ensure, register } from "../core/mount";
import { navigate, onNavigate } from "../core/router";
import { cfg, digits, t } from "../core/page";
import { add, cart, pruneSnapshots, type CartResult, type LineSnapshot } from "../stores/cart";
import { toast } from "../stores/toasts";

/** The cart drawer's id (the shell renders it once, `pages::cart::CartDrawer`). */
export const DRAWER_ID = "cart-drawer";

export interface AddItem {
  product: number;
  variant: number;
  qty?: number;
  addons?: string[];
}

/** "3 items" / "٣ منتجات", with the language's plural rules. */
export function items(n: number): string {
  const form = new Intl.PluralRules(cfg().lang).select(n);
  return t(`cart.items.${form === "zero" ? "other" : form}`, { n: digits(n) });
}

/** Open the cart drawer once its content can be drawn (or go to /cart if that can't load). */
function viewCart(): void {
  ensure("cart-drawer").then(
    () => openDrawer(DRAWER_ID),
    () => void navigate("/cart"),
  );
}

/** On /cart and /checkout the cart is the page: no drawer over it. */
export const onCartPages = (): boolean => /^\/(cart|checkout)(\/|$)/.test(location.pathname);

/**
 * The one "add to cart": puts the item in the cart (instantly, no request) and says so — a toast
 * with "View cart" (opens the drawer) and the badge's pop (it pops on every increase). Quick add
 * and the product page both call this; `snapshot` is what the line shows and is priced with.
 */
export function addAndConfirm(item: AddItem, snapshot: LineSnapshot): CartResult {
  const r = add(item, snapshot);
  if (r.ok) {
    void ensure("cart-drawer").catch(() => {}); // warm-up: "View cart" is the likely next tap
    const view = onCartPages() ? undefined : { label: t("cart.view"), run: viewCart };
    toast(snapshot.name ? t("cart.added_item", { name: snapshot.name }) : t("cart.added"), "success", view);
  } else {
    // Same words as the no-JS form's `?cart_err=full|max_qty`.
    toast(t(`cart.err.${r.reason}`), r.reason === "full" ? "error" : "info");
  }
  return r;
}

// ── Header badge ──────────────────────────────────────────────────────────────────────────────
// Public pages are cached, so the server renders the bag empty; the count comes from the cookie.
register("cart-badge", (el) => {
  const badge = $(".cart-btn__badge", el);
  let prev: number | null = null;
  let flip = false;
  const unsub = cart.subscribe(({ count: n }) => {
    el.dataset.count = String(n);
    if (badge) badge.textContent = n ? digits(n) : "";
    el.setAttribute("aria-label", n ? t("cart.badge", { items: items(n) }) : t("cart.badge_empty"));
    // Pop on every increase (not on the first fill): two identical animations, alternated.
    if (badge && prev !== null && n > prev) {
      badge.classList.remove(flip ? "is-pop-a" : "is-pop-b");
      flip = !flip;
      badge.classList.add(flip ? "is-pop-a" : "is-pop-b");
    }
    prev = n;
  });
  // Elsewhere it's the drawer's trigger (core opens it; the drawer's content loads on the intent);
  // on /cart and /checkout it's a plain link to /cart. The router mustn't warm /cart on
  // pointer-down where the tap opens the drawer (a wasted request).
  const route = () => {
    if (onCartPages()) {
      el.removeAttribute("data-drawer-open");
      el.removeAttribute("data-router");
    } else {
      el.setAttribute("data-drawer-open", DRAWER_ID);
      el.setAttribute("data-router", "off");
    }
  };
  route();
  const offNav = onNavigate(route);
  return () => {
    unsub();
    offNav();
  };
});

// ── Quick add (the "+" on product cards) ──────────────────────────────────────────────────────
// A real form without JS (POST /cart/add, 303 back with ?added=); with JS nothing is sent. The
// card carries the line's snapshot (`data-snapshot`, from the catalogue). The confirmation is the
// toast and the badge's pop; the button stays a "+".
register("quick-add", (form) => {
  const onSubmit = (ev: Event) => {
    ev.preventDefault();
    let snap: LineSnapshot = { name: "" };
    try {
      snap = JSON.parse(form.dataset.snapshot ?? "{}") as LineSnapshot;
    } catch {
      /* no snapshot: the drawer asks the server for it */
    }
    addAndConfirm({ product: Number(form.dataset.product), variant: Number(form.dataset.variant) }, snap);
  };
  form.addEventListener("submit", onSubmit);
  return () => form.removeEventListener("submit", onSubmit);
});

// Snapshots of lines no longer in the cart: dropped once the page has settled (at boot, so an
// Undo — which needs its line's snapshot — is never cut short).
if ("requestIdleCallback" in window) requestIdleCallback(() => pruneSnapshots(), { timeout: 5000 });
else setTimeout(pruneSnapshots, 2000);
