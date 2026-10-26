// The product page: the variant picker and the add (`data-ts="variant-picker"`) and the gallery
// (`data-ts="gallery"`). Everything here is state: switching variants reads the page's own JSON
// (every variant's price, stock, SKU, image and availability, formatted by the server), add-ons and
// quantity are form state, and adding goes into the cookie cart — no request.

import { register } from "../core/mount";
import { $, $$, fill, on, prefersReducedMotion } from "../core/dom";
import { digits, t } from "../core/page";
import { SPRING, SpringValue, VelocityTracker, project, rubberband, type SpringConfig } from "../core/motion";
import { setStepper, stepperValue } from "../core/stepper";
import { onOnline } from "../core/net";
import type { LineSnapshot } from "../stores/cart";
import { addAndConfirm } from "./cart";


/** The page's JSON (`pages/product/panel.rs`, `PickerData`). */
interface PickerData {
  product: number;
  href: string;
  name: string;
  selected: number;
  maxQty: number;
  variants: VariantData[];
  /** Every add-on option, by id: name and surcharge in piastres. */
  addons: Record<string, { name: string; price: number }>;
}
interface VariantData {
  id: number;
  sku: string;
  stock: number;
  stockState: "in" | "low" | "out";
  stockText: string;
  price: string;
  was: string | null;
  save: string | null;
  priceTpl: string;
  image: number | null;
  /** The cart line's priced snapshot (piastres): `api::cart::Snapshot`. */
  unit: number;
  compareAt: number | null;
  snapMax: number;
  label: string | null;
  img: string | null;
  labels: string[];
  values: { s: "on" | "open" | "out" | "none"; v?: number; why?: string }[][];
  notes: string[];
}

/** The gallery listens for this on the page (`.pdp`): a variant whose photo differs. */
const VARIANT_EVENT = "pdp:variant";

const plainClick = (ev: MouseEvent) => ev.button === 0 && !ev.metaKey && !ev.ctrlKey && !ev.shiftKey && !ev.altKey;

