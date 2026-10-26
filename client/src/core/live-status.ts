// Driving a server-rendered LiveStatus (components::live_status): its state (live | retrying |
// offline | idle) and line, and the "Retrying in N s" countdown whose Retry button runs the retry
// at once. The visible line (`data-slot="msg"`) ticks; the polite line screen readers hear
// (`data-slot="said"`) changes only with the state. Imported by the feature that polls.

import { patch } from "./dom";
import { digits, t } from "./page";

export type LiveState = "live" | "retrying" | "offline" | "idle";

const countdowns = new WeakMap<Element, { timer: number; retry: () => void }>();

function root(el: Element): HTMLElement | null {
  return el.closest<HTMLElement>(".live-status") ?? el.querySelector<HTMLElement>(".live-status");
}

function stop(box: HTMLElement): void {
  const c = countdowns.get(box);
  if (c) clearInterval(c.timer);
  countdowns.delete(box);
}

/** Show a state; `message` overrides its own line (e.g. "This order is complete." when idle). */
export function setLiveStatus(el: Element, state: LiveState, message?: string): void {
  const box = root(el);
  if (!box) return;
  stop(box);
  const line =
    message ?? (state === "live" ? t("live.on") : state === "offline" ? t("live.offline") : state === "retrying" ? t("live.failed") : "");
  const changed = box.dataset.state !== state || message !== undefined;
  box.dataset.state = state;
  patch(box, changed ? { msg: line, said: line } : { msg: line });
}

/**
 * The last update failed: count down to the next try (`retry` runs when it reaches zero, or at
 * once from the Retry button). Returns a cancel function; any `setLiveStatus` cancels it too.
 */
export function retryIn(el: Element, ms: number, retry: () => void): () => void {
  const box = root(el);
  if (!box) return () => {};
  stop(box);
  box.dataset.state = "retrying";
  const until = Date.now() + ms;
  const tick = () => {
    const left = Math.max(0, Math.ceil((until - Date.now()) / 1000));
    patch(box, { msg: t("live.retrying", { s: digits(left) }) });
    if (left === 0) {
      stop(box);
      retry();
    }
  };
  patch(box, { said: t("live.retrying_said") });
  const entry = { timer: window.setInterval(tick, 1000), retry };
  countdowns.set(box, entry);
  tick();
  // The button: one listener per box, calling whatever retry is current.
  if (!box.dataset.retryWired) {
    box.dataset.retryWired = "";
    box.addEventListener("click", (ev) => {
      if (!(ev.target as Element).closest("[data-live-retry]")) return;
      const c = countdowns.get(box);
      if (!c) return;
      stop(box);
      c.retry();
    });
  }
  return () => stop(box);
}
