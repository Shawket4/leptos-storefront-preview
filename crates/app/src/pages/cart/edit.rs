//! Editing a cart line: its variant, add-ons and quantity, with the product page's own picker
//! (`pages::product::picker`: the same axes, add-on groups and rules).
//!
//! - **`/cart/edit?key=<line key>[&v=<variant>]`**, the editor as a page (personal: the line's
//!   quantity comes from the `cart` cookie). Without JS it is the whole thing: variant values are
//!   links (`&v=`), and the form posts `POST /cart/update`, which goes on to `/cart?updated=1`, or
//!   comes back here with `?cart_err=`.
//! - **With JS**, a line's "Edit" (drawer, /cart) opens [`LineEditorSheet`], a bottom sheet in
//!   every page's shell, and `features/line-editor.ts` puts this page's [`LineEditor`] in it
//!   (fetched through the router's page loader, warmed on finger-down), runs the picker
//!   (`data-ts="line-editor"`) and replaces the line in the cart store: no request.

use super::tx;
use crate::api::types::{ProductView, url_encode};
use crate::components::button::{Button, ButtonKind, ButtonSize};
use crate::components::drawer::{Drawer, DrawerSide};
use crate::components::icons::Icon;
use crate::components::notice::{Notice, Tone};
use crate::components::price::{Price, PriceSize};
use crate::components::skeleton::{Skeleton, SkeletonLines};
use crate::components::states::{StateTone, StateView};
use crate::components::stepper::Stepper;
use crate::pages::product::model::{Stock, stock, stock_text, text};
use crate::pages::product::picker::Picker;
use crate::prefs::{Lang, Numerals, use_ui};
use crate::request::req;
use crate::strings;
use domain::VariantId;
use domain::cart::CartLine;
use leptos::either::Either;
use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use leptos_router::hooks::use_query_map;

/// The editor page for a line (the "Edit" link's `href`).
pub fn edit_href(key: &str) -> String {
    format!("/cart/edit?key={}", url_encode(key))
}

#[component]
pub fn CartEditPage() -> impl IntoView {
    // Personal (the line's quantity is the cookie's): never cached anywhere.
    crate::server::cache::private();
    let rq = req();
    let (lang, num) = (rq.prefs.lang, rq.prefs.numerals);
    let (key, v, err) = use_query_map().with_untracked(|q| {
        // A hand-typed `+` arrives as a space.
        let key = q.get("key").unwrap_or_default().replace(' ', "+");
        let v = q.get("v").and_then(|v| v.parse::<u32>().ok());
        let err = q.get("cart_err").filter(|c| {
            matches!(
                c.as_str(),
                "out_of_stock" | "addon_required" | "bad_addon" | "unknown_product" | "bad_request"
            )
        });
        (key, v, err)
    });
    let line = crate::server::cart::from_request()
        .into_iter()
        .find(|l| l.key() == key);
    let st = crate::server::state();
    let catalog = crate::server::current_catalog();
    let found = line.and_then(|l| {
        let slug = catalog.product(l.product)?.slug.clone();
        Some((l, catalog.product_view(&slug, st.media())?))
    });
    let g = move |k: &str| strings::get(k, lang);
    let title = format!(
        "{} · {}",
        g("cart.edit_title"),
        rq.tenant.name.pick(lang.is_rtl())
    );
    let Some((line, pv)) = found else {
        if let Some(res) = use_context::<leptos_axum::ResponseOptions>() {
            res.set_status(http::StatusCode::NOT_FOUND);
        }
        return Either::Left(view! {
            <Title text=title/>
            <Meta name="robots" content="noindex"/>
            <div class="container le-page">
                <StateView tone=StateTone::Empty art="empty-cart" page=true
                    title=g("cart.err.line_gone") body=g("cart.empty_body")>
                    <Button href="/cart" kind=ButtonKind::Primary size=ButtonSize::Lg>{g("cart.view")}</Button>
                </StateView>
            </div>
        });
    };
    let selected = v
        .filter(|v| pv.product.variant(VariantId(*v)).is_some())
        .unwrap_or(line.variant.0);
    Either::Right(view! {
        <Title text=title/>
        <Meta name="robots" content="noindex"/>
        <crate::client_js::PreloadFeature module="features/line-editor.ts"/>
        <div class="container le-page">
            <header class="le-page__head">
                <h1 class="t-h1" tabindex="-1">{g("cart.edit_title")}</h1>
            </header>
            {err.map(|code| view! {
                <Notice tone=Tone::Danger title=g(&format!("cart.err.{code}")) attr:tabindex="-1" attr:autofocus=""/>
            })}
            <div class="card le-page__card">
                <LineEditor pv line selected lang num/>
            </div>
            <Button href="/cart" kind=ButtonKind::Ghost full_width=true class="le-page__cancel">{g("cart.back_to_cart")}</Button>
        </div>
    })
}