register("variant-picker", (root) => {
  const json = $("script[data-ts-json]", root)?.textContent;
  if (!json) return;
  const data = JSON.parse(json) as PickerData;
  const byId = new Map(data.variants.map((v) => [v.id, v]));
  const form = $<HTMLFormElement>("form.pp-form", root);
  const stepper = $("[data-ts~=stepper]", root);
  const buyRow = $(".pp-buy", root);
  const bar = $(".pp-bar", root);
  let cur = byId.get(data.selected) ?? data.variants[0]!;
  let addedTimer = 0;
  /** An add was tried with this variant: group problems show (and follow the answers live). */
  let attempted = false;

  const pageHref = (v: VariantData) => `${data.href}?v=${v.id}`;

  /** The address says which variant (no history entry; one-off notices dropped). */
  const writeUrl = (v: VariantData | null) => {
    const url = new URL(location.href);
    for (const k of ["added", "cart_err"]) url.searchParams.delete(k);
    if (v) url.searchParams.set("v", String(v.id));
    const next = url.pathname + (url.search === "?" ? "" : url.search) + url.hash;
    if (next !== location.pathname + location.search + location.hash) history.replaceState(history.state, "", next);
  };

  const select = (v: VariantData) => {
    if (v.id === cur.id) return;
    cur = v;
    // Price: the design system's Price for this variant, from its server-rendered template.
    $("[data-price]", root)?.replaceChildren(fill(v.priceTpl, {}));
    const save = $("[data-save]", root);
    if (save) {
      save.textContent = v.save ?? "";
      save.classList.toggle("is-empty", !v.save);
    }
    const stock = $("[data-stock]", root);
    stock?.classList.toggle("is-low", v.stockState === "low");
    stock?.classList.toggle("is-out", v.stockState === "out");
    const stockText = $("[data-stock-text]", root);
    if (stockText) stockText.textContent = v.stockText;
    const sku = $("[data-sku]", root);
    if (sku) sku.textContent = v.sku;
    // Every axis: the picked value's name, each value's state, why the others can't be picked.
    for (const axis of $$(".pp-axis[data-axis]", root)) {
      const ai = Number(axis.dataset.axis);
      const value = $(".pp-axis__value", axis);
      if (value) value.textContent = v.labels[ai] ?? "";
      const note = $(".pp-axis__note", axis);
      if (note) {
        note.textContent = v.notes[ai] ?? "";
        note.classList.toggle("is-empty", !v.notes[ai]);
      }
      for (const a of $$<HTMLAnchorElement>("a[data-index]", axis)) {
        const st = v.values[ai]?.[Number(a.dataset.index)];
        if (!st) continue;
        const enabled = st.s === "on" || st.s === "open";
        a.classList.toggle("is-on", st.s === "on");
        a.classList.toggle("is-off", !enabled);
        a.setAttribute("aria-checked", String(st.s === "on"));
        a.tabIndex = st.s === "on" ? 0 : -1;
        if (enabled && st.v !== undefined) a.setAttribute("href", `${data.href}?v=${st.v}`);
        else a.removeAttribute("href");
        a.toggleAttribute("aria-disabled", !enabled);
        if (enabled) {
          a.removeAttribute("aria-describedby");
          if (a.classList.contains("pp-swatch")) a.title = a.getAttribute("aria-label") ?? "";
          else a.removeAttribute("title");
        } else {
          a.setAttribute("aria-disabled", "true");
          if (note?.id) a.setAttribute("aria-describedby", note.id);
          a.title = st.why ?? "";
        }
      }
    }
    // The form posts what's on screen (no-JS twin), and the quantity follows the stock.
    const variantInput = form?.querySelector<HTMLInputElement>("input[name=variant]");
    if (variantInput) variantInput.value = String(v.id);
    const next = form?.querySelector<HTMLInputElement>("input[name=next]");
    if (next) next.value = pageHref(v);
    const out = v.stock === 0;
    if (stepper) setStepper(stepper, { max: Math.max(1, Math.min(v.stock, data.maxQty)), disabled: out });
    for (const b of $$<HTMLButtonElement>("button[data-add]", root)) b.disabled = out;
    for (const l of $$("[data-add-label]", root)) l.textContent = t(out ? "product.out" : "product.add");
    const barPrice = $("[data-bar-price]", root);
    if (barPrice) barPrice.textContent = v.price;
    $(".pp-msgs", root)?.replaceChildren();
    // A new variant is a fresh start: no add-on message until the next try to add.
    attempted = false;
    validate();
    writeUrl(v);
    if (v.image !== null) root.dispatchEvent(new CustomEvent(VARIANT_EVENT, { bubbles: true, detail: { image: v.image } }));
  };

  // ── Add-on groups: limits as you pick. A group's problem shows only once an add was tried
  // (like checkout): then on every unmet group at once, and from there it follows the answers
  // live. Changing the variant starts over. ──
  const groupState = (g: HTMLElement) => {
    const min = Number(g.dataset.min ?? 0);
    const max = g.dataset.max ? Number(g.dataset.max) : Infinity;
    const picked = $$<HTMLInputElement>("input:checked", g).filter((i) => i.value !== "").length;
    return { ok: picked >= min && picked <= max, picked, max };
  };
  const markGroup = (g: HTMLElement, bad: boolean) => {
    g.classList.toggle("is-bad", bad);
    if (bad) g.setAttribute("aria-invalid", "true");
    else g.removeAttribute("aria-invalid");
  };
  const limitGroup = (g: HTMLElement) => {
    const { picked, max } = groupState(g);
    for (const box of $$<HTMLInputElement>("input[type=checkbox]", g)) {
      const full = !box.checked && picked >= max;
      box.disabled = full;
      box.closest(".pp-opt")?.classList.toggle("is-full", full);
    }
  };
  for (const g of $$(".pp-group", root)) limitGroup(g);
  /** Mark the unmet groups (only after an attempt); returns them. */
  const validate = (): HTMLElement[] => {
    const groups = $$(".pp-group", root);
    const bad = groups.filter((g) => !groupState(g).ok);
    for (const g of groups) markGroup(g, attempted && bad.includes(g));
    return bad;
  };

  const submit = (ev: SubmitEvent) => {
    ev.preventDefault();
    if (cur.stock === 0) return;
    attempted = true;
    const first = validate()[0];
    if (first) {
      first.scrollIntoView({ block: "center", behavior: prefersReducedMotion() ? "auto" : "smooth" });
      $<HTMLInputElement>("input:not(:disabled)", first)?.focus({ preventScroll: true });
      return;
    }
    // The line's add-ons in the cart's order (sorted ids), names and prices alike.
    const addons = $$<HTMLInputElement>(".pp-groups input:checked", root)
      .map((i) => i.value)
      .filter(Boolean)
      .sort();
    const qty = stepper ? Math.max(1, stepperValue(stepper)) : 1;
    // Priced: the cart computes the line from this, so the add needs no request at all.
    const snap: LineSnapshot = { name: data.name, href: pageHref(cur), unit: cur.unit, maxQty: cur.snapMax };
    if (cur.img) snap.img = cur.img;
    if (cur.label) snap.variant = cur.label;
    if (cur.compareAt !== null) snap.compareAt = cur.compareAt;
    if (addons.length) {
      snap.addons = addons.map((id) => data.addons[id]?.name ?? id);
      snap.addonPrices = addons.map((id) => data.addons[id]?.price ?? 0);
    }
    const r = addAndConfirm({ product: data.product, variant: cur.id, qty, addons }, snap);
    if (!r.ok) return;
    attempted = false;
    // The button says it worked, for a moment.
    for (const b of $$("button[data-add]", root)) b.classList.add("is-added");
    for (const l of $$("[data-add-label]", root)) l.textContent = t("product.added_short");
    clearTimeout(addedTimer);
    addedTimer = window.setTimeout(() => {
      for (const b of $$("button[data-add]", root)) b.classList.remove("is-added");
      for (const l of $$("[data-add-label]", root)) l.textContent = t(cur.stock === 0 ? "product.out" : "product.add");
    }, 1800);
  };

  // Arrow keys move the choice to the next value that can be picked (radio semantics).
  const onKey = (ev: KeyboardEvent, a: HTMLElement) => {
    const rtl = document.documentElement.dir === "rtl";
    const fwd = { ArrowDown: true, ArrowUp: false, ArrowRight: !rtl, ArrowLeft: rtl }[ev.key];
    if (fwd === undefined) return;
    ev.preventDefault();
    const axis = a.closest<HTMLElement>(".pp-axis");
    if (!axis) return;
    const ai = Number(axis.dataset.axis);
    const states = cur.values[ai] ?? [];
    const at = states.findIndex((s) => s.s === "on");
    const n = states.length;
    for (let k = 1; k < n; k++) {
      const i = (((at + (fwd ? k : -k)) % n) + n) % n;
      const st = states[i];
      if (st?.s === "open" && st.v !== undefined) {
        const v = byId.get(st.v);
        if (!v) return;
        select(v);
        $<HTMLElement>(`a[data-index="${i}"]`, axis)?.focus();
        return;
      }
    }
  };

  // A notice from the no-JS add was rendered for this visit only: drop its parameter.
  writeUrl(new URL(location.href).searchParams.has("v") ? cur : null);
  for (const axis of $$(".pp-axis", root))
    for (const a of $$<HTMLAnchorElement>("a[data-index]", axis)) a.tabIndex = a.classList.contains("is-on") ? 0 : -1;

  const offs = [
    on(root, "click", "a[data-axis][data-index]", (ev, a) => {
      if (!plainClick(ev)) return;
      ev.preventDefault();
      const st = cur.values[Number(a.dataset.axis)]?.[Number(a.dataset.index)];
      const v = st?.s === "open" && st.v !== undefined ? byId.get(st.v) : undefined;
      if (v) select(v);
    }),
    on(root, "keydown", "a[data-axis][data-index]", onKey),
    on(root, "change", ".pp-group input", (_ev, el) => {
      const g = el.closest<HTMLElement>(".pp-group");
      if (!g) return;
      limitGroup(g);
      validate(); // live, once an add was tried
    }),
  ];
  form?.addEventListener("submit", submit);

  // The phone's bar appears once the main button has scrolled away above.
  let io: IntersectionObserver | null = null;
  if (buyRow && bar) {
    io = new IntersectionObserver((entries) => {
      const e = entries[entries.length - 1];
      if (!e) return;
      const show = !e.isIntersecting && e.boundingClientRect.top < 0;
      bar.classList.toggle("is-shown", show);
      bar.inert = !show;
      bar.setAttribute("aria-hidden", String(!show));
    });
    io.observe(buyRow);
  }

  return () => {
    for (const off of offs) off();
    form?.removeEventListener("submit", submit);
    io?.disconnect();
    clearTimeout(addedTimer);
  };
});

