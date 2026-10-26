//! The cart on the client.
//!
//! **One queue.** Every cart mutation an island makes (stepper, remove, undo, quick add) goes
//! through [`CartOps::submit`]: it is applied to the view at once (optimistic), queued, and sent
//! one at a time. Each response becomes the confirmed cart and whatever is still queued is applied
//! again on top of it, so a slow answer can never overwrite a newer one, and a failed op rolls back
//! simply by leaving the queue.
//!
//! Why not hand each form a `ServerAction` and let `<ActionForm>` dispatch it? An `Action` keeps
//! whichever result *finishes* last — in Leptos 0.8.22 `ArcAction::dispatch` reads a `dispatched`
//! counter that nothing ever advances, so every completion counts as the latest — which is fine
//! for one form and wrong for three quick taps on "+". The forms themselves are unchanged:
//! [`CartForm`] renders what `<ActionForm>` renders (`<form method="post" action="/api/…">` with the
//! server function's argument names) and parses the submit with the same `FromFormData`, so
//! without JS each control is a plain POST that server_fn answers with a redirect back.
//!
//! **The badge stays honest** when the cart changes behind the islands' back — a server-rendered
//! form on `/cart` (enhanced by the islands router), another tab: after every navigation and when
//! the tab becomes visible again, the readable `cart_n` cookie (refreshed by every cart response)
//! is compared with the store.
//!
//! **Server-function redirects are soft.** A redirect hook turns `leptos_axum::redirect` answers
//! to island calls into islands-router navigations (a synthetic same-origin link click) instead of
//! a full page load — checkout steps use it.

use super::math::{self, Op};
use super::shim::toast::{ToastKind, Toasts, use_toasts};
use crate::api::cart::{cart_add, cart_get, cart_remove, cart_set_qty};
use crate::api::types::CartView;
use crate::prefs::{Ui, use_ui};
use crate::stores::{CartStore, use_cart};
use domain::Piastres;
use leptos::prelude::*;
use std::collections::VecDeque;

/// What a queued op's caller hears back: the confirmed cart, or an error code (see `copy`).
pub type OpResult = Result<CartView, String>;

#[derive(Clone)]
struct Job {
    op: Op,
    done: Option<Callback<OpResult>>,
}

#[derive(Clone, Copy)]
pub struct CartOps {
    pub cart: CartStore,
    /// Kept here because async continuations (after an `.await`) have no owner to read context.
    pub ui: Ui,
    pub toasts: Toasts,
    /// The server's last word (`None` until the cart is first fetched).
    confirmed: RwSignal<Option<CartView>>,
    queue: StoredValue<VecDeque<Job>>,
    inflight: StoredValue<Option<Job>>,
    /// Ops queued or on the wire: totals show as provisional.
    pub busy: RwSignal<bool>,
    /// The first fetch of the cart is under way.
    pub loading: RwSignal<bool>,
    pub load_failed: RwSignal<bool>,
    /// Line order as the shopper first saw it.
    order: StoredValue<Vec<String>>,
    /// The tenant's delivery fee, remembered from any response that charged it.
    fee: StoredValue<Piastres>,
    /// The count before pending ops, while no cart has been fetched (from the cookie).
    base_count: StoredValue<u32>,
}

pub fn use_ops() -> CartOps {
    use_context::<CartOps>().unwrap_or_else(|| {
        let ops = CartOps::new(use_cart(), use_ui(), use_toasts());
        provide_context(ops);
        ops
    })
}

/// Provide the client cart store (server: per request; client: once, from `client::init`).
pub fn provide() {
    let toasts = super::shim::toast::provide();
    let ops = CartOps::new(use_cart(), use_ui(), toasts);
    provide_context(ops);
    #[cfg(feature = "hydrate")]
    client::init(ops);
}

impl CartOps {
    fn new(cart: CartStore, ui: Ui, toasts: Toasts) -> Self {
        CartOps {
            cart,
            ui,
            toasts,
            confirmed: RwSignal::new(None),
            queue: StoredValue::new(VecDeque::new()),
            inflight: StoredValue::new(None),
            busy: RwSignal::new(false),
            loading: RwSignal::new(false),
            load_failed: RwSignal::new(false),
            order: StoredValue::new(Vec::new()),
            fee: StoredValue::new(Piastres::ZERO),
            base_count: StoredValue::new(cart.count.get_untracked()),
        }
    }

