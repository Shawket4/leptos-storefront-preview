//! A product line: its picture, name (a link), a detail line (variant, add-ons), a meta line
//! (quantity × unit, a stock note) and a price, with trailing controls (a `Stepper`, a remove
//! button) as children. Cart lines (`Regular`: the controls get the row's whole width under the
//! picture, so a 320px drawer fits), checkout's mini lines and the order's lines (`Compact`).
//! Put rows in an [`ItemList`].
//!
//! `slots` is for rows browser code clones from a `<template>` and fills (`fill()` / `patch()`,
//! core/dom.ts): every part gets a stable `data-slot` — `name`, `detail`, `meta`, `price` — the
//! links `data-attr-href="href"` and the picture `data-attr-src="img"`; the parts without a value
//! are rendered hidden so the client can show them. Extra attributes on the row (`data-key`) go on
//! the component: `<ItemRow attr:data-key=key …>`.
//!
//! ```ignore
//! <ItemList>
//!     <ItemRow img=l.img name=l.name href=l.href detail=l.variant meta=each price=l.total>
//!         <Stepper name="qty" value=l.qty min=1 max=l.max label=… removable=true/>
//!     </ItemRow>
//! </ItemList>
//! ```

use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ItemSize {
    /// Summaries: a 48px picture, everything on one row.
    Compact,
    /// The cart: a 72px picture; the trailing controls and price on their own row below.
    #[default]
    Regular,
}

/// A list of [`ItemRow`]s — or, without children, the empty list browser code fills with rows
/// cloned from a `<template>` (`<ItemList attr:id="lines"/>`).
#[component]
pub fn ItemList(
    #[prop(optional)] size: ItemSize,
    #[prop(optional, into)] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let base = match size {
        ItemSize::Compact => "item-list item-list--compact",
        ItemSize::Regular => "item-list",
    };
    let cls = if class.is_empty() {
        base.to_owned()
    } else {
        format!("{base} {class}")
    };
    view! { <ul class=cls role="list">{children.map(|c| c())}</ul> }
}

#[component]
pub fn ItemRow(
    /// The picture's URL (a square thumbnail); none: an empty well.
    #[prop(optional, into)]
    img: Option<String>,
    #[prop(into)] name: String,
    /// The product page; the name and picture link to it.
    #[prop(optional, into)]
    href: Option<String>,
    /// Variant and add-ons ("Black · 256 GB").
    #[prop(optional, into)]
    detail: Option<String>,
    /// "2 × EGP 59.99", "Only 3 left" (formatted).
    #[prop(optional, into)]
    meta: Option<String>,
    /// The line's amount (formatted).
    #[prop(optional, into)]
    price: Option<String>,
    #[prop(optional)] size: ItemSize,
    /// Stable `data-slot`s (and hidden empty parts) for browser code that fills the row.
    #[prop(optional)]
    slots: bool,
    #[prop(optional, into)] class: String,
    /// Trailing controls: a `Stepper`, a remove button.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let base = match size {
        ItemSize::Compact => "item-row item-row--compact",
        ItemSize::Regular => "item-row",
    };
    let cls = if class.is_empty() {
        base.to_owned()
    } else {
        format!("{base} {class}")
    };
    let slot = move |k: &'static str| slots.then_some(k);
    let px = if size == ItemSize::Compact {
        "48"
    } else {
        "72"
    };
    // An optional part: shown with its value, or (for browser code) present but hidden.
    let part = move |v: Option<String>, k: &'static str, cls: &'static str| match v {
        Some(v) => Some(view! { <p class=cls data-slot=slot(k)>{v}</p> }.into_any()),
        None => slots.then(|| view! { <p class=cls data-slot=k hidden=true></p> }.into_any()),
    };
    let link = href.is_some() || slots;
    let media = {
        // With a source, or (a template's) without one: the client sets its src.
        let picture = (img.is_some() || slots).then(|| {
            view! {
                <img class="item-row__img" src=img.clone() data-attr-src=slot("img") alt="" width=px height=px
                    loading="lazy" decoding="async"/>
            }
        });
        if link {
            view! {
                <a class="item-row__media" href=href.clone() data-attr-href=slot("href") tabindex="-1" aria-hidden="true">
                    {picture}
                </a>
            }
            .into_any()
        } else {
            view! { <span class="item-row__media" aria-hidden="true">{picture}</span> }.into_any()
        }
    };
    let name_view = if link {
        view! { <a class="item-row__name" href=href data-attr-href=slot("href") data-slot=slot("name")>{name}</a> }
            .into_any()
    } else {
        view! { <span class="item-row__name" data-slot=slot("name")>{name}</span> }.into_any()
    };
    let price_view = match price {
        Some(p) => Some(
            view! { <span class="item-row__price mono" data-slot=slot("price")>{p}</span> }
                .into_any(),
        ),
        None => slots.then(|| {
            view! { <span class="item-row__price mono" data-slot="price" hidden=true></span> }
                .into_any()
        }),
    };
    let detail = part(detail, "detail", "item-row__detail");
    let meta = part(meta, "meta", "item-row__meta");
    view! {
        <li class=cls>
            {media}
            <div class="item-row__main">
                {name_view}
                {detail}
                {meta}
            </div>
            <div class="item-row__side">
                {children.map(|c| view! { <div class="item-row__controls">{c()}</div> })}
                {price_view}
            </div>
        </li>
    }
}
