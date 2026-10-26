// The cart: owned by the browser and kept in the `cart` cookie, so the server reads the very same
// state whenever it needs to price it (/cart, the drawer's quote, checkout) and no-JS forms can
// change it too. Adding, removing, changing quantities and editing a line (`replace`) are local and
// instant; the server is asked only for data the client doesn't have (prices, stock, totals) via
// `quote()`.
//
// Cookie format (crates/domain/src/cart.rs is the Rust side — keep them identical):
//   lines joined by "|", fields by ":", add-on ids by "+"
//   product:variant:qty[:addon+addon]      e.g.  2307:1:2|2501:3:1:as-is+pro-install

import { createStore } from "../core/store";
import { cfg } from "../core/page";
import { cached, key as cacheKey } from "../core/cache";
import { NetError, getJSON } from "../core/net";
import type { CartQuote } from "../api/types";

export interface CartLine {
  product: number;
  variant: number;
  qty: number;
  /** Add-on (modifier option) ids, sorted. */
  addons: string[];
}

export interface CartState {
  lines: CartLine[];
  /** Total units (the header badge). */
  count: number;
}

/**
 * What the cart keeps for a line, from the card or product page it was added from: enough to show
 * and price it without asking the server (the client computes the cart). Prices in piastres.
 * Rust: `api::cart::Snapshot` (the server sends the same shape: /cart, checkout, the quote).
 */
export interface LineSnapshot {
  name: string;
  img?: string;
  href?: string;
  /** The variant's label, e.g. "256 GB". */
  variant?: string;
  /** Add-on names, in the same order as `addonPrices`. */
  addons?: string[];
  /** The variant's unit price, without add-ons. */
  unit?: number;
  /** The variant's "was" price (only when higher than `unit`). */
  compareAt?: number;
  addonPrices?: number[];
  /** The most that can be ordered (stock when it was added, at most the cart's limit). */
  maxQty?: number;
  /** The product has something to choose (variants, add-ons): the line offers "Edit". Older
   * snapshots don't say; they offer it too. */
  editable?: boolean;
}

/** A snapshot the cart can be computed from (older or partial ones are filled by `quote()`). */
export type PricedSnapshot = LineSnapshot & { unit: number; maxQty: number };
export const isPriced = (s: LineSnapshot | undefined): s is PricedSnapshot =>
  !!s && typeof s.unit === "number" && typeof s.maxQty === "number";

export const lineKey = (l: Pick<CartLine, "product" | "variant" | "addons">): string =>
  [l.product, l.variant, ...(l.addons.length ? [[...l.addons].sort().join("+")] : [])].join(":");

const COOKIE = () => cfg().cart.cookie;
const MAX_AGE = 60 * 60 * 24 * 30;

export function parseCookie(raw: string): CartLine[] {
  const lines: CartLine[] = [];
  for (const part of raw.split("|")) {
    const [p, v, q, a] = part.split(":");
    const product = Number(p), variant = Number(v), qty = Number(q);
    if (!Number.isInteger(product) || !Number.isInteger(variant) || !Number.isInteger(qty) || qty < 1) continue;
    const addons = a ? a.split("+").filter((x) => /^[a-z0-9-]+$/i.test(x)).sort() : [];
    lines.push({ product, variant, qty, addons });
  }
  return lines;
}

export const serialize = (lines: CartLine[]): string =>
  lines.map((l) => [l.product, l.variant, l.qty, ...(l.addons.length ? [l.addons.join("+")] : [])].join(":")).join("|");

function readCookie(): CartLine[] {
  const name = COOKIE() + "=";
  const raw = document.cookie.split("; ").find((c) => c.startsWith(name))?.slice(name.length) ?? "";
  return raw ? parseCookie(decodeURIComponent(raw)) : [];
}

const count = (lines: CartLine[]) => lines.reduce((n, l) => n + l.qty, 0);

/** The one cart store. Subscribe for the badge, the drawer, the /cart page. */
export const cart = createStore<CartState>({ lines: [], count: 0 });

