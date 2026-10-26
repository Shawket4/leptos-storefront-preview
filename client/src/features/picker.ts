// The variant picker and the add-on groups, written once for the product page
// (`features/product.ts`) and the cart's line editor (`features/line-editor.ts`). One markup too:
// `pages::product::picker` renders the axes, the groups, the page's JSON and the price templates
// for both. Everything here is state: switching variants reads the JSON (every variant's price,
// stock, SKU, image and availability, formatted by the server); add-ons and the quantity are form
// state; `line()` is the cart line the form describes, with its priced snapshot — no request.

import { $, $$, fill, on, patch, prefersReducedMotion } from "../core/dom";
import { t } from "../core/page";
import { setStepper, stepperValue } from "../core/stepper";
import type { LineSnapshot } from "../stores/cart";

/** The page's JSON (`pages::product::picker`, `PickerData`). */
export interface PickerData {
  product: number;
  /** The product page (the cart line's link: `?v=` is added). */
  href: string;
  /** A value's no-JS link, `{v}` standing for the variant id. */
  chipHref: string;
  name: string;
  selected: number;
  maxQty: number;
  /** Something to choose (variants, add-ons): the cart line offers "Edit". */
  editable: boolean;
  variants: VariantData[];
  /** Every add-on option, by id: name and surcharge in piastres. */
  addons: Record<string, { name: string; price: number }>;
  /** The add-on groups' rules (`ChoiceList` ids). */
  groups: GroupData[];
}
interface GroupData {
  id: string;
  min: number;
  max: number | null;
  problem: string;
}
export interface VariantData {
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

/** What a cart line is made of (`stores/cart.ts`'s add and replace take it). */
export interface LineItem {
  product: number;
  variant: number;
  qty: number;
  addons: string[];
}

export interface PickerOpts {
  /** The main button's words when the variant can be had (`product.add`, `cart.update_item`). */
  okLabel: string;
  /** A variant was picked (the product page: its address, the gallery). */
  onSelect?: (v: VariantData) => void;
  /** Anything changed: variant, an add-on, the quantity (the line editor's total). */
  onChange?: () => void;
}

export interface Picker {
  data: PickerData;
  current(): VariantData;
  /** The add-ons ticked now (sorted ids: the cart's order). */
  addons(): string[];
  qty(): number;
  /** The unit price with the ticked add-ons, piastres. */
  unit(): number;
  /**
   * The cart line the form describes, with its snapshot; `null` when it can't be had (sold out) or a
   * group isn't satisfied (then every unmet group is marked, the first scrolled to and focused).
   */
  line(): { item: LineItem; snap: LineSnapshot } | null;
  /** The main buttons' words back from a moment's "Added" (the product page). */
  resetLabels(): void;
  destroy(): void;
}

export const plainClick = (ev: MouseEvent) => ev.button === 0 && !ev.metaKey && !ev.ctrlKey && !ev.shiftKey && !ev.altKey;

/** The picker in `root` (its `data-ts-json`, axes, groups, stepper and main buttons). */
export function createPicker(root: HTMLElement, opts: PickerOpts): Picker | null {
  const json = $("script[data-ts-json]", root)?.textContent;
  if (!json) return null;
  const data = JSON.parse(json) as PickerData;
  const byId = new Map(data.variants.map((v) => [v.id, v]));
  const form = $<HTMLFormElement>("form.pp-form", root);
  const stepper = $("[data-ts~=stepper]", root);
  let cur = byId.get(data.selected) ?? data.variants[0]!;
  /** An add was tried with this variant: group problems show (and follow the answers live). */
  let attempted = false;

  const chipHref = (v: number) => data.chipHref.replace("{v}", String(v));
  const label = () => t(cur.stock === 0 ? "product.out" : opts.okLabel);

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
    const img = $<HTMLImageElement>("img[data-variant-img]", root);
    if (img && v.img && img.getAttribute("src") !== v.img) img.src = v.img;
    // Every axis (`ChoiceChips`): the picked value's name, each value's state — the look follows
    // aria-checked / aria-disabled — and why the others can't be picked (the group's note).
    for (const axis of $$(".choice-chips[data-axis]", root)) {
      const ai = Number(axis.dataset.axis);
      const noteText = v.notes[ai] ?? "";
      patch(axis, { value: v.labels[ai] ?? "", note: noteText || null });
      const noteId = `${axis.id}-note`;
      for (const a of $$<HTMLAnchorElement>("a[data-index]", axis)) {
        const st = v.values[ai]?.[Number(a.dataset.index)];
        if (!st) continue;
        const enabled = st.s === "on" || st.s === "open";
        a.setAttribute("aria-checked", String(st.s === "on"));
        a.tabIndex = st.s === "on" ? 0 : -1;
        if (enabled && st.v !== undefined) a.setAttribute("href", chipHref(st.v));
        else a.removeAttribute("href");
        const swatch = a.classList.contains("choice-chip--swatch");
        if (enabled) {
          a.removeAttribute("aria-disabled");
          a.removeAttribute("aria-describedby");
          if (swatch) a.title = a.getAttribute("aria-label") ?? "";
          else a.removeAttribute("title");
        } else {
          a.setAttribute("aria-disabled", "true");
          a.setAttribute("aria-describedby", noteId);
          a.title = st.why ?? "";
        }
      }
    }
    // The form posts what's on screen (no-JS twin), and the quantity follows the stock.
    const variantInput = form?.querySelector<HTMLInputElement>("input[name=variant]");
    if (variantInput) variantInput.value = String(v.id);
    const out = v.stock === 0;
    if (stepper) setStepper(stepper, { max: Math.max(1, Math.min(v.stock, data.maxQty)), disabled: out });
    for (const b of $$<HTMLButtonElement>("button[data-add]", root)) b.disabled = out;
    for (const l of $$("[data-add-label]", root)) l.textContent = label();
    const barPrice = $("[data-bar-price]", root);
    if (barPrice) barPrice.textContent = v.price;
    $(".pp-msgs", root)?.replaceChildren();
    // A new variant is a fresh start: no add-on message until the next try.
    attempted = false;
    validate();
    opts.onSelect?.(v);
    opts.onChange?.();
  };

