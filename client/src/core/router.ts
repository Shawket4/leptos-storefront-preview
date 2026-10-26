// Client-side navigation: same-origin links and forms are fetched (`X-Router: 1`), parsed, and
// swapped in by region — `<main>` and every `[data-region=<name>]` — while the rest of the frame
// stays (the header with its search box, `#cart-drawer`, `#toaster`). A change of language,
// numerals or theme swaps the whole frame (its words change too), keeping only the toasts on
// screen; another shop or a new deploy (different script or stylesheet) is a real page load.
//
// History: push/replace with the scroll position kept in `history.state` (restored on back/forward),
// focus to the new page's <main>, a view transition when the browser has them and motion isn't
// reduced. Pages go through the page cache (core/cache.ts: fresh 30 s, then served stale for 5 min
// while revalidated); back/forward are served from it at once. Personal pages (Cache-Control
// private) are never kept. Prefetch on intent follows one gesture model (below).
//
// The progress bar, the failure alert and the offline banner are features/core.ts: they read
// `routing` and `navFailure`. Without fetch/DOMParser, or for links this can't take, navigations are
// ordinary page loads, which is always a correct fallback.

import { createStore } from "./store";
import { NetError, isOnline, request } from "./net";
import { cached, invalidate, key, noteGeneration, peek } from "./cache";
import { preload, unmountAll } from "./mount";
import { prefersReducedMotion } from "./dom";

type Listener = (url: URL) => void;
const listeners = new Set<Listener>();

export interface NavigateOpts {
  /** Replace the current history entry instead of pushing one (filters, `?v=`). */
  replace?: boolean;
  /** "top" (default for new pages) or "keep" (same page with new data: filters, sort). */
  scroll?: "top" | "keep";
}

/** A navigation is under way (listings dim their results; the progress bar shows past 150 ms). */
export const routing = createStore<{ pending: boolean; to: string | null }>({ pending: false, to: null });

/** The last navigation that failed, until the next one starts (features/core.ts shows it). */
export interface NavFailure {
  url: string;
  why: "offline" | "timeout" | "network" | "server";
  /** The same navigation again; null for a form post (never re-sent behind the visitor's back). */
  retry: (() => void) | null;
}
export const navFailure = createStore<NavFailure | null>(null);

// ── Pages ─────────────────────────────────────────────────────────────────────────────────────
interface Page {
  html: string;
  /** Where it ended up after redirects. */
  url: string;
  /** Cache-Control private/no-store: never kept in the page cache. */
  personal: boolean;
}

class NotHtml extends Error {
  constructor(readonly url: string) {
    super("not html");
  }
}

const PAGE_CACHE = { ttl: 30_000, swr: 5 * 60_000 };
const noHash = (u: URL | string): string => {
  const x = new URL(String(u), location.href);
  x.hash = "";
  return x.href.replace(/\?$/, "");
};
// Theme is part of the key: a page cached in light mode carries data-theme="light".
const pageKey = (href: string) => key("page", document.documentElement.dataset.theme ?? "", href, "");

