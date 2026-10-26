// Core agent: page-wide behaviours — the one drawer and stepper, the navigation progress bar and
// failure alert, the offline banner, prefs links that keep the live address, `<details
// data-popover>` light dismiss, rail buttons, and pull-to-refresh. The singletons they drive are
// server-rendered in the shell (components::shell, components::offline) and looked up when needed,
// so a full-frame swap (a language change) never leaves a listener on a removed element.

import { register } from "../core/mount";
import { mountDrawer } from "../core/drawer";
import { mountStepper } from "../core/stepper";
import { mountSegToggle } from "../core/segmented";
import { $, $$, on, prefersReducedMotion } from "../core/dom";
import { t } from "../core/page";
import { isOnline, onOnline } from "../core/net";
import { navFailure, refresh, routing, type NavFailure } from "../core/router";
import { SPRING, SpringValue, rubberband } from "../core/motion";
import { toast } from "../stores/toasts";

register("drawer", mountDrawer);
register("stepper", mountStepper);
register("seg-toggle", mountSegToggle);

// ── Navigation progress ──────────────────────────────────────────────────────────────────────
// A thin bar at the top, only when a navigation takes longer than 150 ms; it completes when the
// new page is in (static under reduced motion, see shell.css).
let barTimer = 0;
let barShown = false;
function progress(on: boolean, ok = true): void {
  clearTimeout(barTimer);
  if (on) {
    barShown = false;
    barTimer = window.setTimeout(() => {
      const b = $(".nav-progress");
      if (!b) return;
      b.dataset.state = "loading";
      barShown = true;
    }, 150);
    return;
  }
  const b = $(".nav-progress");
  if (!b || !barShown) return;
  barShown = false;
  b.dataset.state = ok ? "done" : "idle";
  if (ok) window.setTimeout(() => b.dataset.state === "done" && (b.dataset.state = "idle"), 320);
}
routing.subscribe((r) => progress(r.pending, !navFailure.get()));

// ── A navigation that failed ─────────────────────────────────────────────────────────────────
// Offline: the offline banner (the page stays; the navigation is tried again when the connection
// is back). Otherwise the alert with Retry (a post is never re-sent: no Retry then).
let retryAfterOnline: (() => void) | null = null;

function showAlert(f: { why: Exclude<NavFailure["why"], "offline">; retry: (() => void) | null }): void {
  const el = $(".nav-alert");
  if (!el) return;
  el.dataset.open = "true";
  const retry = $<HTMLButtonElement>(".nav-alert__retry", el);
  if (retry) {
    retry.hidden = !f.retry;
    retry.onclick = () => {
      hideAlert();
      f.retry?.();
    };
  }
  const close = $<HTMLButtonElement>(".nav-alert__close", el);
  if (close) close.onclick = hideAlert;
  // Set the reason after showing it, so screen readers announce the alert with it.
  requestAnimationFrame(() => {
    const why = $(".nav-alert__why", el);
    if (why) why.textContent = t(`nav.why.${f.why}`);
  });
}
function hideAlert(): void {
  const el = $(".nav-alert");
  if (el) el.dataset.open = "false";
}

navFailure.subscribe((f) => {
  if (!f) {
    hideAlert();
    return;
  }
  if (f.why === "offline") {
    retryAfterOnline = f.retry;
    setOffline(true);
  } else showAlert({ why: f.why, retry: f.retry });
});

// ── Offline banner ───────────────────────────────────────────────────────────────────────────
let offline = false;
function setOffline(on: boolean, still = false): void {
  const el = $(".offline");
  if (!el) return;
  const was = offline;
  offline = on;
  el.dataset.open = String(on);
  // Children shown/hidden inside the live region are what screen readers announce.
  for (const c of $$("strong, .offline__hint, .offline__still", el)) {
    c.hidden = !on || (c.classList.contains("offline__hint") && still) || (c.classList.contains("offline__still") && !still);
  }
  if (was && !on) toast(t("net.back_online"), "success");
}
addEventListener("offline", () => setOffline(true));
onOnline(() => {
  if (!offline) return;
  setOffline(false);
  const again = retryAfterOnline;
  retryAfterOnline = null;
  again?.();
});
on(document, "click", ".offline__retry", () => {
  if (!isOnline()) {
    setOffline(true, true);
    return;
  }
  setOffline(false);
  const again = retryAfterOnline;
  retryAfterOnline = null;
  if (again) again();
  else void refresh();
});
if (!isOnline()) setOffline(true);

