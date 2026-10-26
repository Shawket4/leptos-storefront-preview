//! Temporary stand-in for `components::breadcrumbs::Breadcrumbs` (not on main yet), with the
//! documented name. Swap the import and delete this file when it lands. The quantity stepper and
//! the add-on inputs stay local until `form`/`stepper` (tier 3) land: the page's form needs named
//! inputs that also post without JS.

use crate::components::icons::Icon;
use crate::prefs::{Lang, tr};
use leptos::prelude::*;

/// One step: label and link (`None` = the current page).
#[derive(Clone, Debug, PartialEq)]
pub struct Crumb {
    pub label: String,
    pub href: Option<String>,
}

#[component]
pub fn Breadcrumbs(items: Vec<Crumb>, lang: Lang) -> impl IntoView {
    let last = items.len().saturating_sub(1);
    view! {
        <nav class="crumbs" aria-label=tr(lang, "Breadcrumb", "مسار التصفح")>
            <ol class="crumbs__list" role="list">
                {items
                    .into_iter()
                    .enumerate()
                    .map(|(i, c)| {
                        let here = c.href.is_none();
                        view! {
                            <li class="crumbs__item" class:crumbs__item--here=here>
                                {match c.href {
                                    Some(h) => view! { <a class="crumbs__link" href=h>{c.label}</a> }.into_any(),
                                    None => view! { <span class="crumbs__here" aria-current="page">{c.label}</span> }.into_any(),
                                }}
                                {(i < last).then(|| view! { <Icon name="chevron-right" size=14 class="crumbs__sep flip-rtl"/> })}
                            </li>
                        }
                    })
                    .collect_view()}
            </ol>
        </nav>
    }
}
