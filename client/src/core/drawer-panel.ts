// The drawer panel (markup: components::drawer::Drawer → `<dialog data-ts="drawer"
// data-side="end|start|bottom" id=…>`), loaded on demand; the API features use is `drawer.ts`. A modal dialog: the browser traps focus, makes the page
// behind inert and gives Escape; the page doesn't scroll behind it (drawer.css). Motion is a
// spring (core/motion.ts) along one path in and out: it follows the finger 1:1 when dragged toward
// its edge, rubber-bands past open, and on release the projected velocity decides — past halfway
// it closes (carrying the finger's speed), otherwise it springs back (a little bounce after a
// flick). Reduced motion: a short cross-fade.
//
// Close: `[data-drawer-close]` inside, Escape, the scrim, a swipe, `closeDrawer(id)`, or following a
// link inside it. The element fires `drawer:open` (as it starts opening) and `drawer:close` (once
// closed), both bubbling.
//
// A drawer that is open when the router swaps its region out comes back open in the new page
// (same `id`), instantly, with its scroll position and focus (by id, else name+value, else name).

import { SPRING, SpringValue, VelocityTracker, project, rubberband } from "./motion";
import { prefersReducedMotion } from "./dom";
import { register } from "./mount";
import { registerDrawer, type DrawerControl } from "./drawer";

type Side = "end" | "start" | "bottom";

// ── Survivors: open drawers carried across a router swap ─────────────────────────────────────
interface Survivor {
  at: number;
  scroll: number;
  focus: { id: string; name: string; value: string } | null;
}
const survivors = new Map<string, Survivor>();

function focusKey(el: Element | null, within: Element): Survivor["focus"] {
  if (!(el instanceof HTMLElement) || !within.contains(el)) return null;
  const name = el.getAttribute("name") ?? "";
  return { id: el.id, name, value: el instanceof HTMLInputElement ? el.value : "" };
}

function findFocus(root: Element, f: NonNullable<Survivor["focus"]>): HTMLElement | null {
  const esc = (s: string) => CSS.escape(s);
  if (f.id) {
    const el = root.querySelector<HTMLElement>(`#${esc(f.id)}`);
    if (el) return el;
  }
  if (f.name) {
    return (
      root.querySelector<HTMLElement>(`[name="${esc(f.name)}"][value="${esc(f.value)}"]`) ??
      root.querySelector<HTMLElement>(`[name="${esc(f.name)}"]`)
    );
  }
  return null;
}