const channel = "BroadcastChannel" in self ? new BroadcastChannel("cart") : null;
channel?.addEventListener("message", () => syncFromCookie());

function commit(lines: CartLine[]): void {
  const value = serialize(lines);
  document.cookie = value
    ? `${COOKIE()}=${encodeURIComponent(value)}; Path=/; Max-Age=${MAX_AGE}; SameSite=Lax`
    : `${COOKIE()}=; Path=/; Max-Age=0; SameSite=Lax`;
  cart.set({ lines, count: count(lines) });
  channel?.postMessage("changed");
}

/** Re-read the cookie (the server may have changed it: a no-JS form, an order placed). */
export function syncFromCookie(): void {
  const lines = readCookie();
  if (serialize(lines) !== serialize(cart.get().lines)) cart.set({ lines, count: count(lines) });
}

export type CartResult = { ok: true; key: string } | { ok: false; reason: "full" | "max_qty" };

export function add(
  item: { product: number; variant: number; qty?: number; addons?: string[] },
  snap?: LineSnapshot,
): CartResult {
  const { maxLines, maxQty } = cfg().cart;
  const addons = [...(item.addons ?? [])].sort();
  const k = lineKey({ product: item.product, variant: item.variant, addons });
  const qty = Math.max(1, item.qty ?? 1);
  const lines = cart.get().lines.map((l) => ({ ...l }));
  const found = lines.find((l) => lineKey(l) === k);
  if (found) {
    if (found.qty >= maxQty) return { ok: false, reason: "max_qty" };
    found.qty = Math.min(maxQty, found.qty + qty);
  } else {
    if (lines.length >= maxLines) return { ok: false, reason: "full" };
    lines.push({ product: item.product, variant: item.variant, qty: Math.min(maxQty, qty), addons });
  }
  if (snap) saveSnapshot(k, snap);
  commit(lines);
  return { ok: true, key: k };
}

export function setQty(k: string, qty: number): void {
  if (qty < 1) {
    remove(k);
    return;
  }
  const max = cfg().cart.maxQty;
  commit(cart.get().lines.map((l) => (lineKey(l) === k ? { ...l, qty: Math.min(max, qty) } : l)));
}

export type ReplaceResult = { ok: true; key: string; changed: boolean; undo: () => void } | { ok: false; reason: "gone" };

/**
 * Change line `k` into `item` (the line editor: another variant, other add-ons, another quantity).
 * The line keeps its place; if it becomes identical to another line, the two merge. Returns the
 * line's key now and an undo that puts the cart's lines back as they were (leaving later changes to
 * other lines alone).
 */
export function replace(
  k: string,
  item: { product: number; variant: number; qty: number; addons: string[] },
  snap?: LineSnapshot,
): ReplaceResult {
  const before = cart.get().lines;
  const at = before.findIndex((l) => lineKey(l) === k);
  if (at < 0) return { ok: false, reason: "gone" };
  const max = cfg().cart.maxQty;
  const old = before[at]!;
  const addons = [...item.addons].sort();
  const next = { product: old.product, variant: item.variant, qty: Math.min(max, Math.max(1, item.qty)), addons };
  const nk = lineKey(next);
  if (snap) saveSnapshot(nk, snap);
  if (nk === k && next.qty === old.qty) return { ok: true, key: k, changed: false, undo: () => {} };
  const lines = before.map((l) => ({ ...l }));
  const other = lines.findIndex((l) => lineKey(l) === nk);
  /** What the merge added to the other line (the undo takes it back). */
  let merged = 0;
  if (other >= 0 && other !== at) {
    const was = lines[other]!.qty;
    lines[other]!.qty = Math.min(max, was + next.qty);
    merged = lines[other]!.qty - was;
    lines.splice(at, 1);
  } else lines[at] = next;
  commit(lines);
  const undo = () => {
    const now = cart.get().lines.map((l) => ({ ...l }));
    const i = now.findIndex((l) => lineKey(l) === nk);
    if (nk !== k && i >= 0) {
      if (merged) now[i]!.qty -= merged;
      else now.splice(i, 1);
    }
    const j = now.findIndex((l) => lineKey(l) === k);
    if (j >= 0) now[j] = { ...old };
    else now.splice(Math.min(at, now.length), 0, { ...old });
    commit(now.filter((l) => l.qty > 0));
  };
  return { ok: true, key: nk, changed: true, undo };
}

