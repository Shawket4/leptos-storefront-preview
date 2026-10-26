// Features attach to server-rendered markup by `data-ts="<feature>"`. `register` once at boot;
// `mountAll(root)` runs on load and after every router swap, mounting each element once.
//
// A feature only some pages have is declared `lazy([...names], () => import("../features/x"))`
// (main.ts): the first time `mountAll` meets one of its names, the chunk is imported once (its
// module registers the names), then everything waiting for it mounts. A name ending in `*` covers
// a prefix (`"sc-*"`). `preload(names)` starts those imports early (the router, when it warms a page).

type Unmount = () => void;
type MountFn = (el: HTMLElement) => Unmount | void;

const features = new Map<string, MountFn>();
const mounted = new WeakMap<HTMLElement, Map<string, Unmount | undefined>>();

interface Lazy {
  names: string[];
  loader: () => Promise<unknown>;
  promise: Promise<void> | null;
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

/** Start loading the chunks behind these names (no mounting); unknown or loaded names are ignored. */
export function preload(names: Iterable<string>): void {
  for (const n of names) {
    if (features.has(n)) continue;
    const l = lazyFor(n);
    if (l) load(l).catch(() => {});
  }
}

/** `root` itself (when it carries data-ts) and everything inside it with data-ts. */
const withTs = (root: ParentNode): HTMLElement[] => {
  const all = Array.from(root.querySelectorAll<HTMLElement>("[data-ts]"));
  return root instanceof HTMLElement && root.hasAttribute("data-ts") ? [root, ...all] : all;
};

export function mountAll(root: ParentNode = document): void {
  const waiting = new Set<Lazy>();
  for (const el of withTs(root)) {
    for (const name of (el.dataset.ts ?? "").split(/\s+/).filter(Boolean)) {
      const fn = features.get(name);
      if (!fn) {
        const l = lazyFor(name);
        if (l) waiting.add(l);
        else if (__DEV__) console.warn(`no feature registered for data-ts="${name}"`);
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
    load(l).then(() => {
      if (root instanceof Node && !root.isConnected) return; // swapped out meanwhile
      // The chunk must register what it was declared for, or mountAll would wait on it forever.
      for (const el of withTs(root))
        for (const name of (el.dataset.ts ?? "").split(/\s+/))
          if (name && !features.has(name) && lazyFor(name) === l) {
            if (__DEV__) console.error(`lazy chunk loaded but data-ts="${name}" isn't registered`);
            missing.add(name);
          }
      mountAll(root);
    }, reportError);
  }
}

/** Run the cleanups of features mounted inside `root` (the router calls this before a swap). */
export function unmountAll(root: ParentNode): void {
  for (const el of withTs(root)) {
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
