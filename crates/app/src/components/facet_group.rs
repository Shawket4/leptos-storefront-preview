//! A filter group: a `<fieldset>` with its legend around a facet's controls — usually
//! [`Checkbox`](super::form::Checkbox)es with a count (`meta`), or anything else a facet needs
//! (the price boxes). Groups in a column are divided by a hairline.
//!
//! `show_first`: only that many options show; the rest wait behind "Show all N" — a `<details>`,
//! so it works without JS — which says "Show fewer" while open. Pass `expanded` when a checked
//! option is among the hidden ones, so it starts open. One child per option: a `collect_view()`
//! of them, or several written out.
//!
//! ```ignore
//! <FacetGroup legend=tr(lang, "Brand", "الماركة") show_first=6 expanded=hidden_checked>
//!     {brands.iter().map(|b| view! {
//!         <Checkbox name="brand" value=b.id.clone() label=b.name.clone() meta=format::count(b.count, num) checked=on(b)/>
//!     }).collect_view()}
//! </FacetGroup>
//! ```

use super::icons::Icon;
use crate::format;
use crate::prefs::{Lang, use_ui};
use leptos::prelude::*;

#[component]
pub fn FacetGroup(
    #[prop(into)] legend: TextProp,
    /// Show this many options; the rest go behind "Show all N".
    #[prop(optional)]
    show_first: Option<usize>,
    /// Start with the rest shown (a checked option is among them).
    #[prop(optional)]
    expanded: bool,
    #[prop(optional, into)] class: String,
    children: ChildrenFragment,
) -> impl IntoView {
    let ui = use_ui();
    let lang = ui.lang.get_untracked();
    let num = ui.numerals.get_untracked();
    let cls = if class.is_empty() {
        "facet".to_owned()
    } else {
        format!("facet {class}")
    };
    let mut nodes = children().nodes.into_iter().collect::<Vec<_>>();
    let total = nodes.len();
    let rest = match show_first {
        Some(n) if n < total => nodes.split_off(n),
        _ => Vec::new(),
    };
    let more = (!rest.is_empty()).then(|| {
        let n = format::count(total as u32, num);
        let (show, hide) = match lang {
            Lang::En => (format!("Show all {n}"), "Show fewer"),
            Lang::Ar => (format!("اعرض الكل ({n})"), "اعرض أقل"),
        };
        view! {
            <details class="facet__more" open=expanded>
                <summary class="facet__toggle" data-press="">
                    <span class="facet__show">{show}</span>
                    <span class="facet__hide">{hide}</span>
                    <Icon name="chevron-down" size=16 class="facet__chev"/>
                </summary>
                <div class="facet__options">{rest}</div>
            </details>
        }
    });
    view! {
        <fieldset class=cls>
            <legend class="facet__legend">{move || legend.get()}</legend>
            <div class="facet__options">{nodes}</div>
            {more}
        </fieldset>
    }
}
