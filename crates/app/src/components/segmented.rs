//! A segmented control made of links: sort orders, list/grid views, language, theme. Each option
//! is a plain link (works without JS; the islands router patches the page in place), the current
//! one marked with `aria-current="true"` (always rendered as `true`/`false`, so a patched page
//! updates it in place).

use super::icons::Icon;
use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct SegItem {
    pub label: String,
    pub href: String,
    pub current: bool,
    /// A lucide icon before the label.
    pub icon: Option<&'static str>,
    /// `lang` for a label written in another language (the language switch).
    pub lang: Option<&'static str>,
}

impl SegItem {
    pub fn new(label: impl Into<String>, href: impl Into<String>, current: bool) -> Self {
        SegItem {
            label: label.into(),
            href: href.into(),
            current,
            icon: None,
            lang: None,
        }
    }
    pub fn icon(mut self, icon: &'static str) -> Self {
        self.icon = Some(icon);
        self
    }
    pub fn lang(mut self, lang: &'static str) -> Self {
        self.lang = Some(lang);
        self
    }
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
    let mut cls = String::from("seg");
    if full {
        cls.push_str(" seg--full");
    }
    if !class.is_empty() {
        cls.push(' ');
        cls.push_str(&class);
    }
    view! {
        <div class=cls role="group" aria-label=label>
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
