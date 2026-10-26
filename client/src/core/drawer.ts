// PLACEHOLDER written by the cart agent so `openDrawer`/`closeDrawer` exist on its branch before
// core's drawer lands. Same API and DOM contract as AGENTS.md ("core/drawer.ts"); bare behaviour
// (a modal <dialog>, no springs or swipe). Core's file replaces this one on merge — take theirs.

import { register } from "./mount";

const dialog = (id: string): HTMLDialogElement | null => {
  const el = document.getElementById(id);
  return el instanceof HTMLDialogElement ? el : null;
};

export function openDrawer(id: string): void {
  const d = dialog(id);
  if (!d || d.open) return;
  d.showModal();
  d.dispatchEvent(new CustomEvent("drawer:open"));
}

export function closeDrawer(id: string): void {
  const d = dialog(id);
  if (!d?.open) return;
  d.close();
  d.dispatchEvent(new CustomEvent("drawer:close"));
}

register("drawer", (el) => {
  const onCancel = (ev: Event) => {
    ev.preventDefault();
    closeDrawer(el.id);
  };
  const onClick = (ev: MouseEvent) => {
    const t = ev.target instanceof Element ? ev.target : null;
    if (t?.closest("[data-drawer-close], .drawer__scrim, .drawer__close")) closeDrawer(el.id);
  };
  el.addEventListener("cancel", onCancel);
  el.addEventListener("click", onClick);
  return () => {
    el.removeEventListener("cancel", onCancel);
    el.removeEventListener("click", onClick);
  };
});

document.addEventListener("click", (ev) => {
  const t = ev.target instanceof Element ? ev.target.closest<HTMLElement>("[data-drawer-open]") : null;
  if (!t || ev.defaultPrevented) return;
  ev.preventDefault();
  openDrawer(t.dataset.drawerOpen ?? "");
});
