// The showcase's live demos (/showcase, /showcase/stream); /ui's specimens are features/ui-kit.ts.
// Each feature is mounted on server-rendered markup (`data-ts="sc-…"`), reads its
// inputs from `data-*`, writes text through `patch()` into `[data-slot]`s, and adds rows only by
// filling server-rendered templates (`tpl-sc-logline`, `tpl-sc-kv`, `tpl-sc-row`). Strings come
// from `strings/showcase.rs` through `t()`; counts and timings go through `digits()`.

import { register } from "../core/mount";
import { $, $$, fill, offAll, on, patch, prefersReducedMotion, reveal } from "../core/dom";
import { cfg, digits, t } from "../core/page";
import { NetError, getJSON, request } from "../core/net";
import { cached, inspect, invalidate, key as cacheKey, peek, stats } from "../core/cache";
import { onNavigate, prefetch, refresh, routing } from "../core/router";
import { SPRING, SpringValue, VelocityTracker, project, rubberband, type SpringConfig } from "../core/motion";
import type { SegChange } from "../core/segmented";
import { add, cart, lineKey, quote, remove, serialize, type LineSnapshot } from "../stores/cart";
import { toast, toasts } from "../stores/toasts";
import type { CacheInfo } from "../api/types";

// ── Small shared helpers ───────────────────────────────────────────────────────────────────────

const ms = (n: number): string => t("showcase.ms", { n: digits(Math.round(n)) });
const kb = (bytes: number): string => t("showcase.kb", { n: digits((bytes / 1024).toFixed(1)) });

function why(e: unknown): string {
  if (e instanceof NetError) return t("showcase.failed", { why: e.message });
  return t("showcase.failed", { why: String(e) });
}

/** A capped log, newest first, of `tpl-sc-logline` rows. */
function logger(list: HTMLElement | null, max = 6): (text: string) => void {
  return (text) => {
    if (!list) return;
    list.prepend(fill("sc-logline", { text }));
    while (list.children.length > max) list.lastElementChild?.remove();
  };
}

// Router timing for the whole tab: when a navigation started (the routing store) and how long the
// last one took, so a demo mounted on the new page can say how the trip went.
let navCount = 0;
let navStarted = 0;
let lastNav: { url: string; ms: number } | null = null;
routing.subscribe((r) => {
  if (r.pending && !navStarted) navStarted = performance.now();
});
onNavigate((url) => {
  navCount++;
  lastNav = { url: url.pathname + url.search, ms: navStarted ? performance.now() - navStarted : 0 };
  navStarted = 0;
});

// ── /showcase: the part filter ─────────────────────────────────────────────────────────────────

// The chips are links (`?cat=…`, server-rendered when JS is off). With JS the filter is state:
// set on the page root at once, the address kept in step, no request.
register("sc-filter", (root) =>
  on(root, "click", "a[data-cat-chip]", (ev, a) => {
    if (ev.button !== 0 || ev.metaKey || ev.ctrlKey || ev.shiftKey || ev.altKey) return;
    ev.preventDefault();
    const code = a.dataset.catChip ?? "all";
    if (code === "all") delete root.dataset.filter;
    else root.dataset.filter = code;
    for (const c of $$("a[data-cat-chip]", root)) c.setAttribute("aria-current", String(c === a));
    const url = new URL(location.href);
    if (code === "all") url.searchParams.delete("cat");
    else url.searchParams.set("cat", code);
    history.replaceState(history.state, "", url.pathname + url.search + "#features");
    $("#features", root)?.scrollIntoView({ block: "start", behavior: prefersReducedMotion() ? "auto" : "smooth" });
  }),
);

// ── In-order SSR: this document's own timing ───────────────────────────────────────────────────

function navTiming(): PerformanceNavigationTiming | undefined {
  return performance.getEntriesByType("navigation")[0] as PerformanceNavigationTiming | undefined;
}