// ── Staging the real singletons for /ui (never touches the router's stores) ─────────────────
export type PreviewNet = "offline" | "online" | { fail: "offline" | "timeout" | "server" } | { progress: number };
export function previewNet(state: PreviewNet): void {
  if (state === "offline") setOffline(true);
  else if (state === "online") setOffline(false);
  else if ("fail" in state) {
    if (state.fail === "offline") setOffline(true);
    else showAlert({ why: state.fail, retry: () => {} });
  } else {
    progress(true);
    window.setTimeout(() => progress(false), Math.max(200, state.progress));
  }
}
// The /ui page's buttons (`data-preview="offline|online|alert:<why>|progress:<ms>"`).
on(document, "click", "button[data-preview]", (_ev, b) => {
  const [what, arg] = (b.dataset.preview ?? "").split(":");
  if (what === "offline" || what === "online") previewNet(what);
  else if (what === "alert") previewNet({ fail: arg === "offline" || arg === "timeout" ? arg : "server" });
  else if (what === "progress") previewNet({ progress: Number(arg) || 1500 });
});

// ── Preference and shop links come back to where you are ─────────────────────────────────────
// Their `next` is the page as the server rendered it, and the address may have moved since (the
// listing's filters replace it). At press time, point `next` at the live address, so switching the
// language keeps the filters; the server's href stays the no-JS link. A shop link that sends
// elsewhere on purpose (the other shop's home, from a deep page) keeps its `next`.
function freshNext(ev: Event): void {
  const a = (ev.target as Element | null)?.closest?.<HTMLAnchorElement>('a[href^="/__prefs"], a[href^="/__tenant/"]');
  if (!a) return;
  const u = new URL(a.href, location.href);
  const next = u.searchParams.get("next");
  if (next === null) return;
  const n = new URL(next, location.href);
  if (u.pathname.startsWith("/__tenant/") && n.pathname !== location.pathname) return;
  u.searchParams.set("next", location.pathname + location.search + n.hash);
  a.setAttribute("href", u.pathname + u.search);
}
document.addEventListener("pointerdown", freshNext, true);
document.addEventListener("click", freshNext, true);

// ── Popovers: <details data-popover> ─────────────────────────────────────────────────────────
// Light dismiss (outside press, Escape, a link inside), one open at a time, and an exit that runs
// the entry backwards (CSS [data-closing]) before the element really closes.
const openPopovers = () => $$<HTMLDetailsElement>("details[data-popover][open]");
function closePopover(det: HTMLDetailsElement, focus = false): void {
  if (!det.open || det.hasAttribute("data-closing")) return;
  const done = () => {
    det.removeAttribute("data-closing");
    det.open = false;
    if (focus) $("summary", det)?.focus();
  };
  if (prefersReducedMotion()) return done();
  det.setAttribute("data-closing", "");
  window.setTimeout(done, 140);
}
document.addEventListener(
  "pointerdown",
  (e) => {
    for (const det of openPopovers()) if (!det.contains(e.target as Node)) closePopover(det);
  },
  true,
);
document.addEventListener("keydown", (e) => {
  if (e.key === "Escape") for (const det of openPopovers()) closePopover(det, true);
});
document.addEventListener("click", (e) => {
  const t = e.target as Element;
  const summary = t.closest?.("details[data-popover] > summary");
  const det = summary?.parentElement;
  if (det instanceof HTMLDetailsElement && det.open) {
    e.preventDefault();
    closePopover(det);
    return;
  }
  const link = t.closest?.("details[data-popover] a[href]")?.closest("details");
  if (link instanceof HTMLDetailsElement) closePopover(link);
});
document.addEventListener(
  "toggle",
  (e) => {
    const t = e.target;
    if (t instanceof HTMLDetailsElement && t.matches("details[data-popover]") && t.open)
      for (const det of openPopovers()) if (det !== t) closePopover(det);
  },
  true,
);

// ── Rails (components::layout::Rail): prev/next buttons and their end states ────────────────
function railState(r: HTMLElement | null): void {
  const track = r && $(".rail__track", r);
  if (!r || !track) return;
  const max = track.scrollWidth - track.clientWidth, x = Math.abs(track.scrollLeft);
  r.dataset.at = max <= 2 ? "none" : x <= 2 ? "start" : x >= max - 2 ? "end" : "mid";
}
register("rail", (r) => {
  const track = $(".rail__track", r);
  if (!track) return;
  let frame = 0;
  const onScroll = () => {
    if (!frame) frame = requestAnimationFrame(() => ((frame = 0), railState(r)));
  };
  const offStep = on(r, "click", "[data-rail-step]", (_e, b) => {
    const dir = getComputedStyle(track).direction === "rtl" ? -1 : 1;
    track.scrollBy({ left: Number(b.dataset.railStep) * track.clientWidth * 0.85 * dir, behavior: prefersReducedMotion() ? "auto" : "smooth" });
  });
  const ro = new ResizeObserver(() => railState(r));
  ro.observe(track);
  track.addEventListener("scroll", onScroll, { passive: true });
  railState(r);
  return () => {
    offStep();
    ro.disconnect();
    cancelAnimationFrame(frame);
    track.removeEventListener("scroll", onScroll);
  };
});

