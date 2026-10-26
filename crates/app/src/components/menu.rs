//! A popover menu of links — the listing's sort order, any "pick one of these pages" choice.
//!
//! A `<details data-popover>` disclosure, so it opens and every option works without JS (each is
//! a plain link; the current one is `aria-current="true"` and ticked). features/core.ts adds the
//! popover behaviour to every `details[data-popover]`: light dismiss (a press outside, Escape —
//! focus back on the trigger — or a link inside), one open at a time, and the exit motion. The
//! panel grows out of the trigger's corner on a spring and leaves the same way.
//!
//! The trigger reads "<icon> Sort  Popular ▾". Browser code that applies a choice itself updates
//! the value in place with `patch(menu, { value })` (it is `data-slot="value"`) and moves
//! `aria-current`; give it hooks with [`MenuItem::attr`] and `attr:` on the component.
//!
//! ```ignore
//! let items = Sort::ALL.map(|s| MenuItem::new(s.label(), with_sort(q, s).href(), s == q.sort).attr("data-sort", s.code()));
//! <Menu label=tr(lang, "Sort", "الترتيب") value=q.sort.label() items icon="arrow-up-down" align=MenuAlign::End compact=true/>
//! ```

use super::attrs::{Attrs, spread};
use super::icons::Icon;
use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct MenuItem {
    pub label: String,
    pub href: String,
    /// The option in force (ticked, `aria-current="true"`).
    pub current: bool,
    /// Extra attributes on the link (`data-*`, `rel`).
    pub attrs: Attrs,
}

impl MenuItem {
    pub fn new(label: impl Into<String>, href: impl Into<String>, current: bool) -> Self {
        MenuItem {
            label: label.into(),
            href: href.into(),
            current,
            attrs: Vec::new(),
        }
    }
    /// One more attribute on the link: `.attr("data-sort", "price-asc")`.
    pub fn attr(mut self, name: &'static str, value: impl Into<String>) -> Self {
        self.attrs.push((name, value.into()));
        self
    }
}

/// Which edge of the trigger the panel lines up with (it grows out of that corner).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MenuAlign {
    /// The trigger's start edge: a menu at the start of a row.
    #[default]
    Start,
    /// The trigger's end edge: a menu at the end of a row (the listing's sort).
    End,
}

#[component]
pub fn Menu(
    /// What the menu chooses ("Sort"): the trigger's first word, and the options' name for
    /// screen readers.
    #[prop(into)]
    label: String,
    /// The choice in force ("Popular"), after the label.
    #[prop(optional, into)]
    value: Option<String>,
    items: Vec<MenuItem>,
    /// A lucide icon at the start of the trigger.
    #[prop(optional)]
    icon: Option<&'static str>,
    #[prop(optional)] align: MenuAlign,
    /// Phones (< 480px): the trigger shows only the icon and the value (give it an `icon`); the
    /// label stays for screen readers.
    #[prop(optional)]
    compact: bool,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let mut cls = String::from("menu");
    if align == MenuAlign::End {
        cls.push_str(" menu--end");
    }
    if compact && icon.is_some() {
        cls.push_str(" menu--compact");
    }
    if !class.is_empty() {
        cls.push(' ');
        cls.push_str(&class);
    }
    let list_label = label.clone();
    view! {
        <details class=cls data-popover="">
            <summary class="menu__trigger" data-press="">
                {icon.map(|i| view! { <Icon name=i size=18 class="menu__icon"/> })}
                <span class="menu__label">{label}</span>
                {value.map(|v| view! { <span class="menu__value" data-slot="value">{v}</span> })}
                <Icon name="chevron-down" size=16 class="menu__chev"/>
            </summary>
            <div class="menu__panel">
                <ul class="menu__list" role="list" aria-label=list_label>
                    {items
                        .into_iter()
                        .map(|it| {
                            let current = if it.current { "true" } else { "false" };
                            view! {
                                <li>
                                    <a class="menu__item" href=it.href aria-current={current} {..spread(it.attrs)}>
                                        <span class="menu__item-label">{it.label}</span>
                                        <Icon name="check" size=18 class="menu__tick"/>
                                    </a>
                                </li>
                            }
                        })
                        .collect_view()}
                </ul>
            </div>
        </details>
    }
}
