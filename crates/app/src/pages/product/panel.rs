//! The purchase panel: price, stock, SKU, the variant picker, Madar-style add-on groups, quantity,
//! add to cart, and the phone's sticky add-to-cart bar. Rendered by the server for the variant in
//! the URL (`?v=`).
//!
//! **Without JS** variant values are links (`?v=<id>`), and the form posts to the cart's no-JS
//! endpoint (`POST /cart/add`: `product`, `variant`, `qty`, `addon`…, `next`), which comes back
//! here with `?added=` or `?cart_err=` (rendered as a notice).
//!
//! **With JS** (`data-ts="variant-picker"`, `features/product.ts` on `features/picker.ts`) every
//! variant's outcome is in the page's JSON (`picker.rs`): price (as a `<template>` of the design
//! system's `Price`), "you save", stock, SKU, image, and for every axis value what picking it does
//! and why it can't be picked. So switching variants, choosing add-ons and the quantity are local;
//! the add goes through `addAndConfirm` (the cookie cart), and nothing is posted.
//!
//! The axes, the add-on groups, the JSON and the price templates are `picker.rs`'s, shared with the
//! cart's line editor.

use super::model::{self, Notice, Stock, stock, stock_text, text, variant_href};
use super::picker::{Picker, saving};
use crate::api::types::ProductView;
use crate::components::button::{Button, ButtonKind, ButtonSize};
use crate::components::icons::Icon;
use crate::components::notice::{Notice as NoticeBox, Tone};
use crate::components::price::{Price, PriceSize};
use crate::components::stepper::Stepper;
use crate::format;
use crate::prefs::{Lang, Numerals, tr};
use crate::strings;
use leptos::either::Either;
use leptos::prelude::*;

#[component]
pub fn PurchasePanel(
    pv: ProductView,
    selected: u32,
    notice: Option<Notice>,
    lang: Lang,
    num: Numerals,
) -> impl IntoView {
    let p = &pv.product;
    let pid = p.id.0;
    let cur = model::variant(&p.variants, selected).clone();
    let form_id = format!("pdp-buy-{pid}");
    let qty_max = cur.stock.clamp(1, domain::cart::MAX_QTY);
    let page_href = variant_href(&p.slug, cur.id.0);
    let picker = Picker {
        pv: &pv,
        cur: &cur,
        prefix: "pp",
        lang,
        num,
    };
    let json = picker.json(&format!("/p/{}?v={{v}}", p.slug));
    let axes = picker.axes(|v| variant_href(&p.slug, v));
    // On the product page every group starts at its default ("No, thanks" where there is one).
    let groups = picker.groups(&[]);
    let templates = picker.price_templates(PriceSize::Xl);
    let has_axes = !p.options.is_empty();
    let has_groups = !p.modifier_groups.is_empty();

    // ── Live text, for the variant on load ──
    let st = stock(cur.stock);
    let out = st == Stock::Out;
    let save = saving(&cur, lang, num);
    let no_save = save.is_none();
    let add_label = if out {
        strings::get("product.out", lang)
    } else {
        strings::get("product.add", lang)
    };
    let price_now = format::money(cur.price, lang, num);
    let notice = notice.map(|n| notice_view(n, lang, num));
    let has_notice = notice.is_some();

    view! {
        <div class="pp" data-ts="variant-picker">
            <script type="application/json" data-ts-json="" inner_html=json></script>
            {templates}
            <div class="pp-card">
                <div class="pp-price">
                    <div class="pp-price__now" data-price="">
                        <Price amount=cur.price compare_at=cur.compare_at.filter(|c| *c > cur.price) size=PriceSize::Xl lang numerals=num/>
                    </div>
                    <p class="pp-price__save" class:is-empty=no_save data-save="">{save}</p>
                    <p class="pp-price__tax">{tr(lang, "VAT included", "شامل ضريبة القيمة المضافة")}</p>
                </div>
                <div class="pp-stock">
                    <p class="pp-stock__state" class:is-low=matches!(st, Stock::Low(_)) class:is-out=out role="status" data-stock="">
                        <Icon name="circle-check" size=16 class="pp-stock__ic pp-stock__ic--in"/>
                        <Icon name="timer" size=16 class="pp-stock__ic pp-stock__ic--low"/>
                        <Icon name="circle-alert" size=16 class="pp-stock__ic pp-stock__ic--out"/>
                        <span data-stock-text="">{stock_text(st, lang, num)}</span>
                    </p>
                    <p class="pp-sku">
                        <span class="pp-sku__k">{tr(lang, "SKU", "رقم المنتج")}</span>
                        " "
                        <span class="mono" dir="ltr" data-sku="">{cur.sku.clone()}</span>
                    </p>
                </div>
                <form class="pp-form" id=form_id.clone() method="post" action="/cart/add">
                    <input type="hidden" name="product" value=pid/>
                    <input type="hidden" name="variant" value=cur.id.0/>
                    <input type="hidden" name="next" value=page_href/>
                    // Variant links are the no-JS path; with JS a pick is local, so the router neither takes
                    // nor prefetches them (`data-router="off"`).
                    {has_axes.then(|| view! { <div class="pp-axes" data-router="off">{axes}</div> })}
                    {has_groups.then(|| view! { <div class="pp-groups">{groups}</div> })}
                    <div class="pp-buy">
                        <Stepper
                            name="qty"
                            value=Signal::stored(1u32)
                            min=1
                            max=qty_max
                            label=tr(lang, "Quantity", "الكمية").to_owned()
                            disabled=out
                            class="pp-qty"
                        />
                        <Button
                            kind=ButtonKind::Primary
                            size=ButtonSize::Lg
                            button_type="submit"
                            full_width=true
                            class="pp-add"
                            disabled=out
                            attr:data-add=""
                        >
                            <span class="pp-add__main">
                                <span class="pp-add__state" aria-hidden="true"><Icon name="check" size=18/></span>
                                <span class="pp-add__label trim" data-add-label="">{add_label}</span>
                            </span>
                        </Button>
                    </div>
                    // A notice from the no-JS add takes focus on load, so the browser scrolls to it.
                    <div class="pp-msgs" aria-live="polite" tabindex=has_notice.then_some("-1") autofocus=has_notice>
                        {notice}
                    </div>
                </form>
            </div>
            <div class="pp-bar" aria-hidden="true" inert="">
                <div class="pp-bar__info">
                    <p class="pp-bar__name">{text(&p.name, lang)}</p>
                    <p class="pp-bar__price mono" data-bar-price="">{price_now}</p>
                </div>
                <Button
                    kind=ButtonKind::Primary
                    size=ButtonSize::Md
                    button_type="submit"
                    class="pp-bar__add"
                    disabled=out
                    attr:form=form_id
                    attr:data-add=""
                >
                    <span data-add-label="">{add_label}</span>
                </Button>
            </div>
        </div>
    }
}

/// `?added=` / `?cart_err=`: the cart's words for it (`strings/cart.rs`).
fn notice_view(n: Notice, lang: Lang, num: Numerals) -> impl IntoView {
    match n {
        Notice::Added(qty) => {
            let msg = strings::get("cart.added", lang)
                .replace("{qty}", &format::count(qty, num))
                .replace("{items}", &format::items(qty, lang, num));
            Either::Left(view! {
                <NoticeBox tone=Tone::Success title=msg>
                    <a class="link" href="/cart">{strings::get("cart.view", lang)}</a>
                </NoticeBox>
            })
        }
        Notice::Error(code) => {
            let key = format!("cart.err.{code}");
            Either::Right(view! { <NoticeBox tone=Tone::Danger title=strings::get(&key, lang)/> })
        }
    }
}
