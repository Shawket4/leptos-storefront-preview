// The cart's browser code: the one "add to cart" (`addAndConfirm`), the header badge, quick add,
// the drawer's content and the /cart page. State lives in stores/cart.ts (the `cart` cookie):
// every change is local and instant; the server is asked only for what the client doesn't have —
// prices, stock and totals (`quote()`), about 300 ms after the last change.
//
// One renderer (`CartView`) draws the lines and the summary of the drawer *and* of /cart, from the
// server's templates (`#tpl-cart-line`, `#tpl-cart-gone`): the store's lines in order, each filled
// from the latest quote — or, until it lands, from the card the item was added from (snapshot).

import type { CartQuote, CartQuoteLine } from "../api/types";
import { $, $$, fill, patch } from "../core/dom";
import { openDrawer } from "../core/drawer";
import { mountAll, register, unmountAll } from "../core/mount";
import { NetError, onOnline } from "../core/net";
import { cfg, digits, t } from "../core/page";
import { setStepper, stepperValue } from "../core/stepper";
import { add, cart, lineKey, quote, remove, setQty, snapshot, type CartLine, type CartResult, type LineSnapshot } from "../stores/cart";
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

register("quick-add", (form) => {
  const btn = $<HTMLButtonElement>("button", form);
  let timer = 0;
  const onSubmit = (ev: Event) => {
    ev.preventDefault();
    const d = form.dataset;
    const snap: LineSnapshot = { name: d.name ?? "" };
    if (d.img) snap.img = d.img;
    if (d.price) snap.price = d.price;
    if (d.href) snap.href = d.href;
    const r = addAndConfirm({ product: Number(d.product), variant: Number(d.variant) }, snap);
    if (!btn || !r.ok) return;
    btn.dataset.state = "added";
    clearTimeout(timer);
    timer = window.setTimeout(() => (btn.dataset.state = "idle"), ADDED_MS);
  };
  form.addEventListener("submit", onSubmit);
  return () => {
    clearTimeout(timer);
    form.removeEventListener("submit", onSubmit);
  };
});

// ── The one cart renderer: lines + summary, for the drawer and for /cart ────────────────────────

interface Parts {
  /** Where events are listened for (the dialog, or the page). */
  root: HTMLElement;
  list: HTMLElement;
  summary: HTMLElement | null;
  /** Shown instead of the lines when the cart is empty. */
  empty: HTMLElement | null;
  /** Hidden when the cart is empty (/cart's layout, the drawer's footer). */
  full: HTMLElement | null;
  /** "3 items" (/cart's heading). */
  count: HTMLElement | null;
}

interface Gone {
  el: HTMLElement;
  at: number;
  undo: () => void;
  timer: number;
}

const GONE_MS = 6000;

/** The quote is for exactly these lines and quantities. */
const sameCart = (q: CartQuote, lines: CartLine[]): boolean =>
  q.lines.length === lines.length && q.lines.every((ql, i) => ql.key === lineKey(lines[i]!) && ql.qty === lines[i]!.qty);

/** What a line shows: the quote's data, else the snapshot from the card it was added from. */
function lineValues(l: CartLine, key: string, ql: CartQuoteLine | undefined) {
  const snap = snapshot(key);
  const gone = ql?.issue === "unavailable";
  const name = (gone ? snap?.name : ql?.name || snap?.name) || (gone ? t("cart.unavailable_name") : "");
  const unit = gone ? null : ql?.unit_price || snap?.price || null;
  return {
    key,
    name,
    href: ql?.href || snap?.href || null,
    img: ql?.img || snap?.img || null,
    rm_label: t("cart.remove_item", { name }),
    detail: ql ? [ql.variant_label, ...ql.addons].filter(Boolean).join(" · ") || null : null,
    unit,
    was: ql?.compare_at ?? null,
    total: (!gone && ql?.line_total) || null,
    breakdown: l.qty > 1 && unit ? t("cart.breakdown", { qty: digits(l.qty), unit }) : null,
    low: ql && !ql.issue && ql.max_qty <= 5 ? t("cart.low_stock", { n: digits(ql.max_qty) }) : null,
    issue: ql?.issue
      ? ql.issue === "capped"
        ? t("cart.issue.capped", { n: digits(ql.max_qty) })
        : t(`cart.issue.${ql.issue}`)
      : null,
    fix: ql?.issue === "capped" ? t("cart.fix", { n: digits(ql.max_qty) }) : null,
  };
}

