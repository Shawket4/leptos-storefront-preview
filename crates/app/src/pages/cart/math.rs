//! Optimistic cart arithmetic: what the server *will* answer, computed in the browser from numbers
//! the server already sent. Unit prices come from the server (priced by `madar-catalog`); the
//! client only multiplies and sums, so a stepper press shows the new subtotal instantly and the
//! server's response — which always replaces this — agrees to the piastre in the common case.

use crate::api::types::{CartLineView, CartView};
use domain::Piastres;

/// A cart mutation, as queued by the client (`store::CartOps`).
#[derive(Clone, Debug, PartialEq)]
pub enum Op {
    SetQty {
        key: String,
        qty: u32,
    },
    Remove {
        key: String,
    },
    /// `line` is known when the add restores a removed line (undo), so it can reappear in place.
    Add {
        product: u32,
        variant: u32,
        qty: u32,
        addons: Vec<String>,
        line: Option<Box<CartLineView>>,
    },
}

impl Op {
    pub fn key(&self) -> String {
        match self {
            Op::SetQty { key, .. } | Op::Remove { key } => key.clone(),
            Op::Add {
                product,
                variant,
                addons,
                line,
                ..
            } => line
                .as_ref()
                .map(|l| l.key.clone())
                .unwrap_or_else(|| line_key(*product, *variant, addons)),
        }
    }
}

/// Same shape as the server's `cart::line_key`: `product-variant` then `+addon` ids, sorted.
pub fn line_key(product: u32, variant: u32, addons: &[String]) -> String {
    let mut k = format!("{product}-{variant}");
    let mut a = addons.to_vec();
    a.sort();
    for x in a {
        k.push('+');
        k.push_str(&x);
    }
    k
}

/// `"1001-2+care1"` → `(1001, 2, ["care1"])` (to re-add a removed line with its add-ons).
pub fn parse_key(key: &str) -> Option<(u32, u32, Vec<String>)> {
    let mut parts = key.split('+');
    let (p, v) = parts.next()?.split_once('-')?;
    Some((
        p.parse().ok()?,
        v.parse().ok()?,
        parts.map(str::to_owned).collect(),
    ))
}

/// The VAT inside a tax-inclusive amount at 14%: `gross − round(gross / 1.14)`, rounding half away
/// from zero exactly like `madar_money::tax::compute` (integer form: no float, no Decimal).
pub fn vat_included(gross: Piastres) -> Piastres {
    let g = gross.0.max(0);
    let net = (200 * g + 114) / 228;
    Piastres(g - net)
}

/// Savings per unit the server would count for this line (exact unless add-ons change the base).
fn unit_saving(l: &CartLineView) -> i64 {
    l.compare_at
        .map(|c| (c.0 - l.unit_price.0).max(0))
        .unwrap_or(0)
}

/// Apply `pending` ops to the server's last answer and re-total. Returns the view to display and
/// the quantity added for products whose line isn't known yet (a quick add before the cart was
/// ever loaded) — the badge counts those, the line list can't show them until the server answers.
pub fn rebase(base: &CartView, pending: &[Op], fee: Piastres) -> (CartView, u32) {
    let mut v = base.clone();
    let mut unseen = 0u32;
    for op in pending {
        match op {
            Op::SetQty { key, qty } => {
                if *qty == 0 {
                    v.lines.retain(|l| &l.key != key);
                } else if let Some(l) = v.lines.iter_mut().find(|l| &l.key == key) {
                    l.qty = (*qty).min(l.max_qty.max(1));
                }
            }
            Op::Remove { key } => v.lines.retain(|l| &l.key != key),
            Op::Add { qty, line, .. } => {
                let key = op.key();
                if let Some(l) = v.lines.iter_mut().find(|l| l.key == key) {
                    l.qty = (l.qty + qty).min(l.max_qty.max(1));
                } else if let Some(line) = line {
                    let mut l = (**line).clone();
                    l.qty = (*qty).min(l.max_qty.max(1));
                    v.lines.push(l);
                } else {
                    unseen += qty;
                }
            }
        }
    }
    retotal(base, &mut v, fee);
    (v, unseen)
}

