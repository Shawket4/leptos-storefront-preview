//! Language, numerals and theme controls — plain links to `/__prefs?…&next=<this page>`.
//!
//! No island: the server sets the cookie and redirects back; the islands router follows the link
//! and patches the page in place (and `client::init` mirrors the new `<html>` attributes into the
//! `Ui` store, so islands re-render their own text). Without JS it is an ordinary page load.
//!
//! `Menu` (header) is a `<details>` disclosure — a popover anchored to its trigger that works
//! without JS; the shell's script adds light dismiss (outside click, Escape) and the exit
//! motion. `Inline` (footer) lays the three groups out in the open.

use super::icons::Icon;
use super::segmented::{SegItem, Segmented};
use crate::prefs::{Lang, Numerals, Prefs, Theme, tr};
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PrefsVariant {
    /// A compact popover behind one round button (header).
    #[default]
    Menu,
    /// All three groups in the open (footer).
    Inline,
}

/// Percent-encode a query component (everything but unreserved characters and `/`).
pub fn encode_component(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 8);
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' | b'/' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

/// The `/__prefs` link that sets one preference and comes back to `next`.
pub fn prefs_href(key: &str, value: &str, next: &str) -> String {
    format!("/__prefs?{key}={value}&next={}", encode_component(next))
}

fn groups(p: Prefs, next: &str) -> impl IntoView + use<> {
    let lang = p.lang;
    let lang_items = [Lang::En, Lang::Ar]
        .into_iter()
        .map(|l| {
            SegItem::new(l.endonym(), prefs_href("lang", l.code(), next), p.lang == l)
                .lang(l.code())
        })
        .collect::<Vec<_>>();
    let num_items = vec![
        SegItem::new(
            "123",
            prefs_href("num", "latn", next),
            p.numerals == Numerals::Latn,
        ),
        SegItem::new(
            "١٢٣",
            prefs_href("num", "arab", next),
            p.numerals == Numerals::Arab,
        ),
    ];
    let theme_items = [
        (Theme::Light, "sun", tr(lang, "Light", "فاتح")),
        (Theme::Dark, "moon", tr(lang, "Dark", "داكن")),
        (Theme::System, "monitor", tr(lang, "Auto", "تلقائي")),
    ]
    .into_iter()
    .map(|(t, icon, label)| {
        SegItem::new(label, prefs_href("theme", t.code(), next), p.theme == t).icon(icon)
    })
    .collect::<Vec<_>>();

    let l_lang = tr(lang, "Language", "اللغة");
    let l_num = tr(lang, "Numbers", "الأرقام");
    let l_theme = tr(lang, "Appearance", "المظهر");
    view! {
        <div class="prefs__group">
            <p class="prefs__label">
                <Icon name="languages" size=16/>
                <span>{l_lang}</span>
            </p>
            <Segmented items=lang_items label=l_lang full=true/>
        </div>
        <div class="prefs__group">
            <p class="prefs__label">
                <Icon name="percent" size=16/>
                <span>{l_num}</span>
            </p>
            <Segmented items=num_items label=l_num full=true class="seg--figures"/>
        </div>
        <div class="prefs__group">
            <p class="prefs__label">
                <Icon name="sun" size=16/>
                <span>{l_theme}</span>
            </p>
            <Segmented items=theme_items label=l_theme full=true/>
        </div>
    }
}

/// The preference controls for the current request. Server component (reads `request::req()`).
#[component]
pub fn PrefsControls(#[prop(optional)] variant: PrefsVariant) -> impl IntoView {
    let rq = crate::request::req();
    let p = rq.prefs;
    let next = rq.path_and_query.clone();
    let lang = p.lang;
    match variant {
        PrefsVariant::Inline => view! {
            <div class="prefs prefs--inline">{groups(p, &next)}</div>
        }
        .into_any(),
        PrefsVariant::Menu => {
            let label = tr(lang, "Language and display", "اللغة وطريقة العرض");
            let short = match lang {
                Lang::En => "EN",
                Lang::Ar => "ع",
            };
            view! {
                <details class="prefs prefs--menu" data-popover="">
                    <summary class="prefs__trigger" aria-label=label data-press="">
                        <Icon name="globe" size=20/>
                        <span class="prefs__code" aria-hidden="true">{short}</span>
                    </summary>
                    <div class="prefs__panel">{groups(p, &next)}</div>
                </details>
            }
            .into_any()
        }
    }
}