    /// Queue a mutation: shown now, sent in turn. `done` hears the outcome (errors are then the
    /// caller's to report; without `done`, a failure toasts here).
    pub fn submit(self, op: Op, done: Option<Callback<OpResult>>) {
        if matches!(op, Op::Add { .. }) {
            self.cart.pulse.update(|n| *n += 1);
        }
        let key = op.key();
        let mut cancelled = false;
        self.queue.update_value(|q| {
            match &op {
                // The newest quantity for a line wins over one not sent yet.
                Op::SetQty { .. } | Op::Remove { .. } => {
                    q.retain(|j| !(matches!(j.op, Op::SetQty { .. }) && j.op.key() == key))
                }
                // Undo before the removal even left: the two cancel out.
                Op::Add { line: Some(_), .. } => {
                    let removal = |j: &Job| {
                        matches!(j.op, Op::Remove { .. } | Op::SetQty { qty: 0, .. })
                            && j.op.key() == key
                    };
                    if let Some(i) = q.iter().position(removal) {
                        q.remove(i);
                        cancelled = true;
                    }
                }
                Op::Add { .. } => {}
            }
            if !cancelled {
                q.push_back(Job { op, done });
            }
        });
        self.busy.set(true);
        self.render();
        self.pump();
    }

    /// The displayed cart: the confirmed one with everything pending applied on top.
    fn render(self) {
        let mut pending: Vec<Op> = self
            .inflight
            .with_value(|f| f.iter().map(|j| j.op.clone()).collect());
        self.queue
            .with_value(|q| pending.extend(q.iter().map(|j| j.op.clone())));
        match self.confirmed.get_untracked() {
            Some(base) => {
                let (mut v, unseen) = math::rebase(&base, &pending, self.fee.get_value());
                self.order.with_value(|o| math::sort_lines(&mut v, o));
                let count = v.count + unseen;
                self.cart.view.set(Some(v));
                self.cart.count.set(count);
            }
            None => {
                let adds: u32 = pending
                    .iter()
                    .map(|o| if let Op::Add { qty, .. } = o { *qty } else { 0 })
                    .sum();
                self.cart.count.set(self.base_count.get_value() + adds);
            }
        }
    }

    fn pump(self) {
        if self.inflight.with_value(Option::is_some) {
            return;
        }
        let Some(job) = self.queue.try_update_value(|q| q.pop_front()).flatten() else {
            self.busy.set(false);
            return;
        };
        self.inflight.set_value(Some(job.clone()));
        leptos::task::spawn_local(async move {
            let res = send(&job.op).await;
            self.inflight.set_value(None);
            let out = match res {
                Ok(v) => {
                    self.adopt(v.clone());
                    Ok(v)
                }
                Err(e) => Err(super::copy::code(&e)),
            };
            self.render();
            match (&out, job.done) {
                (_, Some(cb)) => cb.run(out),
                (Err(code), None) => {
                    let lang = self.ui.lang.get_untracked();
                    self.toasts
                        .show(ToastKind::Error, super::copy::message(code, lang));
                }
                _ => {}
            }
            self.pump();
        });
    }

    fn adopt(self, v: CartView) {
        self.order.update_value(|o| {
            for l in &v.lines {
                if !o.contains(&l.key) {
                    o.push(l.key.clone());
                }
            }
        });
        if v.delivery_fee.0 > 0 {
            self.fee.set_value(v.delivery_fee);
        }
        self.base_count.set_value(v.count);
        self.load_failed.set(false);
        self.confirmed.set(Some(v));
    }

    /// Fetch the cart from the server (the drawer calls this each time it opens: the cached cart
    /// shows at once, the server's answer replaces it a moment later).
    pub fn revalidate(self) {
        if self.loading.get_untracked() || self.busy.get_untracked() {
            return;
        }
        self.loading.set(true);
        leptos::task::spawn_local(async move {
            let r = cart_get().await;
            self.loading.set(false);
            match r {
                // An op started meanwhile: its answer is newer than this one.
                Ok(v) if !self.busy.get_untracked() => {
                    self.adopt(v);
                    self.render();
                }
                Ok(_) => {}
                Err(_) => self
                    .load_failed
                    .set(self.confirmed.with_untracked(Option::is_none)),
            }
        });
    }

    /// Has the server's cart been seen yet?
    pub fn loaded(self) -> bool {
        self.confirmed.with(Option::is_some)
    }

