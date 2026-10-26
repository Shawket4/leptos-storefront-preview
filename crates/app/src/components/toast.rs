//! Toasts: short, inverted notes ("Added to your cart") that report what just happened.
//!
//! [`Toasts`] is a global store (provided on the server per request and once on the client, see
//! `stores::provide_all`); [`show`] queues a message from any island; the [`Toaster`] island,
//! mounted once by the shell, renders the queue in a polite live region. Success and info leave
//! after 3.5 s, errors after 6 s; each can be closed. Entry rises on a spring and the exit goes
//! back the same way.

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
    fn icon(self) -> &'static str {
        match self {
            ToastKind::Success => "circle-check",
            ToastKind::Error => "circle-x",
            ToastKind::Info => "info",
        }
    }
    fn code(self) -> &'static str {
        match self {
            ToastKind::Success => "success",
            ToastKind::Error => "error",
            ToastKind::Info => "info",
        }
    }
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
#[derive(Clone, Copy, Debug)]
pub struct Toasts {
    pub list: RwSignal<Vec<Toast>>,
    next: RwSignal<u32>,
}

/// At most this many on screen; the oldest leaves first.
const MAX: usize = 3;

impl Toasts {
    fn new() -> Self {
        Toasts {
            list: RwSignal::new(Vec::new()),
            next: RwSignal::new(1),
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
        after_ms(kind.ttl_ms(), move || this.dismiss(id));
        id
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

/// Run `f` after `ms` in the browser; nothing on the server (toasts only live in the client).
fn after_ms(ms: u64, f: impl FnOnce() + 'static) {
    #[cfg(feature = "hydrate")]
    {
        set_timeout(f, std::time::Duration::from_millis(ms));
    }
    #[cfg(not(feature = "hydrate"))]
    {
        let _ = (ms, f);
    }
}

/// Provide the toast store (called on the server per request and once on the client).
pub fn provide() {
    provide_context(Toasts::new());
}

pub fn use_toasts() -> Toasts {
    use_context::<Toasts>().unwrap_or_else(|| {
        let t = Toasts::new();
        provide_context(t);
        t
    })
}

/// Queue a toast from anywhere in an island: `toast::show(msg, ToastKind::Success)`.
pub fn show(msg: String, kind: ToastKind) {
    use_toasts().show(msg, kind);
}

/// Renders the toast queue. Mounted once by the app shell.
#[island]
pub fn Toaster() -> impl IntoView {
    let toasts = use_toasts();
    let ui = use_ui();
    let region = move || tr(ui.lang.get(), "Notifications", "التنبيهات");
    let close = move || tr(ui.lang.get(), "Dismiss", "إغلاق");
    view! {
        <section class="toaster" aria-label=region>
            <ol class="toaster__list" role="status" aria-live="polite">
                <For each=move || toasts.list.get() key=|t| t.id let:t>
                    {
                        let id = t.id;
                        let leaving = t.leaving;
                        view! {
                            <li
                                class="toast"
                                data-kind=t.kind.code()
                                data-leaving=move || if leaving.get() { "true" } else { "false" }
                            >
                                <Icon name=t.kind.icon() size=20 class="toast__icon"/>
                                <p class="toast__msg">{t.msg.clone()}</p>
                                <button
                                    type="button"
                                    class="toast__close"
                                    aria-label=close
                                    on:click=move |_| toasts.dismiss(id)
                                >
                                    <Icon name="x" size=16/>
                                </button>
                            </li>
                        }
                    }
                </For>
            </ol>
        </section>
    }
}