/** The `data-ts` names in a page's HTML: its on-demand feature chunks can start loading. */
const featuresIn = (html: string) => new Set(Array.from(html.matchAll(/data-ts="([^"]+)"/g), (m) => m[1]!.split(/\s+/)).flat());

async function fetchPage(url: string, init?: RequestInit): Promise<Page> {
  const res = await request(url, {
    timeoutMs: 12_000,
    retries: 0,
    ok: (r) => r.status < 500, // a 404 is a page; a 5xx is a failure
    init: { ...init, headers: { "X-Router": "1", Accept: "text/html" } },
  });
  noteGeneration(res.headers.get("x-catalog-gen"));
  if (!(res.headers.get("content-type") ?? "").includes("text/html")) throw new NotHtml(res.url || url);
  let html: string;
  try {
    html = await res.text();
  } catch {
    throw new NetError(isOnline() ? "http" : "offline");
  }
  return { html, url: res.url || url, personal: /private|no-store/.test(res.headers.get("cache-control") ?? "") };
}

// A personal page warmed by a prefetch is handed to the navigation that wanted it, once.
const warmPersonal = new Map<string, { page: Page; at: number }>();
const consumed = new WeakSet<Page>();

async function loadGet(href: string, opts: { fresh?: boolean; onUpdate?: (p: Page) => void } = {}): Promise<Page> {
  const k = pageKey(href);
  const held = warmPersonal.get(k);
  warmPersonal.delete(k);
  if (held && !opts.fresh && Date.now() - held.at < 10_000) return held.page;
  if (opts.fresh) invalidate(k);
  try {
    const page = await cached(k, PAGE_CACHE, () => fetchPage(href), (p) => (p.personal ? invalidate(k) : opts.onUpdate?.(p)));
    if (page.personal) invalidate(k);
    return page;
  } catch (e) {
    // Offline or failing: an expired copy beats an error page.
    const old = e instanceof NetError ? peek<Page>(k) : undefined;
    if (old) return old;
    throw e;
  }
}

// ── Navigation ────────────────────────────────────────────────────────────────────────────────
interface Go {
  url: URL;
  method: "GET" | "POST";
  body?: BodyInit;
  push: boolean;
  /** restore = back/forward (`y`); if-moved = a form post: top only when it lands elsewhere. */
  scroll: "top" | "keep" | "restore" | "if-moved";
  y?: number;
  fresh?: boolean;
  pop?: boolean;
}

let started = false;
let seq = 0;
let shown = ""; // the page in the document (no hash)
let posting = false;
const fails = new Map<string, number>();

function saveScroll(): void {
  try {
    history.replaceState({ ...(history.state ?? {}), lsp: 1, y: Math.round(scrollY) }, "");
  } catch {
    /* Safari rate-limits replaceState; the next save gets it */
  }
}

async function go(g: Go): Promise<void> {
  const id = ++seq;
  const href = g.url.href;
  if (!g.pop) saveScroll();
  routing.set({ pending: true, to: href });
  navFailure.set(null);
  if (g.method === "POST") posting = true;
  let page: Page;
  try {
    if (g.method === "POST") page = await fetchPage(href, { method: "POST", body: g.body ?? null });
    else if (g.url.pathname.startsWith("/__")) page = await fetchPage(href); // sets a cookie: never cached
    else page = await loadGet(noHash(g.url), { fresh: !!g.fresh, onUpdate: (p) => revalidated(p, id) });
  } catch (e) {
    if (g.method === "POST") posting = false;
    if (id !== seq) return;
    // The failure first, so the progress bar knows not to play "done".
    if (e instanceof NotHtml) location.assign(e.url);
    else failed(g, e);
    routing.set({ pending: false, to: null });
    return;
  }
  if (g.method === "POST") {
    posting = false;
    invalidate("|page|"); // what the post changed may show on any page
  }
  consumed.add(page);
  if (id !== seq) return; // a newer navigation took over
  preload(featuresIn(page.html), { intent: false }); // its feature chunks load while it's swapped in
  fails.delete(href);
  if (g.method === "GET" && !g.pop && !g.url.pathname.startsWith("/__") && !landingWarmed) warmLanding();
  await show(page, g);
  if (id === seq) routing.set({ pending: false, to: null });
}

function failed(g: Go, e: unknown): void {
  const why: NavFailure["why"] =
    e instanceof NetError
      ? e.code === "offline"
        ? "offline"
        : e.code === "timeout"
          ? "timeout"
          : (e.status ?? 0) >= 500
            ? "server"
            : "network"
      : "network";
  if (__DEV__ && !(e instanceof NetError)) console.error(e);
  const n = (fails.get(g.url.href) ?? 0) + 1;
  fails.set(g.url.href, n);
  // A second failure of the same page while online: let the browser try (its own error page).
  if (n >= 2 && g.method === "GET" && why !== "offline" && isOnline()) {
    location.assign(g.url.href);
    return;
  }
  navFailure.set({ url: g.url.href, why, retry: g.method === "GET" ? () => void go(g) : null });
}

const parse = (html: string) =>
  // <noscript> content is markup in a parsed copy (scripting off) but inert text here: drop it.
  new DOMParser().parseFromString(html.replace(/(<noscript\b[^>]*>)[\s\S]*?(<\/noscript>)/gi, "$1$2"), "text/html");

/** Script, stylesheet and shop of a document: when they differ, only a real page load is right. */
const build = (d: Document) =>
  [
    d.querySelector('script[type="module"][src]')?.getAttribute("src"),
    d.querySelector('link[rel="stylesheet"]')?.getAttribute("href"),
    d.documentElement.dataset.tenant,
  ].join(" ");

async function show(page: Page, g: Go): Promise<void> {
  const doc = parse(page.html);
  const final = new URL(page.url, location.href);
  if (!final.hash) final.hash = g.url.hash;
  if (build(doc) !== build(document)) {
    location.assign(final.href);
    return;
  }
  if (!g.pop) {
    const to = final.href.replace(/\?(?=#|$)/, "");
    const state = { lsp: 1, y: 0 };
    if (g.push && noHash(final) !== noHash(location.href)) history.pushState(state, "", to);
    else history.replaceState({ ...(history.state ?? {}), ...state, y: Math.round(scrollY) }, "", to);
  }
  const scroll =
    g.scroll === "if-moved" ? (final.pathname !== new URL(shown).pathname ? "top" : "keep") : g.scroll;
  // The product page names its main image (`view-transition-name: p{id}`); going back, the card
  // with that `data-vt` on the listing takes the name, so the image travels home.
  const outgoing = document.querySelector<HTMLElement>('main [style*="view-transition-name"]')?.style.getPropertyValue("view-transition-name").trim();
  const update = () => {
    const focus = scroll === "keep" ? focusKeyOf(document.activeElement) : null;
    apply(doc);
    shown = noHash(final);
    if (scroll === "keep") {
      if (focus && (!document.activeElement || document.activeElement === document.body)) findFocus(document, focus)?.focus({ preventScroll: true });
    } else {
      const target = final.hash ? document.getElementById(decodeURIComponent(final.hash.slice(1))) : null;
      if (target) target.scrollIntoView({ block: "start", behavior: "instant" });
      else scrollTo({ top: scroll === "restore" ? (g.y ?? 0) : 0, left: 0, behavior: "instant" });
      if (g.pop && outgoing) nameMedia(document.querySelector<HTMLElement>(`main [data-vt="${CSS.escape(outgoing)}"]`));
      document.querySelector<HTMLElement>("main")?.focus({ preventScroll: true });
    }
    emitNavigate(new URL(location.href));
  };
  const animate =
    scroll !== "keep" &&
    typeof document.startViewTransition === "function" &&
    !prefersReducedMotion() &&
    document.visibilityState === "visible";
  if (!animate) {
    update();
    return;
  }
  // A transition can be skipped or aborted (a newer one, the tab hidden, Chrome's DOM-update
  // timeout on a slow device): its promises reject, and the swap must still happen exactly once.
  let ran = false;
  const once = () => {
    if (ran) return;
    ran = true;
    update();
  };
  const vt = document.startViewTransition(once);
  const fallback = window.setTimeout(once, 1500);
  vt.ready.catch(() => {});
  vt.finished.then(
    () => nameMedia(null),
    () => nameMedia(null),
  );
  await vt.updateCallbackDone.catch((e: unknown) => {
    if (__DEV__) console.warn("view transition:", e);
  });
  clearTimeout(fallback);
  once(); // aborted before the browser called it
}

/** The background refresh of a page that was shown stale: swap what changed, keep everything else. */
function revalidated(page: Page, id: number): void {
  if (id !== seq || noHash(page.url) !== shown) return;
  // Someone is typing on this page: never pull it from under them.
  const a = document.activeElement;
  if (a instanceof HTMLInputElement || a instanceof HTMLTextAreaElement || a instanceof HTMLSelectElement) return;
  const doc = parse(page.html);
  if (build(doc) !== build(document)) return;
  apply(doc);
  emitNavigate(new URL(location.href));
}

// ── The swap ──────────────────────────────────────────────────────────────────────────────────
const PREFS = ["lang", "dir", "data-theme", "data-numerals", "data-tenant"];

function apply(doc: Document): void {
  const root = document.documentElement, next = doc.documentElement;
  const prefsChanged = PREFS.some((a) => root.getAttribute(a) !== next.getAttribute(a));
  for (const a of PREFS) {
    const v = next.getAttribute(a);
    if (v === null) root.removeAttribute(a);
    else root.setAttribute(a, v);
  }
  syncHead(doc);
  if (prefsChanged || !swapRegions(doc)) swapFrame(doc);
  for (const id of ["app-config", "app-strings"]) {
    const a = document.getElementById(id), b = doc.getElementById(id);
    if (a && b && a.textContent !== b.textContent) a.textContent = b.textContent;
  }
}

/** leptos_meta's tags (after the `<!--HEAD-->` marker) and the shell's theme metas. */
function syncHead(doc: Document): void {
  const tail = (head: HTMLHeadElement) => {
    const out: ChildNode[] = [];
    let seen = false;
    for (const n of Array.from(head.childNodes)) {
      if (seen) out.push(n);
      else if (n.nodeType === Node.COMMENT_NODE && (n as Comment).data.trim() === "HEAD") seen = true;
    }
    return seen ? out : null;
  };
  const before = tail(document.head), after = tail(doc.head);
  if (before && after) {
    for (const n of before) n.remove();
    for (const n of after) document.head.append(document.adoptNode(n));
  }
  for (const sel of ['meta[name="color-scheme"]', 'meta[name="theme-color"]']) {
    const a = document.head.querySelector(sel), b = doc.head.querySelector(sel);
    if (a && b) a.setAttribute("content", b.getAttribute("content") ?? "");
  }
}

const regionsOf = (d: Document) => {
  const m = new Map<string, HTMLElement>();
  const main = d.querySelector<HTMLElement>("main");
  if (main) m.set("main", main);
  for (const el of Array.from(d.querySelectorAll<HTMLElement>("[data-region]"))) m.set(el.dataset.region ?? "", el);
  return m;
};

/** Swap `<main>` and the named regions; false if the two pages don't have the same set. */
function swapRegions(doc: Document): boolean {
  const old = regionsOf(document), next = regionsOf(doc);
  if (old.size !== next.size || ![...old.keys()].every((k) => next.has(k))) return false;
  for (const [name, el] of old) {
    const fresh = next.get(name)!;
    if (el.outerHTML === fresh.outerHTML) continue; // unchanged: keep it (and its state) as it is
    replace(el, document.adoptNode(fresh));
  }
  return true;
}

/** Everything in the frame, for a language/numerals/theme change; the toasts on screen stay. */
function swapFrame(doc: Document): void {
  const old = document.querySelector<HTMLElement>(".app"), fresh = doc.querySelector<HTMLElement>(".app");
  if (!old || !fresh) {
    location.reload();
    return;
  }
  const app = document.adoptNode(fresh);
  const toaster = document.getElementById("toaster"), slot = app.querySelector("#toaster");
  if (toaster && slot) {
    for (const a of Array.from(slot.attributes)) toaster.setAttribute(a.name, a.value);
    slot.replaceWith(toaster);
  }
  replace(old, app);
}

/** Replace a region, unmounting what was mounted in it and keeping sideways/inner scroll offsets. */
function replace(el: HTMLElement, fresh: HTMLElement): void {
  const offsets: Array<[string, number, number]> = [];
  for (const s of [el, ...Array.from(el.querySelectorAll<HTMLElement>("*"))]) {
    if ((s.scrollLeft || s.scrollTop) && s !== document.scrollingElement) offsets.push([pathOf(s, el), s.scrollLeft, s.scrollTop]);
  }
  unmountAll(el);
  el.replaceWith(fresh);
  for (const [path, left, top] of offsets) {
    const s = path ? fresh.querySelector<HTMLElement>(path) : fresh;
    if (s) {
      s.scrollLeft = left;
      s.scrollTop = top;
    }
  }
}

/** A selector for `el` inside `root` by class names (the same component renders the same classes). */
function pathOf(el: Element, root: Element): string {
  const parts: string[] = [];
  for (let e: Element | null = el; e && e !== root; e = e.parentElement) {
    const cls = Array.from(e.classList, (c) => "." + CSS.escape(c)).join("");
    parts.unshift(e.tagName.toLowerCase() + cls);
  }
  return parts.join(" > ");
}

interface FocusKey {
  id: string;
  name: string;
  value: string;
}
function focusKeyOf(el: Element | null): FocusKey | null {
  if (!(el instanceof HTMLElement) || el === document.body) return null;
  return { id: el.id, name: el.getAttribute("name") ?? "", value: el instanceof HTMLInputElement ? el.value : "" };
}
function findFocus(root: ParentNode, f: FocusKey): HTMLElement | null {
  if (f.id) return root.querySelector<HTMLElement>(`#${CSS.escape(f.id)}`);
  if (!f.name) return null;
  const n = CSS.escape(f.name);
  return root.querySelector<HTMLElement>(`[name="${n}"][value="${CSS.escape(f.value)}"]`) ?? root.querySelector<HTMLElement>(`[name="${n}"]`);
}

// A product image travels from its card to the product page (and back): only the clicked card is
// named — a product can be on a page twice, and duplicate names abort the transition.
function nameMedia(el: HTMLElement | null): void {
  for (const x of Array.from(document.querySelectorAll<HTMLElement>("[data-vt-named]"))) {
    x.style.removeProperty("view-transition-name");
    x.style.removeProperty("view-transition-class");
    x.removeAttribute("data-vt-named");
  }
  if (!el?.dataset.vt) return;
  el.style.setProperty("view-transition-name", el.dataset.vt);
  el.style.setProperty("view-transition-class", "product-media");
  el.setAttribute("data-vt-named", "");
}

// ── What the router takes ─────────────────────────────────────────────────────────────────────
const NOT_PAGES = /^\/(?:__|api\/|media\/|pkg\/|js\/|fonts\/)|\.(?:webp|png|jpe?g|gif|svg|avif|pdf|json|xml|txt|zip|css|js|woff2?)$/i;

/** The URL a link goes to, if the router should take it. */
function routable(a: Element | null): URL | null {
  if (!(a instanceof HTMLAnchorElement) || !a.hasAttribute("href")) return null;
  if ((a.target && a.target !== "_self") || a.hasAttribute("download") || /\bexternal\b/.test(a.rel)) return null;
  if (a.closest('[data-router="off"]')) return null;
  const u = new URL(a.href, location.href);
  if (u.origin !== location.origin || !/^https?:$/.test(u.protocol)) return null;
  if (NOT_PAGES.test(u.pathname) && !/^\/__(?:prefs|tenant|sim)\b/.test(u.pathname)) return null;
  return u;
}

const sameDocument = (u: URL) => noHash(u) === shown;

function onClick(ev: MouseEvent): void {
  if (ev.defaultPrevented || ev.button !== 0 || ev.metaKey || ev.ctrlKey || ev.shiftKey || ev.altKey) return;
  const a = (ev.target as Element | null)?.closest?.("a[href]") ?? null;
  const u = routable(a);
  if (!u || !a) return;
  if (sameDocument(u) && u.hash) return; // a jump within the page: the browser's
  ev.preventDefault();
  const prefs = u.pathname.startsWith("/__");
  if (!prefs) nameMedia(a.closest(".pcard")?.querySelector<HTMLElement>("[data-vt]") ?? null);
  void go({ url: u, method: "GET", push: true, scroll: prefs ? "keep" : "top" });
}

function onSubmit(ev: SubmitEvent): void {
  if (ev.defaultPrevented) return;
  const form = ev.target;
  if (!(form instanceof HTMLFormElement)) return;
  const by = ev.submitter instanceof HTMLButtonElement || ev.submitter instanceof HTMLInputElement ? ev.submitter : null;
  const attr = (name: string, formName: string) => by?.getAttribute(name) || form.getAttribute(formName);
  const method = (attr("formmethod", "method") ?? "get").toLowerCase();
  const target = attr("formtarget", "target");
  if (method === "dialog" || (target && target !== "_self") || form.closest('[data-router="off"]')) return;
  const url = new URL(attr("formaction", "action") || location.href, location.href);
  if (url.origin !== location.origin) return;
  if (method === "post" && posting) {
    ev.preventDefault(); // one submission at a time (a second tap must not place a second order)
    return;
  }
  let data: FormData;
  try {
    data = by ? new FormData(form, by) : new FormData(form);
  } catch {
    data = new FormData(form);
  }
  // Older browsers leave the pressed button out; without JS it is sent (`name="op" value="remove"`).
  if (by?.name && !data.has(by.name)) data.append(by.name, by.value);
  const pairs = Array.from(data, ([k, v]): [string, string] => [k, typeof v === "string" ? v : v.name]);
  ev.preventDefault();
  if (method === "get") {
    url.search = new URLSearchParams(pairs).toString();
    url.hash = "";
    void go({ url, method: "GET", push: true, scroll: "top" });
  } else {
    const multipart = (attr("formenctype", "enctype") ?? "").toLowerCase().includes("multipart");
    void go({ url, method: "POST", body: multipart ? data : new URLSearchParams(pairs), push: true, scroll: "if-moved" });
  }
}

function onPop(ev: PopStateEvent): void {
  const u = new URL(location.href);
  if (sameDocument(u)) return; // a hash entry of this page
  const y = typeof ev.state?.y === "number" ? (ev.state.y as number) : 0;
  void go({ url: u, method: "GET", push: false, scroll: "restore", y, pop: true });
}

// ── Prefetch: the gesture model ─────────────────────────────────────────────────────────────────
// - a mouse or pen hover warms after HOVER_DELAY_MS (a pass-over warms nothing); one pending
//   hover at a time — entering another target or leaving cancels it;
// - keyboard focus (`:focus-visible`) waits the same; blur cancels; focus moved by code warms
//   nothing;
// - pointerdown warms at once (a phone's only signal; on desktop it buys the ~100 ms to the click)
//   and is never throttled;
// - the same href is warmed at most once per DEDUPE_MS; hovers and focus warm at most
//   HOVER_BUDGET_PER_S targets a second; nothing is warmed on Save-Data or 2G.
// Warming is the router's own GET into the page cache, so the click finds it there (or joins it
// mid-flight). /__ links set preferences (state-changing GETs): never warmed.
const HOVER_DELAY_MS = 65, DEDUPE_MS = 2000, HOVER_BUDGET_PER_S = 6;
const stats = { warmed: 0, skipped: 0 };
const lastWarm = new Map<string, number>();
const hoverWarms: number[] = [];
let hoverTimer = 0;
let hovered: Element | null = null;

const connectionAllowsPrefetch = () => {
  const c = (navigator as Navigator & { connection?: { saveData?: boolean; effectiveType?: string } }).connection;
  return !c || (!c.saveData && !/(^|-)2g$/.test(c.effectiveType ?? ""));
};

function warmable(a: Element | null): string | null {
  const u = routable(a);
  if (!u || u.pathname.startsWith("/__") || sameDocument(u)) return null;
  return noHash(u);
}

function warm(href: string): void {
  if (!connectionAllowsPrefetch()) {
    stats.skipped++;
    return;
  }
  const now = Date.now(), at = lastWarm.get(href);
  if (at !== undefined && now - at < DEDUPE_MS) {
    stats.skipped++;
    return;
  }
  lastWarm.set(href, now);
  if (lastWarm.size > 500) for (const [k, t] of lastWarm) if (now - t >= DEDUPE_MS) lastWarm.delete(k);
  stats.warmed++;
  loadGet(href).then(
    (p) => {
      if (p.personal && !consumed.has(p)) warmPersonal.set(pageKey(href), { page: p, at: Date.now() });
      preload(featuresIn(p.html), { intent: false }); // and the chunks it needs, so the tap finds them ready
    },
    () => {}, // a failed warm costs nothing
  );
}

function cancelHover(): void {
  clearTimeout(hoverTimer);
  hoverTimer = 0;
}

function scheduleHover(href: string): void {
  cancelHover();
  hoverTimer = window.setTimeout(() => {
    hoverTimer = 0;
    const now = Date.now();
    while (hoverWarms.length && now - hoverWarms[0]! >= 1000) hoverWarms.shift();
    if (hoverWarms.length >= HOVER_BUDGET_PER_S) {
      stats.skipped++;
      return;
    }
    hoverWarms.push(now);
    warm(href);
  }, HOVER_DELAY_MS);
}

const linkOf = (t: EventTarget | null) => (t instanceof Element ? t.closest("a[href]") : null);

function startPrefetch(): void {
  document.addEventListener(
    "pointerover",
    (e) => {
      if (e.pointerType === "touch") return; // touch has no hover: its press is handled below
      const a = linkOf(e.target);
      if (a === hovered) return; // moving within the same link is not a new enter
      hovered = a;
      cancelHover();
      const href = warmable(a);
      if (href) scheduleHover(href);
    },
    true,
  );
  document.addEventListener(
    "pointerout",
    (e) => {
      const a = linkOf(e.target);
      if (a && a === hovered && !a.contains(e.relatedTarget as Node | null)) {
        hovered = null;
        cancelHover();
      }
    },
    true,
  );
  document.addEventListener(
    "pointerdown",
    (e) => {
      cancelHover();
      const href = warmable(linkOf(e.target));
      if (href) warm(href);
    },
    { capture: true, passive: true },
  );
  document.addEventListener("focusin", (e) => {
    // Keyboard focus only: focus moved by code (back to a cart line after its editor closed) or by
    // a press (warmed on pointer-down already) is not a visitor heading somewhere.
    const a = linkOf(e.target);
    if (!a?.matches(":focus-visible")) return;
    const href = warmable(a);
    if (href) scheduleHover(href);
  });
  document.addEventListener("focusout", cancelHover);
}

// The page the visit started on was never fetched by the router; warm it once the visitor has
// moved on, so coming back to it is instant too.
let landing = "";
let landingWarmed = false;
function warmLanding(): void {
  landingWarmed = true;
  const href = landing;
  window.setTimeout(() => {
    if (href && href !== shown && connectionAllowsPrefetch()) loadGet(href).catch(() => {});
  }, 2500);
}

// ── API ───────────────────────────────────────────────────────────────────────────────────────
/** Start intercepting links and forms (called once at boot). */
export function startRouter(): void {
  if (started || typeof DOMParser !== "function" || !("pushState" in history)) return;
  started = true;
  shown = landing = noHash(location.href);
  history.scrollRestoration = "manual";
  const nav = performance.getEntriesByType("navigation")[0] as PerformanceNavigationTiming | undefined;
  const y = history.state?.y;
  if (nav && (nav.type === "reload" || nav.type === "back_forward") && typeof y === "number") scrollTo({ top: y, behavior: "instant" });
  else saveScroll();
  // The page is server-rendered with its own address; mark the entry ours.
  addEventListener("click", onClick); // on window: every feature's own handler has run first
  addEventListener("submit", onSubmit);
  addEventListener("popstate", onPop);
  let t = 0;
  addEventListener(
    "scroll",
    () => {
      clearTimeout(t);
      t = window.setTimeout(saveScroll, 120);
    },
    { passive: true },
  );
  addEventListener("pagehide", saveScroll);
  startPrefetch();
  if (__DEV__) (window as unknown as { __router: unknown }).__router = { stats, routing, navFailure };
}

/** Go to `url` (pushes history unless `replace`). */
export async function navigate(url: string, opts: NavigateOpts = {}): Promise<void> {
  const u = new URL(url, location.href);
  if (!started || u.origin !== location.origin) {
    if (opts.replace) location.replace(u.href);
    else location.assign(u.href);
    return;
  }
  await go({ url: u, method: "GET", push: !opts.replace, scroll: opts.scroll ?? "top" });
}

/** Re-fetch the current page, skipping the page cache (pull-to-refresh, the refresh control). */
export async function refresh(): Promise<void> {
  if (!started) {
    location.reload();
    return;
  }
  await go({ url: new URL(location.href), method: "GET", push: false, scroll: "keep", fresh: true });
}

/** Warm the page cache for `url` (hover / finger-down intent). */
export function prefetch(url: string): void {
  if (!started) return;
  const a = document.createElement("a");
  a.href = url;
  const href = warmable(a);
  if (href) warm(href);
}

/**
 * A page's document without showing it, for a part of it shown elsewhere (the cart's line editor,
 * in a sheet). The same loader as a navigation: the page cache, and a page warmed by intent
 * (finger-down) is used. Throws like a navigation would (`NetError`); `null` for a page of another
 * build (a deploy: its markup may not match this code).
 */
export async function fetchDocument(url: string): Promise<Document | null> {
  const page = await loadGet(noHash(new URL(url, location.href)));
  consumed.add(page);
  const doc = parse(page.html);
  return build(doc) === build(document) ? doc : null;
}

/** After every client-side navigation, once the new page's regions are in. */
export function onNavigate(fn: Listener): () => void {
  listeners.add(fn);
  return () => listeners.delete(fn);
}

/** For the implementation: tell listeners a navigation completed. */
export function emitNavigate(url: URL): void {
  for (const fn of [...listeners]) {
    try {
      fn(url);
    } catch (e) {
      reportError(e); // one broken listener doesn't stop the others
    }
  }
}
