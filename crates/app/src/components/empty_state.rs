//! Empty states (empty cart, no results, no orders yet): an illustration, a title, a line, and
//! the way forward. A [`StateView`] with the empty tone — kept as its own component because pages
//! reach for it most.
//!
//! `art` names an illustration in `data/media/brand` (`empty-cart`, `empty-search`, `not-found`,
//! `error`, `order-placed`, `delivery`), served on one fixed ladder so this works in islands too.

use super::button::{Button, ButtonKind};
use super::states::{StateTone, StateView};
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
    /// The page is only this state: the title is the page's `<h1>`.
    #[prop(optional)]
    page: bool,
    /// Smaller art and spacing (drawers, cards).
    #[prop(optional)]
    compact: bool,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let actions = action.map(|(href, label)| {
        Box::new(move || {
            view! { <Button kind=ButtonKind::Primary href=href icon_end="arrow-right">{label}</Button> }
                .into_any()
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
