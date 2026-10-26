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
use crate::api::cart::{cart_add, cart_get, cart_remove, cart_set_qty};
use crate::api::types::{CartLineView, CartView};
use crate::components::toast::{ToastKind, Toasts, use_toasts};
use crate::prefs::{Ui, use_ui};
use crate::stores::{CartStore, use_cart};
use domain::Piastres;
use leptos::prelude::*;
use std::collections::VecDeque;

/// What a queued op's caller hears back: the confirmed cart, or an error code (see `copy`).
pub type OpResult = Result<CartView, String>;

/// Something that went wrong with one line, shown on that line.
#[derive(Clone, Debug, PartialEq)]
pub struct LineIssue {
    pub key: String,
    pub kind: IssueKind,
    /// The change to send again (Retry); none for a note that needs no action.
    pub op: Option<Op>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IssueKind {
    /// No network: the change is rolled back and retried by itself when the browser is online.
    Offline,
    /// The server refused or failed: rolled back, Retry offered.
    Failed,
    /// The server could only keep this many (stock).
    Capped(u32),
}

impl IssueKind {
    pub fn message(self, lang: crate::prefs::Lang, num: crate::prefs::Numerals) -> String {
        use crate::prefs::Lang;
        let n = |x: u32| crate::format::count(x, num);
        match (self, lang) {
            (IssueKind::Offline, Lang::En) => {
                "You're offline — this change isn't saved yet.".into()
            }
            (IssueKind::Offline, Lang::Ar) => {
                "أنت غير متصل بالإنترنت — لم يُحفظ هذا التعديل بعد.".into()
            }
            (IssueKind::Failed, Lang::En) => "This change didn't save.".into(),
            (IssueKind::Failed, Lang::Ar) => "تعذّر حفظ هذا التعديل.".into(),
            (IssueKind::Capped(c), Lang::En) => {
                format!("Only {} available — we've set it to {}.", n(c), n(c))
            }
            (IssueKind::Capped(c), Lang::Ar) => {
                format!("المتوفر {} فقط — تم ضبط الكمية على {}.", n(c), n(c))
            }
        }
    }
}

/// A line removed from the drawer, kept for a few seconds so its "Undo" can put it back in place.
#[derive(Clone, Debug, PartialEq)]
pub struct Removed {
    pub id: u32,
    pub line: CartLineView,
    /// Its index in the list when it was removed.
    pub at: usize,
}

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
    /// Lines just removed in the drawer (their "Undo" rows).
    pub removed: RwSignal<Vec<Removed>>,
    next_removed: StoredValue<u32>,
    /// A failure to show inside the open drawer (toasts sit under its modal layer).
    pub drawer_error: RwSignal<Option<String>>,
    /// Per-line problems (offline, failed, capped).
    pub issues: RwSignal<Vec<LineIssue>>,
    /// The empty drawer's illustration and suggestions (fetched once).
    pub suggestions: RwSignal<Option<super::page::Suggestions>>,
    suggestions_asked: StoredValue<bool>,
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
    let ops = CartOps::new(use_cart(), use_ui(), use_toasts());
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
            removed: RwSignal::new(Vec::new()),
            next_removed: StoredValue::new(1),
            drawer_error: RwSignal::new(None),
            issues: RwSignal::new(Vec::new()),
            suggestions: RwSignal::new(None),
            suggestions_asked: StoredValue::new(false),
        }
    }

    /// Keep a removed line for its "Undo" (8 s); returns the row id.
    pub fn remember_removed(self, line: CartLineView, at: usize) -> u32 {
        let id = self.next_removed.get_value();
        self.next_removed.set_value(id + 1);
        self.removed.update(|r| r.push(Removed { id, line, at }));
        #[cfg(feature = "hydrate")]
        set_timeout(
            move || self.forget_removed(id),
            std::time::Duration::from_secs(8),
        );
        id
    }

    pub fn forget_removed(self, id: u32) {
        self.removed.update(|r| r.retain(|x| x.id != id));
    }

    /// Say what went wrong where the shopper is looking: inside the drawer if it's open (a modal
    /// dialog covers the page's toasts), else as a toast.
    fn report(self, code: &str) {
        let msg = super::copy::message(code, self.ui.lang.get_untracked());
        if self.cart.open.get_untracked() {
            self.drawer_error.set(Some(msg));
            #[cfg(feature = "hydrate")]
            set_timeout(
                move || self.drawer_error.set(None),
                std::time::Duration::from_secs(6),
            );
        } else {
            self.toasts.show(msg, ToastKind::Error);
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
                    self.note_success(&job.op, &v);
                    self.adopt(v.clone());
                    Ok(v)
                }
                Err(e) => Err(super::copy::code(&e)),
            };
            self.render();
            match (&out, job.done) {
                (_, Some(cb)) => cb.run(out),
                (Err(code), None) => self.note_failure(job.op, code),
                _ => {}
            }
            self.pump();
        });
    }

    /// A change for a line went through: clear its old problem, and say so if the server could
    /// only keep fewer than asked (stock).
    fn note_success(self, op: &Op, v: &CartView) {
        let key = op.key();
        let capped = match op {
            Op::SetQty { qty, .. } => v
                .lines
                .iter()
                .find(|l| l.key == key)
                .filter(|l| l.qty < *qty)
                .map(|l| l.qty),
            _ => None,
        };
        self.issues.update(|i| {
            i.retain(|x| x.key != key);
            if let Some(c) = capped {
                i.push(LineIssue {
                    key: key.clone(),
                    kind: IssueKind::Capped(c),
                    op: None,
                });
            }
        });
        if capped.is_some() {
            #[cfg(feature = "hydrate")]
            set_timeout(
                move || {
                    self.issues.update(|i| {
                        i.retain(|x| !(x.key == key && matches!(x.kind, IssueKind::Capped(_))))
                    })
                },
                std::time::Duration::from_secs(8),
            );
        }
    }

    /// A change failed: it has already rolled back (it left the queue); tell the shopper on the
    /// line itself, with a Retry. An undo that failed puts its "Undo" row back.
    fn note_failure(self, op: Op, code: &str) {
        #[cfg(feature = "hydrate")]
        let offline = code == "network" || !window().navigator().on_line();
        #[cfg(not(feature = "hydrate"))]
        let offline = code == "network";
        let key = op.key();
        let on_line = self.cart.view.with_untracked(|v| {
            v.as_ref()
                .is_some_and(|v| v.lines.iter().any(|l| l.key == key))
        });
        match &op {
            Op::Add {
                line: Some(line), ..
            } if !on_line => {
                self.remember_removed((**line).clone(), 0);
                self.report(code);
            }
            _ if on_line => {
                let kind = if offline {
                    IssueKind::Offline
                } else {
                    IssueKind::Failed
                };
                self.issues.update(|i| {
                    i.retain(|x| x.key != key);
                    i.push(LineIssue {
                        key,
                        kind,
                        op: Some(op),
                    });
                });
            }
            _ => self.report(code),
        }
    }

    /// Send a line's failed change again.
    pub fn retry(self, key: &str) {
        let op = self.issues.try_update(|i| {
            let at = i.iter().position(|x| x.key == key)?;
            i.remove(at).op
        });
        if let Some(op) = op.flatten() {
            self.submit(op, None);
        }
    }

    /// Back online: send every change that failed for lack of a network.
    pub fn retry_offline(self) {
        let keys: Vec<String> = self.issues.with_untracked(|i| {
            i.iter()
                .filter(|x| x.kind == IssueKind::Offline)
                .map(|x| x.key.clone())
                .collect()
        });
        for k in keys {
            self.retry(&k);
        }
    }

    pub fn issue(self, key: &str) -> Option<LineIssue> {
        self.issues
            .with(|i| i.iter().find(|x| x.key == key).cloned())
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

    /// Fetch the empty-cart suggestions once (a failure just leaves them out).
    pub fn load_suggestions(self) {
        if self.suggestions_asked.get_value() {
            return;
        }
        self.suggestions_asked.set_value(true);
        leptos::task::spawn_local(async move {
            match super::page::cart_suggestions().await {
                Ok(s) => self.suggestions.set(Some(s)),
                Err(_) => self.suggestions_asked.set_value(false),
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

/// Like [`focus_soon`] for a selector built at run time.
pub fn focus_soon_dyn(selector: String) {
    #[cfg(feature = "hydrate")]
    request_animation_frame(move || client::focus(&selector));
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
        // Changes that failed for lack of a network go out again as soon as it's back.
        listen(&win, "online", move |_| ops.retry_offline());
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
            // Offline: don't let the post vanish into a dead fetch. Keep everything as it is and
            // say so where the shopper is looking.
            if !window().navigator().on_line() {
                ev.prevent_default();
                show_net_notice(&form, ops);
                return;
            }
            hide_net_notice(&form);
            let _ = form.set_attribute("aria-busy", "true");
            let submitter = ev
                .dyn_ref::<web_sys::SubmitEvent>()
                .and_then(|s| s.submitter());
            let focus_id = submitter.and_then(|s| {
                let _ = s.class_list().add_1("is-pending");
                s.get_attribute("data-focus-id")
            });
            // Forms that come back to the same page (the /cart steppers) keep their scroll
            // position; the shell scrolls to the top after posts to another path.
            let keep_y = form
                .has_attribute("data-keep-scroll")
                .then(|| window().scroll_y().unwrap_or(0.0));
            restore_focus_after_patch(focus_id, keep_y, form, ops);
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
    /// Reveal the form's own "couldn't reach the shop" notice (server-rendered, `hidden`, with a
    /// real submit button to try again); forms without one get a toast.
    fn show_net_notice(form: &web_sys::HtmlFormElement, ops: CartOps) {
        match form.query_selector("[data-net-notice]").ok().flatten() {
            Some(n) => {
                let _ = n.remove_attribute("hidden");
                if let Some(el) = n.dyn_ref::<web_sys::HtmlElement>() {
                    let _ = el.focus();
                }
            }
            None => {
                let lang = ops.ui.lang.get_untracked();
                ops.toasts.show(
                    crate::prefs::tr(
                        lang,
                        "You're offline. Nothing was sent — try again when you're back online.",
                        "أنت غير متصل بالإنترنت. لم يُرسل شيء — حاول مرة أخرى عند عودة الاتصال.",
                    ),
                    crate::components::toast::ToastKind::Error,
                );
            }
        }
    }

    fn hide_net_notice(form: &web_sys::HtmlFormElement) {
        if let Some(n) = form.query_selector("[data-net-notice]").ok().flatten() {
            let _ = n.set_attribute("hidden", "");
        }
    }

    fn restore_focus_after_patch(
        focus_id: Option<String>,
        keep_y: Option<f64>,
        form: web_sys::HtmlFormElement,
        ops: CartOps,
    ) {
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
                if let Some(y) = keep_y {
                    window().scroll_to_with_x_and_y(0.0, y);
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
                    if let Some(y) = keep_y {
                        window().scroll_to_with_x_and_y(0.0, y);
                    }
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
                    show_net_notice(&form, ops);
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
