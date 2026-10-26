//! Prices in Madar's money shape: `EGP 1,299.00` / `‎1,299.00 ج.م` — Plex Mono tabular figures,
//! the piastres smaller and lighter, the currency label set in the language's own face (after the
//! figure in Arabic), a struck compare-at price and an optional saving badge.
//!
//! Screen readers get one sentence (`format::money`) from a visually hidden span; the visual
//! parts are `aria-hidden`. The DOM order of the parts is the same in both languages (CSS orders
//! them), so the two languages render the same element tree.

use crate::format::{self, MoneyParts};
use crate::prefs::{Lang, Numerals, tr};
use domain::Piastres;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PriceSize {
    /// 15px — rails, cart lines.
    Sm,
    /// 17px — product cards.
    #[default]
    Md,
    /// 20px — totals.
    Lg,
    /// 30px — the product page.
    Xl,
}

impl PriceSize {
    fn class(self) -> &'static str {
        match self {
            PriceSize::Sm => "price--sm",
            PriceSize::Md => "price--md",
            PriceSize::Lg => "price--lg",
            PriceSize::Xl => "price--xl",
        }
    }
}

/// The saving as a short label: `−20%` / `خصم 20%` (`خصم ٢٠٪` with Arabic-Indic digits).
pub fn off_label(pct: u8, lang: Lang, numerals: Numerals) -> String {
    let n = format::count(pct as u32, numerals);
    let sign = if numerals == Numerals::Arab {
        '\u{066A}'
    } else {
        '%'
    };
    match lang {
        Lang::En => format!("{}{n}{sign}", format::MINUS),
        Lang::Ar => format!("خصم {n}{sign}"),
    }
}

/// The whole price as one spoken sentence.
pub fn spoken(
    amount: Piastres,
    compare_at: Option<Piastres>,
    from: bool,
    lang: Lang,
    numerals: Numerals,
) -> String {
    let now = format::money(amount, lang, numerals);
    let mut s = if from {
        format!("{} {now}", tr(lang, "From", "من"))
    } else {
        now
    };
    if let Some(was) = compare_at.filter(|c| *c > amount) {
        let pct = was.percent_off(amount);
        let was = format::money(was, lang, numerals);
        let pct = format::count(pct as u32, numerals);
        match lang {
            Lang::En => s.push_str(&format!(", was {was}, {pct}% off")),
            Lang::Ar => s.push_str(&format!("، بدلًا من {was}، خصم {pct}%")),
        }
    }
    s
}

/// The markup shared by [`Price`] and [`PriceLive`].
#[allow(clippy::too_many_arguments)]
fn price_view(
    amount: Piastres,
    compare_at: Option<Piastres>,
    size: PriceSize,
    from: bool,
    badge: bool,
    lang: Lang,
    numerals: Numerals,
    class: &str,
) -> impl IntoView + use<> {
    let m = MoneyParts::new(amount, lang, numerals);
    let was = compare_at.filter(|c| *c > amount);
    let cls = {
        let mut c = format!("price {}", size.class());
        if was.is_some() {
            c.push_str(" price--deal");
        }
        if !class.is_empty() {
            c.push(' ');
            c.push_str(class);
        }
        c
    };
    let said = spoken(amount, compare_at, from, lang, numerals);
    let from_label = from.then(|| tr(lang, "From", "من"));
    let was_view = was.map(|w| {
        let wm = MoneyParts::new(w, lang, numerals);
        view! {
            <s class="price__was mono" aria-hidden="true">
                <span class="price__was-int">{wm.whole}</span>
                <span class="price__was-dec">{wm.fraction}</span>
            </s>
        }
    });
    let off = was.filter(|_| badge).map(|w| {
        let label = off_label(w.percent_off(amount), lang, numerals);
        view! { <span class="price__off" aria-hidden="true">{label}</span> }
    });
    view! {
        <span class=cls>
            <span class="sr-only">{said}</span>
            <span class="price__now" aria-hidden="true">
                {from_label.map(|l| view! { <span class="price__from">{l}</span> })}
                <span class="price__sign">{m.sign}</span>
                <span class="price__cur">{m.label}</span>
                <span class="price__fig mono">
                    <span class="price__int">{m.whole}</span>
                    <span class="price__dec">{m.fraction}</span>
                </span>
            </span>
            {was_view}
            {off}
        </span>
    }
}

/// A price for server components.
#[component]
pub fn Price(
    amount: Piastres,
    /// The higher "was" price, struck through with the saving. Takes a `Piastres` or an
    /// `Option<Piastres>` (e.g. `variant.compare_at`).
    #[prop(optional_no_strip, into)]
    compare_at: Option<Piastres>,
    #[prop(optional)] size: PriceSize,
    /// Prefix "From" (variants with different prices).
    #[prop(optional)]
    from: bool,
    /// Hide the `−20%` badge (when the saving is shown elsewhere).
    #[prop(optional)]
    no_badge: bool,
    lang: Lang,
    numerals: Numerals,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    price_view(
        amount, compare_at, size, from, !no_badge, lang, numerals, &class,
    )
}
