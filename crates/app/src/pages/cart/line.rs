//! The cart line and the cart summary, written once with the design system: a line is an
//! `ItemRow` (its slots are what the browser code fills), the summary's figures are `Totals`. The
//! same components render the drawer's and /cart's `<template>` (empty; `features/cart-view.ts`
//! fills it) and /cart's lines on the server (filled), so the page looks the same before and after
//! the browser code takes over. Every control is a real form (`/cart/set`, `/cart/remove`) or link
//! (Edit: `/cart/edit`), so /cart works without JS; with JS the TS intercepts them and nothing is
//! sent (Edit opens the editor's sheet).

use super::tx;
use crate::api::types::CartLineView;
use crate::components::button::{Button, ButtonKind, ButtonSize};
use crate::components::icons::Icon;
use crate::components::item_row::ItemRow;
use crate::components::notice::{Notice, Tone};
use crate::components::stepper::Stepper;
use crate::components::totals::{TotalRow, TotalTone, Totals};
use crate::format;
use crate::prefs::{Lang, Numerals};
use crate::strings;
use leptos::prelude::*;

/// What one line shows (formatted). `Default` is the empty template.
#[derive(Clone, Debug, Default)]
pub struct LineData {
    pub key: String,
    pub qty: u32,
    /// The stepper's upper bound (stock, or the cart's limit before the server has said).
    pub max: u32,
    pub name: String,
    /// Variant and add-ons, e.g. "256 GB · Gift wrap".
    pub detail: Option<String>,
    /// `None` for a line the catalogue lost.
    pub href: Option<String>,
    pub img: Option<String>,
    /// The line's note: its problem, else "2 × EGP 59.99 · Only 3 in stock".
    pub meta: Option<String>,
    /// The line total.
    pub price: Option<String>,
    /// `out_of_stock` / `capped` / `unavailable`.
    pub issue_code: Option<&'static str>,
    /// "Change to 3" (a capped line's fix) and the quantity it sets.
    pub fix: Option<(String, u32)>,
    /// The line editor (`/cart/edit?key=`), for a product with something to choose.
    pub edit_href: Option<String>,
}

pub fn percent(n: u32, num: Numerals) -> String {
    let sign = if num == Numerals::Arab {
        '\u{066A}'
    } else {
        '%'
    };
    format!("{}{sign}", format::count(n, num))
}

impl LineData {
    /// A priced line (/cart on the server). Same words as `features/cart-view.ts` writes.
    pub fn from_view(l: &CartLineView, lang: Lang, num: Numerals) -> LineData {
        let ar = lang.is_rtl();
        let n = |x: u32| format::count(x, num);
        let m = |p| format::money(p, lang, num);
        let mut detail: Vec<String> = Vec::new();
        if !l.variant_label.pick(ar).is_empty() {
            detail.push(l.variant_label.pick(ar).to_owned());
        }
        detail.extend(l.addons.iter().map(|a| a.pick(ar).to_owned()));
        let (issue_code, issue, fix) = match crate::server::cart::issue(l) {
            Some(crate::server::cart::Issue::OutOfStock) => (
                Some("out_of_stock"),
                Some(strings::get("cart.issue.out_of_stock", lang).to_owned()),
                None,
            ),
            Some(crate::server::cart::Issue::Capped(c)) => (
                Some("capped"),
                Some(tx("cart.issue.capped", lang, &[("n", &n(c))])),
                Some((tx("cart.fix", lang, &[("n", &n(c))]), c)),
            ),
            None => (None, None, None),
        };
        let note = {
            let mut bits = Vec::new();
            if l.qty > 1 {
                bits.push(tx(
                    "cart.breakdown",
                    lang,
                    &[("qty", &n(l.qty)), ("unit", &m(l.unit_price))],
                ));
            }
            if l.max_qty <= 5 {
                bits.push(tx("cart.low_stock", lang, &[("n", &n(l.max_qty))]));
            }
            (!bits.is_empty()).then(|| bits.join(" · "))
        };
        LineData {
            key: l.key.clone(),
            qty: l.qty,
            max: l.max_qty.max(l.qty).max(1),
            name: l.name.pick(ar).to_owned(),
            detail: (!detail.is_empty()).then(|| detail.join(" · ")),
            href: Some(format!("/p/{}", l.slug)),
            img: (!l.img.is_missing()).then(|| l.img.src_near(192)),
            meta: issue.or(note),
            price: Some(m(l.line_total)),
            issue_code,
            fix,
            edit_href: l.editable.then(|| super::edit_href(&l.key)),
        }
    }

