// Shapes of the JSON the server returns to the browser code. Mirrors the Rust types named on each
// interface (serde field names, snake_case). Money and dates arrive *formatted* by the server
// (crate::format: the visitor's numerals, bidi-safe) — the client never formats or computes money.

/** `GET {api.cartQuote}` (reads the `cart` cookie). Rust: `api::cart::CartQuote`. */
export interface CartQuote {
  lines: CartQuoteLine[];
  count: number;
  subtotal: string;
  savings: string | null;
  delivery: string;
  delivery_free: boolean;
  /** e.g. "EGP 120.00 more for free delivery"; null once it's free. */
  free_delivery_hint: string | null;
  total: string;
  vat_included: string;
  can_checkout: boolean;
}

/** Rust: `api::cart::CartQuoteLine`. */
export interface CartQuoteLine {
  /** Same as the client's `lineKey`. */
  key: string;
  product: number;
  variant: number;
  qty: number;
  name: string;
  variant_label: string | null;
  addons: string[];
  href: string;
  img: string | null;
  unit_price: string;
  compare_at: string | null;
  line_total: string;
  /** Most the shopper can have now (stock); 0 = unavailable. */
  max_qty: number;
  issue: "out_of_stock" | "capped" | "unavailable" | null;
}

// ── Browse (owner: the browse agent) ─────────────────────────────────────────────────────────

/** `GET {api.searchSuggest}?q=` (public catalogue data). Rust: `pages::search::suggest::Suggest`. */
export interface Suggest {
  /** The question as the server read it (trimmed, at most 80 characters). */
  q: string;
  hits: SuggestHit[];
  categories: SuggestCategory[];
  /** "See all 37 results for “phone”"; null when nothing matched. */
  all: { href: string; label: string } | null;
  /** "8 suggestions" for the live region, or "No matches for “…”" (the empty state's title). */
  status: string;
}

/** Rust: `SuggestHit`. */
export interface SuggestHit {
  href: string;
  /** The name in the page's language, matched runs marked (`hit`). */
  name: { text: string; hit: boolean }[];
  category: string;
  img: string | null;
  srcset: string | null;
  price: string;
  was: string | null;
}

/** Rust: `SuggestCategory`. */
export interface SuggestCategory {
  href: string;
  name: string;
  /** `“phone”` / `«تليفون»`. */
  quoted: string;
  /** The aisle's lucide icon: the `d` of its one `<path>`. */
  icon: string;
}

// ── Showcase (owner: the showcase agent) ───────────────────────────────────────────────────────

/** `GET /api/cache/stats`, `POST /api/cache/bump`. Rust: `api::types::CacheInfo`. */
export interface CacheInfo {
  entries: number;
  hits: number;
  misses: number;
  generation: number;
  base_latency_ms: number;
}
