// The instant search (`data-ts="search-box"`: the header's box, on every page, and the one on
// /search). Loaded on the box's first use, not with the page (main.ts): until then the box is a
// plain GET form to /search. The server renders the box and the row templates; this file asks
// GET /api/search/suggest when the shopper pauses and fills the rows with the answer.

import { register } from "../core/mount";
import { $, $$, fill, on, patch } from "../core/dom";
import { cfg, t } from "../core/page";
import { cached, key, peek } from "../core/cache";
import { NetError, getJSON, isOnline, onOnline } from "../core/net";
import type { Suggest } from "../api/types";

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
      const row = fill("suggest-cat", { href: c.href, quoted: c.quoted, name: c.name });
      row.firstElementChild!.id = id();
      // The aisle's own lucide icon: one path, its geometry from the answer (an attribute, not HTML).
      row.querySelector(".hs-opt__ic path")?.setAttribute("d", c.icon);
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

  /** The field changed (`now`: ask without the typing pause — the shopper already paused). */
  const changed = (now = false) => {
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
    if (now) ask(q);
    else timer = window.setTimeout(() => ask(q), SUGGEST_DEBOUNCE_MS);
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
    on(input, "input", "*", () => changed()),
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

  // This module loads on the box's first use (main.ts), so the shopper may already be typing:
  // adopt the field as it is (value, caret and focus are the DOM's own, untouched) and answer now.
  if (document.activeElement === input && input.value) changed(true);

  return () => {
    clearTimeout(timer);
    seq++;
    closeOverlay(false);
    for (const off of offs) off();
  };
});
