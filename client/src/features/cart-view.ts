// The cart's renderer, loaded on demand (it isn't needed to add to the cart): the drawer's content
// and the /cart page, drawn by one renderer (`CartView`) from the server's template
// (`#tpl-cart-line`). main.ts loads it on /cart (`lazy`) and on the intent to open the cart
// (`lazyOnIntent`: the badge is the drawer's trigger); features/cart.ts preloads it after an add.
//
// The client computes the cart (owner, 2026-10-26): every line keeps a snapshot — its display
// data and prices in piastres, from the card or product page it was added from — so line totals,
// subtotal, savings, delivery and total are worked out here, and adding, removing and changing a
// quantity send **no request**. The server is asked only for lines with no snapshot (added
// without JS, storage cleared: `quote()` once), and checkout re-prices everything on the server.

import { $, $$, fill, patch } from "../core/dom";
import { openDrawer } from "../core/drawer";
import { money } from "../core/money";
import { mountAll, register, unmountAll } from "../core/mount";
import { NetError, onOnline } from "../core/net";
import { cfg, digits, t } from "../core/page";
import { navigate } from "../core/router";
import { setStepper, stepperValue } from "../core/stepper";
import {
  adoptSnapshots,
  cart,
  isPriced,
  lineKey,
  quote,
  remove,
  saveSnapshot,
  setQty,
  snapshot,
  unitWithAddons,
  type CartLine,
  type LineSnapshot,
  type PricedSnapshot,
} from "../stores/cart";
import { toast } from "../stores/toasts";
import { items } from "./cart";

// ── The cart, computed ────────────────────────────────────────────────────────────────────────

type Issue = "out_of_stock" | "capped" | "unavailable" | null;

const issueOf = (l: CartLine, s: PricedSnapshot): Issue => (s.maxQty <= 0 ? "out_of_stock" : l.qty > s.maxQty ? "capped" : null);

/** What a line shows, all computed from its snapshot (`null` hides a slot). Same words as /cart's
 * server render (`pages::cart::line::LineData`): `meta` is the line's problem, else its breakdown
 * and low stock; `price` is the line total. */
function lineValues(l: CartLine, key: string, s: LineSnapshot | undefined, gone: boolean) {
  const name = s?.name || (gone ? t("cart.unavailable_name") : "");
  const base = {
    key,
    name,
    href: s?.href ?? null,
    img: s?.img ?? null,
    rm_label: t("cart.remove_item", { name }),
    detail: [s?.variant, ...(s?.addons ?? [])].filter(Boolean).join(" · ") || null,
    // "Edit" for a product with something to choose (older snapshots don't say: offered too).
    edit: isPriced(s) && !gone && s.editable !== false ? t("cart.edit") : null,
  };
  if (!isPriced(s) || gone) {
    return { ...base, meta: gone ? t("cart.issue.unavailable") : null, price: null, fix: null };
  }
  const unit = unitWithAddons(s);
  const issue = issueOf(l, s);
  const note = [
    l.qty > 1 ? t("cart.breakdown", { qty: digits(l.qty), unit: money(unit) }) : null,
    s.maxQty <= 5 ? t("cart.low_stock", { n: digits(s.maxQty) }) : null,
  ].filter(Boolean);
  return {
    ...base,
    meta:
      issue === "capped"
        ? t("cart.issue.capped", { n: digits(s.maxQty) })
        : issue
          ? t("cart.issue.out_of_stock")
          : note.join(" · ") || null,
    price: money(unit * l.qty),
    fix: issue === "capped" ? t("cart.fix", { n: digits(s.maxQty) }) : null,
  };
}

type Values = ReturnType<typeof lineValues>;

