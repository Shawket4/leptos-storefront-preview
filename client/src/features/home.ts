// The home page's deals countdown (`data-ts="countdown"`).

import { register } from "../core/mount";
import { $ } from "../core/dom";
import { digits } from "../core/page";

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
