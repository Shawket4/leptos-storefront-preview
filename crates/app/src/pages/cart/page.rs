//! `/cart` — rendered from the `cart` cookie, priced now, complete without JS: each line's
//! quantity and Remove are plain forms (`/cart/set`, `/cart/remove`), Edit a link to the line editor
//! (`/cart/edit`), and a no-JS form's outcome comes back as `?added=` / `?updated=` / `?cart_err=`
//! and is shown here. With JS, `features/cart.ts`
//! (`data-ts="cart-page"`) takes the forms over: changes go to the cart store at once, and the
//! lines are re-rendered by the same renderer as the drawer, with the server's quote.

use super::line::{CART_PATH, CartSummary, LineData, LineRow, SummaryData, percent};
use super::parts::QuickAdd;
use super::tx;
use crate::api::types::{CartView, Rail};
use crate::components::button::{Button, ButtonKind, ButtonSize};
use crate::components::icons::Icon;
use crate::components::item_row::{ItemList, ItemRow, ItemSize};
use crate::components::notice::{Notice, Tone};
use crate::components::product_card::CardData;
use crate::components::states::{StateTone, StateView};
use crate::components::totals::{TotalRow, TotalTone};
use crate::format;
use crate::media::Img;
use crate::prefs::{Lang, Numerals, tr};
use crate::request::req;
use crate::strings;
use domain::Piastres;
use leptos::either::Either;
use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use leptos_router::hooks::use_query_map;

/// Art for empty states, resolved on the server (light and dark files).
#[derive(Clone, Default)]
pub struct Art {
    pub light: Option<Img>,
    pub dark: Option<Img>,
}

pub fn art(name: &str) -> Art {
    let m = crate::server::state();
    Art {
        light: m.media().try_img(&format!("brand/{name}-light")),
        dark: m.media().try_img(&format!("brand/{name}-dark")),
    }
}

/// An illustration that follows the theme (both files are lazy, the hidden one never loads).
#[component]
pub fn ArtImage(
    art: Art,
    #[prop(optional)] fallback_icon: Option<&'static str>,
    lang: Lang,
) -> impl IntoView {
    let _ = lang;
    match (art.light, art.dark) {
        (Some(light), dark) => Either::Left(view! {
            <span class="art" aria-hidden="true">
                <img class="art__light" src=light.src_near(240) srcset=light.srcset() sizes="160px" alt=""
                    width="240" height="240" loading="lazy" decoding="async"/>
                {dark.map(|d| view! {
                    <img class="art__dark" src=d.src_near(240) srcset=d.srcset() sizes="160px" alt=""
                        width="240" height="240" loading="lazy" decoding="async"/>
                })}
            </span>
        }),
        (None, _) => Either::Right(view! {
            <span class="art art--icon" aria-hidden="true"><Icon name=fallback_icon.unwrap_or("shopping-bag") size=40/></span>
        }),
    }
}

/// "Prices include 14% VAT (EGP 12.00)." in the visitor's numerals.
pub fn vat_note(vat: Piastres, lang: Lang, num: Numerals) -> String {
    tx(
        "cart.vat",
        lang,
        &[
            ("pct", &percent(14, num)),
            ("vat", &format::money(vat, lang, num)),
        ],
    )
}

/// Quick-addable products (one variant, nothing required, in stock), not already in the cart.
fn picks(exclude: &[domain::ProductId], n: usize) -> Vec<CardData> {
    let catalog = crate::server::current_catalog();
    let mut seen = Vec::new();
    catalog
        .rail(Rail::Bestsellers, 24)
        .into_iter()
        .chain(catalog.rail(Rail::Featured, 24))
        .filter(|c| c.in_stock && c.single_variant && !exclude.contains(&c.id))
        .filter(|c| {
            !seen.contains(&c.id) && {
                seen.push(c.id);
                true
            }
        })
        .take(n)
        .collect()
}

