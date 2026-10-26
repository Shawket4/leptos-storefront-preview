// Pull to refresh, loaded on demand (touch devices, once the page is idle — features/core.ts).
// Touch only, from the very top of the page: a small disc follows the finger down (resisting more
// the further it goes), its arrow turns when letting go would refresh, it spins while the page is
// fetched again in place (`refresh()`), then springs back up. The browser's own pull-to-refresh is
// off (overscroll-behavior-y: contain), so there is only ever one.

import { $ } from "./dom";
import { refresh } from "./router";
import { SPRING, SpringValue, rubberband } from "./motion";

const PTR_TRIGGER = 64; // px of disc travel that refreshes
const PTR_REST = 52; // where the disc waits while the page reloads
function pullToRefresh(): void {
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
