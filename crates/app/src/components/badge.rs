//! Merchandising badges on product cards and pages: small, opaque (they sit on photos), an icon
//! and a word.

use super::icons::Icon;
use super::price::off_label;
use crate::format;
use crate::prefs::{Lang, Numerals, tr};
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BadgeKind {
    /// Percent saved.
    Deal(u8),
    New,
    /// Delivered within hours.
    Express,
    Bestseller,
    OutOfStock,
    /// Only this many left.
    LowStock(u32),
}

impl BadgeKind {
    fn parts(self, lang: Lang, numerals: Numerals) -> (&'static str, Option<&'static str>, String) {
        match self {
            BadgeKind::Deal(p) => (
                "badge--deal",
                Some("badge-percent"),
                off_label(p, lang, numerals),
            ),
            BadgeKind::New => (
                "badge--new",
                Some("sparkles"),
                tr(lang, "New", "جديد").into(),
            ),
            BadgeKind::Express => (
                "badge--express",
                Some("zap"),
                tr(lang, "Express", "توصيل سريع").into(),
            ),
            BadgeKind::Bestseller => (
                "badge--best",
                Some("flame"),
                tr(lang, "Bestseller", "الأكثر مبيعًا").into(),
            ),
            BadgeKind::OutOfStock => (
                "badge--oos",
                None,
                tr(lang, "Out of stock", "غير متوفر").into(),
            ),
            BadgeKind::LowStock(n) => {
                let n = format::count(n, numerals);
                let label = match lang {
                    Lang::En => format!("Only {n} left"),
                    Lang::Ar => format!("فاضل {n} بس"),
                };
                ("badge--low", Some("timer"), label)
            }
        }
    }
}

#[component]
pub fn Badge(
    kind: BadgeKind,
    lang: Lang,
    numerals: Numerals,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let (tone, icon, label) = kind.parts(lang, numerals);
    let cls = if class.is_empty() {
        format!("badge {tone}")
    } else {
        format!("badge {tone} {class}")
    };
    view! {
        <span class=cls>
            {icon.map(|i| view! { <Icon name=i size=14/> })}
            <span class="badge__label">{label}</span>
        </span>
    }
}
