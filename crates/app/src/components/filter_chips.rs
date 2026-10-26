//! Active-filter chips: each one a link to the same page without that filter (works without JS),
//! labelled "Remove filter: …" for screen readers, an × at its end; an optional "Clear all"
//! after them. 32px squircles on the sunk grey, 44px to the finger.
//!
//! Browser code that applies a removal itself hooks onto the chips with their attributes
//! (`FilterChip::attr`) and onto the list with `attr:` on the component.
//!
//! ```ignore
//! let chips = vec![FilterChip::new("Samsung", without_brand_href).attr("data-clear", "brand")];
//! <FilterChips items=chips clear_all=FilterChip::clear_all(cleared_href) scroll=true attr:data-router="off"/>
//! ```

use super::attrs::{Attrs, spread};
use super::icons::Icon;
use crate::prefs::{lang_now, tr};
use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct FilterChip {
    /// What the filter is ("Samsung", "EGP 100 – 500", "In stock").
    pub label: String,
    /// This page without the filter.
    pub href: String,
    /// Extra attributes on the link (`data-*`).
    pub attrs: Attrs,
}

impl FilterChip {
    pub fn new(label: impl Into<String>, href: impl Into<String>) -> Self {
        FilterChip {
            label: label.into(),
            href: href.into(),
            attrs: Vec::new(),
        }
    }
    /// The "Clear all" link (in the page's language) to this page without any filter.
    pub fn clear_all(href: impl Into<String>) -> Self {
        Self::new(tr(lang_now(), "Clear all", "امسح الكل"), href)
    }
    /// One more attribute on the link: `.attr("data-clear", "brand")`.
    pub fn attr(mut self, name: &'static str, value: impl Into<String>) -> Self {
        self.attrs.push((name, value.into()));
        self
    }
}

#[component]
pub fn FilterChips(
    items: Vec<FilterChip>,
    /// A last link that removes every filter ([`FilterChip::clear_all`]).
    #[prop(optional)]
    clear_all: Option<FilterChip>,
    /// Names the list for screen readers; "Active filters" by default.
    #[prop(optional, into)]
    label: Option<String>,
    /// Phones: one line that scrolls sideways, edge to edge of the page column; from 768px the
    /// chips wrap.
    #[prop(optional)]
    scroll: bool,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let lang = lang_now();
    let mut cls = String::from("filter-chips");
    if scroll {
        cls.push_str(" filter-chips--scroll");
    }
    if !class.is_empty() {
        cls.push(' ');
        cls.push_str(&class);
    }
    let label = label.unwrap_or_else(|| tr(lang, "Active filters", "الفلاتر المختارة").to_owned());
    let remove = tr(lang, "Remove filter", "شيل الفلتر");
    let empty = items.is_empty();
    view! {
        <ul class=cls role="list" aria-label=label hidden=empty>
            {items
                .into_iter()
                .map(|it| {
                    let aria = format!("{remove}: {}", it.label);
                    view! {
                        <li>
                            <a class="filter-chip" href=it.href aria-label=aria data-press="" {..spread(it.attrs)}>
                                <span class="filter-chip__label">{it.label}</span>
                                <Icon name="x" size=14/>
                            </a>
                        </li>
                    }
                })
                .collect_view()}
            {clear_all.map(|c| {
                view! {
                    <li>
                        <a class="filter-chips__clear" href=c.href data-press="" {..spread(c.attrs)}>
                            <span class="filter-chip__label">{c.label}</span>
                        </a>
                    </li>
                }
            })}
        </ul>
    }
}