register("sc-doc", (root) => {
  const show = () => {
    const nav = navTiming();
    if (!nav) return;
    patch(root, {
      ttfb: ms(nav.responseStart),
      end: ms(nav.responseEnd),
      bytes: nav.decodedBodySize
        ? t("showcase.bytes_pair", { sent: kb(nav.encodedBodySize), raw: kb(nav.decodedBodySize) })
        : "—",
    });
  };
  if ((navTiming()?.responseEnd ?? 0) > 0) show();
  else addEventListener("load", show, { once: true });
  return () => removeEventListener("load", show);
});

// The stream demo page: when the last byte came, and how many blocks a script had to place.
register("sc-streamtiming", (root) => {
  const show = () => {
    const nav = navTiming();
    const blocks = $$("[data-stream-block]");
    const placed = blocks.filter((b) => b.querySelector("[data-streamed]")).length;
    patch(root, {
      ttfb: nav ? ms(nav.responseStart) : "—",
      end: nav ? ms(nav.responseEnd) : "—",
      patched: t("showcase.n_of", { n: digits(placed), total: digits(blocks.length) }),
    });
  };
  if ((navTiming()?.responseEnd ?? 0) > 0) show();
  else addEventListener("load", show, { once: true });
  return () => removeEventListener("load", show);
});

// ── Server functions: one call, shown raw ──────────────────────────────────────────────────────

register("sc-api", (root) => {
  const url = root.dataset.url ?? "";
  const body = $("[data-out=body]", root);
  const list = $("[data-out=endpoints]", root);
  reveal(root);
  for (const [name, path] of Object.entries(cfg().api)) list?.append(fill("sc-logline", { text: `${name}: GET ${path}` }));
  const call = async (btn: HTMLButtonElement) => {
    btn.disabled = true;
    patch(root, { status: t("showcase.calling"), ms: "—", cc: "—" });
    const t0 = performance.now();
    try {
      const res = await request(url, { retries: 0, init: { headers: { Accept: "application/json" } } });
      const text = await res.text();
      patch(root, {
        status: `${digits(res.status)} ${res.statusText || "OK"}`,
        ms: ms(performance.now() - t0),
        cc: res.headers.get("cache-control") ?? "—",
      });
      if (body) body.textContent = JSON.stringify(JSON.parse(text), null, 2);
    } catch (e) {
      patch(root, { status: why(e), ms: ms(performance.now() - t0) });
    } finally {
      btn.disabled = false;
    }
  };
  return on(root, "click", "button[data-act=call]", (_, b) => void call(b as HTMLButtonElement));
});

// ── The moka read cache: live stats, and a generation bump ─────────────────────────────────────

register("sc-moka", (root) => {
  const statsUrl = root.dataset.stats ?? "";
  const bumpUrl = root.dataset.bump ?? "";
  const meter = $("[data-out-meter]", root);
  const form = $<HTMLFormElement>("form", root);
  reveal(root);
  const show = (c: CacheInfo) => {
    const total = c.hits + c.misses;
    const rate = total ? Math.round((c.hits * 100) / total) : 0;
    patch(root, {
      entries: digits(c.entries),
      hits: digits(c.hits),
      misses: digits(c.misses),
      rate: `${digits(rate)}%`,
      gen: digits(c.generation),
    });
    meter?.style.setProperty("--p", String(rate));
  };
  const read = () =>
    getJSON<CacheInfo>(statsUrl).then(show, (e) => patch(root, { note: why(e) }));
  void read();
  return offAll([
    on(root, "click", "button[data-act=refresh]", () => void read()),
    on(root, "submit", "form", (ev) => {
      ev.preventDefault();
      const btn = form?.querySelector<HTMLButtonElement>("button[type=submit]");
      if (btn) btn.disabled = true;
      request(bumpUrl, {
        retries: 0,
        init: { method: "POST", headers: { Accept: "application/json", "Content-Type": "application/x-www-form-urlencoded" }, body: "" },
      })
        .then((res) => res.json() as Promise<CacheInfo>)
        .then((c) => {
          show(c);
          patch(root, { note: t("showcase.bumped", { gen: digits(c.generation) }) });
        })
        .catch((e) => patch(root, { note: why(e) }))
        .finally(() => btn && (btn.disabled = false));
    }),
  ]);
});

