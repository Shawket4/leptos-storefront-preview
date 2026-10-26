//! Toasts: short, inverted notes ("Added to your cart") that report what just happened.
//!
//! The queue is the browser code's (`client/src/stores/toasts.ts`: `toast(msg, kind, action?)`);
//! this is the region it renders into — mounted once by the shell — and the `<template>` each toast
//! is cloned from (`data-kind` picks the icon). They stack like iOS notifications: the newest in
//! full, at most two older ones tucked behind it; hovering or focusing the stack fans it out.

use super::icons::Icon;
use crate::prefs::{tr, use_ui};
use leptos::prelude::*;

/// The toast region, mounted once by the shell. The browser code fills it (`stores/toasts.ts`):
/// each toast is a clone of `#tpl-toast` with its message (and an optional action) put in, and a
/// `data-kind` that picks the icon. A polite live region, so screen readers hear each one.
/// A manual popover: shown in the top layer, and raised again above any drawer that opens (a
/// modal dialog), so a toast from inside the cart drawer is seen and its action can be pressed.
#[component]
pub fn Toaster() -> impl IntoView {
    let ui = use_ui();
    let lang = ui.lang.get_untracked();
    view! {
        <section class="toaster" id="toaster" popover="manual" aria-label=tr(lang, "Notifications", "التنبيهات")>
            <ol class="toaster__list" role="status" aria-live="polite"></ol>
        </section>
        <template id="tpl-toast">
            <li class="toast">
                <div class="toast__inner">
                    <Icon name="circle-check" size=20 class="toast__icon toast__icon--success"/>
                    <Icon name="circle-x" size=20 class="toast__icon toast__icon--error"/>
                    <Icon name="info" size=20 class="toast__icon toast__icon--info"/>
                    <p class="toast__msg" data-slot="msg"></p>
                    <button type="button" class="toast__action" data-toast-action="" data-slot="action"></button>
                    <button type="button" class="toast__close" data-toast-close=""
                        aria-label=crate::strings::get("toast.dismiss", lang)>
                        <Icon name="x" size=16/>
                    </button>
                </div>
            </li>
        </template>
    }
}