#[component]
pub fn CartPage() -> impl IntoView {
    // Personal: never cached anywhere. Set before the first chunk streams.
    crate::server::cache::private();
    let rq = req();
    let (lang, num) = (rq.prefs.lang, rq.prefs.numerals);
    let q = use_query_map();
    let added = q.with_untracked(|q| q.get("added")).is_some();
    let updated = q.with_untracked(|q| q.get("updated")).is_some();
    let err = q.with_untracked(|q| q.get("cart_err")).filter(|c| {
        matches!(
            c.as_str(),
            "out_of_stock"
                | "max_qty"
                | "full"
                | "addon_required"
                | "bad_addon"
                | "unknown_product"
                | "line_gone"
                | "bad_request"
        )
    });
    let (lines, priced) = crate::server::cart::priced_for_request();
    let catalog = crate::server::current_catalog();
    let st = crate::server::state();
    let snaps: Vec<_> = lines
        .iter()
        .map(|l| crate::server::cart::snapshot(l, &catalog, st.media(), lang))
        .collect();
    let api = crate::api::cart::quote(&lines, &snaps, &priced, lang, num);
    // The browser code keeps these as the lines' snapshots (and says so if a price moved).
    let snapshots_json = snapshots_json(&lines, &snaps);
    let rows: Vec<LineData> = lines
        .iter()
        .map(
            |cl| match priced.view.lines.iter().find(|l| l.key == cl.key()) {
                Some(l) => LineData::from_view(l, lang, num),
                None => LineData::unavailable(cl, lang),
            },
        )
        .collect();
    let empty = rows.is_empty();
    let summary = SummaryData {
        subtotal: api.subtotal,
        savings: api.savings,
        delivery: api.delivery,
        delivery_free: api.delivery_free,
        hint: api.free_delivery_hint,
        total: api.total,
        can_checkout: api.can_checkout,
        empty,
    };
    let in_cart: Vec<_> = priced.view.lines.iter().map(|l| l.product).collect();
    let picks = picks(&in_cart, 4);
    let title = format!(
        "{} · {}",
        strings::get("cart.title", lang),
        rq.tenant.name.pick(lang.is_rtl())
    );
    let count = (!empty).then(|| format::items(api.count, lang, num));

    view! {
        <Title text=title/>
        <Meta name="robots" content="noindex"/>
        <div class="container cart-page" data-ts="cart-page">
            <script type="application/json" data-ts-json="" inner_html=snapshots_json></script>
            <header class="cart-page__head">
                <h1 class="t-h1" id="cart-title" tabindex="-1">{strings::get("cart.title", lang)}</h1>
                <p class="cart-page__count" data-slot="count" hidden=empty>{count}</p>
            </header>
            {added.then(|| view! { <Notice tone=Tone::Success title=strings::get("cart.added", lang) attr:data-cart-notice=""/> })}
            {updated.then(|| view! { <Notice tone=Tone::Success title=strings::get("cart.updated", lang) attr:data-cart-notice=""/> })}
            {err.map(|code| view! {
                <Notice tone=Tone::Danger title=strings::get(&format!("cart.err.{code}"), lang)
                    attr:tabindex="-1" attr:data-cart-notice=""/>
            })}
            <PriceChanged lang/>
            <div class="cart-empty card" data-cart-empty="" hidden=!empty>
                <StateView tone=StateTone::Empty art="empty-cart"
                    title=strings::get("cart.empty_title", lang) body=strings::get("cart.empty_body", lang)>
                    <Button href="/" kind=ButtonKind::Primary size=ButtonSize::Lg>{strings::get("cart.start", lang)}</Button>
                </StateView>
            </div>
            <div class="cart-layout" data-cart-full="" hidden=empty>
                <section class="cart-lines card" aria-labelledby="cart-title">
                    <ItemList attr:data-cart-lines="" attr:aria-label=strings::get("cart.lines_label", lang)>
                        {rows.into_iter().map(|d| view! { <LineRow d lang next=CART_PATH/> }).collect_view()}
                    </ItemList>
                </section>
                <aside class="cart-summary card" aria-label=tr(lang, "Order summary", "ملخص الطلب")>
                    <CartSummary d=summary lang num/>
                    <ul class="assure" role="list">
                        <li><Icon name="banknote" size=18/><span>{tr(lang, "Pay cash on delivery", "ادفع كاش عند الاستلام")}</span></li>
                        <li><Icon name="shield-check" size=18/><span>{tr(lang, "Prices are checked again when you order", "الأسعار بتتراجع تاني وقت الطلب")}</span></li>
                    </ul>
                </aside>
            </div>
            {(!picks.is_empty()).then(|| view! { <Picks picks lang num/> })}
        </div>
    }
}

