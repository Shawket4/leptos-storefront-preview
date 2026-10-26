// The cart's line editor: the form (`data-ts="line-editor"`, pages::cart::edit) and the sheet it
// opens in over the cart (`openEditor`, from a cart line's "Edit" in features/cart-view.ts).
//
// The form is the product's own picker (features/picker.ts) set to the line's choices, the line's
// new total on the button, and "Update item", which changes the line in the cart store (`replace`:
// no request). In the sheet it closes the sheet; as the /cart/edit page it goes on to the cart.
// Either way the toast says so, with Undo.
//
// The sheet (`pages::cart::LineEditorSheet`, in every page's shell) opens at once with its
// skeleton, and the /cart/edit page's editor goes in it, fetched through the router's loader (which
// a finger-down on the link has already started).

import { $, patch } from "../core/dom";
import { closeDrawer, openDrawer } from "../core/drawer";
import { mountAll, register, unmountAll } from "../core/mount";
import { money } from "../core/money";
import { NetError, isOnline } from "../core/net";
import { t } from "../core/page";
import { fetchDocument, navigate } from "../core/router";
import { setStepper } from "../core/stepper";
import { cart, lineKey, replace } from "../stores/cart";
import { toast } from "../stores/toasts";
import { createPicker } from "./picker";

const SHEET = "line-editor";

interface Done {
  /** The line's key now (another variant or add-ons make another key). */
  key: string;
  name: string;
  /** Anything changed (else there's nothing to say). */
  changed: boolean;
  /** The line wasn't in the cart any more. */
  gone?: boolean;
  undo: () => void;
}

function say(d: Done): void {
  if (d.gone) toast(t("cart.err.line_gone"), "error");
  else if (d.changed) toast(t("cart.updated_item", { name: d.name }), "success", { label: t("cart.undo"), run: d.undo });
}

// ── The form ──────────────────────────────────────────────────────────────────────────────────
register("line-editor", (root) => {
  const key = root.dataset.key ?? "";
  // The button's figure: the line's new total, as the choices change.
  let total = () => {};
  const picker = createPicker(root, { okLabel: "cart.update_item", onChange: () => total() });
  if (!picker) return;
  total = () => patch(root, { total: picker.current().stock === 0 ? null : money(picker.unit() * picker.qty()) });
  const form = $<HTMLFormElement>("form", root);
  // The cart is the browser's: the line's quantity now, not the one the page was rendered with —
  // unless the shopper changed it before this code arrived (that's what they meant).
  const line = cart.get().lines.find((l) => lineKey(l) === key);
  const stepper = $("[data-ts~=stepper]", root);
  const input = stepper && $<HTMLInputElement>("input", stepper);
  if (line && stepper && input && input.value === input.defaultValue) setStepper(stepper, { value: line.qty });
  total();

  const submit = (ev: SubmitEvent) => {
    ev.preventDefault();
    const l = picker.line();
    if (!l) return;
    const r = replace(key, l.item, l.snap);
    const d: Done = r.ok
      ? { key: r.key, name: l.snap.name, changed: r.changed, undo: r.undo }
      : { key, name: l.snap.name, changed: false, gone: true, undo: () => {} };
    if (root.closest("dialog")) {
      done = d; // said once the sheet is down
      closeDrawer(SHEET);
    } else {
      say(d);
      void navigate("/cart", { replace: true }); // the edit page isn't kept in history
    }
  };
  form?.addEventListener("submit", submit);
  return () => {
    picker.destroy();
    form?.removeEventListener("submit", submit);
  };
});

// ── The sheet ─────────────────────────────────────────────────────────────────────────────────
let done: Done | null = null;
let seq = 0;
let retry = "";
const bound = new WeakSet<HTMLElement>();

/** Open the line editor at `href` (`/cart/edit?key=…`) in the sheet; its page if there is none. */
export async function openEditor(href: string): Promise<void> {
  const sheet = document.getElementById(SHEET);
  const slot = sheet && $("[data-le-slot]", sheet);
  const skeleton = slot && $("[data-le-skeleton]", slot);
  const err = sheet && $("[data-le-error]", sheet);
  if (!sheet || !slot || !skeleton || !err) {
    void navigate(href);
    return;
  }
  /** Back to the skeleton (the editor that was there goes). */
  const reset = () => {
    const old = $("[data-ts~=line-editor]", slot);
    if (old) {
      unmountAll(old);
      old.remove();
    }
    skeleton.hidden = slot.hidden = false;
    err.hidden = true;
    slot.setAttribute("aria-busy", "true");
  };
  if (!bound.has(sheet)) {
    bound.add(sheet);
    sheet.addEventListener("drawer:close", (ev) => {
      if (ev.target !== sheet) return;
      seq++; // a load still under way is no longer wanted
      reset();
      const d = done;
      done = null;
      if (!d) return;
      say(d);
      // The line was redrawn under its new key: focus goes to it, not to the page behind.
      const a = document.activeElement;
      if (!a || a === document.body || !a.isConnected) {
        const li = document.querySelector<HTMLElement>(`li[data-key="${CSS.escape(d.key)}"]`);
        ($<HTMLElement>("a[data-cart-edit]:not([hidden])", li ?? document) ?? li)?.focus({ preventScroll: true });
      }
    });
    err.addEventListener("click", (ev) => {
      if ((ev.target as Element).closest("[data-le-retry]")) void openEditor(retry);
    });
  }
  const mine = ++seq;
  reset();
  openDrawer(SHEET);
  try {
    const doc = await fetchDocument(href);
    if (mine !== seq) return;
    if (!doc) {
      location.assign(href); // a new deploy: its page, not its markup in this code
      return;
    }
    const editor = doc.querySelector<HTMLElement>("[data-ts~=line-editor]");
    if (!editor) {
      // Not in the cart any more (another tab): the page said so.
      done = { key: "", name: "", changed: false, gone: true, undo: () => {} };
      closeDrawer(SHEET);
      return;
    }
    skeleton.hidden = true;
    slot.append(document.adoptNode(editor));
    slot.setAttribute("aria-busy", "false");
    mountAll(slot);
  } catch (e) {
    if (mine !== seq) return;
    const offline = !isOnline() || (e instanceof NetError && e.code === "offline");
    patch(err, { title: t(offline ? "cart.edit_offline" : "cart.edit_failed") });
    slot.hidden = true;
    err.hidden = false;
    retry = href;
  }
}