// ── Head and meta, read from the live document ─────────────────────────────────────────────────

register("sc-head", (root) => {
  const dl = $("[data-out=head]", root);
  if (!dl) return;
  const meta = (sel: string, attr = "content") => document.head.querySelector(sel)?.getAttribute(attr) ?? null;
  const rows: Array<[string, string | null]> = [
    ["<title>", document.title],
    ["description", meta('meta[name="description"]')],
    ["canonical", meta('link[rel="canonical"]', "href")],
    ["robots", meta('meta[name="robots"]')],
    ["og:type", meta('meta[property="og:type"]')],
    ["og:title", meta('meta[property="og:title"]')],
    ["og:locale", meta('meta[property="og:locale"]')],
    ["og:image", meta('meta[property="og:image"]')],
    ["twitter:card", meta('meta[name="twitter:card"]')],
  ];
  const ld = $$('script[type="application/ld+json"]', document.head)
    .map((s) => {
      try {
        const v = JSON.parse(s.textContent ?? "") as { "@type"?: string };
        return v["@type"] ?? "?";
      } catch {
        return "?";
      }
    })
    .join(", ");
  if (ld) rows.push(["JSON-LD", ld]);
  dl.replaceChildren(...rows.filter(([, v]) => v).map(([k, v]) => fill("sc-kv", { k, v })));
});

// ── The router and the page cache ──────────────────────────────────────────────────────────────

register("sc-router", (root) => {
  const href = root.dataset.href ?? "/";
  const line = logger($("[data-out=log]", root));
  reveal(root);
  if (lastNav) line(t("showcase.navigated", { url: lastNav.url, ms: ms(lastNav.ms) }));
  patch(root, { navs: digits(navCount) });
  return offAll([
    routing.subscribe((r) => patch(root, { pending: String(r.pending), to: r.to ?? "null" })),
    on(root, "click", "button[data-act=prefetch]", () => {
      prefetch(href);
      line(t("showcase.prefetched", { url: href }));
    }),
    on(root, "click", "button[data-act=refresh]", () => void refresh()),
  ]);
});

// ── Stores: live subscriptions, and the toast queue ────────────────────────────────────────────

register("sc-stores", (root) => {
  reveal(root);
  return offAll([
    cart.subscribe((c) =>
      patch(root, { cart: t("showcase.store_cart", { count: digits(c.count), lines: digits(c.lines.length) }) }),
    ),
    toasts.subscribe((list) => patch(root, { toasts: t("showcase.store_toasts", { n: digits(list.length) }) })),
    routing.subscribe((r) =>
      patch(root, { routing: r.pending ? t("showcase.store_pending", { to: r.to ?? "" }) : t("showcase.store_idle") }),
    ),
    on(root, "click", "button[data-toast]", (_, b) => {
      switch (b.dataset.toast) {
        case "success":
          toast(t("showcase.toast_success"), "success");
          break;
        case "error":
          toast(t("showcase.toast_error"), "error");
          break;
        case "action":
          toast(t("showcase.toast_removed"), "info", {
            label: t("showcase.undo"),
            run: () => toast(t("showcase.toast_back"), "success"),
          });
          break;
        default:
          toast(t("showcase.toast_info", { n: digits(14) }), "info");
      }
    }),
  ]);
});

// ── The cart cookie: local changes, server prices on demand ────────────────────────────────────

