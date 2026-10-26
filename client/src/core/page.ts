// What the server tells the browser code about this page: the JSON blocks it renders in the shell
// (`#app-config`, `#app-strings`). Re-read after every navigation (language can change).

export interface AppConfig {
  tenant: string;
  lang: "en" | "ar";
  numerals: "latn" | "arab";
  /** Endpoints the client calls when it needs data (never for state changes). */
  api: { cartQuote: string; searchSuggest: string; orderTrack: string };
  /** Cart limits the server enforces too. */
  cart: { maxLines: number; maxQty: number; cookie: string };
}

function readJson<T>(id: string): T | null {
  const el = document.getElementById(id);
  if (!el?.textContent) return null;
  try {
    return JSON.parse(el.textContent) as T;
  } catch {
    return null;
  }
}

let config: AppConfig | null = null;
let strings: Record<string, string> = {};

/** (Re)load the page's config and strings; called on boot and after every navigation. */
export function readPage(): void {
  config = readJson<AppConfig>("app-config") ?? config;
  strings = readJson<Record<string, string>>("app-strings") ?? strings;
}

export function cfg(): AppConfig {
  if (!config) readPage();
  if (!config) throw new Error("missing #app-config");
  return config;
}

/**
 * A UI string from the server's catalogue (crates/app/src/strings/), in the page's language.
 * `{name}` placeholders are filled from `vars`. Unknown keys show the key (and warn in dev).
 */
export function t(key: string, vars?: Record<string, string | number>): string {
  const s = strings[key];
  if (s === undefined) {
    if (__DEV__) console.warn(`missing string: ${key}`);
    return key;
  }
  return vars ? s.replace(/\{(\w+)\}/g, (m, k: string) => (k in vars ? String(vars[k]) : m)) : s;
}

/**
 * Digits in the visitor's numerals (`cfg().numerals`): counts and clocks the client shows (the
 * cart badge, a countdown). Not for money — money always arrives formatted by the server.
 */
export function digits(n: number | string): string {
  const s = String(n);
  return cfg().numerals === "arab" ? s.replace(/[0-9]/g, (d) => String.fromCharCode(0x0660 + Number(d))) : s;
}
