//! Toasts: short, inverted notes ("Added to your cart") that report what just happened.
//!
//! **Now in the browser code:** `client/src/stores/toasts.ts` (`toast(msg, kind)`) renders into the
//! [`Toaster`] region below from its `<template>`. The reactive store further down is the old
//! WASM-island version, kept only until every caller is ported — it is deprecated.
//!
//! [`Toasts`] is a global store (provided on the server per request and once on the client, see
//! `stores::provide_all`); [`show`] queues a message from any island; the [`Toaster`] island,
//! mounted once by the shell, renders the queue in a polite live region. Success and info leave
//! after 3.5 s, errors after 6 s (not while the stack is hovered or focused); each can be closed.
//!
//! They stack like iOS notifications: the newest in full, at most two older ones tucked behind
//! it (scaled .94 and .88, 8px of each peeking out); a fourth sends the oldest away. Hovering
//! the stack (a mouse) or focusing into it fans them out into a list. Entry rises on a spring and
//! the exit goes back the same way; reduced motion swaps both for a fade.

#![allow(deprecated)]
use super::icons::Icon;
use crate::prefs::{tr, use_ui};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToastKind {
    #[default]
    Success,
    Error,
    Info,
}

impl ToastKind {
    /// How long it stays, in ms. Errors linger: they usually need reading.
    fn ttl_ms(self) -> u64 {
        match self {
            ToastKind::Error => 6000,
            _ => 3500,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Toast {
    pub id: u32,
    pub msg: String,
    pub kind: ToastKind,
    /// Playing its exit; removed shortly after.
    pub leaving: RwSignal<bool>,
}

/// The toast queue. Copy; get it with [`use_toasts`].
#[deprecated(note = "toasts live in the browser code now: client/src/stores/toasts.ts")]
#[derive(Clone, Copy, Debug)]
pub struct Toasts {
    pub list: RwSignal<Vec<Toast>>,
    next: RwSignal<u32>,
    /// The stack is under the pointer / holds focus: timers wait.
    hovered: RwSignal<bool>,
    focused: RwSignal<bool>,
}

/// At most this many on screen; the oldest leaves first.
const MAX: usize = 3;

impl Toasts {
    fn new() -> Self {
        Toasts {
            list: RwSignal::new(Vec::new()),
            next: RwSignal::new(1),
            hovered: RwSignal::new(false),
            focused: RwSignal::new(false),
        }
    }

    /// Queue a toast; returns its id.
    pub fn show(&self, msg: impl Into<String>, kind: ToastKind) -> u32 {
        let id = self.next.get_untracked();
        self.next.set(id + 1);
        let toast = Toast {
            id,
            msg: msg.into(),
            kind,
            leaving: RwSignal::new(false),
        };
        self.list.update(|l| {
            l.push(toast);
            let live = l.iter().filter(|t| !t.leaving.get_untracked()).count();
            if live > MAX {
                if let Some(old) = l.iter().find(|t| !t.leaving.get_untracked()) {
                    old.leaving.set(true);
                }
            }
        });
        let this = *self;
        after_ms(kind.ttl_ms(), move || this.expire(id));
        id
    }

    /// Its time is up: leave, unless someone is reading the stack (then look again in a second).
    fn expire(&self, id: u32) {
        if self.hovered.get_untracked() || self.focused.get_untracked() {
            let this = *self;
            after_ms(1000, move || this.expire(id));
        } else {
            self.dismiss(id);
        }
    }

    /// Play a toast's exit, then drop it.
    pub fn dismiss(&self, id: u32) {
        let Some(t) = self
            .list
            .with_untracked(|l| l.iter().find(|t| t.id == id).cloned())
        else {
            return;
        };
        if t.leaving.get_untracked() {
            return;
        }
        t.leaving.set(true);
        let list = self.list;
        after_ms(200, move || list.update(|l| l.retain(|t| t.id != id)));
    }
}

/// Timers never run on the server, where this deprecated store still lives: a no-op.
fn after_ms(ms: u64, f: impl FnOnce() + 'static) {
    let _ = (ms, f);
}

/// Provide the toast store (called on the server per request and once on the client).
pub fn provide() {
    provide_context(Toasts::new());
}

#[deprecated(note = "toasts live in the browser code now: client/src/stores/toasts.ts")]
#[allow(deprecated)]
pub fn use_toasts() -> Toasts {
    use_context::<Toasts>().unwrap_or_else(|| {
        let t = Toasts::new();
        provide_context(t);
        t
    })
}

/// Queue a toast from anywhere in an island: `toast::show(msg, ToastKind::Success)`.
#[deprecated(note = "toasts live in the browser code now: client/src/stores/toasts.ts")]
#[allow(deprecated)]
pub fn show(msg: String, kind: ToastKind) {
    use_toasts().show(msg, kind);
}

/// The toast region, mounted once by the shell. The browser code fills it (`stores/toasts.ts`):
/// each toast is a clone of `#tpl-toast` with its message (and an optional action) put in, and a
/// `data-kind` that picks the icon. A polite live region, so screen readers hear each one.
#[component]
pub fn Toaster() -> impl IntoView {
    let ui = use_ui();
    let lang = ui.lang.get_untracked();
    view! {
        <section class="toaster" id="toaster" aria-label=tr(lang, "Notifications", "التنبيهات")>
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