/// `{key: snapshot}` for the browser code (`<script type="application/json">`, `<` escaped).
pub fn snapshots_json(
    lines: &[domain::cart::CartLine],
    snaps: &[Option<crate::api::cart::Snapshot>],
) -> String {
    let map: std::collections::BTreeMap<String, &crate::api::cart::Snapshot> = lines
        .iter()
        .zip(snaps)
        .filter_map(|(l, s)| s.as_ref().map(|s| (l.key(), s)))
        .collect();
    serde_json::to_string(&map)
        .unwrap_or_else(|_| "{}".into())
        .replace('<', "\\u003c")
}

/// Shown by the browser code when the server's prices differ from what the cart showed.
#[component]
pub fn PriceChanged(lang: Lang) -> impl IntoView {
    view! {
        <Notice tone=Tone::Info title=strings::get("cart.price_changed", lang) attr:data-price-changed="" attr:hidden=true/>
    }
}

/// "Popular right now": a few bestsellers not in the cart, each with quick add.
#[component]
fn Picks(picks: Vec<CardData>, lang: Lang, num: Numerals) -> impl IntoView {
    let ar = lang.is_rtl();
    view! {
        <section class="picks" aria-labelledby="picks-title">
            <h2 class="t-h3" id="picks-title">{strings::get("cart.picks", lang)}</h2>
            <ItemList size=ItemSize::Compact class="picks__list">
                {picks.into_iter().map(|c| {
                    let price = format::money(c.price, lang, num);
                    let href = format!("/p/{}", c.slug);
                    let name = c.name.pick(ar).to_owned();
                    let card = c.clone();
                    match (!c.img.is_missing()).then(|| c.img.src_near(192)) {
                        Some(img) => view! {
                            <ItemRow size=ItemSize::Compact name img href price class="card pick">
                                <QuickAdd card lang numerals=num/>
                            </ItemRow>
                        }.into_any(),
                        None => view! {
                            <ItemRow size=ItemSize::Compact name href price class="card pick">
                                <QuickAdd card lang numerals=num/>
                            </ItemRow>
                        }.into_any(),
                    }
                }).collect_view()}
            </ItemList>
        </section>
    }
}

/// A priced cart's totals for checkout (its summary and review step), through core's `Totals`.
#[component]
pub fn Totals(
    v: CartView,
    lang: Lang,
    num: Numerals,
    /// An extra delivery charge (the express slot) on top of the cart's own fee.
    #[prop(optional)]
    extra_fee: Piastres,
) -> impl IntoView {
    let fee = v.delivery_fee + extra_fee;
    let m = |p| format::money(p, lang, num);
    let g = |k: &str| strings::get(k, lang);
    let mut rows = vec![TotalRow::new(g("cart.subtotal"), m(v.subtotal))];
    if v.savings.0 > 0 {
        rows.push(TotalRow::new(g("cart.savings"), m(v.savings)).tone(TotalTone::Saving));
    }
    rows.push(if fee.0 == 0 {
        TotalRow::new(g("cart.delivery"), g("cart.free")).tone(TotalTone::Free)
    } else {
        TotalRow::new(g("cart.delivery"), m(fee))
    });
    rows.push(TotalRow::new(g("cart.total"), m(v.total + extra_fee)).tone(TotalTone::Total));
    view! { <crate::components::totals::Totals rows note=vat_note(v.tax_included, lang, num)/> }
}