    /// A line the catalogue no longer has (it can only be removed).
    pub fn unavailable(l: &domain::cart::CartLine, lang: Lang) -> LineData {
        LineData {
            key: l.key(),
            qty: l.qty,
            max: l.qty.max(1),
            name: strings::get("cart.unavailable_name", lang).to_owned(),
            meta: Some(strings::get("cart.issue.unavailable", lang).to_owned()),
            issue_code: Some("unavailable"),
            ..Default::default()
        }
    }
}

/// One line: an `ItemRow` with the line's controls. `next` is where the no-JS forms come back to.
#[component]
pub fn LineRow(d: LineData, lang: Lang, next: &'static str) -> impl IntoView {
    let template = d.key.is_empty();
    let rm_label = if template {
        String::new()
    } else {
        tx("cart.remove_item", lang, &[("name", &d.name)])
    };
    let qty_label = (!template).then(|| tx("cart.qty_of", lang, &[("name", &d.name)]));
    let edit_label = (!template).then(|| tx("cart.edit_item", lang, &[("name", &d.name)]));
    let edit_href = d.edit_href.clone();
    let (fix_label, fix_qty) = d
        .fix
        .clone()
        .map(|(l, q)| (Some(l), q))
        .unwrap_or((None, 0));
    let key = d.key.clone();
    let controls = move || {
        let key = key.clone();
        view! {
            <form class="cline__qty" method="post" action="/cart/set" data-cart-form="set">
                <input type="hidden" name="key" value=key.clone() data-attr-value="key"/>
                <input type="hidden" name="next" value=next/>
                <Stepper name="qty" value=d.qty.max(1) min=1 max=d.max.max(1) removable=true label=qty_label.clone()/>
                // No-JS only: with scripting on, every change is instant (cart.css hides it).
                <Button button_type="submit" kind=ButtonKind::Secondary size=ButtonSize::Sm class="cline__update">
                    {strings::get("cart.update", lang)}
                </Button>
            </form>
            // The line editor: a page without JS; with JS the sheet over the cart (cart-view.ts),
            // so it doesn't close the drawer (`data-drawer-keep`).
            <a class="link-btn cline__edit" href=edit_href.clone() data-attr-href="edit_href" data-cart-edit=""
                data-drawer-keep="" aria-label=edit_label.clone() data-slot="edit" hidden=edit_href.is_none()>
                {strings::get("cart.edit", lang)}
            </a>
            <form class="cline__remove" method="post" action="/cart/remove" data-cart-form="remove">
                <input type="hidden" name="key" value=key.clone() data-attr-value="key"/>
                <input type="hidden" name="next" value=next/>
                <button type="submit" class="link-btn" aria-label=rm_label.clone() data-attr-aria-label="rm_label">
                    {strings::get("cart.remove", lang)}
                </button>
            </form>
            <form class="cline__fix" method="post" action="/cart/set" data-cart-form="set">
                <input type="hidden" name="key" value=key data-attr-value="key"/>
                <input type="hidden" name="qty" value=fix_qty.to_string() data-fix-qty=""/>
                <input type="hidden" name="next" value=next/>
                <button type="submit" class="link-btn" data-slot="fix" hidden=fix_label.is_none()>{fix_label.clone()}</button>
            </form>
        }
    };
    let (detail, meta, price) = (
        d.detail.unwrap_or_default(),
        d.meta.unwrap_or_default(),
        d.price.unwrap_or_default(),
    );
    let issue = d.issue_code.unwrap_or("");
    // A lost line has no picture or page to link (ItemRow takes them as plain values).
    match (d.img, d.href) {
        (Some(img), Some(href)) => view! {
            <ItemRow name=d.name img href detail meta price slots=true class="cline"
                attr:data-key=d.key attr:data-issue=issue>
                {controls()}
            </ItemRow>
        }
        .into_any(),
        (None, Some(href)) => view! {
            <ItemRow name=d.name href detail meta price slots=true class="cline"
                attr:data-key=d.key attr:data-issue=issue>
                {controls()}
            </ItemRow>
        }
        .into_any(),
        _ => view! {
            <ItemRow name=d.name detail meta price slots=true class="cline"
                attr:data-key=d.key attr:data-issue=issue>
                {controls()}
            </ItemRow>
        }
        .into_any(),
    }
}

