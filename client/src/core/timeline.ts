// Moving a server-rendered Timeline (components::timeline, `ol.track`) while a page polls: each
// step's `data-state` (done | current | todo), its `aria-current` and what it says to screen
// readers, the line's fill, and the times (`data-slot="t{i}"`, rendered with `slots`). Imported by
// the feature that polls (order tracking); nothing mounts on its own.

import { patch } from "./dom";
import { t } from "./page";

export type TimelineState = "done" | "current" | "todo";

export interface TimelineUpdate {
  /** How far the line has grown, 0..1. */
  progress?: number;
  /** One per step, in order (missing entries keep theirs). */
  states?: TimelineState[];
  /** One per step: a formatted time, or null to hide it. */
  times?: (string | null)[];
  /** A stopped run (a cancelled order): the line doesn't fill. */
  stopped?: boolean;
}

export function setTimeline(el: Element, u: TimelineUpdate): void {
  const ol = el.matches("ol.track") ? (el as HTMLElement) : el.querySelector<HTMLElement>("ol.track");
  if (!ol) return;
  if (u.progress !== undefined) ol.style.setProperty("--p", String(Math.min(1, Math.max(0, u.progress))));
  if (u.stopped !== undefined) ol.dataset.stopped = String(u.stopped);
  const steps = Array.from(ol.querySelectorAll<HTMLElement>(":scope > .track__step"));
  u.states?.forEach((state, i) => {
    const li = steps[i];
    if (!li || li.dataset.state === state) return;
    li.dataset.state = state;
    li.setAttribute("aria-current", state === "current" ? "step" : "false");
    const said = li.querySelector("[data-said]");
    if (said) said.textContent = `, ${t(`timeline.${state}`)}`;
  });
  if (u.times) patch(ol, Object.fromEntries(u.times.map((time, i) => [`t${i}`, time])));
}