/** `data-ts="drawer"`. */
function mountDrawer(node: HTMLElement): () => void {
  if (!(node instanceof HTMLDialogElement)) throw new Error("data-ts=drawer must be a <dialog>");
  const el = node;
  const panel = el.querySelector<HTMLElement>(".drawer__panel");
  const scrim = el.querySelector<HTMLElement>(".drawer__scrim");
  const body = el.querySelector<HTMLElement>(".drawer__body");
  if (!panel || !scrim) throw new Error(`drawer #${el.id}: missing .drawer__panel/.drawer__scrim`);
  const side = (el.dataset.side ?? "end") as Side;
  let extent = 400;
  let opener: HTMLElement | null = null;
  // "closed" → show → "open" → hide → "closing" → finish → "closed" (show can interrupt "closing").
  let state: "closed" | "open" | "closing" = "closed";
  let fadeTimer = 0;

  // +1 when "toward the panel's own edge" is +x on screen (end in LTR, start in RTL).
  const sign = () => ((side === "end") !== (document.documentElement.dir === "rtl") ? 1 : -1);
  const draw = (p: number) => {
    panel.style.transform = side === "bottom" ? `translate3d(0,${p}px,0)` : `translate3d(${sign() * p}px,0,0)`;
    scrim.style.opacity = String(Math.min(1, Math.max(0, 1 - p / Math.max(extent, 1))));
  };
  // p = distance from open along the dismiss direction: 0 = open, extent = off screen.
  const pos = new SpringValue(0, draw);
  const measure = () => {
    const r = panel.getBoundingClientRect();
    extent = Math.max(1, side === "bottom" ? r.height : r.width);
  };

  const emit = (type: "drawer:open" | "drawer:close") => el.dispatchEvent(new CustomEvent(type, { bubbles: true }));

  const show = (instant: boolean) => {
    clearTimeout(fadeTimer);
    delete el.dataset.leaving;
    state = "open";
    if (!el.open) {
      const active = document.activeElement;
      opener = active instanceof HTMLElement && active !== document.body ? active : null;
      el.showModal();
      measure();
      if (!instant) pos.jump(extent);
      panel.focus({ preventScroll: true });
      emit("drawer:open");
    }
    if (instant || prefersReducedMotion()) pos.jump(0);
    else pos.to(0, { spring: SPRING.default });
  };

  const finish = () => {
    state = "closed";
    delete el.dataset.leaving;
    if (el.open) el.close();
    pos.jump(extent);
    const back = opener;
    opener = null;
    if (back?.isConnected) back.focus({ preventScroll: true });
    emit("drawer:close");
  };

  const hide = (velocity?: number) => {
    if (!el.open || state !== "open") return;
    state = "closing";
    if (prefersReducedMotion()) {
      el.dataset.leaving = "";
      fadeTimer = window.setTimeout(finish, 160);
      return;
    }
    pos.to(extent, velocity === undefined ? { spring: SPRING.snappy, onRest: finish } : { spring: SPRING.default, velocity, onRest: finish });
  };

  const api: DrawerControl = { el, open: () => show(false), close: () => hide(), isOpen: () => state === "open" };
  const unregister = el.id ? registerDrawer(el.id, api) : () => {};

  // Escape: leave along the way it came, not in one frame. (If the browser closes it anyway —
  // a second Escape without activation — `close` brings our state back in line.)
  const onCancel = (ev: Event) => {
    ev.preventDefault();
    hide();
  };
  const onClose = () => {
    if (state === "closed") return; // our own finish()
    state = "closed";
    pos.stop();
    pos.jump(extent);
    emit("drawer:close");
  };
  const onClick = (ev: MouseEvent) => {
    if (dragged) {
      dragged = false;
      ev.preventDefault();
      ev.stopPropagation();
      return;
    }
    const t = ev.target as Element;
    if (t.closest("[data-drawer-close]")) {
      ev.preventDefault();
      hide();
    } else if (t.closest("a[href]") && !ev.defaultPrevented) {
      // A link inside (a product in the cart): the router takes it; the drawer gets out of the way.
      hide();
    }
  };

  // ── Drag to dismiss ──
  // The bottom sheet drags from its grabber and head (its body scrolls); side drawers drag
  // sideways anywhere (vertical scrolling stays native: touch-action pan-y). A drag starts after
  // 10px along the dismiss axis; a click after a real drag is swallowed.
  let drag: { id: number; x0: number; y0: number; p0: number; active: boolean; v: VelocityTracker } | null = null;
  let dragged = false;
  const onDown = (ev: PointerEvent) => {
    if (ev.button !== 0 || !el.open) return;
    const t = ev.target as Element;
    if (side === "bottom" && (!t.closest(".drawer__grabber, .drawer__head") || t.closest("button, a"))) return;
    if (t.closest("input, select, textarea, [data-drawer-nodrag]")) return;
    // Grabbing a drawer on its way out takes it back: the drag decides again.
    if (state === "closing") {
      state = "open";
      clearTimeout(fadeTimer);
      delete el.dataset.leaving;
    }
    pos.stop();
    const v = new VelocityTracker();
    v.add(0, ev.timeStamp);
    drag = { id: ev.pointerId, x0: ev.clientX, y0: ev.clientY, p0: pos.value, active: false, v };
    dragged = false;
  };
  const onMove = (ev: PointerEvent) => {
    if (!drag || drag.id !== ev.pointerId) return;
    const dx = ev.clientX - drag.x0, dy = ev.clientY - drag.y0;
    // Movement toward the panel's edge is positive.
    const along = side === "bottom" ? dy : dx * sign();
    const across = side === "bottom" ? dx : dy;
    if (!drag.active) {
      if (Math.abs(along) < 10 && Math.abs(across) < 10) return;
      if (Math.abs(across) > Math.abs(along)) {
        drag = null; // not ours (a scroll)
        return;
      }
      drag.active = true;
      dragged = true;
      measure();
      panel.setPointerCapture(ev.pointerId);
    }
    ev.preventDefault();
    drag.v.add(along, ev.timeStamp);
    const raw = drag.p0 + along;
    pos.jump(raw < 0 ? -rubberband(-raw, extent) : raw);
  };
  const onUp = (ev: PointerEvent) => {
    const d = drag;
    drag = null;
    if (!d || !d.active || d.id !== ev.pointerId) return;
    const v = d.v.velocity();
    const p = pos.value;
    if (p + project(v) > extent / 2) {
      state = "closing";
      pos.to(extent, { spring: SPRING.default, velocity: Math.max(v, 0), onRest: finish });
    } else {
      pos.to(0, { spring: Math.abs(v) > 300 ? SPRING.bouncy : SPRING.default, velocity: v });
    }
  };

  el.addEventListener("cancel", onCancel);
  el.addEventListener("close", onClose);
  el.addEventListener("click", onClick, true);
  panel.addEventListener("pointerdown", onDown);
  panel.addEventListener("pointermove", onMove);
  panel.addEventListener("pointerup", onUp);
  panel.addEventListener("pointercancel", onUp);
  pos.jump(extent);

  // Came back from a router swap while open: open again at once, where it was.
  const back = el.id ? survivors.get(el.id) : undefined;
  if (back && el.id) {
    survivors.delete(el.id);
    if (Date.now() - back.at < 5000) {
      show(true);
      if (body) body.scrollTop = back.scroll;
      const f = back.focus && findFocus(el, back.focus);
      if (f) f.focus({ preventScroll: true });
    }
  }

  return () => {
    if (el.id && el.open && state === "open") {
      survivors.set(el.id, { at: Date.now(), scroll: body?.scrollTop ?? 0, focus: focusKey(document.activeElement, el) });
    }
    pos.stop();
    clearTimeout(fadeTimer);
    el.removeEventListener("cancel", onCancel);
    el.removeEventListener("close", onClose);
    el.removeEventListener("click", onClick, true);
    panel.removeEventListener("pointerdown", onDown);
    panel.removeEventListener("pointermove", onMove);
    panel.removeEventListener("pointerup", onUp);
    panel.removeEventListener("pointercancel", onUp);
    state = "closed"; // the swap removes it: no drawer:close for a drawer that comes straight back
    if (el.open) el.close();
    unregister();
  };
}

register("drawer", mountDrawer);
