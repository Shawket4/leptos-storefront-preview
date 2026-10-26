// Features attach to server-rendered markup by `data-ts="<feature>"`. `register` once at boot;
// `mountAll(root)` runs on load and after every router swap, mounting each element once.
//
// A feature only some pages have is declared `lazy([...names], () => import("../features/x"))`
// (main.ts): the first time `mountAll` meets one of its names, the chunk is imported once (its
// module registers the names), then everything waiting for it mounts. A name ending in `*` covers
// a prefix (`"sc-*"`). `preload(names)` starts those imports early (the router, when it warms a page).
//
// A feature that is on every page but needed only when the visitor reaches for it (the header's
// instant search, the cart drawer's contents) is `lazyOnIntent(names, loader, opts)`: `mountAll`
// arms its elements instead of loading. A pointer coming over one, or a touch, starts the import;
// focus, a press or a key loads it — and as soon as the chunk is in, every element waiting for it
// mounts and the listeners come off. The event itself is never cancelled: a press on a link still
// follows it (through the router), focus and keystrokes stay where they went. `opts.now(el)` loads
// at once (a search page whose field is already filled); `opts.idle` also starts the import when
// the page has settled; `opts.triggers` (a selector) arms other elements anywhere in the page too
// (the header's cart button for the drawer). `ensure(name)` loads and mounts it now (a programmatic
// open). Elements a router swap removes are disarmed with them (`unmountAll`).

type Unmount = () => void;
type MountFn = (el: HTMLElement) => Unmount | void;

export interface IntentOpts {
  /** Load and mount at once for this element (e.g. its field already has focus or a value). */
  now?: (el: HTMLElement) => boolean;
  /** Also start the import when the page has settled (after `load`, on idle). */
  idle?: boolean;
  /** Other intent targets anywhere in the page (a CSS selector). */
  triggers?: string;
}

const features = new Map<string, MountFn>();
const mounted = new WeakMap<HTMLElement, Map<string, Unmount | undefined>>();

interface Lazy {
  names: string[];
  loader: () => Promise<unknown>;
  promise: Promise<void> | null;
  /** lazyOnIntent: its options, the armed elements (→ disarm) and the page-wide trigger listener. */
  intent?: IntentOpts;
  armed?: Map<HTMLElement, () => void>;
  offTriggers?: (() => void) | null;
}
const lazies: Lazy[] = [];
/** Names a loaded chunk didn't register after all (dev mistake): never waited on again. */
const missing = new Set<string>();

export function register(feature: string, fn: MountFn): void {
  if (features.has(feature)) throw new Error(`feature registered twice: ${feature}`);
  features.set(feature, fn);
}

/** Declare features that live in a chunk loaded on demand (main.ts). */
export function lazy(names: string[], loader: () => Promise<unknown>): void {
  lazies.push({ names, loader, promise: null });
}

/** Declare features loaded when the visitor reaches for them (see the module docs). */
export function lazyOnIntent(names: string[], loader: () => Promise<unknown>, opts: IntentOpts = {}): void {
  const l: Lazy = { names, loader, promise: null, intent: opts, armed: new Map(), offTriggers: null };
  lazies.push(l);
  if (opts.idle) whenSettled(() => void loadAndMount(l).catch(() => {}));
}

/** Load (if need be) and mount a feature declared with `lazyOnIntent` (or `lazy`) now. */
export function ensure(name: string): Promise<void> {
  if (features.has(name)) {
    mountAll(document);
    return Promise.resolve();
  }
  const l = lazyFor(name);
  if (!l) return Promise.reject(new Error(`ensure: no feature "${name}"`));
  return loadAndMount(l);
}

const covers = (pattern: string, name: string) =>
  pattern.endsWith("*") ? name.startsWith(pattern.slice(0, -1)) : pattern === name;
const lazyFor = (name: string) => (missing.has(name) ? undefined : lazies.find((l) => l.names.some((p) => covers(p, name))));

function load(l: Lazy): Promise<void> {
  l.promise ??= l.loader().then(
    () => {},
    (e: unknown) => {
      l.promise = null; // offline, say: the next mountAll tries again
      throw e;
    },
  );
  return l.promise;
}

/** The chunk is in: whatever was waiting for it mounts, wherever it is; intent listeners go. */
function loadAndMount(l: Lazy): Promise<void> {
  return load(l).then(() => {
    disarm(l);
    verify(l, document);
    mountAll(document);
  });
}

/**
 * Start loading the chunks behind these names (no mounting); unknown or loaded names are ignored.
 * `intent: false` leaves `lazyOnIntent` features alone (the router, warming a page, doesn't
 * load what the visitor hasn't reached for).
 */
