// The one state primitive: a value with subscribers. Every store is a module-level singleton
// created once (stores/*.ts) — features import the store, they never make their own copy.

export interface Store<T> {
  get(): T;
  set(next: T): void;
  update(fn: (prev: T) => T): void;
  /** Calls `fn` now and on every change; returns an unsubscribe. */
  subscribe(fn: (value: T) => void): () => void;
}

export function createStore<T>(initial: T): Store<T> {
  let value = initial;
  const subs = new Set<(v: T) => void>();
  const set = (next: T) => {
    if (Object.is(next, value)) return;
    value = next;
    for (const fn of [...subs]) fn(value);
  };
  return {
    get: () => value,
    set,
    update: (fn) => set(fn(value)),
    subscribe(fn) {
      subs.add(fn);
      fn(value);
      return () => subs.delete(fn);
    },
  };
}
