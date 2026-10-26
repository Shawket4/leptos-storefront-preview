//! The one drawer / bottom sheet: the cart drawer (from the inline end — the left in Arabic), the
//! listing's filter sheet (from the bottom on phones), the /ui demos.
//!
//! Server markup only. The behaviour is `client/src/core/drawer.ts` (`data-ts="drawer"`): a modal
//! `<dialog>` (the browser traps focus, makes the page behind inert and gives Escape), moved by a
//! spring that opens and closes along the same path, follows the finger 1:1 when dragged toward its
//! edge, rubber-bands past open, and on release lets the velocity decide; reduced motion gets a
//! short cross-fade. It starts closed; open it with any `[data-drawer-open="<id>"]` (a link to the
//! no-JS page works best: `<a href="/cart" data-drawer-open="cart-drawer">`) or `openDrawer(id)`.
//! Anything inside marked `[data-drawer-close]` closes it. A drawer that is open when the router
//! swaps its region comes back open in the new page (same `id`), instantly.

use super::button::{ButtonKind, IconButton};
use crate::prefs::{tr, use_ui};
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DrawerSide {
    /// From the inline end, full height (cart).
    #[default]
    End,
    /// From the inline start, full height.
    Start,
    /// From the bottom, up to 88% of the screen (filters on phones).
    Bottom,
}

impl DrawerSide {
    fn code(self) -> &'static str {
        match self {
            DrawerSide::End => "end",
            DrawerSide::Start => "start",
            DrawerSide::Bottom => "bottom",
        }
    }
}

#[component]
pub fn Drawer(
    /// Unique id on the page: the triggers' `data-drawer-open` and `openDrawer(id)` use it.
    #[prop(optional, into)]
    id: Option<String>,
    #[prop(optional)] side: DrawerSide,
    /// The heading.
    #[prop(into)]
    title: Signal<String>,
    /// Pinned under the scrolling body: the drawer's actions ("Checkout", "Show 24 results").
    #[prop(optional)]
    footer: Option<Children>,
    /// Extra classes on the `<dialog>`.
    #[prop(optional, into)]
    class: String,
    children: Children,
) -> impl IntoView {
    let lang = use_ui().lang.get_untracked();
    let id = id.unwrap_or_else(|| "drawer".to_owned());
    let title_id = format!("{id}-title");
    let labelled_by = title_id.clone();
    let side_code = side.code();
    let cls = match (side, class.is_empty()) {
        (_, true) => format!("drawer drawer--{side_code}"),
        (_, false) => format!("drawer drawer--{side_code} {class}"),
    };
    view! {
        <dialog class=cls id=id data-ts="drawer" data-side=side_code aria-labelledby=labelled_by>
            <div class="drawer__scrim" data-drawer-close=""></div>
            <div class="drawer__panel" tabindex="-1">
                {(side == DrawerSide::Bottom)
                    .then(|| view! { <div class="drawer__grabber" aria-hidden="true"></div> })}
                <header class="drawer__head">
                    <h2 class="drawer__title" id=title_id>{title.get_untracked()}</h2>
                    <IconButton
                        icon="x"
                        label=tr(lang, "Close", "إغلاق")
                        kind=ButtonKind::Ghost
                        class="drawer__close"
                        attr:data-drawer-close=""
                    />
                </header>
                <div class="drawer__body">{children()}</div>
                {footer.map(|f| view! { <footer class="drawer__foot">{f()}</footer> })}
            </div>
        </dialog>
    }
}