/** fetch() calls started since `since` (performance time): what a cart change cost the network. */
const fetchesSince = (since: number) =>
  (performance.getEntriesByType("resource") as PerformanceResourceTiming[]).filter(
    (e) => e.startTime >= since && (e.initiatorType === "fetch" || e.initiatorType === "xmlhttprequest"),
  ).length;

register("sc-cart", (root) => {
  const d = root.dataset;
  const item = { product: Number(d.product), variant: Number(d.variant) };
  const k = lineKey({ ...item, addons: [] });
  // The priced snapshot the server rendered (like a card's quick add): the cart shows and prices
  // the line from it, so adding asks the server nothing.
  let snap: LineSnapshot | undefined;
  try {
    snap = d.snapshot ? (JSON.parse(d.snapshot) as LineSnapshot) : undefined;
  } catch {
    snap = undefined;
  }
  const name = snap?.name ?? "";
  reveal(root);
  const afterChange = (t0: number) =>
    setTimeout(() => {
      const n = fetchesSince(t0);
      patch(root, { requests: n === 0 ? t("showcase.requests_none") : digits(n) });
    }, 400);
  return offAll([
    cart.subscribe((c) => patch(root, { cookie: serialize(c.lines) || t("showcase.cart_empty"), count: digits(c.count) })),
    on(root, "click", "button[data-act=add]", () => {
      const t0 = performance.now();
      const r = add(item, snap);
      if (!r.ok) toast(t(r.reason === "full" ? "showcase.cart_full" : "showcase.cart_max"), "error");
      afterChange(t0);
    }),
    on(root, "click", "button[data-act=remove]", () => {
      if (!cart.get().lines.some((l) => lineKey(l) === k)) return;
      const t0 = performance.now();
      const undo = remove(k);
      toast(t("showcase.removed", { name }), "info", { label: t("showcase.undo"), run: undo });
      afterChange(t0);
    }),
    on(root, "click", "button[data-act=quote]", () => {
      const t0 = performance.now();
      patch(root, { quote: t("showcase.calling") });
      quote().then(
        (q) =>
          patch(root, {
            quote: q.lines.length
              ? t("showcase.quote_line", { subtotal: q.subtotal, delivery: q.delivery, total: q.total, ms: ms(performance.now() - t0) })
              : t("showcase.quote_empty"),
          }),
        (e) => patch(root, { quote: why(e) }),
      );
    }),
  ]);
});

// ── Templates: rows filled from data ───────────────────────────────────────────────────────────

interface Row {
  name: string;
  price: string;
  img: string;
  href: string;
}

register("sc-tpl", (root) => {
  let rows: Row[] = [];
  try {
    rows = JSON.parse($("script[data-ts-json]", root)?.textContent ?? "[]") as Row[];
  } catch {
    rows = [];
  }
  const out = $("[data-out=rows]", root);
  let next = 0;
  reveal(root);
  const count = () => patch(root, { count: t("showcase.filled", { n: digits(out?.children.length ?? 0) }) });
  return offAll([
    on(root, "click", "button[data-act=fill]", () => {
      const row = rows[next++ % Math.max(rows.length, 1)];
      if (!row || !out) return;
      out.append(fill("sc-row", { ...row }));
      count();
    }),
    on(root, "click", "button[data-act=clear]", () => {
      out?.replaceChildren();
      count();
    }),
  ]);
});

// ── The client data cache: fresh, stale, miss ──────────────────────────────────────────────────

const DEMO_KEY = "showcase-cache-demo";
const DEMO_OPTS = { ttl: 5_000, swr: 10_000 };

