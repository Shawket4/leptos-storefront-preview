//! The variant picker and the add-on groups, written once for the product page's purchase panel
//! (`panel.rs`) and the cart's line editor (`pages::cart::edit`): the page's JSON (every variant's
//! outcome, so the browser switches variants without asking), the axes (`ChoiceChips`, every value
//! a link), the add-on groups (`ChoiceList`) and the variants' price templates. The browser side is
//! one module too: `client/src/features/picker.ts`.
//!
//! Both can be on one page (the editor's sheet over a product page), so every id and template name
//! carries the picker's prefix: `pp` on the product page, `le` in the line editor.

use super::head::script_json;
use super::model::{
    self, Avail, Stock, bounds, group_hint, group_problem, stock, stock_text, surcharge, text,
};
use crate::api::types::ProductView;
use crate::components::choice_chips::{ChoiceChip, ChoiceChips, ChoiceKind, ChoiceState};
use crate::components::choice_list::{ChoiceList, ChoiceRow};
use crate::components::price::{Price, PriceSize};
use crate::format;
use crate::prefs::{Lang, Numerals, tr};
use crate::server::TenantCatalog;
use domain::{OptionKind, Piastres, SelectionType, Variant};
use leptos::prelude::*;
use serde::Serialize;

/// What the browser code needs to switch variants without asking the server.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PickerData {
    product: u32,
    /// The product page (`?v=` is added per variant): the cart line's link.
    href: String,
    /// A value's no-JS link, `{v}` standing for the variant id.
    chip_href: String,
    name: String,
    selected: u32,
    /// Most of one line (stock caps it further).
    max_qty: u32,
    /// The product has something to choose (variants or add-ons): its cart line offers "Edit".
    editable: bool,
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

/// "You save EGP 200.00" for a variant on sale.
pub fn saving(v: &Variant, lang: Lang, num: Numerals) -> Option<String> {
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

/// One picker: the product, the variant shown, and where it is (`pp` / `le`, see the module docs).
pub struct Picker<'a> {
    pub pv: &'a ProductView,
    pub cur: &'a Variant,
    pub prefix: &'static str,
    pub lang: Lang,
    pub num: Numerals,
}

impl Picker<'_> {
    fn price_tpl(&self, i: usize) -> String {
        format!("{}-price-{i}", self.prefix)
    }

    /// An add-on group's fieldset id (its hint and error lines hang off it).
    fn group_id(&self, group: &str) -> String {
        format!("{}-g-{}-{group}", self.prefix, self.pv.product.id.0)
    }

    /// The page's JSON for `<script type="application/json" data-ts-json>`. `chip_href` is a value's
    /// no-JS link with `{v}` for the variant id.
    pub fn json(&self, chip_href: &str) -> String {
        let (pv, lang, num) = (self.pv, self.lang, self.num);
        let p = &pv.product;
        let keys = price_keys(&p.variants);
        let max_qty = domain::cart::MAX_QTY;
        let data = PickerData {
            product: p.id.0,
            href: format!("/p/{}", p.slug),
            chip_href: chip_href.to_owned(),
            name: text(&p.name, lang),
            selected: self.cur.id.0,
            max_qty,
            editable: p.has_choices(),
            variants: p
                .variants
                .iter()
                .map(|v| {
                    let (labels, values, notes) = axis_states(pv, v, lang);
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
                        price_tpl: self.price_tpl(price_key_index(&keys, v)),
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
                        id: self.group_id(&g.id),
                        min,
                        max: (max != u32::MAX).then_some(max),
                        problem: group_problem(g, lang, num),
                    }
                })
                .collect(),
        };
        serde_json::to_value(&data)
            .map(|v| script_json(&v))
            .unwrap_or_default()
    }

    /// One `<template id="tpl-{prefix}-price-{i}">` per distinct price: the variant's `Price`, at
    /// the size the picker shows it.
    pub fn price_templates(&self, size: PriceSize) -> AnyView {
        let (lang, num, prefix) = (self.lang, self.num, self.prefix);
        price_keys(&self.pv.product.variants)
            .into_iter()
            .enumerate()
            .map(move |(i, (amount, was))| {
                view! {
                    <template id=format!("tpl-{prefix}-price-{i}")>
                        <Price amount=amount compare_at=was size=size lang numerals=num/>
                    </template>
                }
            })
            .collect_view()
            .into_any()
    }

    /// The variant picker: one `ChoiceChips` group per axis, every value a link (`href(variant)`).
    pub fn axes(&self, href: impl Fn(u32) -> String) -> AnyView {
        let (pv, lang) = (self.pv, self.lang);
        let p = &pv.product;
        let pid = p.id.0;
        let (_, states, notes) = axis_states(pv, self.cur, lang);
        p.options
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
                            chip = chip.href(href(v));
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
                view! {
                    <ChoiceChips
                        label=text(&axis.name, lang)
                        items=items
                        kind=kind
                        show_value=true
                        id=format!("{}-ax-{pid}-{ai}", self.prefix)
                        note=notes[ai].clone()
                        attr:data-axis=ai.to_string()
                    />
                }
            })
            .collect_view()
            .into_any()
    }

    /// The add-on groups (`ChoiceList`), with `picked` ticked (none: each group's default).
    /// Checkboxes post as `addon`; a single-choice group's radios need a name of their own (one
    /// radio group each): `addon.<group>`.
    pub fn groups(&self, picked: &[String]) -> AnyView {
        let (lang, num) = (self.lang, self.num);
        self.pv
            .product
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
                    let none = !g.options.iter().any(|o| picked.contains(&o.id));
                    rows.push(ChoiceRow::new("", tr(lang, "No, thanks", "لا، شكرًا")).checked(none));
                }
                rows.extend(g.options.iter().map(|o| {
                    ChoiceRow::new(o.id.clone(), text(&o.name, lang))
                        .meta(surcharge(o.price, lang, num))
                        .checked(picked.contains(&o.id))
                }));
                view! {
                    <ChoiceList
                        legend=text(&g.name, lang)
                        name=name
                        items=rows
                        single=single
                        id=self.group_id(&g.id)
                        hint=group_hint(g, lang, num)
                    />
                }
            })
            .collect_view()
            .into_any()
    }
}
