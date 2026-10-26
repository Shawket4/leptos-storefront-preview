//! The product page's rules, applied once on the server: which variant a pick leads to, why a
//! value is unavailable (and where it is in stock), stock wording, how add-on groups validate.
//! The server renders the page with them and precomputes every variant's outcome into the page's
//! JSON, so the browser switches variants without asking (`features/product.ts`).

use crate::format;
use crate::prefs::{Lang, Numerals, tr};
use domain::{L10n, ModifierGroup, Piastres, ProductOption, SelectionType, Variant};

/// `sizes` of the main gallery image; the `<head>` preload uses the same string so the browser
/// picks the same file twice.
pub const STAGE_SIZES: &str =
    "(min-width: 1280px) 600px, (min-width: 960px) 46vw, (min-width: 768px) 46vw, 100vw";

/// "Only N left" from this many down.
pub const LOW_STOCK: u32 = 5;

/// The outcome of a no-JS add, from the cart's 303 back here (AGENTS.md, "No-JS cart forms").
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Notice {
    /// `?added=<qty>`.
    Added(u32),
    /// `?cart_err=<code>`.
    Error(String),
}

pub fn text(l: &L10n, lang: Lang) -> String {
    l.pick(lang.is_rtl()).to_owned()
}

// ── Variants ────────────────────────────────────────────────────────────────────────────────

/// The variant with this id, else the first (a stale `?v=` never breaks the page).
pub fn variant(vs: &[Variant], id: u32) -> &Variant {
    vs.iter().find(|v| v.id.0 == id).unwrap_or(&vs[0])
}

pub fn by_opts<'a>(vs: &'a [Variant], opts: &[String]) -> Option<&'a Variant> {
    vs.iter().find(|v| v.opts == opts)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Avail {
    /// The current pick on this axis.
    Selected,
    /// Picking it lands on an in-stock variant (`target`).
    Open(u32),
    /// That combination exists but is sold out.
    SoldOut,
    /// The catalogue has no such combination.
    Missing,
}

impl Avail {
    pub fn enabled(self) -> bool {
        matches!(self, Avail::Selected | Avail::Open(_))
    }
}

/// What picking `value` on `axis` would do, given the current variant.
pub fn avail(vs: &[Variant], current: &Variant, axis: usize, value: &str) -> Avail {
    if current.opts.get(axis).map(String::as_str) == Some(value) {
        return Avail::Selected;
    }
    let mut opts = current.opts.clone();
    if axis >= opts.len() {
        return Avail::Missing;
    }
    opts[axis] = value.to_owned();
    match by_opts(vs, &opts) {
        Some(v) if v.stock > 0 => Avail::Open(v.id.0),
        Some(_) => Avail::SoldOut,
        None => Avail::Missing,
    }
}

fn value_label<'a>(axis: &'a ProductOption, id: &str) -> Option<&'a L10n> {
    axis.values.iter().find(|v| v.id == id).map(|v| &v.label)
}

/// Why a value can't be picked right now, and where it *is* available:
/// "Silver: sold out in 128 GB · in stock in 256 GB, 512 GB".
pub fn reason(
    axes: &[ProductOption],
    vs: &[Variant],
    current: &Variant,
    axis: usize,
    value: &str,
    lang: Lang,
) -> String {
    let Some(this) = axes.get(axis).and_then(|a| value_label(a, value)) else {
        return String::new();
    };
    let state = avail(vs, current, axis, value);
    let others: Vec<String> = axes
        .iter()
        .enumerate()
        .filter(|(i, _)| *i != axis)
        .filter_map(|(i, a)| current.opts.get(i).and_then(|o| value_label(a, o)))
        .map(|l| text(l, lang))
        .collect();
    let with = others.join(" · ");
    let what = match (state, others.is_empty()) {
        (Avail::SoldOut, true) => tr(lang, "sold out", "مش متوفر حاليًا").to_owned(),
        (Avail::SoldOut, false) => match lang {
            Lang::En => format!("sold out in {with}"),
            Lang::Ar => format!("مش متوفر حاليًا في {with}"),
        },
        (_, true) => tr(lang, "not available", "مش متاح").to_owned(),
        (_, false) => match lang {
            Lang::En => format!("not offered in {with}"),
            Lang::Ar => format!("مش متاح في {with}"),
        },
    };
    // Two axes: name the values of the other axis that this one *is* in stock with.
    let alts: Vec<String> = if axes.len() == 2 {
        let other = 1 - axis;
        axes[other]
            .values
            .iter()
            .filter(|ov| {
                vs.iter().any(|v| {
                    v.stock > 0
                        && v.opts.get(axis).map(String::as_str) == Some(value)
                        && v.opts.get(other) == Some(&ov.id)
                })
            })
            .map(|ov| text(&ov.label, lang))
            .collect()
    } else {
        vec![]
    };
    let alts = if alts.is_empty() {
        String::new()
    } else {
        match lang {
            Lang::En => format!(" · in stock in {}", alts.join(", ")),
            Lang::Ar => format!(" · متوفر في {}", alts.join("، ")),
        }
    };
    format!("{}: {what}{alts}", text(this, lang))
}

