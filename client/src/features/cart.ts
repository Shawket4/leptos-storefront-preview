// The cart's browser code: the one "add to cart" (`addAndConfirm`), the header badge and quick
// add. State lives in stores/cart.ts (the `cart` cookie): every change here is local and
// instant; the server is asked only for what the client doesn't have (prices, stock, totals).

import { register } from "../core/mount";
import { cfg, digits, t } from "../core/page";
import { openDrawer } from "../core/drawer";
import { add, cart, type CartResult, type LineSnapshot } from "../stores/cart";
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
  const key = form === "zero" ? "other" : form;
  return t(`cart.items.${key}`, { n: digits(n) });
}

/** On /cart and /checkout the cart is the page: no drawer over it. */
export const onCartPages = (): boolean => /^\/(cart|checkout)(\/|$)/.test(location.pathname);

/**
 * The one "add to cart": puts the item in the cart (instantly, no request) and says so — a toast
 * with "View cart" (opens the drawer) and the badge's pop (it pops on every increase). Quick add
 * and the product page both call this.
 */
export function addAndConfirm(item: AddItem, snapshot: LineSnapshot): CartResult {
  const r = add(item, snapshot);
  if (r.ok) {
    const view = onCartPages() ? undefined : { label: t("cart.view"), run: () => openDrawer(DRAWER_ID) };
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
  const badge = el.querySelector<HTMLElement>(".cart-btn__badge");
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
  const onClick = (ev: MouseEvent) => {
    if (onCartPages() || ev.button !== 0 || ev.metaKey || ev.ctrlKey || ev.shiftKey || ev.altKey) return;
    ev.preventDefault();
    openDrawer(DRAWER_ID);
  };
  el.addEventListener("click", onClick);
  return () => {
    unsub();
    el.removeEventListener("click", onClick);
  };
});

// ── Quick add (the "+" on product cards) ──────────────────────────────────────────────────────
// A real form without JS (POST /cart/add, 303 back with ?added=); with JS nothing is sent.
const ADDED_MS = 1600;

function quickAdd(form: HTMLElement): void {
  const d = form.dataset;
  const btn = form.querySelector<HTMLButtonElement>("button[type=submit], button:not([type])");
  const snap: LineSnapshot = { name: d.name ?? "" };
  if (d.img) snap.img = d.img;
  if (d.price) snap.price = d.price;
  if (d.href) snap.href = d.href;
  const r = addAndConfirm({ product: Number(d.product), variant: Number(d.variant) }, snap);
  if (!btn) return;
  btn.dataset.state = r.ok ? "added" : "idle";
  if (r.ok) {
    clearTimeout(Number(btn.dataset.timer));
    btn.dataset.timer = String(window.setTimeout(() => (btn.dataset.state = "idle"), ADDED_MS));
  }
}

register("quick-add", (form) => {
  const onSubmit = (ev: Event) => {
    ev.preventDefault();
    quickAdd(form);
  };
  form.addEventListener("submit", onSubmit);
  return () => form.removeEventListener("submit", onSubmit);
});