/** The cart's totals from its snapshots (piastres), and whether every line could be priced. */
function totals(lines: CartLine[], gone: Set<string>) {
  let subtotal = 0;
  let savings = 0;
  let priced = true;
  let blocked = false;
  for (const l of lines) {
    const k = lineKey(l);
    const s = snapshot(k);
    if (gone.has(k)) blocked = true;
    if (!isPriced(s) || gone.has(k)) {
      priced = priced && gone.has(k);
      continue;
    }
    subtotal += unitWithAddons(s) * l.qty;
    if (s.compareAt && s.compareAt > s.unit) savings += (s.compareAt - s.unit) * l.qty;
    if (issueOf(l, s)) blocked = true;
  }
  const { deliveryFee, freeDeliveryOver } = cfg().cart;
  const delivery = subtotal === 0 || subtotal >= freeDeliveryOver ? 0 : deliveryFee;
  return { subtotal, savings, delivery, total: subtotal + delivery, priced, blocked, left: freeDeliveryOver - subtotal };
}

function paintLine(el: HTMLElement, l: CartLine, s: LineSnapshot | undefined, v: Values, gone: boolean): void {
  patch(el, v);
  if (v.href) for (const a of $$<HTMLAnchorElement>("a.item-row__media, a.item-row__name", el)) a.setAttribute("href", v.href);
  const img = $<HTMLImageElement>(".item-row__img", el);
  if (img && v.img && img.getAttribute("src") !== v.img) img.src = v.img;
  el.dataset.issue = gone ? "unavailable" : isPriced(s) ? (issueOf(l, s) ?? "") : "";
  el.toggleAttribute("data-loading", !v.name);
  $(".cline__remove button", el)?.setAttribute("aria-label", v.rm_label);
  const edit = $<HTMLAnchorElement>("a[data-cart-edit]", el);
  if (edit) {
    edit.setAttribute("href", editHref(v.key));
    edit.setAttribute("aria-label", t("cart.edit_item", { name: v.name }));
  }
  const stepper = $("[data-ts~=stepper]", el);
  if (stepper && v.name) {
    const label = t("cart.qty_of", { name: v.name });
    stepper.setAttribute("aria-label", label);
    $("input", stepper)?.setAttribute("aria-label", label);
  }
  const max = isPriced(s) ? Math.max(l.qty, s.maxQty) : cfg().cart.maxQty;
  if (stepper) setStepper(stepper, { value: l.qty, min: 1, max });
  const fixQty = $<HTMLInputElement>("[data-fix-qty]", el);
  if (fixQty && isPriced(s)) fixQty.value = String(s.maxQty);
}

/** The line editor's page (`pages::cart::edit_href`: the same address, so the same cache entry). */
const editHref = (key: string) => `/cart/edit?key=${encodeURIComponent(key)}`;

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

interface Parts {
  /** Where events are listened for (the dialog, or the page). */
  root: HTMLElement;
  list: HTMLElement;
  summary: HTMLElement | null;
  /** Shown instead of the lines when the cart is empty. */
  empty: HTMLElement | null;
  /** Hidden when the cart is empty (/cart's layout; the drawer hides its footer itself). */
  full: HTMLElement | null;
  /** "3 items" (/cart's heading). */
  count: HTMLElement | null;
}

/** The one renderer: the store's lines from #tpl-cart-line, and the summary, all computed. */
class CartView {
  /** Lines the server says it no longer sells (from a quote). */
  private gone = new Set<string>();
  /** Keys a quote was already asked for (a missing snapshot is asked for once). */
  private asked = new Set<string>();
  private error: "offline" | "failed" | null = null;
  private seq = 0;
  private els = new Map<string, HTMLElement>();

  constructor(private p: Parts) {
    // Lines the server rendered (/cart) are adopted as they are.
    for (const li of $$<HTMLElement>(":scope > li[data-key]", p.list)) if (li.dataset.key) this.els.set(li.dataset.key, li);
  }

