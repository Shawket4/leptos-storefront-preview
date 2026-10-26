// The one drawer / bottom sheet — the API every feature uses, loaded with the page. The panel
// itself (springs, swipe to dismiss, focus, Escape, survival across router swaps) is
// `drawer-panel.ts`, loaded on demand the first time a page has a drawer (`data-ts="drawer"`).
//
// Open: any `[data-drawer-open="<id>"]` (pressed), or `openDrawer(id)`; a press that comes before
// the panel code has loaded opens the drawer as soon as it has. Close: `closeDrawer(id)` or the
// panel's own controls. The element fires `drawer:open` / `drawer:close`, both bubbling.

import { preload } from "./mount";

/** What a mounted drawer offers (drawer-panel.ts registers one per drawer element). */
export interface DrawerControl {
  el: HTMLDialogElement;
  open(): void;
  close(): void;
  isOpen(): boolean;
}

const drawers = new Map<string, DrawerControl>();
const pending = new Set<string>();

/** Open the drawer with this id (once its panel code is in, if it isn't yet). */
export function openDrawer(id: string): void {
  const d = drawers.get(id);
  if (d) d.open();
  else if (document.getElementById(id)?.matches("[data-ts~=drawer]")) {
    pending.add(id);
    preload(["drawer"]);
  } else if (__DEV__) console.warn(`openDrawer: no drawer #${id}`);
}

/** Close the drawer with this id (no-op if it isn't open). */
export function closeDrawer(id: string): void {
  pending.delete(id);
  drawers.get(id)?.close();
}

/** For drawer-panel.ts: a drawer element is mounted (opens at once if a press was waiting). */
export function registerDrawer(id: string, d: DrawerControl): () => void {
  drawers.set(id, d);
  if (pending.delete(id)) d.open();
  return () => {
    if (drawers.get(id) === d) drawers.delete(id);
  };
}

// Triggers: one delegated listener for the whole document, from the start.
document.addEventListener("click", (ev) => {
  if (ev.defaultPrevented || ev.button !== 0 || ev.metaKey || ev.ctrlKey || ev.shiftKey || ev.altKey) return;
  const t = (ev.target as Element | null)?.closest?.<HTMLElement>("[data-drawer-open]");
  const id = t?.dataset.drawerOpen;
  if (!id || !document.getElementById(id)?.matches("[data-ts~=drawer]")) return; // no such drawer: the link is followed
  ev.preventDefault();
  openDrawer(id);
});
