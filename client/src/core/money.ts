// Money in the visitor's language and numerals, for the totals the client computes from piastres
// (the cart). An exact port of `crate::format::money` (crates/app/src/format.rs):
//
//   en  EGP 1,234.50      −EGP 50.00
//   ar  ⁦1,234.50⁩ ج.م     ⁦−50.00⁩ ج.م   (LRI…PDI keeps sign, digits and label together)
//
// Two decimals, thousands grouped, a true minus (U+2212); Arabic-Indic digits (٠١٢…, ٬ and ٫)
// when the visitor chose them. The label comes from the server (`cfg().cart.currency`).

import { cfg } from "./page";

const MINUS = "−";
const LRI = "⁦";
const PDI = "⁩";

/** ASCII digits and `,` `.` in the visitor's numerals (`format::digits`). */
function numerals(s: string): string {
  if (cfg().numerals !== "arab") return s;
  return s.replace(/[0-9,.]/g, (c) => (c === "," ? "٬" : c === "." ? "٫" : String.fromCharCode(0x0660 + Number(c))));
}

/** `1234567` → `1,234,567` (`format::group`). */
const group = (n: number): string => String(n).replace(/\B(?=(\d{3})+(?!\d))/g, ",");

/** Piastres → `EGP 1,234.50` / `⁦1,234.50⁩ ج.م`, exactly as the server writes it. */
export function money(piastres: number): string {
  const c = cfg();
  const p = Math.trunc(piastres);
  const sign = p < 0 ? MINUS : "";
  const abs = Math.abs(p);
  const whole = numerals(group(Math.floor(abs / 100)));
  const fraction = numerals("." + String(abs % 100).padStart(2, "0"));
  return c.lang === "ar" ? `${LRI}${sign}${whole}${fraction}${PDI} ${c.cart.currency}` : `${sign}${c.cart.currency} ${whole}${fraction}`;
}
