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

// ── Cart agent: order tracking ─────────────────────────────────────────────────────────────────

/** `GET {api.orderTrack}?id=` → `OrderTrack | null`. Rust: `pages::order::TrackJson`. */
export interface OrderTrack {
  id: string;
  /** `received`, `confirmed`, `preparing`, `ready`, `out_for_delivery`, `delivered`, `cancelled`, `rejected`. */
  status: string;
  headline: string;
  eta: string;
  /** One per timeline step, in order. */
  steps: { state: "done" | "current" | "todo"; time: string | null }[];
  /** How far the timeline's line has grown, 0..1. */
  progress: number;
  cancelled: boolean;
  /** A final status: nothing will change any more (stop polling). */
  done: boolean;
  /** When the next step is due, from now (ms), for pacing the polls. */
  next_in_ms: number | null;
  /** "Updated 06:02 PM". */
  updated: string;
}