type Values = ReturnType<typeof lineValues>;

function paintLine(el: HTMLElement, l: CartLine, ql: CartQuoteLine | undefined, v: Values): void {
  patch(el, v);
  if (v.href) for (const a of $$<HTMLAnchorElement>("a.cline__media, a.cline__name", el)) a.setAttribute("href", v.href);
  const img = $<HTMLImageElement>(".cline__img", el);
  if (img && v.img && img.getAttribute("src") !== v.img) img.src = v.img;
  el.dataset.issue = ql?.issue ?? "";
  // Its total is for another quantity (or not in yet): dimmed until the quote catches up.
  el.toggleAttribute("data-stale", !ql || ql.qty !== l.qty);
  el.toggleAttribute("data-loading", !v.name);
  $(".cline__remove button", el)?.setAttribute("aria-label", v.rm_label);
  const stepper = $("[data-ts~=stepper]", el);
  if (stepper && v.name) {
    const label = t("cart.qty_of", { name: v.name });
    stepper.setAttribute("aria-label", label);
    $("input", stepper)?.setAttribute("aria-label", label);
  }
  const max = ql ? Math.max(l.qty, ql.max_qty) : cfg().cart.maxQty;
  if (stepper) setStepper(stepper, { value: l.qty, min: 1, max });
  const fixQty = $<HTMLInputElement>("[data-fix-qty]", el);
  if (fixQty && ql) fixQty.value = String(ql.max_qty);
}

/** Put `want` in `list`, in order, moving only what's out of place; drop the rest. */
function reconcile(list: HTMLElement, want: HTMLElement[]): void {
  let cursor = list.firstElementChild;
  for (const el of want) {
    if (el === cursor) {
      cursor = cursor.nextElementSibling;
      continue;
    }
    list.insertBefore(el, cursor);
  }
  while (cursor) {
    const next = cursor.nextElementSibling;
    if (cursor instanceof HTMLElement) unmountAll(cursor);
    cursor.remove();
    cursor = next;
  }
}

class CartView {
  quote: CartQuote | null = null;
  /** A quote is being waited for: totals are dimmed. */
  private pending = false;
  private error: "offline" | "failed" | null = null;
  private timer = 0;
  private seq = 0;
  private aborts = 0;
  private gone = new Map<string, Gone>();
  private els = new Map<string, HTMLElement>();

  constructor(private p: Parts) {
    // Lines the server rendered (/cart) are adopted as they are.
    for (const li of $$<HTMLElement>(":scope > li[data-key]", p.list)) if (li.dataset.key) this.els.set(li.dataset.key, li);
  }

  /** Ask for the server's numbers `delay` ms from now (the last change wins); dim meanwhile. */
  ask(delay = 300): void {
    clearTimeout(this.timer);
    this.pending = true;
    this.paintSummary();
    this.timer = window.setTimeout(() => void this.fetch(), delay);
  }

  private async fetch(): Promise<void> {
    const seq = ++this.seq;
    if (!cart.get().lines.length) {
      this.pending = false;
      this.paint();
      return;
    }
    try {
      const q = await quote((fresh) => seq === this.seq && this.accept(fresh));
      if (seq === this.seq) this.accept(q);
    } catch (e) {
      if (seq !== this.seq) return;
      // The server priced a different cart (another tab changed it): the store has re-read the
      // cookie, so ask again — a few times at most.
      if (e instanceof NetError && e.code === "aborted" && ++this.aborts < 3) return this.ask(300);
      this.error = e instanceof NetError && e.code === "offline" ? "offline" : "failed";
      this.paintSummary();
    }
  }

  private accept(q: CartQuote): void {
    this.quote = q;
    this.error = null;
    this.aborts = 0;
    const current = sameCart(q, cart.get().lines);
    this.pending = !current;
    this.paint();
    if (!current) this.ask(300);
  }

  /** Retry a failed quote (the button, or the connection coming back). */
  retry(): void {
    if (this.error) this.ask(0);
  }

  paint(): void {
    this.paintLines();
    this.paintSummary();
  }

