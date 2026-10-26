//! TEMPORARY stand-ins for the tier-3 design-system components the listing and search pages use
//! (`Breadcrumbs`, `EmptyState`, `Drawer`), with the names and props of the contract in AGENTS.md.
//! When they land on main: delete this file and the "shim" section at the end of
//! `style/pages/category.css`, and import from `crate::components::…`.

use crate::components::icons::Icon;
use crate::prefs::{lang_now, tr, tr_signal};
use leptos::either::Either;
use leptos::prelude::*;

/// Contract: `breadcrumbs::Breadcrumbs(items)`; the last item is the current page.
#[component]
pub fn Breadcrumbs(items: Vec<(String, Option<String>)>) -> impl IntoView {
    let lang = lang_now();
    let n = items.len();
    view! {
        <nav class="crumbs" aria-label=tr(lang, "Breadcrumb", "مسار الصفحة")>
            <ol role="list">
                {items
                    .into_iter()
                    .enumerate()
                    .map(|(i, (label, href))| {
                        let last = i + 1 == n;
                        view! {
                            <li>
                                {match href.filter(|_| !last) {
                                    Some(href) => Either::Left(view! { <a href=href>{label}</a> }),
                                    None => Either::Right(view! { <span aria-current="page">{label}</span> }),
                                }}
                            </li>
                        }
                    })
                    .collect_view()}
            </ol>
        </nav>
    }
}

/// Contract: `empty_state::EmptyState(art, title, body, action?)`. `art` is an icon name here.
#[component]
pub fn EmptyState(
    art: &'static str,
    #[prop(into)] title: String,
    #[prop(into)] body: String,
    #[prop(optional)] action: Option<AnyView>,
) -> impl IntoView {
    view! {
        <div class="empty-state">
            <span class="empty-state__art" aria-hidden="true">
                <Icon name=art size=28/>
            </span>
            <p class="empty-state__title">{title}</p>
            <p class="empty-state__body">{body}</p>
            {action.map(|a| view! { <div class="empty-state__action">{a}</div> })}
        </div>
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DrawerSide {
    #[default]
    End,
    Bottom,
}

/// Contract: `drawer::{Drawer(open, side?, title, children), DrawerSide}`. A minimal modal sheet:
/// scrim, title, close button, Escape, focus moved in on open; the real one adds the spring,
/// drag-to-dismiss and a focus trap.
#[component]
pub fn Drawer(
    open: RwSignal<bool>,
    #[prop(optional)] side: DrawerSide,
    #[prop(into)] title: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let side = match side {
        DrawerSide::End => "end",
        DrawerSide::Bottom => "bottom",
    };
    let close_label = tr_signal("Close", "إغلاق");
    let panel = NodeRef::<leptos::html::Div>::new();
    Effect::new(move |_| {
        let is_open = open.get();
        if let Some(root) = document().document_element() {
            let _ = root.class_list().toggle_with_force("drawer-lock", is_open);
        }
        if is_open && let Some(p) = panel.get() {
            let _ = p.focus();
        }
    });
    view! {
        <Show when=move || open.get()>
            <div class="shim-drawer" data-side=side>
                <div class="shim-drawer__scrim" on:click=move |_| open.set(false)></div>
                <div
                    class="shim-drawer__panel"
                    role="dialog"
                    aria-modal="true"
                    aria-label=move || title.get()
                    tabindex="-1"
                    node_ref=panel
                    on:keydown=move |ev| {
                        if ev.key() == "Escape" {
                            open.set(false);
                        }
                    }
                >
                    <div class="shim-drawer__head">
                        <h2 class="shim-drawer__title">{move || title.get()}</h2>
                        <button
                            type="button"
                            class="shim-drawer__close"
                            aria-label=move || close_label.get()
                            on:click=move |_| open.set(false)
                        >
                            <Icon name="x"/>
                        </button>
                    </div>
                    {children()}
                </div>
            </div>
        </Show>
    }
}
