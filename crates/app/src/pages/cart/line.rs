//! The markup of a cart line and of the cart summary, written once. The same components render
//! the drawer's and /cart's `<template>`s (empty; `features/cart.ts` fills them through `fill()` /
//! `patch()`) and /cart's lines on the server (filled), so the page looks the same before and
//! after the browser code takes over. Every control is a real form (`/cart/set`, `/cart/remove`),
//! so /cart works without JS; with JS the TS intercepts them and nothing is sent.

use super::tx;
use crate::api::types::CartLineView;
use crate::components::icons::Icon;
use crate::components::stepper::Stepper;
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
    pub href: String,
    pub img: Option<String>,
    pub unit: Option<String>,
    pub was: Option<String>,
    pub total: Option<String>,
    /// "2 × EGP 1,899.00".
    pub breakdown: Option<String>,
    /// "Only 3 in stock".
    pub low: Option<String>,
    /// `out_of_stock` / `capped` / `unavailable`.
    pub issue_code: Option<&'static str>,
    pub issue: Option<String>,
    /// "Change to 3" (a capped line's fix) and the quantity it sets.
    pub fix: Option<(String, u32)>,
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
    /// A priced line (/cart on the server).
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
        LineData {
            key: l.key.clone(),
            qty: l.qty,
            max: l.max_qty.max(l.qty).max(1),
            name: l.name.pick(ar).to_owned(),
            detail: (!detail.is_empty()).then(|| detail.join(" · ")),
            href: format!("/p/{}", l.slug),
            img: (!l.img.is_missing()).then(|| l.img.src_near(192)),
            unit: Some(m(l.unit_price)),
            was: l.compare_at.map(m),
            total: Some(m(l.line_total)),
            breakdown: (l.qty > 1).then(|| {
                tx(
                    "cart.breakdown",
                    lang,
                    &[("qty", &n(l.qty)), ("unit", &m(l.unit_price))],
                )
            }),
            low: (issue_code.is_none() && l.max_qty <= 5)
                .then(|| tx("cart.low_stock", lang, &[("n", &n(l.max_qty))])),
            issue_code,
            issue,
            fix,
        }
    }

    /// A line the catalogue no longer has (it can only be removed).
    pub fn unavailable(l: &domain::cart::CartLine, lang: Lang) -> LineData {
        LineData {
            key: l.key(),
            qty: l.qty,
            max: l.qty.max(1),
            name: strings::get("cart.unavailable_name", lang).to_owned(),
            issue_code: Some("unavailable"),
            issue: Some(strings::get("cart.issue.unavailable", lang).to_owned()),
            ..Default::default()
        }
    }
}

/// One line. `next` is where the no-JS forms come back to.
#[component]
pub fn LineRow(d: LineData, lang: Lang, next: &'static str) -> impl IntoView {
    let template = d.key.is_empty();
    let rm_label = if template {
        String::new()
    } else {
        tx("cart.remove_item", lang, &[("name", &d.name)])
    };
    let qty_label = (!template).then(|| tx("cart.qty_of", lang, &[("name", &d.name)]));
    let (fix_label, fix_qty) = d
        .fix
        .clone()
        .map(|(l, q)| (Some(l), q))
        .unwrap_or((None, 0));
    let key = d.key.clone();
    view! {
        <li class="cline" data-key=d.key.clone() data-issue=d.issue_code.unwrap_or("")>
            <a class="cline__media" href=d.href.clone() data-attr-href="href" tabindex="-1" aria-hidden="true">
                <img class="cline__img" src=d.img.clone() data-attr-src="img" alt="" width="96" height="96"
                    loading="lazy" decoding="async"/>
            </a>
            <div class="cline__main">
                <a class="cline__name" href=d.href.clone() data-attr-href="href" data-slot="name">{d.name.clone()}</a>
                <p class="cline__detail" data-slot="detail" hidden=d.detail.is_none()>{d.detail.clone()}</p>
                <p class="cline__unit">
                    <span class="cline__price mono" data-slot="unit">{d.unit.clone()}</span>
                    <s class="cline__was mono" data-slot="was" hidden=d.was.is_none()>{d.was.clone()}</s>
                </p>
                <p class="cline__stock" data-slot="low" hidden=d.low.is_none()>{d.low.clone()}</p>
                <form class="cline__remove" method="post" action="/cart/remove" data-cart-form="remove">
                    <input type="hidden" name="key" value=key.clone() data-attr-value="key"/>
                    <input type="hidden" name="next" value=next/>
                    <button type="submit" class="link-btn" aria-label=rm_label data-attr-aria-label="rm_label">
                        {strings::get("cart.remove", lang)}
                    </button>
                </form>
            </div>
            <div class="cline__side">
                <form class="cline__qty" method="post" action="/cart/set" data-cart-form="set">
                    <input type="hidden" name="key" value=key.clone() data-attr-value="key"/>
                    <input type="hidden" name="next" value=next/>
                    <Stepper name="qty" value=d.qty.max(1) min=1 max=d.max.max(1) removable=true label=qty_label/>
                    <button type="submit" class="link-btn cline__update">{strings::get("cart.update", lang)}</button>
                </form>
                <p class="cline__total">
                    <span class="mono" data-slot="total">{d.total.clone()}</span>
                    <span class="cline__breakdown" data-slot="breakdown" hidden=d.breakdown.is_none()>{d.breakdown.clone()}</span>
                </p>
            </div>
            <div class="line-issue cline__issue" role="status">
                <Icon name="triangle-alert" size=16/>
                <span data-slot="issue" hidden=d.issue.is_none()>{d.issue.clone()}</span>
                <form class="line-issue__fix" method="post" action="/cart/set" data-cart-form="set">
                    <input type="hidden" name="key" value=key data-attr-value="key"/>
                    <input type="hidden" name="qty" value=fix_qty.to_string() data-fix-qty=""/>
                    <input type="hidden" name="next" value=next/>
                    <button type="submit" class="link-btn" data-slot="fix" hidden=fix_label.is_none()>{fix_label.clone()}</button>
                </form>
            </div>
        </li>
    }
}