// ── Gallery ─────────────────────────────────────────────────────────────────────────────────────
// Apple-style (`.claude/skills/apple-design`): the strip follows the finger 1:1 from where it was
// grabbed, the release velocity decides next/previous, a slow release lands where its momentum
// projects, the ends rubber-band, and every movement starts from the live position — grab it
// mid-flight and it stops under the finger; a flick hands its speed to the spring.

/** Faster than this (px/s) and the release direction wins over the position. */
const FLICK_V = 280;
/** Movement (px) before a gesture commits to a direction. */
const SLOP = 8;
const TAP_ZOOM = 2.5;
const MAX_ZOOM = 4;
const DOUBLE_TAP_MS = 300;
/** After a flick only: a little life at the landing. */
const FLICK: SpringConfig = { damping: 0.88, response: 0.32 };

const isRtl = () => document.documentElement.dir === "rtl";

interface Drag {
  id: number;
  x0: number;
  y0: number;
  /** Strip position when grabbed. */
  pos0: number;
  /** The finger's (un-rubber-banded) position, for the release. */
  raw: number;
  axis: "" | "x" | "y";
  tracker: VelocityTracker;
}

/** One horizontal strip of slides (the stage, the viewer): position along the reading direction. */
class Strip {
  w = 0;
  rtl = false;
  index: number;
  drag: Drag | null = null;
  readonly x: SpringValue;