/// The `?v=` link for a variant, keeping nothing else (the canonical form of a variant URL).
pub fn variant_href(slug: &str, id: u32) -> String {
    format!("/p/{slug}?v={id}")
}

// ── Stock ───────────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stock {
    In,
    Low(u32),
    Out,
}

pub fn stock(n: u32) -> Stock {
    match n {
        0 => Stock::Out,
        n if n <= LOW_STOCK => Stock::Low(n),
        _ => Stock::In,
    }
}

pub fn stock_text(s: Stock, lang: Lang, numerals: Numerals) -> String {
    match s {
        Stock::In => tr(lang, "In stock", "متوفر").to_owned(),
        Stock::Low(n) => {
            let n = format::count(n, numerals);
            match lang {
                Lang::En => format!("Only {n} left"),
                Lang::Ar => format!("فاضل {n} بس"),
            }
        }
        Stock::Out => tr(lang, "Out of stock", "غير متوفر").to_owned(),
    }
}

// ── Add-on groups (Madar modifier groups) ───────────────────────────────────────────────────

/// `(min, max)` picks, exactly as the server checks them (`api::cart::check_addons`).
pub fn bounds(g: &ModifierGroup) -> (u32, u32) {
    let min = if g.required { g.min.max(1) } else { g.min };
    let max = match g.selection {
        SelectionType::Single => 1,
        SelectionType::Multi => g.max.unwrap_or(u32::MAX),
    };
    (min, max)
}

/// "Required" / "Optional" / "Choose up to 2" / "Choose 1–3".
pub fn group_hint(g: &ModifierGroup, lang: Lang, numerals: Numerals) -> String {
    let (min, max) = bounds(g);
    let n = |x: u32| format::count(x, numerals);
    match (g.selection, min, max) {
        (SelectionType::Single, 0, _) => tr(lang, "Optional", "اختياري").to_owned(),
        (SelectionType::Single, _, _) => tr(lang, "Required", "مطلوب").to_owned(),
        (SelectionType::Multi, 0, u32::MAX) => {
            tr(lang, "Optional · pick any", "اختياري · اختار براحتك").to_owned()
        }
        (SelectionType::Multi, 0, m) => match lang {
            Lang::En => format!("Optional · up to {}", n(m)),
            Lang::Ar => format!("اختياري · لحد {}", n(m)),
        },
        (SelectionType::Multi, lo, u32::MAX) => match lang {
            Lang::En => format!("Required · at least {}", n(lo)),
            Lang::Ar => format!("مطلوب · {} على الأقل", n(lo)),
        },
        (SelectionType::Multi, lo, hi) => match lang {
            Lang::En => format!("Required · {}–{}", n(lo), n(hi)),
            Lang::Ar => format!("مطلوب · من {} لـ{}", n(lo), n(hi)),
        },
    }
}

/// The message under a group that blocks the add.
pub fn group_problem(g: &ModifierGroup, lang: Lang, numerals: Numerals) -> String {
    let (min, _) = bounds(g);
    match (g.selection, min) {
        (SelectionType::Single, _) => tr(
            lang,
            "Please choose one option.",
            "يُرجى اختيار أحد الخيارات.",
        )
        .to_owned(),
        (SelectionType::Multi, m) => {
            let m = format::count(m.max(1), numerals);
            match lang {
                Lang::En => format!("Please choose at least {m}."),
                Lang::Ar => format!("يُرجى اختيار {m} على الأقل."),
            }
        }
    }
}

/// `+ EGP 15.00` / `مجانًا`.
pub fn surcharge(p: Piastres, lang: Lang, numerals: Numerals) -> String {
    if p.0 == 0 {
        tr(lang, "Free", "مجانًا").to_owned()
    } else {
        format!("+ {}", format::money(p, lang, numerals))
    }
}
