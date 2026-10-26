// The listing (category and /search results): `data-ts="listing"`. The URL is the state and the
// server renders every result; this file only makes the trip quick.

import { register } from "../core/mount";
import { $, $$, on, patch } from "../core/dom";
import { navigate, routing } from "../core/router";

// ── Listing ─────────────────────────────────────────────────────────────────────────────────────
// The URL is the state. A filter or sort change edits the filter form(s) at once (that is the
// shopper's state: checkboxes, the chosen sort, a chip gone) and, after a short quiet so several
// taps make one trip, navigates to the listing they describe (replace, scroll kept). The server
// renders the new results; on phones the sheet stays open across that swap (core/drawer.ts).

/** Taps (checkboxes, sort, chips) go out together after this much quiet. */
const QUIET_MS = 250;
/** Typing a price waits longer (a pause between digits is not "done"). */
const TYPING_MS = 800;

/** `ListingQuery::to_query`'s encoding (`api/types.rs` `url_encode`): space as `+`, `!'()*` escaped. */
const enc = (s: string) =>
  encodeURIComponent(s)
    .replace(/[!'()*]/g, (c) => "%" + c.charCodeAt(0).toString(16).toUpperCase())
    .replace(/%20/g, "+");

/** Whole pounds typed by a shopper (Western or Arabic-Indic digits, separators ignored). */
function amount(v: FormDataEntryValue | null): number | null {
  if (typeof v !== "string") return null;
  let d = "";
  for (const ch of v.trim()) {
    const c = ch.charCodeAt(0);
    if (c >= 48 && c <= 57) d += ch;
    else if (c >= 0x660 && c <= 0x669) d += String(c - 0x660);
    else if (c >= 0x6f0 && c <= 0x6f9) d += String(c - 0x6f0);
    else if (ch === "." || ch === "٫") break; // whole pounds
    else if (!",٬   ".includes(ch)) return null;
  }
  const n = d ? Number(d) : NaN;
  return Number.isFinite(n) && n > 0 ? n : null;
}

/**
 * The listing a filter form describes, in `ListingQuery::to_query`'s canonical order (q, min, max,
 * brands sorted, stock, deal, sort; page dropped), so one listing is one URL (and one cache entry).
 */
function listingHref(form: HTMLFormElement): string {
  const fd = new FormData(form);
  const parts: string[] = [];
  const q = String(fd.get("q") ?? "").trim();
  if (q) parts.push("q=" + enc(q));
  let min = amount(fd.get("min"));
  let max = amount(fd.get("max"));
  if (min !== null && max !== null && min > max) [min, max] = [max, min];
  if (min !== null) parts.push(`min=${min}`);
  if (max !== null) parts.push(`max=${max}`);
  for (const b of (fd.getAll("brand") as string[]).filter(Boolean).sort()) parts.push("brand=" + enc(b));
  if (fd.get("stock")) parts.push("stock=1");
  if (fd.get("deal")) parts.push("deal=1");
  const sort = String(fd.get("sort") ?? "");
  if (sort && sort !== "relevance") parts.push("sort=" + enc(sort));
  const base = form.getAttribute("action") || location.pathname;
  return parts.length ? `${base}?${parts.join("&")}` : base;
}

register("listing", (root) => {
  const forms = () => $$<HTMLFormElement>("form[data-filters]", root);
  const results = $(".lx-results", root);
  const body = $(".lx-results-body", root);
  const progress = $(".lx-progress", root);
  // The design system's Menu (a `details[data-popover]`: core gives it light dismiss and Escape).
  const sortMenu = $<HTMLDetailsElement>("details.lx-sort", root);
  let timer = 0;
  let source: HTMLFormElement | null = null;

  const busy = (on: boolean) => {
    body?.classList.toggle("is-pending", on);
    progress?.classList.toggle("is-on", on);
    results?.setAttribute("aria-busy", String(on));
    for (const b of $$(".lx-show", root)) b.classList.toggle("is-pending", on);
  };

  /** Make every filter form say what `from` says (the sidebar and the phone sheet). */
  const mirror = (from: HTMLFormElement) => {
    for (const f of forms()) {
      if (f === from) continue;
      for (const el of $$<HTMLInputElement>("input[name]", f)) {
        if (el.type === "checkbox") {
          el.checked = !!from.querySelector<HTMLInputElement>(
            `input[type=checkbox][name="${CSS.escape(el.name)}"][value="${CSS.escape(el.value)}"]:checked`,
          );
        } else {
          const twin = from.querySelector<HTMLInputElement>(`input[name="${CSS.escape(el.name)}"]`);
          if (twin) el.value = twin.value;
        }
      }
    }
  };

  const go = () => {
    clearTimeout(timer);
    timer = 0;
    const form = source ?? forms()[0];
    source = null;
    if (!form?.isConnected) return;
    const href = listingHref(form);
    if (href === location.pathname + location.search) return;
    busy(true); // the router's `pending` takes over; a plain page load keeps this until unload
    void navigate(href, { replace: true, scroll: "keep" });
  };
  const schedule = (form: HTMLFormElement, ms = QUIET_MS) => {
    source = form;
    mirror(form);
    clearTimeout(timer);
    timer = window.setTimeout(go, ms);
  };
  const formOf = (el: Element) => el.closest<HTMLFormElement>("form[data-filters]");

  const offs = [
    on(root, "change", "form[data-filters] input", (_ev, el) => {
      const form = formOf(el);
      if (form) schedule(form, el.matches("[type=checkbox]") ? QUIET_MS : 0);
    }),
    on(root, "input", "form[data-filters] input[inputmode=numeric]", (_ev, el) => {
      const form = formOf(el);
      if (form) schedule(form, TYPING_MS);
    }),
    // Enter in a price box (and the no-JS Apply): go now.
    on(root, "submit", "form[data-filters]", (ev, el) => {
      ev.preventDefault();
      schedule(el as HTMLFormElement, 0);
    }),
    // Sort: the menu says the new order at once; the results follow.
    on(root, "click", "details.lx-sort a[data-sort]", (ev, a) => {
      if (ev.button !== 0 || ev.metaKey || ev.ctrlKey || ev.shiftKey || ev.altKey) return;
      ev.preventDefault();
      const code = a.dataset.sort ?? "relevance";
      for (const opt of $$("a[data-sort]", sortMenu ?? root)) opt.setAttribute("aria-current", String(opt === a));
      if (sortMenu) {
        patch(sortMenu, { value: a.textContent?.trim() ?? "" });
        sortMenu.open = false;
      }
      for (const f of forms()) {
        const s = f.querySelector<HTMLInputElement>("input[name=sort]");
        if (s) s.value = code;
      }
      const form = forms()[0];
      if (form) schedule(form);
    }),
    // A chip (or Clear) resets its field in the form; the chip goes at once.
    on(root, "click", "[data-clear]", (ev, el) => {
      if (ev.button !== 0 || ev.metaKey || ev.ctrlKey || ev.shiftKey || ev.altKey) return;
      ev.preventDefault();
      const field = el.dataset.clear ?? "";
      const value = el.dataset.value ?? "";
      for (const f of forms()) {
        for (const input of $$<HTMLInputElement>("input[name]", f)) {
          const n = input.name;
          const hit =
            field === "all"
              ? n !== "q" && n !== "sort"
              : field === "price"
                ? n === "min" || n === "max"
                : n === field && (field !== "brand" || input.value === value);
          if (!hit) continue;
          if (input.type === "checkbox") input.checked = false;
          else input.value = "";
        }
      }
      const chips = $(".lx-chips", root);
      if (field === "all") {
        if (chips) chips.hidden = true;
      } else {
        const li = el.closest("li");
        if (li) li.hidden = true;
      }
      const form = forms()[0];
      if (form) schedule(form);
    }),
    // "Show N items" closes the sheet (core/drawer.ts): don't make the shopper wait for the quiet.
    on(root, "click", ".lx-show", () => {
      if (timer) go();
    }),
  ];
  // Results dim while the router fetches the next ones.
  const unsub = routing.subscribe((r) => busy(r.pending));
  // Back here through the back-forward cache after a plain page load: not loading any more.
  const shown = (ev: PageTransitionEvent) => {
    if (ev.persisted) busy(false);
  };
  addEventListener("pageshow", shown);

  return () => {
    clearTimeout(timer);
    for (const off of offs) off();
    unsub();
    removeEventListener("pageshow", shown);
  };
});