  constructor(
    readonly viewport: HTMLElement,
    readonly track: HTMLElement,
    readonly n: number,
    start: number,
    private readonly onIndex: (i: number) => void,
  ) {
    this.index = start;
    this.x = new SpringValue(0, (p) => this.draw(p));
  }

  private draw(p: number): void {
    const x = this.rtl ? p : -p;
    this.track.style.transform = `translate3d(${x.toFixed(2)}px,0,0)`;
  }
  private get max(): number {
    return Math.max(0, this.n - 1) * this.w;
  }
  /** On screen for a raw (finger) position: rubber-banded past either end. */
  private shown(raw: number): number {
    if (raw < 0) return -rubberband(-raw, this.w);
    if (raw > this.max) return this.max + rubberband(raw - this.max, this.w);
    return raw;
  }

  /** Re-read the slide width (mount, resize, open) and rest on the index. */
  measure(): void {
    this.rtl = isRtl();
    const w = this.viewport.clientWidth;
    if (w > 0 && !this.drag) {
      this.w = w;
      this.x.jump(this.index * w);
    }
  }
  /** Spring to slide `i` from the live position, handing over velocity `v` (px/s). */
  go(i: number, v = 0, spring: SpringConfig = SPRING.default): void {
    this.index = Math.min(Math.max(i, 0), this.n - 1);
    this.onIndex(this.index);
    this.x.to(this.index * this.w, { velocity: v, spring });
  }
  jump(i: number): void {
    this.index = Math.min(Math.max(i, 0), this.n - 1);
    this.onIndex(this.index);
    this.x.jump(this.index * this.w);
  }
  step(d: number): void {
    this.go(this.index + d);
  }

  /** Pointer down: grab the strip where it is on screen (interrupting any spring). */
  down(ev: PointerEvent): void {
    const pos0 = this.x.value;
    this.x.jump(pos0);
    const tracker = new VelocityTracker();
    tracker.add(pos0, ev.timeStamp);
    this.drag = { id: ev.pointerId, x0: ev.clientX, y0: ev.clientY, pos0, raw: pos0, axis: "", tracker };
  }
  /** Pointer move: decide the axis after the slop, then follow 1:1. */
  moved(ev: PointerEvent): Drag["axis"] {
    const d = this.drag;
    if (!d || d.id !== ev.pointerId) return "";
    const dx = ev.clientX - d.x0, dy = ev.clientY - d.y0;
    if (!d.axis && (Math.abs(dx) > SLOP || Math.abs(dy) > SLOP)) d.axis = Math.abs(dx) > Math.abs(dy) ? "x" : "y";
    if (d.axis === "y") {
      this.drag = null; // the page scrolls
      return "y";
    }
    if (d.axis === "x" && this.w > 0) {
      d.raw = d.pos0 + dx * (this.rtl ? 1 : -1);
      d.tracker.add(d.raw, ev.timeStamp);
      this.x.jump(this.shown(d.raw));
    }
    return d.axis;
  }
  /** Pointer up: a tap, or a swipe that springs to the slide its velocity points at. */
  up(ev: PointerEvent, cancelled: boolean): "tap" | "swipe" | "none" {
    const d = this.drag;
    if (!d || d.id !== ev.pointerId) return "none";
    this.drag = null;
    if (!d.axis && !cancelled) {
      this.go(this.index, 0, SPRING.snappy);
      return "tap";
    }
    if (d.axis !== "x") {
      this.go(this.index);
      return "none";
    }
    // A sample at release: a finger that rested before lifting carries no speed.
    d.tracker.add(d.raw, ev.timeStamp);
    const v = cancelled ? 0 : d.tracker.velocity();
    let target = this.index;
    if (this.w > 0) {
      if (Math.abs(v) > FLICK_V) target = this.index + (v > 0 ? 1 : -1); // one photo per flick
      else {
        const landing = d.raw + project(v);
        target = Math.min(Math.max(Math.round(landing / this.w), this.index - 1), this.index + 1);
      }
    }
    this.go(target, v, Math.abs(v) > FLICK_V ? FLICK : SPRING.default);
    return "swipe";
  }
}