register("sc-cache", (root) => {
  const url = root.dataset.url ?? "";
  const k = cacheKey(DEMO_KEY);
  const bar = $("[data-out-ttl]", root);
  const line = logger($("[data-out=log]", root));
  reveal(root);
  bar?.style.setProperty("--fresh", String(DEMO_OPTS.ttl / (DEMO_OPTS.ttl + DEMO_OPTS.swr)));
  const tick = () => {
    const e = inspect().find((x) => x.key === k);
    const s = stats();
    patch(root, {
      key: k,
      age: e ? t("showcase.cache_age", { s: digits((e.ageMs / 1000).toFixed(1)) }) : t("showcase.cache_none"),
      counts: t("showcase.cache_counts", { hits: digits(s.hits), stale: digits(s.stale), misses: digits(s.misses) }),
    });
    bar?.style.setProperty("--age", String(e ? Math.min(e.ageMs / (e.ttl + e.swr), 1) : 0));
    bar?.toggleAttribute("data-empty", !e);
  };
  tick();
  const timer = setInterval(tick, 250);
  const read = async () => {
    const had = peek(k) !== undefined;
    let fetched = false;
    const t0 = performance.now();
    try {
      await cached(
        k,
        DEMO_OPTS,
        () => {
          fetched = true;
          return getJSON<CacheInfo>(url);
        },
        () => line(t("showcase.cache_refreshed")),
      );
      const took = ms(performance.now() - t0);
      line(t(!had ? "showcase.cache_miss" : fetched ? "showcase.cache_stale" : "showcase.cache_fresh", { ms: took }));
    } catch (e) {
      line(why(e));
    }
    tick();
  };
  return offAll([
    () => clearInterval(timer),
    on(root, "click", "button[data-act=read]", () => void read()),
    on(root, "click", "button[data-act=invalidate]", () => {
      invalidate(DEMO_KEY);
      line(t("showcase.cache_dropped"));
      tick();
    }),
  ]);
});

// ── Springs: drag, throw, interrupt ────────────────────────────────────────────────────────────

register("sc-spring", (root) => {
  const track = $("[data-track]", root);
  const puck = $<HTMLButtonElement>("[data-puck]", root);
  const ghost = $("[data-ghost]", root);
  if (!track || !puck || !ghost) return;
  reveal(root);
  const rtl = getComputedStyle(root).direction === "rtl";
  const sign = rtl ? -1 : 1;
  let preset: SpringConfig = SPRING.default;
  let max = 0;
  const measure = () => (max = Math.max(track.clientWidth - puck.offsetWidth, 0));
  measure();
  const stops = () => [0, max / 2, max];
  const nearest = (x: number) => stops().reduce((a, b) => (Math.abs(b - x) < Math.abs(a - x) ? b : a), 0);
  // Past either end the drawn position rubber-bands, so an overshoot stays near the track.
  const drawn = (x: number) => (x < 0 ? -rubberband(-x, max) : x > max ? max + rubberband(x - max, max) : x);
  const spring = new SpringValue(0, (x) => {
    puck.style.transform = `translate3d(${sign * drawn(x)}px,0,0)`;
    patch(root, {
      x: t("showcase.px", { n: digits(Math.round(x)) }),
      v: t("showcase.pxs", { n: digits(Math.round(spring.velocity)) }),
    });
  });
  const aim = (target: number, velocity?: number) => {
    ghost.style.transform = `translate3d(${sign * target}px,0,0)`;
    ghost.hidden = false;
    spring.to(target, { velocity, spring: preset, onRest: () => (ghost.hidden = true) });
  };
  const tracker = new VelocityTracker();
  let drag: { x0: number; from: number } | null = null;

  const down = (ev: PointerEvent) => {
    if (ev.button !== 0) return;
    puck.setPointerCapture(ev.pointerId);
    spring.stop();
    drag = { x0: ev.clientX, from: spring.value };
    tracker.reset();
    tracker.add(sign * ev.clientX, ev.timeStamp);
  };
  const move = (ev: PointerEvent) => {
    if (!drag) return;
    spring.jump(drag.from + sign * (ev.clientX - drag.x0));
    tracker.add(sign * ev.clientX, ev.timeStamp);
  };
  // Release: hand the finger's velocity to the spring and aim where the flick would come to rest.
  const release = () => {
    if (!drag) return;
    drag = null;
    const v = Math.max(-4000, Math.min(4000, tracker.velocity()));
    const rest = spring.value + project(v);
    patch(root, { proj: t("showcase.px", { n: digits(Math.round(rest)) }) });
    aim(nearest(Math.min(Math.max(rest, 0), max)), v);
  };

  const ro = new ResizeObserver(() => {
    measure();
    spring.jump(nearest(Math.min(spring.value, max)));
  });
  ro.observe(track);
  puck.addEventListener("pointerdown", down);
  puck.addEventListener("pointermove", move);
  puck.addEventListener("pointerup", release);
  puck.addEventListener("pointercancel", release);
  return offAll([
    () => ro.disconnect(),
    () => spring.stop(),
    () => puck.removeEventListener("pointerdown", down),
    () => puck.removeEventListener("pointermove", move),
    () => puck.removeEventListener("pointerup", release),
    () => puck.removeEventListener("pointercancel", release),
    on(puck, "keydown", "[data-puck]", (ev) => {
      const fwd = rtl ? "ArrowLeft" : "ArrowRight";
      const back = rtl ? "ArrowRight" : "ArrowLeft";
      if (ev.key !== fwd && ev.key !== back) return;
      ev.preventDefault();
      const s = stops();
      const i = s.indexOf(nearest(spring.value));
      aim(s[Math.min(Math.max(i + (ev.key === fwd ? 1 : -1), 0), s.length - 1)] ?? 0);
    }),
    // The preset picker is core's SegmentedToggle: it fires `change` with the pressed value.
    on(root, "change", "[data-ts~=seg-toggle]", (ev) => {
      const v = (ev as CustomEvent<SegChange>).detail?.value ?? "default";
      preset = SPRING[v as keyof typeof SPRING] ?? SPRING.default;
    }),
    // Interruptible: the second target starts from wherever the first left the dot, at its speed.
    on(root, "click", "button[data-act=interrupt]", () => {
      aim(max);
      setTimeout(() => aim(0), 160);
    }),
  ]);
});