  private paintLines(): void {
    const lines = cart.get().lines;
    const byKey = new Map((this.quote?.lines ?? []).map((l) => [l.key, l] as const));
    const want: HTMLElement[] = [];
    const keys = new Set<string>();
    for (const l of lines) {
      const key = lineKey(l);
      keys.add(key);
      const ql = byKey.get(key);
      const v = lineValues(l, key, ql);
      let el = this.els.get(key);
      if (!el) {
        el = fill("cart-line", v).firstElementChild as HTMLElement;
        el.dataset.key = key;
        mountAll(el);
        this.els.set(key, el);
      }
      paintLine(el, l, ql, v);
      want.push(el);
    }
    for (const key of [...this.els.keys()]) if (!keys.has(key)) this.els.delete(key);
    // A removed line leaves its "Undo" where it was, until it's undone or times out.
    for (const [key, g] of [...this.gone].sort((a, b) => a[1].at - b[1].at)) {
      if (keys.has(key)) this.forget(key);
      else want.splice(Math.min(g.at, want.length), 0, g.el);
    }
    reconcile(this.p.list, want);
    const empty = !want.length;
    this.p.list.hidden = empty;
    if (this.p.empty) this.p.empty.hidden = !empty;
    if (this.p.full) this.p.full.hidden = empty && !this.p.full.contains(this.p.list);
    if (this.p.count) {
      const n = cart.get().count;
      this.p.count.hidden = !n;
      this.p.count.textContent = items(n);
    }
  }

  private paintSummary(): void {
    const s = this.p.summary;
    if (!s) return;
    const lines = cart.get().lines;
    const q = this.quote;
    const current = !!q && sameCart(q, lines);
    if (q) {
      patch(s, {
        subtotal: q.subtotal,
        savings: q.savings,
        total: q.total,
        vat: q.vat_included,
        delivery: q.delivery_free ? null : q.delivery,
        free: q.delivery_free ? t("cart.free") : null,
        hint: q.free_delivery_hint,
        ship_done: q.free_delivery_hint || !q.lines.length ? null : t("cart.free_done"),
        blocked: current && !q.can_checkout ? t("cart.blocked") : null,
      });
    }
    s.setAttribute("aria-busy", String(!current || this.pending));
    $("[data-checkout]", s)?.setAttribute("aria-disabled", String(current && !q.can_checkout));
    const err = $("[data-quote-error]", s);
    if (err) {
      err.hidden = !this.error;
      patch(err, { qerr: this.error === "offline" ? t("cart.quote_offline") : t("cart.quote_failed") });
    }
    // The drawer's footer goes with an empty cart (on /cart the whole layout does).
    if (!this.p.full) (s.closest<HTMLElement>(".drawer__foot") ?? s).hidden = !lines.length;
  }

  /** Remove a line now; an "Undo" takes its place (and the focus, if it was there). */
  removeLine(key: string): void {
    const at = cart.get().lines.findIndex((l) => lineKey(l) === key);
    if (at < 0) return;
    const el = this.els.get(key);
    const hadFocus = !!el && el.contains(document.activeElement);
    const name = (el && $("[data-slot=name]", el)?.textContent) || snapshot(key)?.name || "";
    const row = fill("cart-gone", { text: t("cart.removed", { name }) }).firstElementChild as HTMLElement;
    row.dataset.gone = key;
    this.forget(key);
    const g: Gone = { el: row, at, undo: () => {}, timer: window.setTimeout(() => this.forget(key, true), GONE_MS) };
    this.gone.set(key, g);
    g.undo = remove(key); // the store changes → paint
    if (hadFocus) $<HTMLElement>("[data-undo]", row)?.focus();
  }

  private undo(key: string): void {
    const g = this.gone.get(key);
    if (!g) return;
    g.undo(); // the line is back → paint drops the row
    $<HTMLElement>("[data-step='1'], input", this.els.get(key) ?? document.body)?.focus();
  }

  /** Drop a removed line's "Undo" (timed out, undone, or the drawer closed). */
  forget(key?: string, repaint = false): void {
    for (const [k, g] of [...this.gone]) {
      if (key !== undefined && k !== key) continue;
      clearTimeout(g.timer);
      this.gone.delete(k);
      if (repaint) {
        const focused = g.el.contains(document.activeElement);
        this.paintLines();
        // The Undo had focus: hand it to the drawer's panel (or /cart's heading), never <body>.
        if (focused) $<HTMLElement>(".drawer__panel, #cart-title", this.p.root)?.focus();
      }
    }
  }

