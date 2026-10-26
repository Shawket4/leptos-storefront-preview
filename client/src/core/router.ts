// Client-side navigation. API is final; the implementation is the core agent's: link and form
// interception, fetch + swap of the page's regions, history and scroll, the progress bar, failure
// and retry, prefetch on intent (Madar's gesture model) and the page cache (core/cache.ts).
// Until then navigations are ordinary page loads, which is always a correct fallback.

import { createStore } from "./store";

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

/** Start intercepting links and forms (called once at boot). */
export function startRouter(): void {}

/** Go to `url` (pushes history unless `replace`). */
export async function navigate(url: string, opts: NavigateOpts = {}): Promise<void> {
  if (opts.replace) location.replace(url);
  else location.assign(url);
}

/** Re-fetch the current page, skipping the page cache (pull-to-refresh, the refresh control). */
export async function refresh(): Promise<void> {
  location.reload();
}

/** Warm the page cache for `url` (hover / finger-down intent). */
export function prefetch(_url: string): void {}

/** After every client-side navigation, once the new page's regions are in. */
export function onNavigate(fn: Listener): () => void {
  listeners.add(fn);
  return () => listeners.delete(fn);
}

/** For the implementation: tell listeners a navigation completed. */
export function emitNavigate(url: URL): void {
  for (const fn of [...listeners]) fn(url);
}
