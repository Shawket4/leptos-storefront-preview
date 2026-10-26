//! The purchase panel: price, stock, SKU, the variant picker, Madar-style add-on groups, quantity,
//! add to cart, and the phone's sticky add-to-cart bar. Rendered by the server for the variant in
//! the URL (`?v=`).
//!
//! **Without JS** variant values are links (`?v=<id>`), and the form posts to the cart's no-JS
//! endpoint (`POST /cart/add`: `product`, `variant`, `qty`, `addon`…, `next`), which comes back
//! here with `?added=` or `?cart_err=` (rendered as a notice).
//!
//! **With JS** (`data-ts="variant-picker"`, `features/product.ts`) every variant's outcome is in the
//! page's JSON (`PickerData`): price (as a `<template>` of the design system's `Price`), "you save",
//! stock, SKU, image, and for every axis value what picking it does and why it can't be picked. So
//! switching variants, choosing add-ons and the quantity are local; the add goes through
//! `addAndConfirm` (the cookie cart), and nothing is posted.

use super::head::script_json;
use super::model::{
    self, Avail, Notice, Stock, bounds, group_hint, group_problem, stock, stock_text, surcharge,
    text, variant_href,
};
use crate::api::types::ProductView;
use crate::components::button::{Button, ButtonKind, ButtonSize};
use crate::components::choice_chips::{ChoiceChip, ChoiceChips, ChoiceKind, ChoiceState};
use crate::components::choice_list::{ChoiceList, ChoiceRow};
use crate::components::icons::Icon;
use crate::components::notice::{Notice as NoticeBox, Tone};
use crate::components::price::{Price, PriceSize};
use crate::components::stepper::Stepper;
use crate::format;
use crate::prefs::{Lang, Numerals, tr};
use crate::server::TenantCatalog;
use crate::strings;
use domain::{OptionKind, Piastres, SelectionType, Variant};
use leptos::either::Either;
use leptos::prelude::*;
use serde::Serialize;

/// What the browser code needs to switch variants without asking the server.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PickerData {
    product: u32,
    /// The product page (`?v=` is added per variant).
    href: String,
    name: String,
    selected: u32,
    /// Most of one line (stock caps it further).
    max_qty: u32,
    variants: Vec<VariantData>,
    /// Every add-on option of the product, by id: its name and surcharge (piastres), for the cart
    /// line's snapshot.
    addons: std::collections::BTreeMap<String, AddonData>,
    /// The add-on groups' rules, for checking before an add (the server checks the no-JS post).
    groups: Vec<GroupData>,
}

#[derive(Serialize)]
struct GroupData {
    /// The group's fieldset (`ChoiceList` id; its error line is `{id}-error`).
    id: String,
    min: u32,
    /// `None`: any number.
    max: Option<u32>,
    /// What to say when the group blocks the add.
    problem: String,
}

#[derive(Serialize)]
struct AddonData {
    name: String,
    price: i64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct VariantData {
    id: u32,
    sku: String,
    stock: u32,
    /// `in` / `low` / `out`.
    stock_state: &'static str,
    stock_text: String,
    /// Formatted: the cart line's snapshot and the phone bar.
    price: String,
    was: Option<String>,
    /// "You save EGP 200.00".
    save: Option<String>,
    /// `<template id="tpl-{price_tpl}">`: this variant's `Price`.
    price_tpl: String,
    /// Index into the gallery's photos.
    image: Option<usize>,
    // ── The cart line's priced snapshot (`api::cart::Snapshot`, as `server::cart::snapshot`
    // builds it), so an add needs no request and the drawer prices the line at once. ──
    /// The variant's price without add-ons, piastres.
    unit: i64,
    /// Its "was" price, piastres (only when higher).
    compare_at: Option<i64>,
    /// Stock, at most the cart's per-line limit.
    snap_max: u32,
    /// "Titanium Black · 256 GB" (none for a single-variant product).
    label: Option<String>,
    /// The line's picture (the variant's photo, else the first).
    img: Option<String>,
    /// The picked value's label on each axis.
    labels: Vec<String>,
    /// Per axis, per value (in markup order): what picking it does from this variant.
    values: Vec<Vec<ValueState>>,
    /// Per axis: why the values that can't be picked can't ("Silver: sold out in 128 GB").
    notes: Vec<String>,
}

#[derive(Serialize)]
struct ValueState {
    /// `on` (the current pick), `open` (leads to variant `v`), `out` (sold out), `none` (no such
    /// combination).
    s: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    v: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    why: Option<String>,
}

fn stock_code(s: Stock) -> &'static str {
    match s {
        Stock::In => "in",
        Stock::Low(_) => "low",
        Stock::Out => "out",
    }
}