  /** The controls inside a line or the summary. Returns an unsubscribe. */
  listen(): () => void {
    const r = this.p.root;
    const lineKeyOf = (el: Element) => el.closest<HTMLElement>("li[data-key]")?.dataset.key ?? "";
    const setLine = (key: string, n: number) => (n <= 0 ? this.removeLine(key) : setQty(key, n));
    const onChange = (ev: Event) => {
      const input = ev.target;
      if (!(input instanceof HTMLInputElement) || input.name !== "qty" || !input.closest("[data-ts~=stepper]")) return;
      const key = lineKeyOf(input);
      if (key) setLine(key, stepperValue(input));
    };
    const onSubmit = (ev: SubmitEvent) => {
      const form = ev.target;
      if (!(form instanceof HTMLFormElement) || !form.dataset.cartForm) return;
      ev.preventDefault();
      const key = lineKeyOf(form);
      if (!key) return;
      if (form.dataset.cartForm === "remove") this.removeLine(key);
      else setLine(key, Number(new FormData(form).get("qty")));
    };
    const onClick = (ev: MouseEvent) => {
      const el = ev.target instanceof Element ? ev.target : null;
      const undo = el?.closest<HTMLElement>("[data-undo]");
      if (undo) return this.undo(undo.closest<HTMLElement>("[data-gone]")?.dataset.gone ?? "");
      if (el?.closest("[data-quote-retry]")) return this.retry();
      if (el?.closest("[data-checkout][aria-disabled=true]")) ev.preventDefault();
    };
    r.addEventListener("change", onChange);
    r.addEventListener("submit", onSubmit);
    r.addEventListener("click", onClick);
    const offOnline = onOnline(() => this.retry());
    return () => {
      clearTimeout(this.timer);
      this.seq++;
      this.forget();
      offOnline();
      r.removeEventListener("change", onChange);
      r.removeEventListener("submit", onSubmit);
      r.removeEventListener("click", onClick);
    };
  }
}

// ── The drawer's content (inside core's drawer) ───────────────────────────────────────────────
register("cart-drawer", (el) => {
  const dialog = el.closest<HTMLElement>("[data-ts~=drawer]");
  const list = $("[data-cart-lines]", el);
  if (!dialog || !list) return;
  const view = new CartView({
    root: dialog,
    list,
    summary: $("[data-cart-summary]", dialog),
    empty: $("[data-cart-empty]", el),
    full: null,
    count: null,
  });
  let open = false;
  const onOpen = () => {
    open = true;
    view.paint();
    view.ask(0);
  };
  const onClose = () => {
    open = false;
    view.forget(); // undos don't outlive the drawer
  };
  dialog.addEventListener("drawer:open", onOpen);
  dialog.addEventListener("drawer:close", onClose);
  let first = true;
  const unsub = cart.subscribe(() => {
    if (first) return void (first = false);
    if (!open) return;
    view.paint();
    view.ask();
  });
  const off = view.listen();
  // Brought back open by a router swap: core reopened it before this content was mounted.
  if (dialog instanceof HTMLDialogElement && dialog.open) onOpen();
  return () => {
    unsub();
    off();
    dialog.removeEventListener("drawer:open", onOpen);
    dialog.removeEventListener("drawer:close", onClose);
  };
});

// ── /cart ─────────────────────────────────────────────────────────────────────────────────────
// The server rendered the lines and the quote they came from (`data-ts-json`); from here every
// change goes to the store and the page is re-rendered by the same renderer as the drawer.
register("cart-page", (el) => {
  const list = $("[data-cart-lines]", el);
  if (!list) return;
  const view = new CartView({
    root: el,
    list,
    summary: $("[data-cart-summary]", el),
    empty: $("[data-cart-empty]", el),
    full: $("[data-cart-full]", el),
    count: $("[data-slot=count]", el),
  });
  try {
    view.quote = JSON.parse($("script[data-ts-json]", el)?.textContent ?? "null") as CartQuote | null;
  } catch {
    view.quote = null;
  }
  el.dataset.live = ""; // the no-JS "Update" buttons go (cart.css)
  // A no-JS form's outcome was shown by the server; don't show it again on reload or back.
  const url = new URL(location.href);
  if (url.searchParams.has("added") || url.searchParams.has("cart_err")) {
    url.searchParams.delete("added");
    url.searchParams.delete("cart_err");
    history.replaceState(history.state, "", url.pathname + url.search + url.hash);
  }
  const unsub = cart.subscribe(() => {
    view.paint();
    if (!view.quote || !sameCart(view.quote, cart.get().lines)) view.ask();
  });
  const off = view.listen();
  return () => {
    unsub();
    off();
  };
});