/// The cart's totals, formatted. `None` renders the empty slots the client fills.
#[derive(Clone, Debug, Default)]
pub struct SummaryData {
    pub subtotal: String,
    pub savings: Option<String>,
    pub delivery: String,
    pub delivery_free: bool,
    pub hint: Option<String>,
    pub total: String,
    pub can_checkout: bool,
    pub empty: bool,
}

/// Free-delivery hint, totals, "VAT included" and the way on. One markup for the drawer's footer
/// and /cart; the browser code fills the `Totals` slots (subtotal, savings, delivery/free, total).
#[component]
pub fn CartSummary(
    d: SummaryData,
    lang: Lang,
    num: Numerals,
    #[prop(optional)] in_drawer: bool,
) -> impl IntoView {
    let g = move |k: &str| strings::get(k, lang);
    // "VAT included" as text: the exact figure is checkout's (the cart is computed in the browser).
    let vat = tx("cart.vat_included", lang, &[("pct", &percent(14, num))]);
    let done = !d.empty && d.hint.is_none();
    let blocked = !d.empty && !d.can_checkout;
    let rows = vec![
        TotalRow::new(g("cart.subtotal"), d.subtotal.clone()).slot("subtotal"),
        TotalRow::new(g("cart.savings"), d.savings.clone().unwrap_or_default())
            .tone(TotalTone::Saving)
            .slot("savings")
            .hidden(d.savings.is_none()),
        TotalRow::new(g("cart.delivery"), d.delivery.clone())
            .slot("delivery")
            .hidden(d.delivery_free),
        TotalRow::new(g("cart.delivery"), g("cart.free"))
            .tone(TotalTone::Free)
            .slot("free")
            .hidden(!d.delivery_free),
        TotalRow::new(g("cart.total"), d.total.clone())
            .tone(TotalTone::Total)
            .slot("total"),
    ];
    view! {
        <div class="csum" data-cart-summary="" aria-busy="false">
            <p class="ship">
                <Icon name="truck" size=18/>
                <span data-slot="hint" hidden=d.hint.is_none()>{d.hint.clone()}</span>
                <span class="ship__done" data-slot="ship_done" hidden=!done>{g("cart.free_done")}</span>
            </p>
            <Totals rows note=vat/>
            <Notice tone=Tone::Danger title=g("cart.quote_failed") slots=true attr:data-quote-error="" attr:hidden=true>
                <button type="button" class="link-btn" data-quote-retry="">{g("cart.retry")}</button>
            </Notice>
            <div class="csum__actions">
                <p class="csum__blocked" data-slot="blocked" hidden=!blocked>{g("cart.blocked")}</p>
                // Blocked is toggled by the browser code as the cart changes, so the link keeps its
                // href (Button's `disabled` would drop it).
                <Button href="/checkout" kind=ButtonKind::Primary size=ButtonSize::Lg full_width=true
                    icon_end="arrow-right" attr:data-checkout="" attr:aria-disabled=blocked.to_string()>
                    {g("cart.checkout")}
                </Button>
                {if in_drawer {
                    view! { <Button href="/cart" kind=ButtonKind::Ghost full_width=true>{g("cart.view")}</Button> }.into_any()
                } else {
                    view! { <Button href="/" kind=ButtonKind::Ghost full_width=true>{g("cart.continue")}</Button> }.into_any()
                }}
            </div>
        </div>
    }
}

/// `next` for the forms on /cart.
pub const CART_PATH: &str = "/cart";
