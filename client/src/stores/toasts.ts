// The one toast queue. Anything that confirms or reports something calls `toast(...)`; the toaster
// region (`#toaster`, server-rendered in the shell) shows them from `<template id="tpl-toast">`.
// Every kind has the same shape, layout and motion; only the icon and its accent colour differ.
//
// The toaster is a manual popover in the top layer. A drawer is a modal <dialog>, also in the top
// layer, and the HTML spec makes everything outside an open modal inert (a toast there could be
// seen but not pressed): so on every `drawer:open` the toaster moves into the open dialog and is
// raised above it, and on `drawer:close` it goes back.

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
const LEAVE_MS = 180; // toast.css: toast-out

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

/** Show the region in the top layer, above whatever is there now (an open drawer). */
function raise(region: HTMLElement): void {
  if (typeof region.showPopover !== "function") return;
  try {
    if (region.matches(":popover-open")) region.hidePopover();
    region.showPopover();
  } catch {
    /* not connected (a frame swap mid-way): the next raise puts it back */
  }
}

/** Render the queue into the toaster region (once, at boot). */
export function mountToaster(): void {
  const region = document.getElementById("toaster");
  const list = region?.querySelector("ol");
  if (!region || !list) return;
  const home = { parent: region.parentElement, next: region.nextSibling };
  raise(region);
  // Into a drawer that opens (outside it, the modal makes the toaster inert), on top.
  document.addEventListener("drawer:open", (ev) => {
    const dialog = ev.target;
    if (dialog instanceof HTMLDialogElement && dialog.open && region.parentElement !== dialog) dialog.append(region);
    raise(region);
  });
  document.addEventListener("drawer:close", () => {
    const open = document.querySelector<HTMLDialogElement>("dialog[open]");
    if (open && region.parentElement === open) return; // another drawer still holds it
    if (region.parentElement !== home.parent && home.parent?.isConnected) home.parent.insertBefore(region, home.next?.parentNode === home.parent ? home.next : null);
    raise(region);
  });
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
    // Leaving the way they came (toast.css [data-leaving]), then gone.
    for (const el of Array.from(list.querySelectorAll<HTMLElement>("[data-toast-id]:not([data-leaving])"))) {
      if (keep.has(el.dataset.toastId ?? "")) continue;
      el.dataset.leaving = "true";
      window.setTimeout(() => el.remove(), LEAVE_MS);
    }
    for (const t of items) {
      if (list.querySelector(`[data-toast-id="${t.id}"]`)) continue;
      const li = fill("toast", { msg: t.msg, action: t.action?.label ?? null }).firstElementChild;
      if (!(li instanceof HTMLElement)) continue;
      li.dataset.toastId = String(t.id);
      li.dataset.kind = t.kind;
      list.append(li);
    }
    // Stack depth for the CSS: newest = 0 (the ones leaving don't count).
    const shown = Array.from(list.querySelectorAll<HTMLElement>("[data-toast-id]:not([data-leaving])"));
    shown.forEach((el, i) => (el.dataset.depth = String(shown.length - 1 - i)));
    if (items.length) raise(region);
  });
}