  // ── Add-on groups: limits as you pick. A group's problem shows only once an add was tried
  // (like checkout): then on every unmet group at once, and from there it follows the answers
  // live. Changing the variant starts over. ──
  const groups = data.groups
    .map((g) => ({ ...g, el: document.getElementById(g.id) }))
    .filter((g): g is GroupData & { el: HTMLElement } => !!g.el && root.contains(g.el));
  const picked = (g: { el: HTMLElement }) => $$<HTMLInputElement>("input:checked", g.el).filter((i) => i.value !== "").length;
  const groupOk = (g: GroupData & { el: HTMLElement }) => {
    const n = picked(g);
    return n >= g.min && (g.max === null || n <= g.max);
  };
  /** `ChoiceList`'s invalid state: the danger edge, a red hint, the problem in its error line. */
  const markGroup = (g: GroupData & { el: HTMLElement }, bad: boolean) => {
    if (bad) g.el.setAttribute("aria-invalid", "true");
    else g.el.removeAttribute("aria-invalid");
    const line = document.getElementById(`${g.id}-error`);
    if (line) patch(line, { error: bad ? g.problem : "" });
  };
  /** A group with a maximum: once it's reached, the rest can't be ticked. */
  const limitGroup = (g: GroupData & { el: HTMLElement }) => {
    if (g.max === null) return;
    const full = picked(g) >= g.max;
    for (const box of $$<HTMLInputElement>("input[type=checkbox]", g.el)) box.disabled = full && !box.checked;
  };
  for (const g of groups) limitGroup(g);
  /** Mark the unmet groups (only after an attempt); returns them. */
  const validate = () => {
    const bad = groups.filter((g) => !groupOk(g));
    for (const g of groups) markGroup(g, attempted && bad.includes(g));
    return bad;
  };

  const addons = () =>
    $$<HTMLInputElement>(".pp-groups input:checked", root)
      .map((i) => i.value)
      .filter(Boolean)
      .sort();
  const qty = () => (stepper ? Math.max(1, stepperValue(stepper)) : 1);

  const line = (): ReturnType<Picker["line"]> => {
    if (cur.stock === 0) return null;
    attempted = true;
    const first = validate()[0]?.el;
    if (first) {
      first.scrollIntoView({ block: "center", behavior: prefersReducedMotion() ? "auto" : "smooth" });
      $<HTMLInputElement>("input:not(:disabled)", first)?.focus({ preventScroll: true });
      return null;
    }
    const ids = addons();
    // Priced: the cart computes the line from this, so the add needs no request at all.
    const snap: LineSnapshot = { name: data.name, href: `${data.href}?v=${cur.id}`, unit: cur.unit, maxQty: cur.snapMax, editable: data.editable };
    if (cur.img) snap.img = cur.img;
    if (cur.label) snap.variant = cur.label;
    if (cur.compareAt !== null) snap.compareAt = cur.compareAt;
    if (ids.length) {
      snap.addons = ids.map((id) => data.addons[id]?.name ?? id);
      snap.addonPrices = ids.map((id) => data.addons[id]?.price ?? 0);
    }
    attempted = false;
    return { item: { product: data.product, variant: cur.id, qty: qty(), addons: ids }, snap };
  };

  // Arrow keys move the choice to the next value that can be picked (radio semantics).
  const onKey = (ev: KeyboardEvent, a: HTMLElement) => {
    const rtl = document.documentElement.dir === "rtl";
    const fwd = { ArrowDown: true, ArrowUp: false, ArrowRight: !rtl, ArrowLeft: rtl }[ev.key];
    if (fwd === undefined) return;
    ev.preventDefault();
    const axis = a.closest<HTMLElement>(".choice-chips[data-axis]");
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

  for (const a of $$<HTMLAnchorElement>(".choice-chips a[data-index]", root))
    a.tabIndex = a.getAttribute("aria-checked") === "true" ? 0 : -1;

  const offs = [
    on(root, "click", "a[data-axis][data-index]", (ev, a) => {
      if (!plainClick(ev)) return;
      ev.preventDefault();
      const st = cur.values[Number(a.dataset.axis)]?.[Number(a.dataset.index)];
      const v = st?.s === "open" && st.v !== undefined ? byId.get(st.v) : undefined;
      if (v) select(v);
    }),
    on(root, "keydown", "a[data-axis][data-index]", onKey),
    on(root, "change", ".pp-groups input", (_ev, el) => {
      const g = groups.find((x) => x.el.contains(el));
      if (g) {
        limitGroup(g);
        validate(); // live, once an add was tried
      }
      opts.onChange?.();
    }),
    on(root, "change", "[data-ts~=stepper] input", () => opts.onChange?.()),
  ];

  return {
    data,
    current: () => cur,
    addons,
    qty,
    unit: () => cur.unit + addons().reduce((sum, id) => sum + (data.addons[id]?.price ?? 0), 0),
    line,
    resetLabels: () => {
      for (const l of $$("[data-add-label]", root)) l.textContent = label();
    },
    destroy: () => {
      for (const off of offs) off();
    },
  };
}
