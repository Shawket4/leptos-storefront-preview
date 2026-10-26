// Features attach to server-rendered markup by `data-ts="<feature>"`. `register` once at boot;
// `mountAll(root)` runs on load and after every router swap, mounting each element once.

type Unmount = () => void;
type MountFn = (el: HTMLElement) => Unmount | void;

const features = new Map<string, MountFn>();
const mounted = new WeakMap<HTMLElement, Map<string, Unmount | undefined>>();

export function register(feature: string, fn: MountFn): void {
  if (features.has(feature)) throw new Error(`feature registered twice: ${feature}`);
  features.set(feature, fn);
}

/** `root` itself (when it carries data-ts) and everything inside it with data-ts. */
const withTs = (root: ParentNode): HTMLElement[] => {
  const all = Array.from(root.querySelectorAll<HTMLElement>("[data-ts]"));
  return root instanceof HTMLElement && root.hasAttribute("data-ts") ? [root, ...all] : all;
};

export function mountAll(root: ParentNode = document): void {
  for (const el of withTs(root)) {
    for (const name of (el.dataset.ts ?? "").split(/\s+/).filter(Boolean)) {
      const fn = features.get(name);
      if (!fn) {
        if (__DEV__) console.warn(`no feature registered for data-ts="${name}"`);
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
