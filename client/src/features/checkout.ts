// Checkout and order tracking in the browser. The checkout steps are server-rendered forms that
// the client router posts (their server functions validate, store the draft and redirect); this
// adds what only the browser can: the address step's HTML5 constraint messages in the page's
// language before the round-trip, focus on the first error the server sent back, a pending state
// and a double-submit guard. Placing the order clears the `cart` cookie on the server; main.ts
// re-reads it after the navigation, so the badge empties by itself.
//
// Order tracking polls `GET {api.orderTrack}?id=` (paced by when the next step is due, backing off
// after failures, paused while the tab is hidden) and fills the server's slots; it stops at a
// final status or when the page goes.

import type { OrderTrack } from "../api/types";
import { $, $$, patch } from "../core/dom";
import { register } from "../core/mount";
import { NetError, getJSON, isOnline, onOnline } from "../core/net";
import { cfg, digits, t } from "../core/page";
import { routing } from "../core/router";
import { setLiveStatus, retryIn } from "../core/live-status";
import { setTimeline } from "../core/timeline";
import { adoptSnapshots, type PricedSnapshot } from "../stores/cart";

type Control = HTMLInputElement | HTMLSelectElement | HTMLTextAreaElement;

/** The message for a control that fails its HTML5 constraints (the server checks again). */
function message(el: Control): string {
  const v = el.validity;
  if (v.valid) return "";
  if (v.valueMissing) return el instanceof HTMLSelectElement || el.type === "radio" ? t("checkout.v.choose") : t("checkout.v.required");
  if (v.tooShort && !(el instanceof HTMLSelectElement)) return t("checkout.v.too_short", { n: digits(el.minLength) });
  if (v.tooLong && !(el instanceof HTMLSelectElement)) return t("checkout.v.too_long", { n: digits(el.maxLength) });
  return el.validationMessage;
}

// ── The address step: what's missing, said inline before the round-trip ──────────────────────
// Each field's error line (components::form renders `{id}-error` always, empty) gets the
// catalogue's words for its HTML5 constraint (`required`, `minlength`, `maxlength`); the first
// field with a problem takes focus. The server's own messages arrive in the same lines.
register("checkout-form", (form) => {
  if (!(form instanceof HTMLFormElement)) return;
  form.noValidate = true; // our words, inline
  let tried = false;
  const controls = () => $$<Control>("input[name], select[name], textarea[name]", form).filter((c) => c.type !== "hidden");

  /** The field's problem (empty when fine). Radios and selects: one of them must be chosen. */
  const problem = (group: Control[]): string => {
    const first = group[0]!;
    if (first instanceof HTMLInputElement && first.type === "radio")
      return first.required && !group.some((c) => (c as HTMLInputElement).checked) ? t("checkout.v.choose") : "";
    if (first instanceof HTMLSelectElement) return first.required && !first.value ? t("checkout.v.choose") : "";
    return message(first);
  };

  /** Show (or clear) the field's message in its error line; returns its first control if wrong. */
  const check = (name: string): Control | null => {
    const group = controls().filter((c) => c.name === name);
    if (!group.length) return null;
    const msg = problem(group);
    for (const c of group) c.setAttribute("aria-invalid", String(!!msg));
    const line = form.querySelector(`#co-${CSS.escape(name)}-error`);
    if (line) patch(line, { error: msg }); // "" clears it (and what a screen reader would read)
    return msg ? group[0]! : null;
  };

  const notes = $<HTMLTextAreaElement>("textarea[name=notes]", form);
  const counter = $("[data-notes-count]", form);
  const count = () => {
    if (notes && counter) counter.textContent = `${digits(notes.value.length)}/${digits(Number(counter.dataset.max) || 200)}`;
  };

  const onSubmit = (ev: SubmitEvent) => {
    tried = true;
    const bad = [...new Set(controls().map((c) => c.name))].map(check).filter((c): c is Control => !!c);
    if (!bad.length) return; // complete: the router posts it
    ev.preventDefault();
    bad[0]!.focus();
  };
  const onInput = (ev: Event) => {
    const c = ev.target as Control;
    if (!c.name) return;
    if (c.name === "notes") count();
    // Once a field has said something (or a submit was tried), it says the moment it's fixed.
    if (tried || c.getAttribute("aria-invalid") === "true") check(c.name);
  };
  const onBlur = (ev: FocusEvent) => {
    const c = ev.target as Control;
    if (c.name && "value" in c && c.value.trim()) check(c.name);
  };
  form.addEventListener("submit", onSubmit);
  form.addEventListener("input", onInput);
  form.addEventListener("change", onInput);
  form.addEventListener("focusout", onBlur);
  return () => {
    form.removeEventListener("submit", onSubmit);
    form.removeEventListener("input", onInput);
    form.removeEventListener("change", onInput);
    form.removeEventListener("focusout", onBlur);
  };
});