    /// The `cart_n` cookie disagrees with the store (the cart changed elsewhere): adopt its count
    /// and refetch the lines if the drawer is open.
    #[cfg(feature = "hydrate")]
    fn sync_from_cookie(self) {
        if self.busy.get_untracked() {
            return;
        }
        let n = client::cookie_count();
        let have = self.cart.count.get_untracked();
        if n != have {
            if n > have {
                self.cart.pulse.update(|p| *p += 1);
            }
            self.base_count.set_value(n);
            self.cart.count.set(n);
            // The lines we hold are someone else's past: show a skeleton until the refetch.
            self.confirmed.set(None);
            self.cart.view.set(None);
            if self.cart.open.get_untracked() {
                self.revalidate();
            }
        }
    }
}

async fn send(op: &Op) -> Result<CartView, ServerFnError> {
    match op.clone() {
        Op::SetQty { key, qty } => cart_set_qty(key, qty).await,
        Op::Remove { key } => cart_remove(key).await,
        Op::Add {
            product,
            variant,
            qty,
            addons,
            ..
        } => {
            cart_add(
                product,
                variant,
                Some(qty),
                (!addons.is_empty()).then(|| addons.join(",")),
            )
            .await
        }
    }
}

/// Did this island hydrate after an islands-router navigation (rather than on page load)? Used to
/// move focus to a new step's heading only when the page changed under the visitor.
pub fn arrived_by_navigation() -> bool {
    #[cfg(feature = "hydrate")]
    {
        client::BOOTED.load(std::sync::atomic::Ordering::Relaxed)
    }
    #[cfg(not(feature = "hydrate"))]
    {
        false
    }
}

/// Focus (and scroll to) the element matching `selector` on the next frame.
pub fn focus_soon(selector: &'static str) {
    #[cfg(feature = "hydrate")]
    request_animation_frame(move || client::focus(selector));
    #[cfg(not(feature = "hydrate"))]
    let _ = selector;
}

/// Navigate through the islands router (view transition, no reload), falling back to a normal
/// navigation where its script isn't running.
pub fn soft_navigate(href: &str) {
    #[cfg(feature = "hydrate")]
    client::soft_navigate(href);
    #[cfg(not(feature = "hydrate"))]
    let _ = href;
}

#[cfg(feature = "hydrate")]
mod client {
    use super::CartOps;
    use leptos::prelude::*;
    use std::sync::atomic::{AtomicBool, Ordering};
    use wasm_bindgen::{JsCast, JsValue, closure::Closure};

    pub static BOOTED: AtomicBool = AtomicBool::new(false);

    pub fn cookie_count() -> u32 {
        let cookies = document()
            .dyn_into::<web_sys::HtmlDocument>()
            .ok()
            .and_then(|d| d.cookie().ok())
            .unwrap_or_default();
        crate::request::parse_cookies(&cookies)
            .find(|(k, _)| *k == "cart_n")
            .and_then(|(_, v)| v.parse().ok())
            .unwrap_or(0)
    }

    pub fn soft_navigate(href: &str) {
        let doc = document();
        let (Ok(a), Some(body)) = (doc.create_element("a"), doc.body()) else {
            let _ = window().location().set_href(href);
            return;
        };
        let _ = a.set_attribute("href", href);
        let _ = a.set_attribute("hidden", "");
        let _ = body.append_child(&a);
        // The islands router listens for clicks on same-origin links on `window`.
        a.unchecked_ref::<web_sys::HtmlElement>().click();
        a.remove();
    }

    pub fn focus(selector: &str) {
        if let Ok(Some(el)) = document().query_selector(selector) {
            if let Some(el) = el.dyn_ref::<web_sys::HtmlElement>() {
                let _ = el.focus();
            }
        }
    }

    fn listen(target: &web_sys::EventTarget, event: &str, f: impl FnMut(web_sys::Event) + 'static) {
        let cb = Closure::<dyn FnMut(web_sys::Event)>::new(f);
        let _ = target.add_event_listener_with_callback(event, cb.as_ref().unchecked_ref());
        cb.forget();
    }