// ── The size of the client, measured in this browser ───────────────────────────────────────────

async function gzipSize(text: string): Promise<number | null> {
  if (!("CompressionStream" in self)) return null;
  const stream = new Blob([text]).stream().pipeThrough(new CompressionStream("gzip"));
  return (await new Response(stream).arrayBuffer()).byteLength;
}

register("sc-bundle", (root) => {
  const src = root.dataset.src ?? "";
  const budget = Number(root.dataset.budget ?? 30);
  const bar = $("[data-out-bar]", root);
  const entry = (performance.getEntriesByType("resource") as PerformanceResourceTiming[]).find((e) => e.name.endsWith(src));
  if (entry && entry.encodedBodySize) {
    patch(root, {
      wire: t(entry.transferSize === 0 ? "showcase.wire_cached" : "showcase.wire_sent", { size: kb(entry.encodedBodySize) }),
    });
  }
  let alive = true;
  patch(root, { gzip: t("showcase.measuring") });
  request(src, { init: { cache: "force-cache" } })
    .then((res) => res.text())
    .then(async (text) => {
      // Dev builds carry an inline source map: measure the code alone.
      const code = text.replace(/\n\/\/# sourceMappingURL=data:[^\n]*\n?$/, "\n");
      const gz = await gzipSize(code);
      if (!alive) return;
      patch(root, { raw: kb(new Blob([code]).size), gzip: gz === null ? "—" : kb(gz) });
      if (gz !== null) {
        bar?.style.setProperty("--kb", String(gz / 1024));
        patch(root, { bar: `${kb(gz)} · ${t("showcase.of_budget", { p: digits(Math.round((gz / 1024 / budget) * 100)) })}` });
      }
    })
    .catch((e) => alive && patch(root, { gzip: why(e) }));
  return () => (alive = false);
});
