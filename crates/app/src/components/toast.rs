//! Toasts: short, inverted notes ("Added to your cart") that report what just happened.
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

/// Renders the toast queue as a stack. Mounted once by the app shell.
#[island]
pub fn Toaster() -> impl IntoView {
    let toasts = use_toasts();
    let ui = use_ui();
    let region = move || tr(ui.lang.get(), "Notifications", "التنبيهات");
    let close = move || tr(ui.lang.get(), "Dismiss", "إغلاق");
    let list = NodeRef::<leptos::html::Ol>::new();

    // Each change (a toast in, a toast leaving): after the DOM settles, give every toast its
    // place in the stack (data-depth, newest = 0), its own height (--h) and its offset when fanned
    // out (--y), and the list the front toast's height (--front-h: the ones behind take it, so
    // each peeks by exactly the same strip).
    #[cfg(feature = "hydrate")]
    Effect::new(move |_| {
        toasts
            .list
            .with(|l| l.iter().for_each(|t| t.leaving.track()));
        request_animation_frame(move || layout_stack(list));
    });

    let on_focus_out = move |ev: web_sys::FocusEvent| {
        // Still inside the stack (Tab from one close button to the next): keep it open.
        let inside = match (list.get_untracked(), ev.related_target()) {
            (Some(ol), Some(t)) => wasm_bindgen::JsCast::dyn_into::<web_sys::Node>(t)
                .map(|n| ol.contains(Some(&n)))
                .unwrap_or(false),
            _ => false,
        };
        if !inside {
            toasts.focused.set(false);
        }
    };
    view! {
        <section class="toaster" aria-label=region>
            <ol
                class="toaster__list"
                role="status"
                aria-live="polite"
                node_ref=list
                on:pointerenter=move |ev: web_sys::PointerEvent| {
                    if ev.pointer_type() == "mouse" {
                        toasts.hovered.set(true);
                    }
                }
                on:pointerleave=move |_| toasts.hovered.set(false)
                on:focusin=move |_| toasts.focused.set(true)
                on:focusout=on_focus_out
            >
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
                                <div class="toast__inner">
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
                                </div>
                            </li>
                        }
                    }
                </For>
            </ol>
        </section>
    }
}

/// See [`Toaster`]: depth, heights and fan-out offsets, from the newest toast back.
#[cfg(feature = "hydrate")]
fn layout_stack(list: NodeRef<leptos::html::Ol>) {
    use wasm_bindgen::JsCast;
    const GAP: f64 = 8.0;
    let Some(ol) = list.get_untracked() else {
        return;
    };
    let items = ol.children();
    let (mut depth, mut y, mut front) = (0u32, 0.0_f64, None::<f64>);
    for i in (0..items.length()).rev() {
        let Some(li) = items
            .item(i)
            .and_then(|e| e.dyn_into::<web_sys::HtmlElement>().ok())
        else {
            continue;
        };
        let h = li
            .first_element_child()
            .and_then(|c| c.dyn_into::<web_sys::HtmlElement>().ok())
            .map(|c| c.offset_height() as f64)
            .unwrap_or_default();
        let style = web_sys::HtmlElement::style(&li);
        let _ = style.set_property("--h", &format!("{h}px"));
        // A leaving toast keeps the place it had and fades there.
        if li.get_attribute("data-leaving").as_deref() == Some("true") {
            continue;
        }
        let _ = li.set_attribute("data-depth", &depth.min(3).to_string());
        let _ = style.set_property("--y", &format!("{y}px"));
        front.get_or_insert(h);
        y += h + GAP;
        depth += 1;
    }
    if let Some(h) = front {
        let _ = web_sys::HtmlElement::style(&ol).set_property("--front-h", &format!("{h}px"));
    }
}
