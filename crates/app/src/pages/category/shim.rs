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
/// a `<dialog>` shown with `showModal()`, so it sits in the top layer (above the header and the
/// tab bar), the page goes inert, focus is trapped and Escape closes it. The real one adds the
/// spring, drag-to-dismiss and focus return.
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
    let dialog = NodeRef::<leptos::html::Dialog>::new();
    Effect::new(move |_| {
        let is_open = open.get();
        let Some(d) = dialog.get() else { return };
        if let Some(root) = document().document_element() {
            let _ = root.class_list().toggle_with_force("drawer-lock", is_open);
        }
        if is_open && !d.open() {
            let _ = d.show_modal();
        } else if !is_open && d.open() {
            d.close();
        }
    });
    view! {
        <dialog
            class="shim-drawer"
            data-side=side
            aria-label=move || title.get()
            node_ref=dialog
            // Escape (or the browser) closed it: keep the signal in step.
            on:close=move |_| open.set(false)
            // A click on the backdrop lands on the <dialog> itself.
            on:click=move |ev| {
                let on_backdrop = ev
                    .target()
                    .zip(dialog.get_untracked())
                    .is_some_and(|(t, d)| {
                        let t: &wasm_bindgen::JsValue = t.as_ref();
                        let d: &wasm_bindgen::JsValue = d.as_ref();
                        t == d
                    });
                if on_backdrop {
                    open.set(false);
                }
            }
        >
            <div class="shim-drawer__panel">
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
        </dialog>
    }
}