/// Recompute line totals and the summary from quantities. Savings move by the per-unit saving of
/// each changed line (so unchanged lines keep the server's exact figure).
fn retotal(base: &CartView, v: &mut CartView, fee: Piastres) {
    let mut savings = base.savings.0;
    for b in &base.lines {
        let now = v
            .lines
            .iter()
            .find(|l| l.key == b.key)
            .map(|l| l.qty)
            .unwrap_or(0);
        savings += unit_saving(b) * (now as i64 - b.qty as i64);
    }
    for l in v.lines.iter_mut() {
        l.line_total = l.unit_price * l.qty;
        if !base.lines.iter().any(|b| b.key == l.key) {
            savings += unit_saving(l) * l.qty as i64;
        }
    }
    v.subtotal = v.lines.iter().map(|l| l.line_total).sum();
    v.count = v.lines.iter().map(|l| l.qty).sum();
    v.savings = Piastres(savings.max(0));
    v.delivery_fee = if v.subtotal.0 == 0 || v.subtotal >= v.free_delivery_over {
        Piastres::ZERO
    } else {
        fee
    };
    v.tax_included = vat_included(v.subtotal);
    v.total = v.subtotal + v.delivery_fee;
}

/// Lines in the order the shopper first saw them (an undone removal returns to its place even
/// though the server appends it). Keys never seen before go last, in server order.
pub fn sort_lines(v: &mut CartView, order: &[String]) {
    let pos = |k: &str| order.iter().position(|o| o == k).unwrap_or(usize::MAX);
    v.lines.sort_by_key(|l| pos(&l.key));
}

/// How far the subtotal is towards free delivery, 0.0 ..= 1.0.
pub fn free_delivery_progress(v: &CartView) -> f32 {
    if v.free_delivery_over.0 <= 0 {
        return 1.0;
    }
    (v.subtotal.0 as f32 / v.free_delivery_over.0 as f32).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vat_matches_madar_rounding() {
        // 114.00 incl. 14% → 14.00 VAT; 1.00 → 0.12
        assert_eq!(vat_included(Piastres(11_400)), Piastres(1_400));
        assert_eq!(vat_included(Piastres(100)), Piastres(12));
    }

    #[test]
    fn keys_round_trip() {
        assert_eq!(
            parse_key(&line_key(1001, 2, &["b".into(), "a".into()])),
            Some((1001, 2, vec!["a".into(), "b".into()]))
        );
        assert_eq!(parse_key("1001-2"), Some((1001, 2, vec![])));
    }
}

/// Whether a line can still be sent as it stands (stock can move after an item is added).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stock {
    Ok,
    /// Nothing left.
    SoldOut,
    /// Fewer left than the line asks for.
    Only(u32),
}

pub fn stock(l: &CartLineView) -> Stock {
    if l.max_qty == 0 {
        Stock::SoldOut
    } else if l.qty > l.max_qty {
        Stock::Only(l.max_qty)
    } else {
        Stock::Ok
    }
}

/// Any line that can't be sent as it is (checkout waits until it's fixed).
pub fn stock_blocked(v: &CartView) -> bool {
    v.lines.iter().any(|l| stock(l) != Stock::Ok)
}

/// The shopper-facing explanation of a stock problem (MSA in Arabic, like other errors).
pub fn stock_message(
    s: Stock,
    lang: crate::prefs::Lang,
    num: crate::prefs::Numerals,
) -> Option<String> {
    use crate::prefs::Lang;
    match (s, lang) {
        (Stock::Ok, _) => None,
        (Stock::SoldOut, Lang::En) => {
            Some("Sold out since you added it. Remove it to check out.".into())
        }
        (Stock::SoldOut, Lang::Ar) => Some("نفدت الكمية بعد إضافته. احذفه لإتمام الطلب.".into()),
        (Stock::Only(n), Lang::En) => {
            Some(format!("Only {} left now.", crate::format::count(n, num)))
        }
        (Stock::Only(n), Lang::Ar) => {
            Some(format!("المتوفر الآن {} فقط.", crate::format::count(n, num)))
        }
    }
}
