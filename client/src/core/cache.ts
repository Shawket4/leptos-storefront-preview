// The client data cache (pages and API reads), like Madar's: a fresh window (TTL), then
// stale-while-revalidate (shown at once, refreshed in the background), then gone. Invalidated by
// TTL, by an explicit refresh, by the server's catalogue generation changing, and by mutations
// (e.g. the cart drops its quote). Keys carry the display prefs so a language switch never shows
// a cached page in the other language.

import { cfg } from "./page";

export interface CacheOpts {
  /** ms the value is served without asking the server. */
  ttl: number;
  /** ms after `ttl` the old value is still served while a fresh one is fetched. */
  swr?: number;
}

interface Entry<T> {
  value: T;
  at: number;
  opts: CacheOpts;
}

const MAX_ENTRIES = 60;
const entries = new Map<string, Entry<unknown>>();
const inflight = new Map<string, Promise<unknown>>();
let generation: string | null = null;

/** A cache key for `parts` in the current tenant and display prefs. */
export function key(...parts: string[]): string {
  const c = cfg();
  return [c.tenant, c.lang, c.numerals, ...parts].join("|");
}

/**
 * The value for `k`: fresh from the cache, or stale (calling `onUpdate` when the refresh lands),
 * or fetched now. Concurrent calls for one key share one fetch.
 */
export async function cached<T>(
  k: string,
  opts: CacheOpts,
  fetcher: () => Promise<T>,
  onUpdate?: (value: T) => void,
): Promise<T> {
  const hit = entries.get(k) as Entry<T> | undefined;
  const age = hit ? Date.now() - hit.at : Infinity;
  if (hit && age <= opts.ttl) return hit.value;
  if (hit && age <= opts.ttl + (opts.swr ?? 0)) {
    void load(k, opts, fetcher).then((v) => onUpdate?.(v), () => {});
    return hit.value;
  }
  return load(k, opts, fetcher);
}

function load<T>(k: string, opts: CacheOpts, fetcher: () => Promise<T>): Promise<T> {
  const running = inflight.get(k) as Promise<T> | undefined;
  if (running) return running;
  const p = fetcher()
    .then((value) => {
      entries.delete(k);
      entries.set(k, { value, at: Date.now(), opts });
      while (entries.size > MAX_ENTRIES) entries.delete(entries.keys().next().value as string);
      return value;
    })
    .finally(() => inflight.delete(k));
  inflight.set(k, p);
  return p;
}

/** The cached value for `k` if any (fresh or stale), without fetching. */
export function peek<T>(k: string): T | undefined {
  return (entries.get(k) as Entry<T> | undefined)?.value;
}

/** Drop every entry whose key contains `part` (or everything with no argument). */
export function invalidate(part?: string): void {
  for (const k of [...entries.keys()]) if (part === undefined || k.includes(part)) entries.delete(k);
}

/** The server's catalogue generation (response header `x-catalog-gen`): a new one drops all pages. */
export function noteGeneration(gen: string | null): void {
  if (!gen) return;
  if (generation !== null && gen !== generation) invalidate();
  generation = gen;
}