/// The "Removed … · Undo" row a removed line leaves for a few seconds (client only).
#[component]
pub fn GoneTemplate(lang: Lang) -> impl IntoView {
    view! {
        <template id="tpl-cart-gone">
            <li class="cline cline--gone">
                <p class="cline__gone-text" data-slot="text"></p>
                <button type="button" class="link-btn cline__undo" data-undo="">
                    <Icon name="rotate-ccw" size=16/>
                    <span>{strings::get("cart.undo", lang)}</span>
                </button>
            </li>
        </template>
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
    pub vat: String,
    pub can_checkout: bool,
    pub empty: bool,
}

/// Free-delivery hint, totals, VAT and the way on. One markup for the drawer's footer and /cart.
#[component]
pub fn CartSummary(
    d: SummaryData,
    lang: Lang,
    num: Numerals,
    #[prop(optional)] in_drawer: bool,
) -> impl IntoView {
    let g = |k: &str| strings::get(k, lang);
    let vat = tx("cart.vat", lang, &[("pct", &percent(14, num))]);
    let (vat_a, vat_b) = vat
        .split_once("{vat}")
        .map(|(a, b)| (a.to_owned(), b.to_owned()))
        .unwrap_or((vat.clone(), String::new()));
    let done = !d.empty && d.hint.is_none();
    let blocked = !d.empty && !d.can_checkout;
    view! {
        <div class="csum" data-cart-summary="" aria-busy="false">
            <p class="ship">
                <Icon name="truck" size=18/>
                <span data-slot="hint" hidden=d.hint.is_none()>{d.hint.clone()}</span>
                <span class="ship__done" data-slot="ship_done" hidden=!done>{g("cart.free_done")}</span>
            </p>
            <dl class="totals">
                <div class="totals__row">
                    <dt>{g("cart.subtotal")}</dt>
                    <dd class="mono" data-slot="subtotal">{d.subtotal.clone()}</dd>
                </div>
                <div class="totals__row totals__row--save">
                    <dt>{g("cart.savings")}</dt>
                    <dd class="mono" data-slot="savings" hidden=d.savings.is_none()>{d.savings.clone()}</dd>
                </div>
                <div class="totals__row">
                    <dt>{g("cart.delivery")}</dt>
                    <dd>
                        <span class="mono" data-slot="delivery" hidden=d.delivery_free>{d.delivery.clone()}</span>
                        <span class="totals__free" data-slot="free" hidden=!d.delivery_free>{g("cart.free")}</span>
                    </dd>
                </div>
                <div class="totals__row totals__row--total">
                    <dt>{g("cart.total")}</dt>
                    <dd class="mono" data-slot="total">{d.total.clone()}</dd>
                </div>
            </dl>
            <p class="totals__vat">{vat_a}<span data-slot="vat">{d.vat.clone()}</span>{vat_b}</p>
            <div class="notice notice--error csum__err" data-quote-error="" role="alert" hidden="">
                <Icon name="circle-alert" size=20/>
                <span data-slot="qerr">{g("cart.quote_failed")}</span>
                <button type="button" class="link-btn" data-quote-retry="">{g("cart.retry")}</button>
            </div>
            <div class="csum__actions">
                <p class="csum__blocked" data-slot="blocked" hidden=!blocked>{g("cart.blocked")}</p>
                <a class="btn btn--primary btn--lg btn--block" href="/checkout" data-checkout=""
                    aria-disabled=blocked.to_string()>
                    <span class="btn__label">{g("cart.checkout")}</span>
                    <Icon name="arrow-right" size=18 class="flip-rtl btn__end"/>
                </a>
                {if in_drawer {
                    view! { <a class="btn btn--ghost btn--md btn--block" href="/cart">{g("cart.view")}</a> }.into_any()
                } else {
                    view! { <a class="btn btn--ghost btn--md btn--block" href="/">{g("cart.continue")}</a> }.into_any()
                }}
            </div>
        </div>
    }
}

/// `next` for the forms on /cart.
pub const CART_PATH: &str = "/cart";