  paint(): void {
    const lines = cart.get().lines;
    const want: HTMLElement[] = [];
    const keys = new Set<string>();
    const missing: string[] = [];
    for (const l of lines) {
      const key = lineKey(l);
      keys.add(key);
      const s = snapshot(key);
      const gone = this.gone.has(key);
      if (!isPriced(s) && !gone) missing.push(key);
      const v = lineValues(l, key, s, gone);
      let el = this.els.get(key);
      if (!el) {
        el = fill("cart-line", v).firstElementChild as HTMLElement;
        el.dataset.key = key;
        mountAll(el);
        this.els.set(key, el);
      }
      paintLine(el, l, s, v, gone);
      want.push(el);
    }
    for (const key of [...this.els.keys()]) if (!keys.has(key)) this.els.delete(key);
    reconcile(this.p.list, want);
    const empty = !lines.length;
    this.p.list.hidden = empty;
    if (this.p.empty) this.p.empty.hidden = !empty;
    if (this.p.full) this.p.full.hidden = empty;
    if (this.p.count) {
      this.p.count.hidden = empty;
      this.p.count.textContent = items(cart.get().count);
    }
    this.paintSummary(lines);
    // Lines with nothing to show or price them by: ask the server, once.
    if (missing.some((k) => !this.asked.has(k))) void this.fill(missing);
  }

  private paintSummary(lines: CartLine[]): void {
    const s = this.p.summary;
    if (!s) return;
    const c = totals(lines, this.gone);
    patch(s, {
      subtotal: money(c.subtotal),
      savings: c.savings > 0 ? money(c.savings) : null,
      total: money(c.total),
      delivery: c.delivery ? money(c.delivery) : null,
      free: c.delivery ? null : t("cart.free"),
      hint: c.subtotal > 0 && c.left > 0 ? t("cart.free_hint", { amount: money(c.left) }) : null,
      ship_done: c.subtotal > 0 && c.left <= 0 ? t("cart.free_done") : null,
      blocked: c.blocked ? t("cart.blocked") : null,
    });
    // Still waiting for a missing line's prices: the totals are partial, dimmed.
    s.setAttribute("aria-busy", String(!c.priced));
    $("[data-checkout]", s)?.setAttribute("aria-disabled", String(c.blocked || !c.priced));
    const err = $("[data-quote-error]", s);
    if (err) {
      err.hidden = !this.error;
      patch(err, { title: this.error === "offline" ? t("cart.quote_offline") : t("cart.quote_failed") });
    }
    // The drawer's footer goes with an empty cart (on /cart the whole layout does).
    if (!this.p.full) (s.closest<HTMLElement>(".drawer__foot") ?? s).hidden = !lines.length;
  }

  /** Get snapshots for lines that have none (added without JS, storage cleared): one quote. */
  private async fill(keys: string[]): Promise<void> {
    for (const k of keys) this.asked.add(k);
    const seq = ++this.seq;
    try {
      const q = await quote();
      if (seq !== this.seq) return;
      this.error = null;
      for (const l of q.lines) {
        if (l.snapshot) saveSnapshot(l.key, l.snapshot);
        else if (l.issue === "unavailable") this.gone.add(l.key);
      }
    } catch (e) {
      if (seq !== this.seq) return;
      // The cart changed under the request: ask again for what's still missing.
      if (e instanceof NetError && e.code === "aborted") for (const k of keys) this.asked.delete(k);
      else {
        this.error = e instanceof NetError && e.code === "offline" ? "offline" : "failed";
        for (const k of keys) this.asked.delete(k);
        this.paintSummary(cart.get().lines);
        return;
      }
    }
    this.paint();
  }

  /** Retry filling missing lines (the button, or the connection coming back). */
  retry(): void {
    if (!this.error) return;
    this.error = null;
    this.paint();
  }