// ── Every step: focus the server's first error, pending state, one submit at a time ──────────
register("checkout", (root) => {
  // The server re-priced the cart from the cookie: if a price moved since the cart showed it (the
  // cart is computed in the browser from its snapshots), say so — and keep the new prices.
  try {
    const server = JSON.parse($("script[data-cart-snapshots]", root)?.textContent ?? "{}") as Record<string, PricedSnapshot>;
    if (adoptSnapshots(server)) $("[data-price-changed]", root)?.removeAttribute("hidden");
  } catch {
    /* nothing to compare */
  }

  // What the server sent back: the first field it flagged, else the step's message.
  const first =
    $<HTMLElement>(':is(input, select, textarea)[aria-invalid="true"], fieldset[aria-invalid="true"] input', root) ??
    $<HTMLElement>(".notice[data-autofocus]", root);
  first?.focus();

  let busy: HTMLFormElement | null = null;
  const release = () => {
    if (!busy) return;
    busy.removeAttribute("aria-busy");
    for (const b of $$("[type=submit]", busy)) b.removeAttribute("aria-busy");
    busy = null;
  };
  const onSubmit = (ev: SubmitEvent) => {
    const form = ev.target;
    if (!(form instanceof HTMLFormElement) || ev.defaultPrevented) return;
    if (busy) {
      ev.preventDefault(); // a second tap while the first is on its way
      return;
    }
    if (!isOnline()) {
      // Nothing would arrive: keep the choices and say so, with the form's own retry.
      ev.preventDefault();
      const note = $<HTMLElement>("[data-net-notice]", form);
      if (note) {
        note.hidden = false;
        note.focus();
      }
      return;
    }
    $<HTMLElement>("[data-net-notice]", form)?.setAttribute("hidden", "");
    busy = form;
    form.setAttribute("aria-busy", "true");
    (ev.submitter ?? $("[type=submit]", form))?.setAttribute("aria-busy", "true");
  };
  // The navigation is over (the next page is in, or it failed and the router says so).
  let started = false;
  const unsub = routing.subscribe(({ pending }) => {
    if (pending) started = true;
    else if (started) {
      started = false;
      release();
    }
  });
  const onShow = () => release();
  root.addEventListener("submit", onSubmit);
  addEventListener("pageshow", onShow);
  return () => {
    unsub();
    root.removeEventListener("submit", onSubmit);
    removeEventListener("pageshow", onShow);
  };
});

// ── Order tracking ────────────────────────────────────────────────────────────────────────────
// The server's Timeline and LiveStatus, moved with core's helpers (timeline.ts, live-status.ts).
register("order-track", (el) => {
  const id = el.dataset.id ?? "";
  const live = $(".live-status", el);
  const say = $("[data-announce]", el);
  $("[data-static]", el)?.setAttribute("hidden", "");
  live?.removeAttribute("hidden");
  if (el.dataset.final === "true") {
    setLiveStatus(el, "idle", t("order.complete"));
    return;
  }
  setLiveStatus(el, "live");

  let stopped = false;
  let timer = 0;
  let failures = 0;
  let cancelRetry: (() => void) | null = null;
  let status = el.dataset.status ?? "";

  const apply = (tr: OrderTrack) => {
    patch(el, { headline: tr.headline, eta: tr.eta, updated: tr.updated });
    setTimeline(el, {
      progress: tr.progress,
      stopped: tr.cancelled,
      states: tr.steps.map((s) => s.state),
      times: tr.steps.map((s) => s.time),
    });
    el.dataset.status = tr.status;
    if (tr.status !== status && say) say.textContent = t("order.update", { status: tr.headline });
    status = tr.status;
  };

  const schedule = (ms: number) => {
    clearTimeout(timer);
    timer = window.setTimeout(() => void poll(), ms);
  };
  const pace = (nextIn: number | null) => Math.min(5000, Math.max(1000, (nextIn ?? 5000) + 400));

  async function poll(): Promise<void> {
    if (stopped) return;
    cancelRetry?.();
    cancelRetry = null;
    if (document.hidden) return schedule(5000);
    if (!isOnline()) return setLiveStatus(el, "offline");
    try {
      const tr = await getJSON<OrderTrack | null>(`${cfg().api.orderTrack}?id=${encodeURIComponent(id)}`, { retries: 0 });
      if (stopped) return;
      failures = 0;
      setLiveStatus(el, "live");
      if (!tr) return; // gone (expired, restarted server): keep what's shown
      apply(tr);
      if (tr.done) return setLiveStatus(el, "idle", t("order.complete"));
      schedule(pace(tr.next_in_ms));
    } catch (e) {
      if (stopped) return;
      if (!isOnline() || (e instanceof NetError && e.code === "offline")) return setLiveStatus(el, "offline");
      // 3 s, 6 s, 12 s, then every 20 s; Retry (in the status) runs it at once.
      failures++;
      cancelRetry = retryIn(el, Math.min(20_000, 3000 * 2 ** Math.min(failures - 1, 3)), () => void poll());
    }
  }

  const now = () => {
    clearTimeout(timer);
    void poll();
  };
  const onVisible = () => {
    if (!document.hidden) now();
  };
  document.addEventListener("visibilitychange", onVisible);
  const offOnline = onOnline(now);
  const nextIn = Number(el.dataset.nextIn);
  schedule(pace(Number.isFinite(nextIn) ? nextIn : null));
  return () => {
    stopped = true;
    clearTimeout(timer);
    cancelRetry?.();
    offOnline();
    document.removeEventListener("visibilitychange", onVisible);
  };
});