register("gallery", (root) => {
  const n = Number(root.dataset.count ?? 1);
  const vp = $(".pg__viewport", root);
  const track = $(".pg__track", root);
  const dialog = $<HTMLDialogElement>("dialog.pv", root);
  const vvp = $(".pv__viewport", root);
  const vtrack = $(".pv__track", root);
  if (!vp || !track || !dialog || !vvp || !vtrack) return;
  const slides = $$(".pg__slide", track);
  const live = $("[data-live]", root);
  const fine = matchMedia("(hover: hover) and (pointer: fine) and (min-width: 768px)").matches;
  let announced = false;

  const counter = (i: number) => `${digits(i + 1)} / ${digits(n)}`;
  const onMain = (i: number) => {
    const c = $("[data-count]", $(".pg__stage", root) ?? root);
    if (c) c.textContent = counter(i);
    $$(".pg__dot", root).forEach((d, k) => d.classList.toggle("is-on", k === i));
    $$(".pg__thumb", root).forEach((th, k) => {
      if (k === i) th.setAttribute("aria-current", "true");
      else th.removeAttribute("aria-current");
    });
    const prev = $<HTMLButtonElement>(".pg__nav--prev", root);
    const next = $<HTMLButtonElement>(".pg__nav--next", root);
    if (prev) prev.disabled = i === 0;
    if (next) next.disabled = i + 1 >= n;
    if (live && announced && n > 1) live.textContent = t("product.photo_of", { n: digits(i + 1), total: digits(n) });
    announced = true;
  };
  const onViewer = (i: number) => {
    const c = $("[data-count]", $(".pv__bar", dialog) ?? dialog);
    if (c && n > 1) c.textContent = counter(i);
  };

  // Take over from the native scroll strip at the photo the shopper is on (they may have swiped
  // before this ran).
  const w0 = vp.clientWidth;
  const scrolled = Math.abs(vp.scrollLeft);
  const start = scrolled > 1 && w0 > 0 ? Math.round(scrolled / w0) : Number(root.dataset.start ?? 0);
  const main = new Strip(vp, track, n, Math.min(start, n - 1), onMain);
  const viewer = new Strip(vvp, vtrack, n, main.index, onViewer);
  root.classList.add("is-js");
  vp.scrollLeft = 0;
  main.measure();
  onMain(main.index);

  // ── Zoom in the viewer: scale and pan, each its own spring ──
  const zs = { s: 1, x: 0, y: 0 };
  const zoomEl = () => $(`.pv__slide[data-i="${viewer.index}"] .pv__zoom`, vtrack);
  const drawZoom = () => {
    const el = zoomEl();
    if (el) el.style.transform = `translate3d(${zs.x.toFixed(2)}px,${zs.y.toFixed(2)}px,0) scale(${zs.s.toFixed(4)})`;
    vvp.classList.toggle("is-zoomed", zs.s > 1.01);
  };
  const sS = new SpringValue(1, (v) => ((zs.s = v), drawZoom()));
  const sX = new SpringValue(0, (v) => ((zs.x = v), drawZoom()));
  const sY = new SpringValue(0, (v) => ((zs.y = v), drawZoom()));
  const box = { w: 0, h: 0, aspect: 1 };
  const aspects = $$(".pv__slide", vtrack).map((s) => {
    const img = $<HTMLImageElement>("img", slides[Number(s.dataset.i)] ?? s);
    const w = Number(img?.getAttribute("width")), h = Number(img?.getAttribute("height"));
    return w > 0 && h > 0 ? w / h : 1;
  });
  const measureZoom = () => {
    const r = vvp.getBoundingClientRect();
    box.w = r.width;
    box.h = r.height;
    box.aspect = aspects[viewer.index] ?? 1;
  };
  const fit = (): [number, number] => {
    const a = box.aspect > 0 ? box.aspect : 1;
    if (box.h <= 0) return [box.w, box.w / a];
    return box.w / box.h > a ? [box.h * a, box.h] : [box.w, box.w / a];
  };
  /** How far the photo may move from centre at scale `s`. */
  const bounds = (s: number): [number, number] => {
    const [iw, ih] = fit();
    return [Math.max(0, (iw * s - box.w) / 2), Math.max(0, (ih * s - box.h) / 2)];
  };
  const band = (v: number, b: number, dim: number) =>
    v > b ? b + rubberband(v - b, dim) : v < -b ? -b - rubberband(-b - v, dim) : v;
  const zoomSet = (s: number, x: number, y: number) => {
    sS.jump(s);
    sX.jump(x);
    sY.jump(y);
  };
  const settle = (s: number, x: number, y: number, vx = 0, vy = 0, spring: SpringConfig = SPRING.default) => {
    s = Math.min(Math.max(s, 1), MAX_ZOOM);
    const [bx, by] = bounds(s);
    sS.to(s, { spring });
    sX.to(Math.min(Math.max(x, -bx), bx), { spring, velocity: vx });
    sY.to(Math.min(Math.max(y, -by), by), { spring, velocity: vy });
  };
  const resetZoom = () => {
    pointers.clear();
    pinch = null;
    pan = null;
    for (const el of $$<HTMLElement>(".pv__zoom", vtrack)) el.style.removeProperty("transform");
    zoomSet(1, 0, 0);
  };
  /** Double-tap: zoom in to the tapped point, or back out. */
  const toggleAt = (cx: number, cy: number) => {
    measureZoom();
    const r = vvp.getBoundingClientRect();
    const fx = cx - r.left - r.width / 2, fy = cy - r.top - r.height / 2;
    if (zs.s > 1.01) settle(1, 0, 0, 0, 0, SPRING.snappy);
    else {
      const k = TAP_ZOOM / Math.max(zs.s, 0.01);
      settle(TAP_ZOOM, fx - (fx - zs.x) * k, fy - (fy - zs.y) * k);
    }
  };

  const openViewer = (i: number) => {
    if (dialog.open) return;
    dialog.showModal();
    document.documentElement.classList.add("pv-open");
    viewer.index = i;
    viewer.measure();
    onViewer(i);
    resetZoom();
  };
  const onClosed = () => {
    document.documentElement.classList.remove("pv-open");
    resetZoom();
    main.jump(viewer.index);
  };

  // ── Stage: swipe, tap to open, hover zoom-to-cursor on desktop ──
  let hovering = false;
  const unhover = () => {
    hovering = false;
    for (const s of slides) {
      s.classList.remove("is-zoom");
      s.style.removeProperty("--zx");
      s.style.removeProperty("--zy");
    }
  };
  const stageDown = (ev: PointerEvent) => {
    if (ev.pointerType === "mouse" && ev.button !== 0) return;
    unhover();
    main.down(ev);
    vp.setPointerCapture(ev.pointerId);
  };
  const stageMove = (ev: PointerEvent) => {
    if (main.moved(ev) === "x") {
      ev.preventDefault();
      return;
    }
    if (!fine || ev.pointerType !== "mouse" || ev.buttons !== 0) return;
    const r = vp.getBoundingClientRect();
    const x = Math.min(Math.max(((ev.clientX - r.left) / r.width) * 100, 0), 100);
    const y = Math.min(Math.max(((ev.clientY - r.top) / r.height) * 100, 0), 100);
    const slide = slides[main.index];
    if (!slide) return;
    if (!hovering) {
      hovering = true;
      // Before zooming a stage photo, ask the browser for its largest file.
      $("img", slide)?.setAttribute("sizes", "1200px");
    }
    slide.classList.add("is-zoom");
    slide.style.setProperty("--zx", `${x.toFixed(1)}%`);
    slide.style.setProperty("--zy", `${y.toFixed(1)}%`);
  };
  const stageUp = (ev: PointerEvent) => {
    if (main.up(ev, false) === "tap") openViewer(main.index);
  };
  const stageCancel = (ev: PointerEvent) => void main.up(ev, true);
  const stageKey = (ev: KeyboardEvent) => {
    const back = isRtl() ? "ArrowRight" : "ArrowLeft";
    const fwd = isRtl() ? "ArrowLeft" : "ArrowRight";
    if (ev.key === fwd) main.step(1);
    else if (ev.key === back) main.step(-1);
    else if (ev.key === "Home") main.go(0);
    else if (ev.key === "End") main.go(n - 1);
    else if (ev.key === "Enter" || ev.key === " ") openViewer(main.index);
    else return;
    ev.preventDefault();
  };

  // ── Viewer: pinch, pan when zoomed, swipe otherwise, double-tap ──
  const pointers = new Map<number, { x: number; y: number }>();
  let pinch: { d0: number; m0: [number, number]; s0: number; t0: [number, number] } | null = null;
  let pan: {
    id: number; x0: number; y0: number; ox: number; oy: number; rx: number; ry: number;
    tx: VelocityTracker; ty: VelocityTracker; moved: boolean;
  } | null = null;
  let lastTap: { t: number; x: number; y: number } | null = null;
  const startPan = (id: number, x: number, y: number, t0: number) => {
    const tx = new VelocityTracker(), ty = new VelocityTracker();
    tx.add(zs.x, t0);
    ty.add(zs.y, t0);
    pan = { id, x0: x, y0: y, ox: zs.x, oy: zs.y, rx: zs.x, ry: zs.y, tx, ty, moved: false };
  };
  const two = (): [{ x: number; y: number }, { x: number; y: number }] | null => {
    const v = [...pointers.values()];
    return v.length >= 2 ? [v[0]!, v[1]!] : null;
  };
  const doubleTap = (x: number, y: number, at: number) => {
    const hit = lastTap && at - lastTap.t < DOUBLE_TAP_MS && Math.abs(lastTap.x - x) < 30 && Math.abs(lastTap.y - y) < 30;
    lastTap = hit ? null : { t: at, x, y };
    if (hit) toggleAt(x, y);
  };
  const vDown = (ev: PointerEvent) => {
    if (ev.pointerType === "mouse" && ev.button !== 0) return;
    vvp.setPointerCapture(ev.pointerId);
    measureZoom();
    zoomSet(zs.s, zs.x, zs.y); // grab the photo where it is
    pointers.set(ev.pointerId, { x: ev.clientX, y: ev.clientY });
    const pair = two();
    if (pair) {
      // Second finger: pinch. The strip lets go where it is.
      viewer.drag = null;
      viewer.go(viewer.index, 0, SPRING.snappy);
      const [a, b] = pair;
      pan = null;
      pinch = {
        d0: Math.max(1, Math.hypot(a.x - b.x, a.y - b.y)),
        m0: [(a.x + b.x) / 2 - box.w / 2, (a.y + b.y) / 2 - box.h / 2],
        s0: zs.s,
        t0: [zs.x, zs.y],
      };
    } else if (zs.s > 1.01) startPan(ev.pointerId, ev.clientX, ev.clientY, ev.timeStamp);
    else viewer.down(ev);
  };
  const vMove = (ev: PointerEvent) => {
    const p = pointers.get(ev.pointerId);
    if (p) {
      p.x = ev.clientX;
      p.y = ev.clientY;
    }
    if (pinch) {
      ev.preventDefault();
      const pair = two();
      if (!pair) return;
      const [a, b] = pair;
      const d = Math.max(1, Math.hypot(a.x - b.x, a.y - b.y));
      const m: [number, number] = [(a.x + b.x) / 2 - box.w / 2, (a.y + b.y) / 2 - box.h / 2];
      const raw = (pinch.s0 * d) / pinch.d0;
      // Resist past the limits instead of stopping dead.
      const s = raw < 1 ? 1 - rubberband(1 - raw, 1) : raw > MAX_ZOOM ? MAX_ZOOM + rubberband(raw - MAX_ZOOM, MAX_ZOOM) : raw;
      const k = s / pinch.s0;
      zoomSet(s, m[0] - (pinch.m0[0] - pinch.t0[0]) * k, m[1] - (pinch.m0[1] - pinch.t0[1]) * k);
      return;
    }
    if (pan) {
      if (pan.id !== ev.pointerId) return;
      ev.preventDefault();
      if (Math.abs(ev.clientX - pan.x0) > SLOP || Math.abs(ev.clientY - pan.y0) > SLOP) pan.moved = true;
      pan.rx = pan.ox + ev.clientX - pan.x0;
      pan.ry = pan.oy + ev.clientY - pan.y0;
      pan.tx.add(pan.rx, ev.timeStamp);
      pan.ty.add(pan.ry, ev.timeStamp);
      const [bx, by] = bounds(zs.s);
      zoomSet(zs.s, band(pan.rx, bx, box.w), band(pan.ry, by, box.h));
      return;
    }
    if (viewer.moved(ev) === "x") ev.preventDefault();
  };
  const vUp = (ev: PointerEvent, cancelled: boolean) => {
    pointers.delete(ev.pointerId);
    const { clientX: x, clientY: y, timeStamp: at } = ev;
    if (pinch) {
      if (pointers.size >= 2) return;
      pinch = null;
      // One finger left: it carries on as a pan; settle when it lifts.
      const [id, p] = [...pointers.entries()][0] ?? [];
      if (id !== undefined && p) startPan(id, p.x, p.y, at);
      else settle(zs.s, zs.x, zs.y);
      return;
    }
    if (pan) {
      if (pan.id !== ev.pointerId) return;
      const pn = pan;
      pan = null;
      if (!pn.moved && !cancelled) return doubleTap(x, y, at);
      pn.tx.add(pn.rx, at);
      pn.ty.add(pn.ry, at);
      const [vx, vy] = cancelled ? [0, 0] : [pn.tx.velocity(), pn.ty.velocity()];
      settle(zs.s, zs.x + project(vx, 0.995), zs.y + project(vy, 0.995), vx, vy);
      return;
    }
    const before = viewer.index;
    const r = viewer.up(ev, cancelled);
    if (r === "tap") doubleTap(x, y, at);
    else if (r === "swipe" && viewer.index !== before) resetZoom();
  };
  const vKey = (ev: KeyboardEvent) => {
    const back = isRtl() ? "ArrowRight" : "ArrowLeft";
    const fwd = isRtl() ? "ArrowLeft" : "ArrowRight";
    if (ev.key !== fwd && ev.key !== back) return;
    ev.preventDefault();
    resetZoom();
    viewer.step(ev.key === fwd ? 1 : -1);
  };

  // ── Photos that fail: a quiet placeholder in their slot, tried again when back online ──
  const mark = (img: HTMLImageElement) => img.closest(".pg__slide, .pg__thumb, .pv__slide")?.classList.add("is-broken");
  const onError = (ev: Event) => {
    if (ev.target instanceof HTMLImageElement) mark(ev.target);
  };
  for (const img of $$<HTMLImageElement>("img", root)) if (img.complete && img.naturalWidth === 0 && img.currentSrc) mark(img);

  const listen: Array<[EventTarget, string, EventListener, AddEventListenerOptions | boolean | undefined]> = [
    [vp, "pointerdown", stageDown as EventListener, undefined],
    [vp, "pointermove", stageMove as EventListener, undefined],
    [vp, "pointerup", stageUp as EventListener, undefined],
    [vp, "pointercancel", stageCancel as EventListener, undefined],
    [vp, "pointerleave", (() => !main.drag && unhover()) as EventListener, undefined],
    [vp, "keydown", stageKey as EventListener, undefined],
    [vp, "dragstart", ((e: Event) => e.preventDefault()) as EventListener, undefined],
    [vvp, "pointerdown", vDown as EventListener, undefined],
    [vvp, "pointermove", vMove as EventListener, undefined],
    [vvp, "pointerup", ((e: PointerEvent) => vUp(e, false)) as EventListener, undefined],
    [vvp, "pointercancel", ((e: PointerEvent) => vUp(e, true)) as EventListener, undefined],
    [vvp, "dragstart", ((e: Event) => e.preventDefault()) as EventListener, undefined],
    [dialog, "keydown", vKey as EventListener, undefined],
    [dialog, "close", onClosed as EventListener, undefined],
    [root, "error", onError, true],
  ];
  for (const [t0, type, fn, opts] of listen) t0.addEventListener(type, fn, opts);
  const offs = [
    on(root, "click", ".pg__thumb", (ev, a) => {
      if (!plainClick(ev)) return;
      ev.preventDefault();
      main.go(Number(a.dataset.i));
    }),
    on(root, "click", ".pg__nav", (_ev, b) => main.step(Number(b.dataset.step))),
    on(root, "click", "[data-open]", () => openViewer(main.index)),
    on(dialog, "click", "[data-close]", () => dialog.close()),
    onOnline(() => {
      for (const slot of $$(".is-broken", root)) {
        slot.classList.remove("is-broken");
        const img = $<HTMLImageElement>("img", slot);
        if (!img) continue;
        for (const attr of ["srcset", "src"]) {
          const v = img.getAttribute(attr);
          if (v) img.setAttribute(attr, v);
        }
      }
    }),
  ];
  const resize = () => {
    main.measure();
    if (dialog.open) viewer.measure();
  };
  addEventListener("resize", resize);
  // A variant whose photo differs (from the purchase panel on this page).
  const page = root.closest(".pdp") ?? document;
  const onVariant = (ev: Event) => {
    const i = (ev as CustomEvent<{ image: number }>).detail?.image;
    if (typeof i === "number" && i >= 0 && i < n && i !== main.index) main.go(i);
  };
  page.addEventListener(VARIANT_EVENT, onVariant);

  return () => {
    for (const [t0, type, fn, opts] of listen) t0.removeEventListener(type, fn, opts);
    for (const off of offs) off();
    removeEventListener("resize", resize);
    page.removeEventListener(VARIANT_EVENT, onVariant);
    main.x.stop();
    viewer.x.stop();
    for (const s of [sS, sX, sY]) s.stop();
    if (dialog.open) dialog.close();
    document.documentElement.classList.remove("pv-open");
  };
});