  /** Remove a line now; the toast offers Undo (it puts the line back where it was). */
  removeLine(key: string): void {
    const el = this.els.get(key);
    const hadFocus = !!el && el.contains(document.activeElement);
    const name = snapshot(key)?.name || (el && $("[data-slot=name]", el)?.textContent) || "";
    const undo = remove(key); // the store changes → paint: line, totals, cookie, badge at once
    toast(name ? t("cart.removed", { name }) : t("cart.removed_one"), "info", { label: t("cart.undo"), run: undo });
    // The focused control went with the line: hand focus to the panel (or /cart's heading).
    if (hadFocus) $<HTMLElement>(".drawer__panel, #cart-title", this.p.root)?.focus();
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
      if (el?.closest("[data-quote-retry]")) return this.retry();
      if (el?.closest("[data-checkout][aria-disabled=true]")) ev.preventDefault();
      // A line's Edit: the editor's sheet over the cart, open at once (its skeleton) while the
      // editor's code and markup come (features/line-editor.ts); the editor's page if they can't.
      const edit = el?.closest<HTMLAnchorElement>("a[data-cart-edit]");
      if (edit && ev.button === 0 && !ev.metaKey && !ev.ctrlKey && !ev.shiftKey && !ev.altKey) {
        ev.preventDefault();
        openDrawer("line-editor");
        import("./line-editor").then(
          (m) => m.openEditor(edit.href),
          () => void navigate(edit.href),
        );
      }
    };
    r.addEventListener("change", onChange);
    r.addEventListener("submit", onSubmit);
    r.addEventListener("click", onClick);
    const offOnline = onOnline(() => this.retry());
    return () => {
      this.seq++;
      offOnline();
      r.removeEventListener("change", onChange);
      r.removeEventListener("submit", onSubmit);
      r.removeEventListener("click", onClick);
    };
  }
}

// ── The drawer's content (inside core's drawer) ───────────────────────────────────────────────
/** The renderer on the drawer's content (in every page's shell; this chunk comes on intent). */
function mountDrawer(el: HTMLElement): () => void {
  const dialog = el.closest<HTMLElement>("[data-ts~=drawer]");
  const list = $("[data-cart-lines]", el);
  if (!dialog || !list) return () => {};
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
  };
  const onClose = () => (open = false);
  dialog.addEventListener("drawer:open", onOpen);
  dialog.addEventListener("drawer:close", onClose);
  const unsub = cart.subscribe(() => open && view.paint());
  const off = view.listen();
  // Already open (opened before this chunk arrived, or brought back open by a router swap).
  if (dialog instanceof HTMLDialogElement && dialog.open) onOpen();
  return () => {
    unsub();
    off();
    dialog.removeEventListener("drawer:open", onOpen);
    dialog.removeEventListener("drawer:close", onClose);
  };
}

// ── /cart ─────────────────────────────────────────────────────────────────────────────────────
// The server rendered the lines and sent their snapshots (`data-ts-json`): they become the cart's
// (a price that moved since the drawer showed it is said plainly); from here every change goes to
// the store and the page is re-rendered by the same renderer as the drawer.
register("cart-page", (el) => {
  const list = $("[data-cart-lines]", el);
  if (!list) return;
  try {
    const server = JSON.parse($("script[data-ts-json]", el)?.textContent ?? "{}") as Record<string, PricedSnapshot>;
    if (adoptSnapshots(server)) $("[data-price-changed]", el)?.removeAttribute("hidden");
  } catch {
    /* nothing to adopt */
  }
  const view = new CartView({
    root: el,
    list,
    summary: $("[data-cart-summary]", el),
    empty: $("[data-cart-empty]", el),
    full: $("[data-cart-full]", el),
    count: $("[data-slot=count]", el),
  });
  // A no-JS form's outcome was shown by the server; don't show it again on reload or back.
  const url = new URL(location.href);
  if (["added", "updated", "cart_err"].some((k) => url.searchParams.has(k))) {
    for (const k of ["added", "updated", "cart_err"]) url.searchParams.delete(k);
    history.replaceState(history.state, "", url.pathname + url.search + url.hash);
  }
  // A quantity changed with the stepper before this chunk arrived: that's what the shopper meant.
  // (Only a changed one: a field still showing what the server rendered says nothing new.)
  for (const li of $$<HTMLElement>("li[data-key]", list)) {
    const key = li.dataset.key ?? "";
    const line = cart.get().lines.find((l) => lineKey(l) === key);
    const input = $<HTMLInputElement>("[data-ts~=stepper] input", li);
    const typed = input && input.value !== input.defaultValue ? stepperValue(input) : 0;
    if (line && typed > 0 && typed !== line.qty) setQty(key, typed);
  }
  const unsub = cart.subscribe(() => view.paint());
  const off = view.listen();
  return () => {
    unsub();
    off();
  };
});

register("cart-drawer", mountDrawer);
