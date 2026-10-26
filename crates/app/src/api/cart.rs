//! The cart's one data endpoint: `GET /api/cart/quote` — the server's pricing of the cart in the
//! request's `cart` cookie (prices, stock caps, totals), money formatted in the visitor's numerals.
//! The browser asks for it only when it needs those numbers (the drawer, /cart after a change);
//! adding, removing and changing quantities never call the server (`client/src/stores/cart.ts`).
//!
//! Shape: `CartQuote` in `client/src/api/types.ts`.

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
    pub fn quote(lines: &[CartLine], p: &Priced, lang: Lang, num: Numerals) -> CartQuote {
        let m = |x: Piastres| format::money(x, lang, num);
        let ar = lang.is_rtl();
        let v = &p.view;
        let out = lines
            .iter()
            .map(|cl| {
                let key = cl.key();
                match v.lines.iter().find(|l| l.key == key) {
                    Some(l) => CartQuoteLine {
                        key,
                        product: l.product.0,
                        variant: l.variant.0,
                        qty: l.qty,
                        name: l.name.pick(ar).to_owned(),
                        variant_label: Some(l.variant_label.pick(ar).to_owned()).filter(|s| !s.is_empty()),
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
            free_delivery_hint: (left > 0 && v.subtotal.0 > 0)
                .then(|| strings::get("cart.free_hint", lang).replace("{amount}", &m(Piastres(left)))),
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
    let priced = cart::price(&lines, &tenant, &st.catalog(&tenant.id), st.media());
    Ok(quote(&lines, &priced, rq.prefs.lang, rq.prefs.numerals))
}
