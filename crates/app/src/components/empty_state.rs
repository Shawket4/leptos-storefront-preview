//! Empty states (empty cart, no results, no orders yet): an illustration, a title, a line, and
//! the way forward. A [`StateView`] with the empty tone — kept as its own component because pages
//! reach for it most.
//!
//! `art` names an illustration in `data/media/brand` (`empty-cart`, `empty-search`, `not-found`,
//! `error`, `order-placed`, `delivery`), served on one fixed ladder (no manifest lookup).
//!
//! `suggestion` adds "Did you mean *term*?" above the action, the term a link (a respelt search).

use super::button::{Button, ButtonKind};
use super::states::{StateTone, StateView};
use crate::prefs::{Lang, lang_now};
use leptos::prelude::*;

#[component]
pub fn EmptyState(
    /// Illustration name (see the module docs).
    art: &'static str,
    #[prop(into)] title: TextProp,
    #[prop(into)] body: TextProp,
    /// `(href, label)` of the one thing to do next.
    #[prop(optional)]
    action: Option<(String, String)>,
    /// `(href, term)`: "Did you mean *term*?", the term a link to that search.
    #[prop(optional)]
    suggestion: Option<(String, String)>,
    /// The page is only this state: the title is the page's `<h1>`.
    #[prop(optional)]
    page: bool,
    /// Smaller art and spacing (drawers, cards).
    #[prop(optional)]
    compact: bool,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let suggest = suggestion.map(|(href, term)| {
        let (before, after) = match lang_now() {
            Lang::En => ("Did you mean ", "?"),
            Lang::Ar => ("تقصد ", "؟"),
        };
        view! { <p class="state__suggest">{before}<a class="link" href=href><bdi>{term}</bdi></a>{after}</p> }
    });
    let actions = (action.is_some() || suggest.is_some()).then(|| {
        Box::new(move || {
            let button = action.map(|(href, label)| {
                view! { <Button kind=ButtonKind::Primary href=href icon_end="arrow-right">{label}</Button> }
            });
            view! { {suggest}{button} }.into_any()
        }) as Children
    });
    match actions {
        Some(children) => view! {
            <StateView tone=StateTone::Empty art=art title=title body=body page=page compact=compact class=class>
                {children()}
            </StateView>
        }
        .into_any(),
        None => view! {
            <StateView tone=StateTone::Empty art=art title=title body=body page=page compact=compact class=class/>
        }
        .into_any(),
    }
}
