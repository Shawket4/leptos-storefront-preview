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

// ── The address step: constraint messages, the city narrowing the areas, the notes counter ───
register("checkout-form", (form) => {
  if (!(form instanceof HTMLFormElement)) return;
  form.noValidate = true; // our messages, inline, in the page's language
  let tried = false;
  const controls = () => $$<Control>("input[name], select[name], textarea[name]", form).filter((c) => c.type !== "hidden");

  /** Show (or clear) the message of the field `name`; returns whether it's valid. */
  const check = (name: string): boolean => {
    const group = controls().filter((c) => c.name === name);
    const msg = group.map(message).find(Boolean) ?? "";
    for (const c of group) c.setAttribute("aria-invalid", String(!!msg));
    const box = form.querySelector<HTMLElement>(`#co-${CSS.escape(name)}-err`);
    if (box) {
      // A message the server sent stays until the field is edited.
      if (msg || box.dataset.client !== undefined || tried) {
        box.hidden = !msg;
        patch(box, { error: msg || null });
        box.toggleAttribute("data-client", !!msg);
      }
    }
    return !msg;
  };

  const area = $<HTMLSelectElement>("select[name=area]", form);
  const narrow = () => {
    const city = $<HTMLInputElement>("input[name=city]:checked", form)?.value;
    if (!area || !city) return;
    for (const og of $$<HTMLOptGroupElement>("optgroup[data-city]", area)) {
      const other = og.dataset.city !== city;
      og.hidden = other;
      og.disabled = other;
    }
    if (area.selectedOptions[0]?.parentElement instanceof HTMLOptGroupElement && area.selectedOptions[0].parentElement.disabled) area.value = "";
  };
  narrow();

  const notes = $<HTMLTextAreaElement>("textarea[name=notes]", form);
  const counter = $("[data-notes-count]", form);
  const count = () => {
    if (notes && counter) counter.textContent = `${digits(notes.value.length)}/${digits(notes.maxLength)}`;
  };

  const onSubmit = (ev: SubmitEvent) => {
    tried = true;
    const names = [...new Set(controls().map((c) => c.name))];
    const bad = names.filter((n) => !check(n));
    if (!bad.length) return; // valid: the router posts it
    ev.preventDefault();
    const first = controls().find((c) => c.name === bad[0]);
    first?.focus();
  };
  const onInput = (ev: Event) => {
    const c = ev.target as Control;
    if (!c.name) return;
    if (c.name === "notes") count();
    if (c.name === "city") narrow();
    if (tried || c.getAttribute("aria-invalid") === "true") check(c.name);
    if (c.name === "city" && area && (tried || area.getAttribute("aria-invalid") === "true")) check("area");
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
  const first = $<HTMLElement>('[aria-invalid="true"]', root) ?? $<HTMLElement>(".notice[data-autofocus]", root);
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
register("order-track", (el) => {
  const id = el.dataset.id ?? "";
  const parts = {
    still: $("[data-static]", el),
    live: $("[data-live]", el),
    done: $("[data-complete]", el),
    net: $("[data-net]", el),
  };
  const say = $("[data-announce]", el);
  const showOnly = (which: "live" | "done" | "net") => {
    if (parts.still) parts.still.hidden = true;
    for (const k of ["live", "done", "net"] as const) {
      const p = parts[k];
      if (p) p.hidden = k !== which;
    }
  };
  if (el.dataset.final === "true") {
    showOnly("done");
    return;
  }
  showOnly("live");

  let stopped = false;
  let timer = 0;
  let clock = 0;
  let failures = 0;
  let status = el.dataset.status ?? "";

  const apply = (tr: OrderTrack) => {
    const values: Record<string, string | null> = { headline: tr.headline, eta: tr.eta, updated: tr.updated };
    tr.steps.forEach((s, i) => (values[`t${i}`] = s.time));
    patch(el, values);
    const ol = $("ol.timeline", el);
    if (ol) {
      ol.style.setProperty("--p", tr.progress.toFixed(4));
      ol.dataset.cancelled = String(tr.cancelled);
      $$("li.timeline__step", ol).forEach((li, i) => {
        const state = tr.steps[i]?.state ?? "todo";
        li.className = `timeline__step is-${state}`;
        li.setAttribute("aria-current", state === "current" ? "step" : "false");
      });
    }
    el.dataset.status = tr.status;
    if (tr.status !== status && say) say.textContent = t("order.update", { status: tr.headline });
    status = tr.status;
  };

  const schedule = (ms: number) => {
    clearTimeout(timer);
    timer = window.setTimeout(() => void poll(), ms);
  };
  const pace = (nextIn: number | null) => Math.min(5000, Math.max(1000, (nextIn ?? 5000) + 400));
  const offline = () => {
    clearInterval(clock);
    patch(el, { net: t("order.offline") });
    showOnly("net");
  };
  const retryIn = (ms: number) => {
    const until = Date.now() + ms;
    const tick = () => patch(el, { net: t("order.retrying", { s: digits(Math.max(0, Math.ceil((until - Date.now()) / 1000))) }) });
    clearInterval(clock);
    tick();
    clock = window.setInterval(tick, 1000);
    showOnly("net");
    schedule(ms);
  };

  async function poll(): Promise<void> {
    if (stopped) return;
    if (document.hidden) return schedule(5000);
    if (!isOnline()) return offline();
    try {
      const tr = await getJSON<OrderTrack | null>(`${cfg().api.orderTrack}?id=${encodeURIComponent(id)}`, { retries: 0 });
      if (stopped) return;
      clearInterval(clock);
      failures = 0;
      if (!tr) return showOnly("live"); // gone (expired, restarted server): keep what's shown
      apply(tr);
      if (tr.done) return showOnly("done");
      showOnly("live");
      schedule(pace(tr.next_in_ms));
    } catch (e) {
      if (stopped) return;
      if (!isOnline() || (e instanceof NetError && e.code === "offline")) return offline();
      // 3 s, 6 s, 12 s, then every 20 s.
      failures++;
      retryIn(Math.min(20_000, 3000 * 2 ** Math.min(failures - 1, 3)));
    }
  }

  const now = () => {
    clearTimeout(timer);
    void poll();
  };
  const onClick = (ev: MouseEvent) => {
    if (ev.target instanceof Element && ev.target.closest("[data-retry]")) now();
  };
  const onVisible = () => {
    if (!document.hidden) now();
  };
  el.addEventListener("click", onClick);
  document.addEventListener("visibilitychange", onVisible);
  const offOnline = onOnline(now);
  const nextIn = Number(el.dataset.nextIn);
  schedule(pace(Number.isFinite(nextIn) ? nextIn : null));
  return () => {
    stopped = true;
    clearTimeout(timer);
    clearInterval(clock);
    offOnline();
    el.removeEventListener("click", onClick);
    document.removeEventListener("visibilitychange", onVisible);
  };
});