/// The editor: the product (picture, name, price and stock of the variant picked), its axes and
/// add-on groups with the line's choices, the quantity and "Update item". One markup for the page
/// and the sheet (the sheet shows this element, cut from the page).
#[component]
pub fn LineEditor(
    pv: ProductView,
    line: CartLine,
    /// The variant shown (the line's, or another one picked without JS).
    selected: u32,
    lang: Lang,
    num: Numerals,
) -> impl IntoView {
    let p = &pv.product;
    let cur = p
        .variant(VariantId(selected))
        .unwrap_or_else(|| p.default_variant())
        .clone();
    let key = line.key();
    let data_key = key.clone();
    let page = edit_href(&key);
    let picker = Picker {
        pv: &pv,
        cur: &cur,
        prefix: "le",
        lang,
        num,
    };
    let json = picker.json(&format!("{page}&v={{v}}"));
    let axes = picker.axes(|v| format!("{page}&v={v}"));
    let groups = picker.groups(&line.addons);
    let templates = picker.price_templates(PriceSize::Lg);
    let has_axes = !p.options.is_empty();
    let has_groups = !p.modifier_groups.is_empty();
    let name = text(&p.name, lang);
    let img = cur
        .image
        .and_then(|i| pv.images.get(i))
        .or_else(|| pv.images.first())
        .filter(|i| !i.is_missing())
        .map(|i| i.src_near(192));
    let st = stock(cur.stock);
    let out = st == Stock::Out;
    let label = if out {
        strings::get("product.out", lang)
    } else {
        strings::get("cart.update_item", lang)
    };
    let qty_label = tx("cart.qty_of", lang, &[("name", &name)]);
    view! {
        <div class="le" data-ts="line-editor" data-key=data_key>
            <script type="application/json" data-ts-json="" inner_html=json></script>
            {templates}
            <div class="le__head">
                <img class="le__img" src=img alt="" width="72" height="72" decoding="async" data-variant-img=""/>
                <div class="le__title">
                    <p class="le__name">{name}</p>
                    <div class="le__price" data-price="">
                        <Price amount=cur.price compare_at=cur.compare_at.filter(|c| *c > cur.price) size=PriceSize::Lg lang numerals=num/>
                    </div>
                    <p class="pp-stock__state le__stock" class:is-low=matches!(st, Stock::Low(_)) class:is-out=out data-stock="">
                        <Icon name="circle-check" size=16 class="pp-stock__ic pp-stock__ic--in"/>
                        <Icon name="timer" size=16 class="pp-stock__ic pp-stock__ic--low"/>
                        <Icon name="circle-alert" size=16 class="pp-stock__ic pp-stock__ic--out"/>
                        <span data-stock-text="">{stock_text(st, lang, num)}</span>
                    </p>
                </div>
            </div>
            <form class="pp-form le__form" method="post" action="/cart/update">
                <input type="hidden" name="key" value=key/>
                <input type="hidden" name="variant" value=cur.id.0/>
                <input type="hidden" name="next" value="/cart"/>
                <input type="hidden" name="back" value=format!("{page}&v={}", cur.id.0)/>
                // Values are links without JS; with JS a pick is local (the router leaves them, and
                // the sheet stays open: `data-drawer-keep`).
                {has_axes.then(|| view! { <div class="pp-axes" data-router="off" data-drawer-keep="">{axes}</div> })}
                {has_groups.then(|| view! { <div class="pp-groups">{groups}</div> })}
                <div class="le__foot pp-buy">
                    <Stepper
                        name="qty"
                        value=Signal::stored(line.qty.max(1))
                        min=1
                        max=cur.stock.clamp(1, domain::cart::MAX_QTY)
                        label=qty_label
                        disabled=out
                        class="pp-qty"
                    />
                    <Button kind=ButtonKind::Primary size=ButtonSize::Lg button_type="submit" full_width=true
                        class="pp-add le__save" disabled=out attr:data-add="">
                        <span class="le__save-label" data-add-label="">{label}</span>
                        // The line's new total, by the browser code as the choices change.
                        <span class="le__save-total mono" data-slot="total" hidden=true></span>
                    </Button>
                </div>
            </form>
        </div>
    }
}

/// The sheet the editor opens in (with JS): in every page's shell, its skeleton until a line's
/// "Edit" puts the editor in it (`features/line-editor.ts`, `openEditor`), or the notice if the
/// editor can't be loaded.
#[component]
pub fn LineEditorSheet() -> impl IntoView {
    let lang = use_ui().lang.get_untracked();
    let g = move |k: &str| strings::get(k, lang);
    view! {
        <Drawer id="line-editor" side=DrawerSide::Bottom title=g("cart.edit_title").to_owned() class="le-sheet">
            <div class="le-sheet__slot" data-le-slot="" aria-busy="true">
                <div class="le le--skeleton" data-le-skeleton="" aria-hidden="true">
                    <div class="le__head">
                        <Skeleton class="le__img"/>
                        <div class="le__title">
                            <SkeletonLines lines=2 class="le__name"/>
                        </div>
                    </div>
                    <Skeleton class="le__skel-chips"/>
                    <Skeleton class="le__skel-list"/>
                </div>
            </div>
            <Notice tone=Tone::Danger title=g("cart.edit_failed") slots=true attr:data-le-error="" attr:hidden=true>
                <button type="button" class="link-btn" data-le-retry="">{g("cart.retry")}</button>
            </Notice>
        </Drawer>
    }
}
