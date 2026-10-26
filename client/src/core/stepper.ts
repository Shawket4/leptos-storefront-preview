// The one quantity stepper (markup: components::stepper::Stepper → `data-ts="stepper"` with
// `data-min`/`data-max` around − / `<input type=number>` / +). The buttons step, typing is clamped
// to min..max when it's committed, ArrowUp/Down step too, and every change fires a bubbling
// `change` on the input — features listen for that (`input.valueAsNumber`, or `stepperValue(el)`).
// `data-removable`: at the minimum the − is a bin and sets 0 (the listener removes the line).

import { t } from "./page";

const num = (s: string | undefined, d: number) => {
  const n = Number(s);
  return Number.isFinite(n) ? n : d;
};

/** The quantity of the stepper `el` is in (or is). */
export function stepperValue(el: Element): number {
  const input = el.closest("[data-ts~=stepper]")?.querySelector<HTMLInputElement>("input") ?? null;
  return input ? num(input.value, 0) : 0;
}

/**
 * Change a stepper from code (a cart line capped by stock): new bounds and/or value, without a
 * `change` event (it isn't the shopper's doing).
 */
export function setStepper(el: Element, opts: { value?: number; min?: number; max?: number; disabled?: boolean }): void {
  const root = el.closest<HTMLElement>("[data-ts~=stepper]");
  if (!root) return;
  if (opts.min !== undefined) root.dataset.min = String(opts.min);
  if (opts.max !== undefined) root.dataset.max = String(opts.max);
  const input = root.querySelector<HTMLInputElement>("input");
  if (input && opts.disabled !== undefined) input.disabled = opts.disabled;
  sync(root, opts.value);
}

function bounds(root: HTMLElement) {
  const min = num(root.dataset.min, 1);
  return { min, max: Math.max(min, num(root.dataset.max, 99)), removable: root.hasAttribute("data-removable") };
}

/** Clamp, write the value back and update the buttons; returns the value. */
function sync(root: HTMLElement, want?: number): number {
  const input = root.querySelector<HTMLInputElement>("input");
  if (!input) return 0;
  const { min, max, removable } = bounds(root);
  input.min = String(removable ? 0 : min);
  input.max = String(max);
  let v = want ?? num(input.value, min);
  v = Math.round(v);
  v = removable && v <= 0 ? 0 : Math.min(max, Math.max(min, v));
  if (input.value !== String(v)) input.value = String(v);
  const off = input.disabled;
  const bin = removable && v <= min;
  root.toggleAttribute("data-bin", bin);
  const [dec, inc] = [root.querySelector<HTMLButtonElement>('[data-step="-1"]'), root.querySelector<HTMLButtonElement>('[data-step="1"]')];
  if (dec) {
    dec.disabled = off || (v <= min && !removable);
    dec.setAttribute("aria-label", t(bin ? "stepper.remove" : "stepper.decrease"));
  }
  if (inc) inc.disabled = off || v >= max;
  return v;
}

/** `data-ts="stepper"` (registered by features/core.ts). */
export function mountStepper(root: HTMLElement): () => void {
  const input = root.querySelector<HTMLInputElement>("input");
  if (!input) throw new Error("stepper without an input");
  let last = sync(root);

  const commit = (v: number) => {
    const next = sync(root, v);
    if (next === last) return;
    last = next;
    input.dispatchEvent(new Event("change", { bubbles: true }));
  };
  const onClick = (ev: MouseEvent) => {
    const b = (ev.target as Element).closest<HTMLButtonElement>("button[data-step]");
    if (!b || b.disabled) return;
    const { min, removable } = bounds(root);
    const step = Number(b.dataset.step);
    commit(step < 0 && removable && last <= min ? 0 : last + step);
  };
  // Typed values are clamped before anyone else hears the change (capture, on the root).
  const onChange = (ev: Event) => {
    if (ev.target !== input || !ev.isTrusted) return;
    const next = sync(root);
    if (next === last) {
      ev.stopPropagation(); // e.g. "7" typed over a max of 5 that was already 5
      return;
    }
    last = next;
  };
  const onKey = (ev: KeyboardEvent) => {
    if (ev.key === "Enter") {
      input.blur(); // commits (fires change) and closes the phone keyboard
      return;
    }
    if (ev.key !== "ArrowUp" && ev.key !== "ArrowDown") return;
    ev.preventDefault(); // our step, clamped, with one change event
    commit(last + (ev.key === "ArrowUp" ? 1 : -1));
  };
  // A wheel over a focused number input would change it silently.
  const onWheel = (ev: WheelEvent) => {
    if (document.activeElement === input) ev.preventDefault();
  };
  root.addEventListener("click", onClick);
  root.addEventListener("change", onChange, true);
  input.addEventListener("keydown", onKey);
  input.addEventListener("wheel", onWheel, { passive: false });
  return () => {
    root.removeEventListener("click", onClick);
    root.removeEventListener("change", onChange, true);
    input.removeEventListener("keydown", onKey);
    input.removeEventListener("wheel", onWheel);
  };
}