fn saving(v: &Variant, lang: Lang, num: Numerals) -> Option<String> {
    v.compare_at.filter(|c| *c > v.price).map(|was| {
        let off = format::money(was - v.price, lang, num);
        match lang {
            Lang::En => format!("You save {off}"),
            Lang::Ar => format!("هتوفّر {off}"),
        }
    })
}

/// Distinct (price, compare-at) pairs, in variant order: one `Price` template each.
fn price_keys(vs: &[Variant]) -> Vec<(Piastres, Option<Piastres>)> {
    let mut keys = Vec::new();
    for v in vs {
        let k = (v.price, v.compare_at.filter(|c| *c > v.price));
        if !keys.contains(&k) {
            keys.push(k);
        }
    }
    keys
}

fn price_key_index(keys: &[(Piastres, Option<Piastres>)], v: &Variant) -> usize {
    let k = (v.price, v.compare_at.filter(|c| *c > v.price));
    keys.iter().position(|x| *x == k).unwrap_or(0)
}

/// Every axis's value labels and availability, as seen from variant `cur`.
fn axis_states(
    pv: &ProductView,
    cur: &Variant,
    lang: Lang,
) -> (Vec<String>, Vec<Vec<ValueState>>, Vec<String>) {
    let p = &pv.product;
    let mut labels = Vec::new();
    let mut values = Vec::new();
    let mut notes = Vec::new();
    for (ai, axis) in p.options.iter().enumerate() {
        labels.push(
            cur.opts
                .get(ai)
                .and_then(|o| axis.values.iter().find(|x| &x.id == o))
                .map(|x| text(&x.label, lang))
                .unwrap_or_default(),
        );
        let mut row = Vec::new();
        let mut why_all = Vec::new();
        for val in &axis.values {
            let st = model::avail(&p.variants, cur, ai, &val.id);
            let why = (!st.enabled())
                .then(|| model::reason(&p.options, &p.variants, cur, ai, &val.id, lang));
            if let Some(w) = &why {
                why_all.push(w.clone());
            }
            row.push(match st {
                Avail::Selected => ValueState {
                    s: "on",
                    v: Some(cur.id.0),
                    why: None,
                },
                Avail::Open(v) => ValueState {
                    s: "open",
                    v: Some(v),
                    why: None,
                },
                Avail::SoldOut => ValueState {
                    s: "out",
                    v: None,
                    why,
                },
                Avail::Missing => ValueState {
                    s: "none",
                    v: None,
                    why,
                },
            });
        }
        values.push(row);
        notes.push(why_all.join(" · "));
    }
    (labels, values, notes)
}

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
    let keys = price_keys(&p.variants);
    let max_qty = domain::cart::MAX_QTY;
    let qty_max = cur.stock.clamp(1, max_qty);
    let page_href = variant_href(&p.slug, cur.id.0);

    let data = PickerData {
        product: pid,
        href: format!("/p/{}", p.slug),
        name: text(&p.name, lang),
        selected: cur.id.0,
        max_qty,
        variants: p
            .variants
            .iter()
            .map(|v| {
                let (labels, values, notes) = axis_states(&pv, v, lang);
                let st = stock(v.stock);
                VariantData {
                    id: v.id.0,
                    sku: v.sku.clone(),
                    stock: v.stock,
                    stock_state: stock_code(st),
                    stock_text: stock_text(st, lang, num),
                    price: format::money(v.price, lang, num),
                    was: v
                        .compare_at
                        .filter(|c| *c > v.price)
                        .map(|c| format::money(c, lang, num)),
                    save: saving(v, lang, num),
                    price_tpl: format!("pdp-price-{}", price_key_index(&keys, v)),
                    image: v.image.filter(|i| *i < pv.images.len()),
                    unit: v.price.0,
                    compare_at: v.compare_at.filter(|c| *c > v.price).map(|c| c.0),
                    snap_max: v.stock.min(max_qty),
                    label: Some(
                        TenantCatalog::variant_label(p, v)
                            .pick(lang.is_rtl())
                            .to_owned(),
                    )
                    .filter(|l| !l.is_empty()),
                    img: v
                        .image
                        .and_then(|i| pv.images.get(i))
                        .or_else(|| pv.images.first())
                        .filter(|i| !i.is_missing())
                        .map(|i| i.src_near(192)),
                    labels,
                    values,
                    notes,
                }
            })
            .collect(),
        addons: p
            .modifier_groups
            .iter()
            .flat_map(|g| &g.options)
            .map(|o| {
                (
                    o.id.clone(),
                    AddonData {
                        name: text(&o.name, lang),
                        price: o.price.0,
                    },
                )
            })
            .collect(),
        groups: p
            .modifier_groups
            .iter()
            .map(|g| {
                let (min, max) = bounds(g);
                GroupData {
                    id: group_id(pid, &g.id),
                    min,
                    max: (max != u32::MAX).then_some(max),
                    problem: group_problem(g, lang, num),
                }
            })
            .collect(),
    };
    let json = serde_json::to_value(&data)
        .map(|v| script_json(&v))
        .unwrap_or_default();
    let (_, states, notes) = axis_states(&pv, &cur, lang);

    // ── Variant picker: one `ChoiceChips` group per axis, every value a link (`?v=`) ──
    let axes = p
        .options
        .iter()
        .enumerate()
        .map(|(ai, axis)| {
            let swatch = axis.kind == OptionKind::Swatch;
            let items = axis
                .values
                .iter()
                .enumerate()
                .map(|(vi, val)| {
                    let st = &states[ai][vi];
                    let state = match st.s {
                        "on" => ChoiceState::Selected,
                        "open" => ChoiceState::Available,
                        _ => ChoiceState::Unavailable,
                    };
                    let mut chip = ChoiceChip::new(text(&val.label, lang), val.id.clone())
                        .state(state)
                        .attr("data-axis", ai.to_string())
                        .attr("data-index", vi.to_string());
                    if let Some(v) = st.v.filter(|_| state != ChoiceState::Unavailable) {
                        chip = chip.href(variant_href(&p.slug, v));
                    }
                    if let Some(why) = &st.why {
                        chip = chip.title(why.clone());
                    }
                    if let Some(c) = &val.swatch {
                        chip = chip.swatch(c.clone());
                    }
                    chip
                })
                .collect::<Vec<_>>();
            let kind = if swatch {
                ChoiceKind::Swatch
            } else {
                ChoiceKind::Chip
            };
            let note = notes[ai].clone();
            view! {
                <ChoiceChips
                    label=text(&axis.name, lang)
                    items=items
                    kind=kind
                    show_value=true
                    id=format!("pp-ax-{pid}-{ai}")
                    note=note
                    attr:data-axis=ai.to_string()
                />
            }
        })
        .collect_view();

    // ── Add-on groups (`ChoiceList`). Checkboxes post as `addon`; a single-choice group's radios
    // need a name of their own (one radio group each): `addon.<group>`. ──
    let groups = p
        .modifier_groups
        .iter()
        .map(|g| {
            let single = g.selection == SelectionType::Single;
            let (min, _) = bounds(g);
            let name = if single {
                format!("addon.{}", g.id)
            } else {
                "addon".to_owned()
            };
            let mut rows = Vec::new();
            if single && min == 0 {
                rows.push(ChoiceRow::new("", tr(lang, "No, thanks", "لا، شكرًا")).checked(true));
            }
            rows.extend(g.options.iter().map(|o| {
                ChoiceRow::new(o.id.clone(), text(&o.name, lang))
                    .meta(surcharge(o.price, lang, num))
            }));
            view! {
                <ChoiceList
                    legend=text(&g.name, lang)
                    name=name
                    items=rows
                    single=single
                    id=group_id(pid, &g.id)
                    hint=group_hint(g, lang, num)
                />
            }
        })
        .collect_view();
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
    let templates = keys
        .iter()
        .enumerate()
        .map(|(i, (amount, was))| {
            view! {
                <template id=format!("tpl-pdp-price-{i}")>
                    <Price amount=*amount compare_at=*was size=PriceSize::Xl lang numerals=num/>
                </template>
            }
        })
        .collect_view();
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

/// An add-on group's fieldset id (its hint and error lines hang off it).
fn group_id(pid: u32, group: &str) -> String {
    format!("pp-g-{pid}-{group}")
}
