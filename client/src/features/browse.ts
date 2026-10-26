// Browse: the listing (category and /search results), the header's instant search and the home
// page's deals countdown. The server renders all of it with its data; this file changes state
// locally and asks the server only for data it doesn't have (new results are a navigation the
// server renders; suggestions come from GET /api/search/suggest).

import { register } from "../core/mount";
import { $, $$, fill, on, patch } from "../core/dom";
import { cfg, digits, t } from "../core/page";
import { navigate, routing } from "../core/router";
import { cached, key, peek } from "../core/cache";
import { NetError, getJSON, isOnline, onOnline } from "../core/net";
import type { Suggest } from "../api/types";

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
    on(root, "click", "a.lx-sort-opt[data-sort]", (ev, a) => {
      if (ev.button !== 0 || ev.metaKey || ev.ctrlKey || ev.shiftKey || ev.altKey) return;
      ev.preventDefault();
      const code = a.dataset.sort ?? "relevance";
      for (const opt of $$("a.lx-sort-opt", root)) opt.setAttribute("aria-current", String(opt === a));
      const v = $(".lx-sort-v", root);
      if (v) v.textContent = a.textContent?.trim() ?? "";
      if (sortMenu) sortMenu.open = false;
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
    // The sort menu: Escape and a press elsewhere close it.
    on(root, "keydown", "details.lx-sort", (ev) => {
      if (ev.key !== "Escape" || !sortMenu?.open) return;
      sortMenu.open = false;
      sortMenu.querySelector("summary")?.focus();
    }),
  ];
  const outside = (ev: PointerEvent) => {
    if (sortMenu?.open && !sortMenu.contains(ev.target as Node)) sortMenu.open = false;
  };
  document.addEventListener("pointerdown", outside, true);
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
    document.removeEventListener("pointerdown", outside, true);
    removeEventListener("pageshow", shown);
  };
});

// ── Instant search ──────────────────────────────────────────────────────────────────────────────
// An ARIA combobox over the server-rendered form. Typing asks the server after a short pause (the
// data layer folds Arabic spellings, knows synonyms and marks the matched runs); answers are cached
// for a minute per question, and an answer to an older question is dropped. The rows come from
// `<template id="tpl-suggest-*">`; the matched runs become `<span data-hit>` — text only, never HTML.

const SUGGEST_DEBOUNCE_MS = 150;
const SUGGEST_TTL_MS = 60_000;

/** Worth asking about: two letters or digits, in any script. */
const askable = (q: string) => (q.match(/[\p{L}\p{N}]/gu)?.length ?? 0) >= 2;

const suggestUrl = (q: string) => `${cfg().api.searchSuggest}?q=${encodeURIComponent(q)}`;

