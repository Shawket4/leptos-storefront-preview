// DOM helpers shared by every module. Features never query outside the root they're mounted on,
// except for the page-wide singletons (header, drawer, toaster) they look up by id.

export const $ = <T extends Element = HTMLElement>(sel: string, root: ParentNode = document): T | null =>
  root.querySelector<T>(sel);

export const $$ = <T extends Element = HTMLElement>(sel: string, root: ParentNode = document): T[] =>
  Array.from(root.querySelectorAll<T>(sel));

/** A delegated listener on `root` for events whose target is inside `sel`. Returns an unsubscribe. */
export function on<K extends keyof HTMLElementEventMap>(
  root: EventTarget,
  type: K,
  sel: string,
  fn: (ev: HTMLElementEventMap[K], el: HTMLElement) => void,
  opts?: AddEventListenerOptions,
): () => void {
  const handler = (ev: Event) => {
    const target = ev.target instanceof Element ? ev.target : null;
    const el = target?.closest<HTMLElement>(sel);
    if (el) fn(ev as HTMLElementEventMap[K], el);
  };
  root.addEventListener(type, handler, opts);
  return () => root.removeEventListener(type, handler, opts);
}

/**
 * Clone a server-rendered `<template id="tpl-{name}">` and fill it: every `[data-slot=key]` gets
 * `values[key]` as text (never HTML), and `[data-attr-{attr}=key]` sets that attribute. Markup and
 * its translated labels stay in Rust; TS only puts data in.
 */
export function fill(name: string, values: Record<string, string | number | null | undefined>): DocumentFragment {
  const tpl = document.getElementById(`tpl-${name}`);
  if (!(tpl instanceof HTMLTemplateElement)) throw new Error(`missing <template id="tpl-${name}">`);
  const frag = tpl.content.cloneNode(true) as DocumentFragment;
  patch(frag, values);
  for (const el of $$("*", frag)) {
    for (const a of Array.from(el.attributes)) {
      if (!a.name.startsWith("data-attr-")) continue;
      const v = values[a.value];
      if (v !== null && v !== undefined) el.setAttribute(a.name.slice("data-attr-".length), String(v));
      el.removeAttribute(a.name);
    }
  }
  return frag;
}

export const prefersReducedMotion = () => matchMedia("(prefers-reduced-motion: reduce)").matches;

/**
 * Update the `[data-slot]`s inside an already-rendered element in place (text only; `null` hides
 * the slot, a value shows it again). Keeps the element — and focus inside it — where it is.
 */
export function patch(root: ParentNode, values: Record<string, string | number | null | undefined>): void {
  for (const el of $$("[data-slot]", root)) {
    const k = el.dataset.slot ?? "";
    if (!(k in values)) continue;
    const v = values[k];
    el.hidden = v === null || v === undefined;
    if (!el.hidden && el.textContent !== String(v)) el.textContent = String(v);
  }
}
