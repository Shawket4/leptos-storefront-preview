// Every request the browser code makes goes through here: one timeout, one offline rule, one retry
// policy, one error shape. (Navigations go through the router, which uses `request` too.)

export type NetErrorCode = "offline" | "timeout" | "http" | "parse" | "aborted";

export class NetError extends Error {
  constructor(
    readonly code: NetErrorCode,
    readonly status?: number,
  ) {
    super(code + (status ? ` ${status}` : ""));
  }
}

export const isOnline = (): boolean => navigator.onLine !== false;

/** Calls `fn` when the connection comes back; returns an unsubscribe. */
export function onOnline(fn: () => void): () => void {
  addEventListener("online", fn);
  return () => removeEventListener("online", fn);
}

export interface RequestOpts {
  timeoutMs?: number;
  /** Extra attempts after a network failure or a 5xx (never after a 4xx). */
  retries?: number;
  signal?: AbortSignal;
  init?: RequestInit;
  /** Which responses count as success (default: 2xx). The router keeps 4xx pages (a 404 is a page). */
  ok?: (res: Response) => boolean;
}

export async function request(url: string, opts: RequestOpts = {}): Promise<Response> {
  const { timeoutMs = 10_000, retries = 1, signal, init, ok = (r: Response) => r.ok } = opts;
  for (let attempt = 0; ; attempt++) {
    if (!isOnline()) throw new NetError("offline");
    const ctrl = new AbortController();
    const timer = setTimeout(() => ctrl.abort("timeout"), timeoutMs);
    signal?.addEventListener("abort", () => ctrl.abort("aborted"), { once: true });
    try {
      const res = await fetch(url, { credentials: "same-origin", ...init, signal: ctrl.signal });
      if (res.status >= 500 && attempt < retries) throw new NetError("http", res.status);
      if (!ok(res)) throw new NetError("http", res.status);
      return res;
    } catch (e) {
      const err =
        e instanceof NetError
          ? e
          : ctrl.signal.aborted
            ? new NetError(ctrl.signal.reason === "timeout" ? "timeout" : "aborted")
            : new NetError(isOnline() ? "http" : "offline");
      const retryable = err.code === "timeout" || (err.code === "http" && (err.status ?? 500) >= 500);
      if (!retryable || attempt >= retries || signal?.aborted) throw err;
      await new Promise((r) => setTimeout(r, 400 * 2 ** attempt));
    } finally {
      clearTimeout(timer);
    }
  }
}

export async function getJSON<T>(url: string, opts: RequestOpts = {}): Promise<T> {
  const res = await request(url, { ...opts, init: { ...opts.init, headers: { Accept: "application/json" } } });
  try {
    return (await res.json()) as T;
  } catch {
    throw new NetError("parse");
  }
}
