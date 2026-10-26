// Rails (components::layout::Rail), loaded on demand (`data-ts="rail"`): prev/next buttons and
// their end states.

import { $, on, prefersReducedMotion } from "./dom";
import { register } from "./mount";

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
