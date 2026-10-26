//! Segmented controls, one look, two kinds.
//!
//! [`Segmented`]: links — sort orders, list/grid views, language, theme. Each option is a plain
//! link (works without JS; the router swaps the page in place), the current one marked
//! `aria-current="true"`. Extra attributes go on an option with [`SegItem::attr`]
//! (`.attr("data-router", "off")` for a link that must be a full page load).
//!
//! [`SegmentedToggle`]: buttons with `aria-pressed`, for choices that are state rather than pages
//! (spring presets, filter toggles). One pressed at a time, or any number with `multiple`.
//! `client/src/core/segmented.ts` (`data-ts="seg-toggle"`) keeps `aria-pressed` in sync and fires a
//! bubbling `change` on the group, `detail: { value, values }`.

use super::icons::Icon;
use leptos::attr::custom::custom_attribute;
use leptos::prelude::*;
use leptos::tachys::html::attribute::any_attribute::{AnyAttribute, IntoAnyAttribute};

#[derive(Clone, Debug, PartialEq)]
pub struct SegItem {
    pub label: String,
    /// A link's address ([`Segmented`]) or a toggle's value ([`SegmentedToggle`]).
    pub href: String,
    /// The current page ([`Segmented`]) or a pressed toggle ([`SegmentedToggle`]).
    pub current: bool,
    /// A lucide icon before the label.
    pub icon: Option<&'static str>,
    /// `lang` for a label written in another language (the language switch).
    pub lang: Option<&'static str>,
    /// Extra attributes on the option (`data-*`, `aria-*`, `title`).
    pub attrs: Vec<(&'static str, String)>,
}

impl SegItem {
    pub fn new(label: impl Into<String>, href: impl Into<String>, current: bool) -> Self {
        SegItem {
            label: label.into(),
            href: href.into(),
            current,
            icon: None,
            lang: None,
            attrs: Vec::new(),
        }
    }
    /// A toggle option ([`SegmentedToggle`]): `value` is what `change` reports.
    pub fn toggle(label: impl Into<String>, value: impl Into<String>, pressed: bool) -> Self {
        Self::new(label, value, pressed)
    }
    pub fn icon(mut self, icon: &'static str) -> Self {
        self.icon = Some(icon);
        self
    }
    pub fn lang(mut self, lang: &'static str) -> Self {
        self.lang = Some(lang);
        self
    }
    /// One more attribute on the option: `.attr("data-router", "off")`.
    pub fn attr(mut self, name: &'static str, value: impl Into<String>) -> Self {
        self.attrs.push((name, value.into()));
        self
    }
}

fn extra(attrs: Vec<(&'static str, String)>) -> Vec<AnyAttribute> {
    attrs
        .into_iter()
        .map(|(k, v)| custom_attribute(k, v).into_any_attr())
        .collect()
}

fn group_class(full: bool, class: &str) -> String {
    let mut cls = String::from("seg");
    if full {
        cls.push_str(" seg--full");
    }
    if !class.is_empty() {
        cls.push(' ');
        cls.push_str(class);
    }
    cls
}

#[component]
pub fn Segmented(
    items: Vec<SegItem>,
    /// Names the group for screen readers ("Sort by", "Theme").
    #[prop(into)]
    label: String,
    /// Stretch to the container's width, segments sharing it equally.
    #[prop(optional)]
    full: bool,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! {
        <div class=group_class(full, &class) role="group" aria-label=label>
            {items
                .into_iter()
                .map(|it| {
                    let current = if it.current { "true" } else { "false" };
                    view! {
                        <a
                            class="seg__item"
                            href=it.href
                            aria-current=current
                            lang=it.lang
                            data-press=""
                            {..extra(it.attrs)}
                        >
                            {it.icon.map(|i| view! { <Icon name=i size=16/> })}
                            <span class="seg__label">{it.label}</span>
                        </a>
                    }
                })
                .collect_view()}
        </div>
    }
}

/// The same control as buttons with `aria-pressed` (state, not pages). Build the options with
/// [`SegItem::toggle`]`(label, value, pressed)`.
#[component]
pub fn SegmentedToggle(
    items: Vec<SegItem>,
    /// Names the group for screen readers ("Spring", "Show").
    #[prop(into)]
    label: String,
    /// Any number pressed (filter toggles); otherwise exactly one (presets).
    #[prop(optional)]
    multiple: bool,
    #[prop(optional)] full: bool,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! {
        <div
            class=group_class(full, &class)
            role="group"
            aria-label=label
            data-ts="seg-toggle"
            data-multiple=multiple.then_some("")
        >
            {items
                .into_iter()
                .map(|it| {
                    let pressed = if it.current { "true" } else { "false" };
                    view! {
                        <button
                            type="button"
                            class="seg__item"
                            value=it.href
                            aria-pressed=pressed
                            lang=it.lang
                            data-press=""
                            {..extra(it.attrs)}
                        >
                            {it.icon.map(|i| view! { <Icon name=i size=16/> })}
                            <span class="seg__label">{it.label}</span>
                        </button>
                    }
                })
                .collect_view()}
        </div>
    }
}