register("search-box", (root) => {
  const form = $<HTMLFormElement>("form.hs-form", root);
  const input = $<HTMLInputElement>("input.hs-input", root);
  const panel = $(".hs-panel", root);
  const list = $("[role=listbox]", root);
  if (!form || !input || !panel || !list) return;
  const groups = {
    cats: $("[data-group=cats]", list),
    hits: $("[data-group=hits]", list),
    all: $("[data-group=all]", list),
  };
  const state = (name: string) => $(`[data-state=${name}]`, panel);
  const status = $("[data-status]", root);
  const clearBtn = $<HTMLButtonElement>(".hs-clear", root);
  const trigger = $<HTMLAnchorElement>(".hs-trigger", root);
  const triggerText = $(".hs-trigger__text", root);
  const idBase = input.id.replace(/-input$/, "");

  let timer = 0;
  let seq = 0;
  let active = -1;
  let shown: Suggest | null = null;
  let failed: "offline" | "error" | null = null;
  let overlay = false;
  let open = false;

  const options = () => $$<HTMLElement>("[role=option]", list);
  const text = () => input.value.trim();

  const setOpen = (v: boolean) => {
    open = v && askable(text());
    panel.hidden = !open;
    input.setAttribute("aria-expanded", String(open));
    if (!open) setActive(-1);
  };
  const setActive = (i: number) => {
    const opts = options();
    active = i >= 0 && i < opts.length ? i : -1;
    opts.forEach((o, k) => o.setAttribute("aria-selected", String(k === active)));
    const el = opts[active];
    if (el) {
      input.setAttribute("aria-activedescendant", el.id);
      el.scrollIntoView({ block: "nearest" });
    } else input.removeAttribute("aria-activedescendant");
  };
  const loading = (on: boolean) => root.classList.toggle("is-loading", on);
  const view = (which: "list" | "loading" | "empty" | "offline" | "error") => {
    list.hidden = which !== "list";
    for (const s of ["loading", "empty", "offline", "error"]) {
      const el = state(s);
      if (el) el.hidden = s !== which;
    }
  };
  const say = (msg: string) => {
    if (status && status.textContent !== msg) status.textContent = msg;
  };

  const render = (s: Suggest) => {
    shown = s;
    failed = null;
    let n = 0;
    const id = () => `${idBase}-o-${n++}`;
    const cats = s.categories.map((c) => {
      const row = fill("suggest-cat", { href: c.href, icon: c.icon, quoted: c.quoted, name: c.name });
      row.firstElementChild!.id = id();
      return row;
    });
    const hits = s.hits.map((h) => {
      const row = fill("suggest-row", {
        href: h.href,
        img: h.img,
        srcset: h.srcset,
        category: h.category,
        price: h.price,
        was: h.was,
      });
      const a = row.firstElementChild as HTMLElement;
      a.id = id();
      const name = $("[data-name]", a);
      for (const seg of h.name) {
        if (seg.hit) {
          const mark = document.createElement("span");
          mark.dataset.hit = "";
          mark.textContent = seg.text;
          name?.append(mark);
        } else name?.append(seg.text);
      }
      if (!h.img) $("img", a)?.remove();
      return row;
    });
    groups.cats?.replaceChildren(...cats);
    groups.hits?.replaceChildren(...hits);
    if (groups.cats) groups.cats.hidden = !cats.length;
    if (groups.hits) groups.hits.hidden = !hits.length;
    if (s.all) {
      const row = fill("suggest-all", { href: s.all.href, label: s.all.label });
      row.firstElementChild!.id = id();
      groups.all?.replaceChildren(row);
    } else groups.all?.replaceChildren();
    if (!cats.length && !hits.length) {
      const empty = state("empty");
      if (empty) patch(empty, { title: s.status });
      view("empty");
    } else view("list");
    setActive(-1);
    say(s.status);
  };

  const fail = (e: unknown) => {
    shown = null;
    failed = e instanceof NetError && e.code === "offline" ? "offline" : !isOnline() ? "offline" : "error";
    view(failed);
    say(failed === "offline" ? t("net.offline") : t("net.failed"));
  };

  const ask = (q: string) => {
    const me = ++seq;
    loading(true);
    if (!shown) view("loading");
    say(t("browse.searching"));
    cached(key("suggest", q), { ttl: SUGGEST_TTL_MS }, () => getJSON<Suggest>(suggestUrl(q)))
      .then((s) => {
        if (me !== seq) return; // a newer question is under way
        loading(false);
        render(s);
        setOpen(document.activeElement === input || overlay);
      })
      .catch((e: unknown) => {
        if (me !== seq) return;
        loading(false);
        fail(e);
      });
  };

  const changed = () => {
    const q = text();
    clearTimeout(timer);
    if (clearBtn) clearBtn.hidden = !input.value;
    if (triggerText) {
      triggerText.textContent = input.value || input.placeholder;
      triggerText.classList.toggle("is-filled", !!input.value);
    }
    if (!askable(q)) {
      seq++;
      loading(false);
      shown = null;
      setOpen(false);
      return;
    }
    // Asked before (this minute): answer at once.
    const known = peek<Suggest>(key("suggest", q));
    if (known && shown?.q !== q) {
      seq++;
      loading(false);
      render(known);
      setOpen(true);
      return;
    }
    if (shown?.q === q && !failed) {
      seq++; // back to the question already on screen
      loading(false);
      setOpen(true);
      return;
    }
    setOpen(true);
    if (!shown) view("loading");
    timer = window.setTimeout(() => ask(q), SUGGEST_DEBOUNCE_MS);
  };

  // ── The phone overlay: the same form, full screen, in the top layer ──
  const openOverlay = () => {
    overlay = true;
    form.classList.add("is-overlay");
    form.setAttribute("popover", "manual");
    form.showPopover();
    document.documentElement.classList.add("hs-lock");
    // Focus inside the tap: iOS raises the keyboard only for a focus made while handling it.
    input.focus();
    if (shown) setOpen(true);
  };
  const closeOverlay = (restore: boolean) => {
    if (!overlay) return;
    overlay = false;
    if (form.matches(":popover-open")) form.hidePopover();
    form.removeAttribute("popover");
    form.classList.remove("is-overlay");
    document.documentElement.classList.remove("hs-lock");
    setOpen(false);
    if (restore) trigger?.focus();
  };

  const onKey = (ev: KeyboardEvent) => {
    const n = options().length;
    switch (ev.key) {
      case "ArrowDown":
      case "ArrowUp": {
        ev.preventDefault();
        if (!open) setOpen(true);
        if (!n) return;
        const down = ev.key === "ArrowDown";
        setActive(active < 0 ? (down ? 0 : n - 1) : (active + (down ? 1 : n - 1)) % n);
        return;
      }
      case "Enter": {
        const opt = open ? options()[active] : undefined;
        if (opt) {
          ev.preventDefault();
          opt.click(); // a link: the router (or the browser) takes it
        }
        return; // otherwise the form submits: /search?q=
      }
      case "Escape":
        if (overlay) {
          ev.preventDefault();
          closeOverlay(true);
        } else if (open) {
          ev.preventDefault();
          setOpen(false);
        } else if (input.value) {
          ev.preventDefault();
          input.value = "";
          changed();
        }
        return;
      case "Tab":
        setOpen(false);
    }
  };

  const offs = [
    on(input, "input", "*", changed),
    on(input, "keydown", "*", onKey),
    on(input, "focus", "*", () => {
      if (shown && shown.q === text()) setOpen(true);
    }),
    on(root, "focusout", "*", (ev) => {
      if (overlay) return;
      const next = (ev as FocusEvent).relatedTarget;
      if (!(next instanceof Node) || !root.contains(next)) setOpen(false);
    }),
    // Keep focus in the field while an option is pressed.
    on(panel, "mousedown", "*", (ev) => ev.preventDefault()),
    on(panel, "mousemove", "[role=option]", (_ev, el) => {
      const i = options().indexOf(el);
      if (i !== active) setActive(i);
    }),
    on(panel, "click", "[role=option]", () => {
      setOpen(false);
      closeOverlay(false);
    }),
    on(panel, "click", "[data-retry]", () => {
      const q = text();
      if (askable(q)) ask(q);
    }),
    on(root, "click", ".hs-clear", () => {
      input.value = "";
      changed();
      input.focus();
    }),
    on(root, "click", ".hs-back", () => closeOverlay(true)),
    on(root, "click", ".hs-trigger", (ev) => {
      const e = ev as MouseEvent;
      if (e.button !== 0 || e.metaKey || e.ctrlKey || e.shiftKey || e.altKey) return;
      ev.preventDefault();
      openOverlay();
    }),
    on(form, "submit", "*", (ev) => {
      clearTimeout(timer);
      if (!text()) {
        ev.preventDefault();
        input.focus();
        return;
      }
      setOpen(false);
      closeOverlay(false);
      input.blur(); // drops the phone keyboard; the submission itself is the router's
    }),
    // Back online after an offline failure: ask again by itself.
    onOnline(() => {
      const q = text();
      if (failed === "offline" && askable(q)) ask(q);
    }),
  ];

  return () => {
    clearTimeout(timer);
    seq++;
    closeOverlay(false);
    for (const off of offs) off();
  };
});

// ── Deals countdown ─────────────────────────────────────────────────────────────────────────────
// The server renders the time left; this only ticks it (a clock, so client formatting is fine).

const two = (n: number) => String(n).padStart(2, "0");

register("countdown", (el) => {
  const ends = Number(el.dataset.ends);
  const clock = $("[data-clock]", el);
  if (!clock || !Number.isFinite(ends)) return;
  let timer = 0;
  const tick = () => {
    const s = Math.max(0, Math.floor((ends - Date.now()) / 1000));
    const text = digits(`${two(Math.floor(s / 3600))}:${two(Math.floor(s / 60) % 60)}:${two(s % 60)}`);
    if (clock.textContent !== text) clock.textContent = text;
    // On the next whole second (setInterval drifts and would skip a second now and then).
    if (s > 0) timer = window.setTimeout(tick, 1000 - (Date.now() % 1000) + 5);
  };
  tick();
  return () => clearTimeout(timer);
});
