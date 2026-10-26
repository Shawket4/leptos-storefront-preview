// SegmentedToggle (components::segmented → `data-ts="seg-toggle"`): segments that are state, not
// pages. Pressing one sets `aria-pressed` (exactly one pressed, or any number with
// `data-multiple`) and fires a bubbling `change` on the group: `detail: { value, values }`.

export interface SegChange {
  /** The option just pressed. */
  value: string;
  /** Every pressed option now. */
  values: string[];
}

/** The pressed values of the toggle group `el` is in (or is). */
export function segValues(el: Element): string[] {
  const g = el.closest("[data-ts~=seg-toggle]");
  return g ? Array.from(g.querySelectorAll<HTMLButtonElement>('button[aria-pressed="true"]'), (b) => b.value) : [];
}

export function mountSegToggle(group: HTMLElement): () => void {
  const onClick = (ev: MouseEvent) => {
    const b = (ev.target as Element).closest<HTMLButtonElement>("button[aria-pressed]");
    if (!b || b.disabled || !group.contains(b)) return;
    if (group.hasAttribute("data-multiple")) b.setAttribute("aria-pressed", String(b.getAttribute("aria-pressed") !== "true"));
    else {
      if (b.getAttribute("aria-pressed") === "true") return; // already the one
      for (const o of Array.from(group.querySelectorAll('button[aria-pressed="true"]'))) o.setAttribute("aria-pressed", "false");
      b.setAttribute("aria-pressed", "true");
    }
    const detail: SegChange = { value: b.value, values: segValues(group) };
    group.dispatchEvent(new CustomEvent("change", { bubbles: true, detail }));
  };
  group.addEventListener("click", onClick);
  return () => group.removeEventListener("click", onClick);
}
