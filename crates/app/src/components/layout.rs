//! Page layout pieces: the content container, sections, section headers and the horizontal rail.

use super::button::IconButton;
use super::icons::Icon;
use crate::prefs::{lang_now, tr};
use leptos::prelude::*;

/// The content column: max 1320px of content with the fluid gutter outside it.
#[component]
pub fn Container(
    children: Children,
    /// `narrow` (760px, reading/forms) or `mid` (1080px, cart/checkout).
    #[prop(optional)]
    width: Option<&'static str>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let mut cls = String::from("container");
    match width {
        Some("narrow") => cls.push_str(" container--narrow"),
        Some("mid") => cls.push_str(" container--mid"),
        _ => {}
    }
    if !class.is_empty() {
        cls.push(' ');
        cls.push_str(&class);
    }
    view! { <div class=cls>{children()}</div> }
}

/// A page section with the standard vertical rhythm.
#[component]
pub fn Section(
    children: Children,
    #[prop(optional, into)] id: Option<String>,
    /// The id of the heading that names this section.
    #[prop(optional, into)]
    labelledby: Option<String>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let cls = if class.is_empty() {
        "section".to_owned()
    } else {
        format!("section {class}")
    };
    view! {
        <section class=cls id=id aria-labelledby=labelledby>
            {children()}
        </section>
    }
}

/// A section's title row: the heading (an `<h2>`), an optional line under it, and an optional
/// "See all" link at the end (its chevron mirrors in Arabic).
#[component]
pub fn SectionHeader(
    #[prop(into)] title: String,
    #[prop(optional, into)] subtitle: Option<String>,
    #[prop(optional, into)] href: Option<String>,
    /// Defaults to "See all" / "عرض الكل".
    #[prop(optional, into)]
    link_label: Option<String>,
    /// The heading's id, for the section's `aria-labelledby`.
    #[prop(optional, into)]
    id: Option<String>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let cls = if class.is_empty() {
        "section-head".to_owned()
    } else {
        format!("section-head {class}")
    };
    let link = href.map(|h| {
        let label = link_label.unwrap_or_else(|| tr(lang_now(), "See all", "عرض الكل").to_owned());
        view! {
            <a class="section-head__link" href=h>
                <span>{label}</span>
                <Icon name="chevron-right" size=18 class="flip-rtl"/>
            </a>
        }
    });
    view! {
        <div class=cls>
            <div class="section-head__text">
                <h2 class="section-head__title" id=id>{title}</h2>
                {subtitle.map(|s| view! { <p class="section-head__sub">{s}</p> })}
            </div>
            {link}
        </div>
    }
}

/// A horizontal, scroll-snapping row (product rails, category tiles). It bleeds into the gutter
/// on phones so the next item peeks, scrolls the right way in Arabic, hides its scrollbar, and on
/// desktop shows prev/next buttons (driven by the shell's small script).
///
/// Each direct child is one item; items are sized by `--rail-cols` (2.3 / 3.3 / 4.3 / 5 visible).
#[component]
pub fn Rail(
    children: Children,
    /// Names the scrollable region for screen readers (e.g. the section title).
    #[prop(optional, into)]
    label: Option<String>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let lang = lang_now();
    let cls = if class.is_empty() {
        "rail".to_owned()
    } else {
        format!("rail {class}")
    };
    view! {
        <div class=cls data-rail="" data-at="start">
            <IconButton
                icon="chevron-left"
                label=tr(lang, "Previous", "السابق")
                class="rail__btn rail__btn--prev"
                attr:data-rail-step="-1"
                attr:tabindex="-1"
            />
            <div class="rail__track" role=label.is_some().then_some("region") aria-label=label tabindex="0">
                {children()}
            </div>
            <IconButton
                icon="chevron-right"
                label=tr(lang, "Next", "التالي")
                class="rail__btn rail__btn--next"
                attr:data-rail-step="1"
                attr:tabindex="-1"
            />
        </div>
    }
}
