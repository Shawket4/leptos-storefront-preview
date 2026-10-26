//! The cart's data for the browser. The browser computes the cart itself from each line's
//! [`Snapshot`] (taken from the card or product page it was added from); it calls
//! `GET /api/cart/quote` only for lines that have none (added without JS, storage cleared) —
//! each quote line carries the snapshot to keep.
//!
//! Shapes: `CartQuote` in `client/src/api/types.ts`, `LineSnapshot` in `client/src/stores/cart.ts`.

use leptos::prelude::*;
use leptos::server_fn::codec::GetUrl;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CartQuote {
    pub lines: Vec<CartQuoteLine>,
    pub count: u32,
    pub subtotal: String,
    pub savings: Option<String>,
    pub delivery: String,
    pub delivery_free: bool,
    /// "Add EGP 120.00 more for free delivery."; `None` once it's free.
    pub free_delivery_hint: Option<String>,
    pub total: String,
    pub vat_included: String,
    pub can_checkout: bool,
}

/// What the browser keeps for a cart line — `LineSnapshot` in `client/src/stores/cart.ts` — to
/// show and price it without asking: display data and prices in piastres.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub img: Option<String>,
    pub href: String,
    /// The variant's label, e.g. "256 GB".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
    /// Add-on names, in the line's add-on order (same order as `addon_prices`).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub addons: Vec<String>,
    /// The variant's unit price, without add-ons.
    pub unit: i64,
    /// The variant's "was" price (only when higher than `unit`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compare_at: Option<i64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub addon_prices: Vec<i64>,
    /// The most that can be ordered now (stock, at most the cart's limit).
    pub max_qty: u32,
    /// The product has something to choose (variants, add-ons): the line offers "Edit".
    #[serde(default)]
    pub editable: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CartQuoteLine {
    /// Same as the client's `lineKey` (`domain::cart::CartLine::key`).
    pub key: String,
    pub product: u32,
    pub variant: u32,
    pub qty: u32,
    pub name: String,
    pub variant_label: Option<String>,
    pub addons: Vec<String>,
    pub href: String,
    pub img: Option<String>,
    pub unit_price: String,
    pub compare_at: Option<String>,
    pub line_total: String,
    /// Most the shopper can have now (stock); 0 = unavailable.
    pub max_qty: u32,
    pub issue: Option<LineIssue>,
    /// What the browser keeps for this line (`None`: unavailable).
    pub snapshot: Option<Snapshot>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LineIssue {
    OutOfStock,
    Capped,
    Unavailable,
}

#[cfg(feature = "ssr")]
pub use imp::quote;

#[cfg(feature = "ssr")]
mod imp {
    use super::*;
    use crate::format;
    use crate::prefs::{Lang, Numerals};
    use crate::server::cart::{Issue, Priced, issue};
    use crate::strings;
    use domain::Piastres;
    use domain::cart::CartLine;

    /// The quote for `lines` (cookie order; lines the catalogue lost come back `unavailable`).
    /// `snaps` are the lines' snapshots (`server::cart::snapshot`), in the same order.
    pub fn quote(
        lines: &[CartLine],
        snaps: &[Option<Snapshot>],
        p: &Priced,
        lang: Lang,
        num: Numerals,
    ) -> CartQuote {
        let m = |x: Piastres| format::money(x, lang, num);
        let ar = lang.is_rtl();
        let v = &p.view;
        let out = lines
            .iter()
            .zip(snaps.iter().cloned().chain(std::iter::repeat(None)))
            .map(|(cl, snapshot)| {
                let key = cl.key();
                match v.lines.iter().find(|l| l.key == key) {
                    Some(l) => CartQuoteLine {
                        key,
                        product: l.product.0,
                        variant: l.variant.0,
                        qty: l.qty,
                        name: l.name.pick(ar).to_owned(),
                        variant_label: Some(l.variant_label.pick(ar).to_owned())
                            .filter(|s| !s.is_empty()),
                        addons: l.addons.iter().map(|a| a.pick(ar).to_owned()).collect(),
                        href: format!("/p/{}", l.slug),
                        img: (!l.img.is_missing()).then(|| l.img.src_near(192)),
                        unit_price: m(l.unit_price),
                        compare_at: l.compare_at.map(m),
                        line_total: m(l.line_total),
                        max_qty: l.max_qty,
                        issue: issue(l).map(|i| match i {
                            Issue::OutOfStock => LineIssue::OutOfStock,
                            Issue::Capped(_) => LineIssue::Capped,
                        }),
                        snapshot,
                    },
                    None => CartQuoteLine {
                        key,
                        product: cl.product.0,
                        variant: cl.variant.0,
                        qty: cl.qty,
                        issue: Some(LineIssue::Unavailable),
                        ..Default::default()
                    },
                }
            })
            .collect();
        let left = (v.free_delivery_over.0 - v.subtotal.0).max(0);
        CartQuote {
            lines: out,
            count: lines.iter().map(|l| l.qty).sum(),
            subtotal: m(v.subtotal),
            savings: (v.savings.0 > 0).then(|| m(v.savings)),
            delivery: m(v.delivery_fee),
            delivery_free: v.delivery_fee.0 == 0,
            free_delivery_hint: (left > 0 && v.subtotal.0 > 0).then(|| {
                strings::get("cart.free_hint", lang).replace("{amount}", &m(Piastres(left)))
            }),
            total: m(v.total),
            vat_included: m(v.tax_included),
            can_checkout: !lines.is_empty() && !p.blocked(),
        }
    }
}

/// `GET /api/cart/quote`: the cart in the `cart` cookie, priced now. Personal: never cached.
#[server(endpoint = "cart/quote", input = GetUrl)]
pub async fn get_quote() -> Result<CartQuote, ServerFnError> {
    use crate::server::{cart, sim};
    // Everything from the request first: after the `.await` the client may have gone.
    let rq = crate::request::init();
    crate::server::cache::private();
    let lines = cart::from_request();
    sim::check_fail(sim::Widget::Cart)?;
    sim::delay(sim::Widget::Cart).await;
    let st = crate::server::state();
    let tenant = st.tenant(&rq.tenant.id).clone();
    let catalog = st.catalog(&tenant.id);
    let priced = cart::price(&lines, &tenant, &catalog, st.media());
    let snaps: Vec<_> = lines
        .iter()
        .map(|l| cart::snapshot(l, &catalog, st.media(), rq.prefs.lang))
        .collect();
    Ok(quote(
        &lines,
        &snaps,
        &priced,
        rq.prefs.lang,
        rq.prefs.numerals,
    ))
}