export function preload(names: Iterable<string>, opts: { intent?: boolean } = {}): void {
  for (const n of names) {
    if (features.has(n)) continue;
    const l = lazyFor(n);
    if (!l || (l.intent && opts.intent === false)) continue;
    if (l.intent) loadAndMount(l).catch(() => {});
    else load(l).catch(() => {});
  }
}

/** `root` itself (when it carries data-ts) and everything inside it with data-ts. */
const withTs = (root: ParentNode): HTMLElement[] => {
  const all = Array.from(root.querySelectorAll<HTMLElement>("[data-ts]"));
  return root instanceof HTMLElement && root.hasAttribute("data-ts") ? [root, ...all] : all;
};

/** The chunk must register what it was declared for, or mountAll would wait on it forever. */
function verify(l: Lazy, root: ParentNode): void {
  for (const el of withTs(root))
    for (const name of (el.dataset.ts ?? "").split(/\s+/))
      if (name && !features.has(name) && lazyFor(name) === l) {
        if (__DEV__) console.error(`lazy chunk loaded but data-ts="${name}" isn't registered`);
        missing.add(name);
      }
}

// ── Intent ──────────────────────────────────────────────────────────────────────────────────
// Every event that means "about to use it" starts the load; none is cancelled or delayed.
const INTENT = ["pointerover", "touchstart", "focusin", "pointerdown", "keydown"] as const;

function arm(l: Lazy, el: HTMLElement): void {
  if (l.armed!.has(el)) return;
  const go = () => void loadAndMount(l).catch(reportError);
  for (const type of INTENT) el.addEventListener(type, go, { passive: true, capture: true });
  l.armed!.set(el, () => {
    for (const type of INTENT) el.removeEventListener(type, go, { capture: true });
  });
  if (l.intent?.triggers && !l.offTriggers) {
    const sel = l.intent.triggers;
    const onDoc = (ev: Event) => {
      if ((ev.target as Element | null)?.closest?.(sel)) go();
    };
    for (const type of INTENT) document.addEventListener(type, onDoc, { passive: true, capture: true });
    l.offTriggers = () => {
      for (const type of INTENT) document.removeEventListener(type, onDoc, { capture: true });
    };
  }
}

function disarm(l: Lazy, el?: HTMLElement): void {
  if (!l.armed) return;
  if (el) {
    l.armed.get(el)?.();
    l.armed.delete(el);
    return;
  }
  for (const off of l.armed.values()) off();
  l.armed.clear();
  l.offTriggers?.();
  l.offTriggers = null;
}

/** After the page's `load`, when the browser is idle (or soon after, if it never is). */
function whenSettled(fn: () => void): void {
  const idle = () => ("requestIdleCallback" in window ? requestIdleCallback(fn, { timeout: 3000 }) : setTimeout(fn, 1500));
  if (document.readyState === "complete") idle();
  else addEventListener("load", idle, { once: true });
}

export function mountAll(root: ParentNode = document): void {
  const waiting = new Set<Lazy>();
  for (const el of withTs(root)) {
    for (const name of (el.dataset.ts ?? "").split(/\s+/).filter(Boolean)) {
      const fn = features.get(name);
      if (!fn) {
        const l = lazyFor(name);
        if (!l) {
          if (__DEV__) console.warn(`no feature registered for data-ts="${name}"`);
        } else if (!l.intent || l.intent.now?.(el)) waiting.add(l);
        else arm(l, el);
        continue;
      }
      let done = mounted.get(el);
      if (!done) mounted.set(el, (done = new Map()));
      if (done.has(name)) continue;
      try {
        done.set(name, fn(el) ?? undefined);
      } catch (e) {
        done.set(name, undefined);
        reportError(e); // loud in the console; the other features still mount
      }
    }
  }
  for (const l of waiting) {
    if (l.intent) {
      loadAndMount(l).catch(reportError);
      continue;
    }
    load(l).then(() => {
      if (root instanceof Node && !root.isConnected) return; // swapped out meanwhile
      verify(l, root);
      mountAll(root);
    }, reportError);
  }
}

/** Run the cleanups of features mounted inside `root` (the router calls this before a swap). */
export function unmountAll(root: ParentNode): void {
  for (const el of withTs(root)) {
    for (const l of lazies) if (l.armed?.has(el)) disarm(l, el);
    const done = mounted.get(el);
    if (!done) continue;
    for (const un of done.values()) {
      try {
        un?.();
      } catch (e) {
        reportError(e);
      }
    }
    mounted.delete(el);
  }
}

if (__DEV__) (window as unknown as { __mount: unknown }).__mount = { ensure, preload };