    pub fn init(ops: CartOps) {
        // 1. Server-function redirects to island calls become islands-router navigations.
        //    (First hook wins; `<ActionForm>` only installs its full-reload default if none is set.)
        let _ = leptos::server_fn::redirect::set_redirect_hook(|loc: &str| soft_navigate(loc));

        // 2. Keep the badge in step with the cookie after every navigation, on back/forward
        //    (incl. bfcache) and when the tab comes back.
        let win: web_sys::EventTarget = window().into();
        let nav = js_sys::Reflect::get(&window(), &JsValue::from_str("navigation"))
            .ok()
            .filter(JsValue::is_object);
        if let Some(nav) = nav {
            listen(nav.unchecked_ref(), "currententrychange", move |_| {
                ops.cart.open.set(false);
                ops.sync_from_cookie();
            });
        }
        listen(&win, "popstate", move |_| ops.sync_from_cookie());
        listen(&win, "pageshow", move |_| ops.sync_from_cookie());
        listen(&document().into(), "visibilitychange", move |_| {
            if !document().hidden() {
                ops.sync_from_cookie();
            }
        });

        // 3. Server-rendered forms marked `data-pending-ui` (the /cart steppers, checkout steps)
        //    are posted by the islands router: show them busy, and put focus back afterwards — the
        //    router's keyed diff moves list items, which drops focus to <body>.
        listen(&document().into(), "submit", move |ev| {
            if ev.default_prevented() {
                return; // an island handled it
            }
            let Some(form) = ev
                .target()
                .and_then(|t| t.dyn_into::<web_sys::HtmlFormElement>().ok())
            else {
                return;
            };
            if !form.has_attribute("data-pending-ui") {
                return;
            }
            let _ = form.set_attribute("aria-busy", "true");
            let submitter = ev
                .dyn_ref::<web_sys::SubmitEvent>()
                .and_then(|s| s.submitter());
            let focus_id = submitter.and_then(|s| {
                let _ = s.class_list().add_1("is-pending");
                s.get_attribute("data-focus-id")
            });
            restore_focus_after_patch(focus_id, form);
        });

        // 4. Islands hydrated from now on arrived by navigation.
        set_timeout(
            || BOOTED.store(true, Ordering::Relaxed),
            std::time::Duration::ZERO,
        );
    }

    /// Wait for the router to patch the page, then (if focus fell to <body>) focus, in order: an
    /// element the server marked `data-autofocus` (an error message), the control that was pressed
    /// (same `data-focus-id`), or the list's `data-focus-fallback` (when that control is gone).
    fn restore_focus_after_patch(focus_id: Option<String>, form: web_sys::HtmlFormElement) {
        let Some(body) = document().body() else {
            return;
        };
        let done = std::rc::Rc::new(std::cell::Cell::new(false));
        let observer_slot: std::rc::Rc<std::cell::RefCell<Option<web_sys::MutationObserver>>> =
            Default::default();
        let restore = {
            let done = done.clone();
            let slot = observer_slot.clone();
            move || {
                if done.replace(true) {
                    return;
                }
                if let Some(o) = slot.borrow_mut().take() {
                    o.disconnect();
                }
                let doc = document();
                let lost = doc.active_element().is_none_or(|a| {
                    doc.body()
                        .is_some_and(|b| a == *b.unchecked_ref::<web_sys::Element>())
                });
                if !lost {
                    return;
                }
                let pick = |sel: String| doc.query_selector(&sel).ok().flatten();
                let target = pick("[data-autofocus]".into())
                    .or_else(|| {
                        focus_id
                            .as_ref()
                            .and_then(|id| pick(format!("[data-focus-id=\"{id}\"]")))
                    })
                    .or_else(|| pick("[data-focus-fallback]".into()));
                if let Some(el) = target.and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok()) {
                    let _ = el.focus();
                }
            }
        };
        let restore = std::rc::Rc::new(restore);
        let on_mutation = {
            let restore = restore.clone();
            Closure::<dyn FnMut(js_sys::Array, web_sys::MutationObserver)>::new(
                move |_: js_sys::Array, _: web_sys::MutationObserver| {
                    let r = restore.clone();
                    request_animation_frame(move || r());
                },
            )
        };
        if let Ok(o) = web_sys::MutationObserver::new(on_mutation.as_ref().unchecked_ref()) {
            let opts = web_sys::MutationObserverInit::new();
            opts.set_child_list(true);
            opts.set_subtree(true);
            let _ = o.observe_with_options(&body, &opts);
            *observer_slot.borrow_mut() = Some(o);
        }
        on_mutation.forget();
        // Nothing came back (offline, server down): stop waiting and un-busy the form.
        let slot = observer_slot.clone();
        set_timeout(
            move || {
                done.set(true);
                if let Some(o) = slot.borrow_mut().take() {
                    o.disconnect();
                }
                if form.is_connected() && form.get_attribute("aria-busy").as_deref() == Some("true")
                {
                    let _ = form.set_attribute("aria-busy", "false");
                    if let Ok(list) = form.query_selector_all(".is-pending") {
                        for i in 0..list.length() {
                            if let Some(el) = list
                                .item(i)
                                .and_then(|n| n.dyn_into::<web_sys::Element>().ok())
                            {
                                let _ = el.class_list().remove_1("is-pending");
                            }
                        }
                    }
                }
            },
            std::time::Duration::from_secs(10),
        );
    }
}
