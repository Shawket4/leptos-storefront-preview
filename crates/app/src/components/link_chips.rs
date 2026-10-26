//! Link chips: a row of ways forward — popular searches and aisles under an empty result, every
//! aisle on a category 404. Each chip is a plain link (icon + label) on a raised squircle, 44px
//! tall; the row wraps. A small label names the row (and the list, for screen readers).
//!
//! ```ignore
//! let items = aisles.iter().map(|c| LinkChip::new(name(c), format!("/c/{}", c.slug)).icon(&c.icon)).collect();
//! <LinkChips items label=tr(lang, "Popular aisles", "أقسام عليها الطلب") center=true/>
//! ```

use super::attrs::{Attrs, spread};
use super::icons::{Icon, IconName};
use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct LinkChip {
    pub label: String,
    pub href: String,
    /// A lucide icon before the label (a literal, or a category's icon from data).
    pub icon: Option<IconName>,
    /// Extra attributes on the link (`data-*`, `rel`).
    pub attrs: Attrs,
}

impl LinkChip {
    pub fn new(label: impl Into<String>, href: impl Into<String>) -> Self {
        LinkChip {
            label: label.into(),
            href: href.into(),
            icon: None,
            attrs: Vec::new(),
        }
    }
    pub fn icon(mut self, icon: impl Into<IconName>) -> Self {
        self.icon = Some(icon.into());
        self
    }
    /// One more attribute on the link: `.attr("data-router", "off")`.
    pub fn attr(mut self, name: &'static str, value: impl Into<String>) -> Self {
        self.attrs.push((name, value.into()));
        self
    }
}

#[component]
pub fn LinkChips(
    items: Vec<LinkChip>,
    /// Names the row ("Popular searches"): shown above it, and the list's name for screen
    /// readers.
    #[prop(into)]
    label: String,
    /// Keep the label for screen readers only.
    #[prop(optional)]
    hide_label: bool,
    /// Centre the label and the chips (empty states, a 404).
    #[prop(optional)]
    center: bool,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let mut cls = String::from("link-chips");
    if center {
        cls.push_str(" link-chips--center");
    }
    if !class.is_empty() {
        cls.push(' ');
        cls.push_str(&class);
    }
    // The visible label repeats the list's name, so it's hidden from screen readers.
    let shown = (!hide_label).then(|| {
        view! { <p class="link-chips__label" aria-hidden="true">{label.clone()}</p> }
    });
    view! {
        <div class=cls>
            {shown}
            <ul class="link-chips__list" role="list" aria-label=label>
                {items
                    .into_iter()
                    .map(|it| {
                        view! {
                            <li>
                                <a class="link-chip" href=it.href data-press="" {..spread(it.attrs)}>
                                    {it.icon.map(|i| view! { <Icon name=i size=18/> })}
                                    <span class="link-chip__label" dir="auto">{it.label}</span>
                                </a>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </div>
    }
}
