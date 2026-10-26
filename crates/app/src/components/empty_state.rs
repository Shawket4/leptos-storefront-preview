//! Empty and error states: one of the brand illustrations (light or dark to match the theme),
//! a title, a line, and the way forward.
//!
//! `art` names an illustration in `data/media/brand` (`empty-cart`, `empty-search`, `not-found`,
//! `error`, `order-placed`, `delivery`): the files are `brand/<art>-light|dark-<w>.webp` on one
//! fixed ladder, so this works inside islands too (no manifest lookup).

use super::button::{Button, ButtonKind};
use crate::media::MEDIA_BASE;
use leptos::prelude::*;

fn art_img(art: &str, variant: &'static str) -> impl IntoView + use<> {
    let base = format!("{MEDIA_BASE}/brand/{art}-{variant}");
    let srcset = format!("{base}-240.webp 240w, {base}-400.webp 400w, {base}-640.webp 640w");
    let src = format!("{base}-400.webp");
    let class = format!("empty__art empty__art--{variant}");
    view! {
        <img
            class=class
            src=src
            srcset=srcset
            sizes="(min-width: 600px) 220px, 180px"
            width="220"
            height="220"
            alt=""
            loading="lazy"
            decoding="async"
        />
    }
}

#[component]
pub fn EmptyState(
    /// Illustration name (see the module docs).
    art: &'static str,
    #[prop(into)] title: TextProp,
    #[prop(into)] body: TextProp,
    /// `(href, label)` of the one thing to do next.
    #[prop(optional)]
    action: Option<(String, String)>,
    /// Heading level for the title: 1 on a page that is only this state, else 2.
    #[prop(optional)]
    page: bool,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let cls = if class.is_empty() {
        "empty".to_owned()
    } else {
        format!("empty {class}")
    };
    let heading = if page {
        view! { <h1 class="empty__title">{move || title.get()}</h1> }.into_any()
    } else {
        view! { <h2 class="empty__title">{move || title.get()}</h2> }.into_any()
    };
    view! {
        <div class=cls>
            <div class="empty__figure" aria-hidden="true">
                {art_img(art, "light")}
                {art_img(art, "dark")}
            </div>
            {heading}
            <p class="empty__body">{move || body.get()}</p>
            {action.map(|(href, label)| view! {
                <Button kind=ButtonKind::Primary href=href icon_end="arrow-right">{label}</Button>
            })}
        </div>
    }
}
