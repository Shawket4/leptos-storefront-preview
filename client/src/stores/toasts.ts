// The one toast queue. Anything that confirms or reports something calls `toast(...)`; the toaster
// region (`#toaster`, server-rendered in the shell) shows them from `<template id="tpl-toast">`.

import { createStore } from "../core/store";
import { fill } from "../core/dom";

export type ToastKind = "success" | "error" | "info";

export interface Toast {
  id: number;
  msg: string;
  kind: ToastKind;
  action?: { label: string; run: () => void };
}

const MAX_VISIBLE = 3;
const LIFETIME = { success: 4000, info: 4000, error: 6500 } as const;

export const toasts = createStore<Toast[]>([]);
let nextId = 1;
let paused = false;
const timers = new Map<number, number>();

export function toast(msg: string, kind: ToastKind = "info", action?: Toast["action"]): number {
  const t: Toast = action ? { id: nextId++, msg, kind, action } : { id: nextId++, msg, kind };
  toasts.update((list) => [...list, t].slice(-MAX_VISIBLE));
  schedule(t);
  return t.id;
}

export function dismiss(id: number): void {
  clearTimeout(timers.get(id));
  timers.delete(id);
  toasts.update((list) => list.filter((t) => t.id !== id));
}

function schedule(t: Toast): void {
  timers.set(t.id, window.setTimeout(() => (paused ? schedule(t) : dismiss(t.id)), LIFETIME[t.kind]));
}

/** Render the queue into the toaster region (once, at boot). */
export function mountToaster(): void {
  const region = document.getElementById("toaster");
  const list = region?.querySelector("ol");
  if (!region || !list) return;
  region.addEventListener("pointerenter", () => (paused = true));
  region.addEventListener("pointerleave", () => (paused = false));
  region.addEventListener("focusin", () => (paused = true));
  region.addEventListener("focusout", () => (paused = false));
  region.addEventListener("click", (ev) => {
    const li = (ev.target as Element).closest<HTMLElement>("[data-toast-id]");
    if (!li) return;
    const id = Number(li.dataset.toastId);
    if ((ev.target as Element).closest("[data-toast-action]")) toasts.get().find((t) => t.id === id)?.action?.run();
    dismiss(id);
  });
  toasts.subscribe((items) => {
    const keep = new Set(items.map((t) => String(t.id)));
    for (const el of Array.from(list.querySelectorAll<HTMLElement>("[data-toast-id]")))
      if (!keep.has(el.dataset.toastId ?? "")) el.remove();
    for (const t of items) {
      if (list.querySelector(`[data-toast-id="${t.id}"]`)) continue;
      const li = fill("toast", { msg: t.msg, action: t.action?.label ?? null }).firstElementChild;
      if (!(li instanceof HTMLElement)) continue;
      li.dataset.toastId = String(t.id);
      li.dataset.kind = t.kind;
      list.append(li);
    }
    // Stack depth for the CSS: newest = 0.
    const shown = Array.from(list.querySelectorAll<HTMLElement>("[data-toast-id]"));
    shown.forEach((el, i) => (el.dataset.depth = String(shown.length - 1 - i)));
  });
}