// ── Category strip (components::header::CatNav): the current chip in view ────────────────────
// Moves the strip sideways only (never the page), at once: on a deep link, and after back/forward.
register("cat-nav", (nav) => {
  const list = $(".cat-nav__list", nav), cur = $('[aria-current="page"]', nav);
  if (!list || !cur || list.scrollWidth <= list.clientWidth) return;
  const pad = 16, l = list.getBoundingClientRect(), c = cur.getBoundingClientRect();
  const delta = c.left < l.left + pad ? c.left - l.left - pad : c.right > l.right - pad ? c.right - l.right + pad : 0;
  if (delta) list.scrollBy({ left: delta, behavior: "instant" });
});

// ── Pull to refresh ──────────────────────────────────────────────────────────────────────────
// Touch only, from the very top of the page: a small disc follows the finger down (resisting more
// the further it goes), its arrow turns when letting go would refresh, it spins while the page is
// fetched again in place (`refresh()`), then springs back up. The browser's own pull-to-refresh is
// off (overscroll-behavior-y: contain), so there is only ever one.
const PTR_TRIGGER = 64; // px of disc travel that refreshes
const PTR_REST = 52; // where the disc waits while the page reloads
function pullToRefresh(): void {
  if (!matchMedia("(pointer: coarse)").matches) return;
  let start: { x: number; y: number } | null = null;
  let pulling = false;
  let busy = false;
  const el = () => $(".ptr");
  const disc = new SpringValue(0, (y) => {
    const r = el();
    if (!r) return;
    const p = Math.min(1, y / PTR_TRIGGER);
    r.style.transform = `translate3d(0,${y.toFixed(1)}px,0)`;
    r.style.opacity = String(Math.min(1, y / 24));
    r.style.setProperty("--ptr-turn", `${(p * 180).toFixed(0)}deg`);
    r.toggleAttribute("data-armed", y >= PTR_TRIGGER);
  });
  const blocked = (t: EventTarget | null) =>
    t instanceof Element && !!t.closest("dialog[open], .rail__track, input, textarea, select, [data-ptr=off]");
  addEventListener(
    "touchstart",
    (e) => {
      if (busy || e.touches.length !== 1 || scrollY > 0 || blocked(e.target)) return;
      start = { x: e.touches[0]!.clientX, y: e.touches[0]!.clientY };
      pulling = false;
    },
    { passive: true },
  );
  addEventListener(
    "touchmove",
    (e) => {
      if (!start || busy) return;
      const dy = e.touches[0]!.clientY - start.y, dx = e.touches[0]!.clientX - start.x;
      if (!pulling) {
        if (Math.abs(dy) < 8 && Math.abs(dx) < 8) return;
        if (dy <= 0 || Math.abs(dx) > Math.abs(dy) || scrollY > 0) {
          start = null; // a scroll or a sideways swipe: not ours
          return;
        }
        pulling = true;
        el()?.setAttribute("data-state", "pull");
      }
      disc.jump(Math.max(0, rubberband(dy, innerHeight * 0.6, 0.55)));
    },
    { passive: true },
  );
  const release = () => {
    if (!start) return;
    start = null;
    if (!pulling) return;
    pulling = false;
    const r = el();
    if (disc.value < PTR_TRIGGER) {
      disc.to(0, { spring: SPRING.snappy, onRest: () => r?.removeAttribute("data-state") });
      return;
    }
    busy = true;
    r?.setAttribute("data-state", "refreshing");
    disc.to(PTR_REST, { spring: SPRING.default });
    const t0 = performance.now();
    void refresh().finally(() => {
      // Let the spinner be seen (a cached-fast reload would only flash it).
      window.setTimeout(() => {
        busy = false;
        el()?.setAttribute("data-state", "done");
        disc.to(0, { spring: SPRING.snappy, onRest: () => el()?.removeAttribute("data-state") });
      }, Math.max(0, 450 - (performance.now() - t0)));
    });
  };
  addEventListener("touchend", release, { passive: true });
  addEventListener("touchcancel", release, { passive: true });
}
pullToRefresh();