/** Remove a line; returns a function that puts it back where it was (for "Undo"). */
export function remove(k: string): () => void {
  const before = cart.get().lines;
  const at = before.findIndex((l) => lineKey(l) === k);
  if (at < 0) return () => {};
  const line = before[at]!;
  commit(before.filter((_, i) => i !== at));
  return () => {
    const now = cart.get().lines.filter((l) => lineKey(l) !== k);
    now.splice(Math.min(at, now.length), 0, line);
    commit(now);
  };
}

export function clear(): void {
  commit([]);
}

// ── Snapshots: each line's display data and prices, kept with the cart (per tenant and language) ─
const SNAP = "cart:snap:";
/** Keep (or replace) a line's snapshot. */
export function saveSnapshot(k: string, s: LineSnapshot): void {
  try {
    localStorage.setItem(SNAP + cfg().tenant + ":" + cfg().lang + ":" + k, JSON.stringify(s));
  } catch {
    /* private mode / quota: the quote fills it in */
  }
}
export function snapshot(k: string): LineSnapshot | undefined {
  try {
    const raw = localStorage.getItem(SNAP + cfg().tenant + ":" + cfg().lang + ":" + k);
    return raw ? (JSON.parse(raw) as LineSnapshot) : undefined;
  } catch {
    return undefined;
  }
}

/**
 * Drop this tenant and language's snapshots of lines that are no longer in the cart (call it at
 * boot: an Undo, which needs its line's snapshot, never outlives a page).
 */
export function pruneSnapshots(): void {
  try {
    const prefix = SNAP + cfg().tenant + ":" + cfg().lang + ":";
    const keep = new Set(readCookie().map(lineKey));
    for (let i = localStorage.length - 1; i >= 0; i--) {
      const k = localStorage.key(i);
      if (k?.startsWith(prefix) && !keep.has(k.slice(prefix.length))) localStorage.removeItem(k);
    }
  } catch {
    /* storage unavailable: nothing kept, nothing to prune */
  }
}

/** A line's unit price with its add-ons (piastres). */
export const unitWithAddons = (s: PricedSnapshot): number => s.unit + (s.addonPrices ?? []).reduce((a, b) => a + b, 0);

/**
 * Adopt the server's snapshots (a page that rendered the cart: /cart, checkout). Returns true when
 * a line's price differs from what this cart had been showing, so the page can say so.
 */
export function adoptSnapshots(server: Record<string, PricedSnapshot>): boolean {
  let moved = false;
  for (const [k, s] of Object.entries(server)) {
    const mine = snapshot(k);
    if (isPriced(mine) && unitWithAddons(mine) !== unitWithAddons(s)) moved = true;
    saveSnapshot(k, s);
  }
  return moved;
}

// ── Server data: prices, stock and totals for exactly this cart (cached per cart contents) ─────
/** The server's pricing of the current cart. Cached 60 s per cart content; any change re-asks. */
export function quote(onUpdate?: (q: CartQuote) => void): Promise<CartQuote> {
  const lines = cart.get().lines;
  const sig = serialize(lines);
  const want = lines.map((l) => `${lineKey(l)}=${l.qty}`).join("|");
  return cached(
    cacheKey("cart-quote", sig),
    { ttl: 60_000, swr: 5 * 60_000 },
    async () => {
      const q = await getJSON<CartQuote>(cfg().api.cartQuote);
      // The server priced the cookie as it was sent. If that isn't this cart (a retry after a
      // change, another tab), it must not be kept under this cart's key: re-read the cookie and
      // let the caller ask again.
      if (q.lines.map((l) => `${l.key}=${l.qty}`).join("|") !== want) {
        syncFromCookie();
        throw new NetError("aborted");
      }
      return q;
    },
    onUpdate,
  );
}
