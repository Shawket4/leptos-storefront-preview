//! The tracking island: the headline, the expected time and the status timeline, kept current.
//!
//! It starts from the server's snapshot (so the server HTML is already right), then polls
//! `order_status` — every 5 s at most, sooner when the next step is due — and stops for good once
//! the order is delivered or cancelled, or when the page it lives on is gone. While the tab is
//! hidden it doesn't fetch. Each new status is announced through a polite live region.

use super::OrderTrack;
use crate::components::icons::Icon;
use crate::format;
use crate::pages::cart::store::{arrived_by_navigation, focus_soon};
use crate::prefs::{Lang, Ui, tr, use_ui};
use domain::OrderStatus;
use leptos::either::EitherOf5;
use leptos::prelude::*;

fn eta_text(t: &OrderTrack, lang: Lang, ui: Ui) -> String {
    let num = ui.numerals.get();
    let at = |ms| format::time_cairo(ms, lang, num);
    match t.status {
        OrderStatus::Delivered => {
            let when = t
                .times
                .iter()
                .find(|(s, _)| *s == OrderStatus::Delivered)
                .map(|(_, ms)| at(*ms))
                .unwrap_or_default();
            match lang {
                Lang::En => format!("Delivered at {when}. Enjoy!"),
                Lang::Ar => format!("اتوصّل الساعة {when}. بالهنا والشفا!"),
            }
        }
        OrderStatus::Cancelled | OrderStatus::Rejected => tr(
            lang,
            "If you paid, the amount will be refunded.",
            "لو كنت دفعت، المبلغ هيرجعلك.",
        )
        .to_owned(),
        _ => match (t.eta_ms, lang) {
            (Some(eta), Lang::En) => format!("Arriving by {}", at(eta)),
            (Some(eta), Lang::Ar) => format!("هيوصلك قبل {}", at(eta)),
            (None, Lang::En) => format!("Delivery window: {}", t.slot.en),
            (None, Lang::Ar) => format!("ميعاد التوصيل: {}", t.slot.ar),
        },
    }
}

/// How the live connection is doing (only the browser ever leaves `Live`).
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(not(feature = "hydrate"), allow(dead_code))]
enum Net {
    Live,
    /// A poll failed; the next try is at `at_ms` (client clock).
    Retrying {
        at_ms: f64,
    },
    /// The browser says it has no network.
    Offline,
}

#[cfg_attr(feature = "split", island(lazy))]
#[cfg_attr(not(feature = "split"), island)]
pub fn OrderLive(id: String, track: OrderTrack) -> impl IntoView {
    let ui = use_ui();
    let t = RwSignal::new(track);
    let announce = RwSignal::new(String::new());
    let mounted = RwSignal::new(false);
    let root = NodeRef::<leptos::html::Div>::new();

    Effect::new(move |_| {
        mounted.set(true);
        // Arrived here by placing an order (an islands-router navigation): land on the status.
        if arrived_by_navigation() {
            focus_soon("#order-headline");
        }
    });
    let net = RwSignal::new(Net::Live);
    // A one-second clock for the retry countdown (ticks only while retrying).
    let clock = RwSignal::new(0.0_f64);
    #[cfg(feature = "hydrate")]
    let retry_now = {
        let p = poll::Poll {
            id: StoredValue::new(id),
            t,
            announce,
            root,
            ui,
            net,
            clock,
            generation: StoredValue::new(0),
            failures: StoredValue::new(0),
        };
        poll::start(p);
        Callback::new(move |_: ()| poll::now(p))
    };
    #[cfg(not(feature = "hydrate"))]
    let retry_now = {
        let _ = (id, clock);
        Callback::new(|_: ()| {})
    };

    let progress = move || {
        t.with(|t| {
            t.status
                .step_index()
                .map(|i| i as f32 / (OrderStatus::STEPS.len() - 1) as f32)
                .unwrap_or(0.0)
        })
    };
    let terminal = move || t.with(|t| t.status.is_terminal());
    let cancelled =
        move || t.with(|t| matches!(t.status, OrderStatus::Cancelled | OrderStatus::Rejected));

    view! {
        <div class="order-live" node_ref=root data-status=move || { format!("{:?}", t.with(|t| t.status)).to_lowercase() }>
            <h1 id="order-headline" class="order-live__headline" tabindex="-1">
                {move || t.with(|t| t.status.headline().pick(ui.lang.get().is_rtl()).to_owned())}
            </h1>
            <p class="order-live__eta">{move || t.with(|t| eta_text(t, ui.lang.get(), ui))}</p>
            <ol class="timeline" style=move || { format!("--p:{:.4}", progress()) } data-cancelled=move || { cancelled().to_string() }>
                {OrderStatus::STEPS.into_iter().enumerate().map(|(i, step)| {
                    let state = move || {
                        let cur = t.with(|t| t.status.step_index()).unwrap_or(0);
                        if cancelled() { "todo" } else if i < cur || (i == cur && terminal()) { "done" } else if i == cur { "current" } else { "todo" }
                    };
                    let time = move || {
                        let (lang, num) = (ui.lang.get(), ui.numerals.get());
                        t.with(|t| {
                            if let Some((_, ms)) = t.times.iter().find(|(s, _)| *s == step) {
                                Some(format::time_cairo(*ms, lang, num))
                            } else if t.next.is_some_and(|(s, _)| s == step) && !cancelled() {
                                let at = format::time_cairo(t.next.map(|(_, ms)| ms).unwrap_or_default(), lang, num);
                                Some(match lang {
                                    Lang::En => format!("Expected around {at}"),
                                    Lang::Ar => format!("متوقع حوالي {at}"),
                                })
                            } else {
                                None
                            }
                        })
                    };
                    view! {
                        <li class=move || { format!("timeline__step is-{}", state()) }
                            aria-current=move || { if state() == "current" { "step" } else { "false" } }>
                            <span class="timeline__dot" aria-hidden="true"><Icon name="check" size=12/></span>
                            <span class="timeline__text">
                                <span class="timeline__label">{move || step.label().pick(ui.lang.get().is_rtl()).to_owned()}</span>
                                {move || time().map(|x| view! { <span class="timeline__time">{x}</span> })}
                            </span>
                        </li>
                    }
                }).collect_view()}
            </ol>
            <div class="order-live__foot">
                {move || {
                    let (lang, num) = (ui.lang.get(), ui.numerals.get());
                    if !mounted.get() {
                        // Server HTML (and no-JS): when this was rendered, and a way to look again.
                        let at = t.with(|t| format::time_cairo(t.now_ms, lang, num));
                        EitherOf5::A(view! {
                            <span>{format!("{} {at}", tr(lang, "Updated", "آخر تحديث"))}</span>
                            <a class="link" href="">{tr(lang, "Refresh", "تحديث")}</a>
                        })
                    } else if terminal() {
                        EitherOf5::B(view! { <span>{tr(lang, "This order is complete.", "الطلب ده خلص.")}</span> })
                    } else {
                        match net.get() {
                            Net::Live => EitherOf5::C(view! {
                                <span class="live"><span class="live__dot" aria-hidden="true"></span>{tr(lang, "Live updates", "تحديثات مباشرة")}</span>
                            }),
                            Net::Retrying { at_ms, .. } => {
                                let secs = move || (((at_ms - clock.get()) / 1000.0).ceil().max(0.0)) as u32;
                                EitherOf5::D(view! {
                                    <span class="net-state net-state--lost" role="status">
                                        <Icon name="rotate-ccw" size=16 class="spin-slow"/>
                                        <span>
                                            {tr(lang, "Connection lost.", "انقطع الاتصال.")}
                                            " "
                                            <span aria-hidden="true">{move || match lang {
                                                Lang::En => format!("Retrying in {} s…", format::count(secs(), num)),
                                                Lang::Ar => format!("هنحاول تاني خلال {} ث…", format::count(secs(), num)),
                                            }}</span>
                                        </span>
                                    </span>
                                    <button type="button" class="link-btn" on:click=move |_| retry_now.run(())>{tr(lang, "Retry now", "حاول دلوقتي")}</button>
                                })
                            }
                            Net::Offline => EitherOf5::E(view! {
                                <span class="net-state net-state--offline" role="status">
                                    <Icon name="triangle-alert" size=16/>
                                    <span>{tr(lang, "You're offline. Updates resume when you're back.", "إنت مش متصل بالإنترنت. التحديثات هترجع أول ما ترجع.")}</span>
                                </span>
                                <button type="button" class="link-btn" on:click=move |_| retry_now.run(())>{tr(lang, "Retry", "حاول تاني")}</button>
                            }),
                        }
                    }
                }}
            </div>
            <p class="sr-only" aria-live="polite" aria-atomic="true">
                {move || { let a = announce.get(); (!a.is_empty()).then(|| view! { <span>{a}</span> }) }}
            </p>
        </div>
    }
}

#[cfg(feature = "hydrate")]
mod poll {
    use super::*;
    use crate::pages::order::order_status;
    use wasm_bindgen::{JsCast, closure::Closure};

    #[derive(Clone, Copy)]
    pub struct Poll {
        pub id: StoredValue<String>,
        pub t: RwSignal<OrderTrack>,
        pub announce: RwSignal<String>,
        pub root: NodeRef<leptos::html::Div>,
        pub ui: Ui,
        pub net: RwSignal<Net>,
        pub clock: RwSignal<f64>,
        /// Bumped by every (re)schedule: a timer from an older schedule does nothing.
        pub generation: StoredValue<u32>,
        pub failures: StoredValue<u32>,
    }

    fn now_ms() -> f64 {
        js_sys::Date::now()
    }

    /// Poll at most every 5 s, at least every 1 s; right after the next step is due if sooner.
    fn wait_ms(t: &OrderTrack) -> Option<u64> {
        if t.status.is_terminal() {
            return None;
        }
        let due = t.next.map(|(_, at)| at - t.now_ms + 400).unwrap_or(5_000);
        Some(due.clamp(1_000, 5_000) as u64)
    }

    /// After a failure: 3 s, 6 s, 12 s, then every 20 s.
    fn backoff_ms(failures: u32) -> u64 {
        (3_000u64 << failures.saturating_sub(1).min(3)).min(20_000)
    }

    fn alive(p: Poll) -> bool {
        p.root.get_untracked().is_some_and(|el| el.is_connected())
    }

    pub fn start(p: Poll) {
        // Back online → look right away; offline → say so and wait.
        let on_online = Closure::<dyn FnMut()>::new(move || {
            if alive(p) {
                now(p);
            }
        });
        let on_offline = Closure::<dyn FnMut()>::new(move || {
            if alive(p) && !p.t.with_untracked(|t| t.status.is_terminal()) {
                p.generation.update_value(|g| *g += 1);
                p.net.set(Net::Offline);
            }
        });
        let w = window();
        let _ = w.add_event_listener_with_callback("online", on_online.as_ref().unchecked_ref());
        let _ = w.add_event_listener_with_callback("offline", on_offline.as_ref().unchecked_ref());
        on_online.forget();
        on_offline.forget();
        // The countdown's clock (cleared once the page is gone).
        let handle: StoredValue<Option<IntervalHandle>> = StoredValue::new(None);
        let h = set_interval_with_handle(
            move || {
                if !alive(p) {
                    if let Some(h) = handle.get_value() {
                        h.clear();
                    }
                    return;
                }
                if matches!(p.net.get_untracked(), Net::Retrying { .. }) {
                    p.clock.set(now_ms());
                }
            },
            std::time::Duration::from_millis(500),
        );
        handle.set_value(h.ok());
        if !crate::components::net::is_online() {
            p.net.set(Net::Offline);
            return;
        }
        if let Some(ms) = wait_ms(&p.t.get_untracked()) {
            schedule(p, ms);
        }
    }

    fn schedule(p: Poll, ms: u64) {
        p.generation.update_value(|g| *g += 1);
        let mine = p.generation.get_value();
        set_timeout(
            move || {
                if p.generation.get_value() == mine {
                    tick(p);
                }
            },
            std::time::Duration::from_millis(ms),
        );
    }

    /// Poll now (the Retry button, coming back online).
    pub fn now(p: Poll) {
        p.generation.update_value(|g| *g += 1);
        tick(p);
    }

    fn tick(p: Poll) {
        // The islands router replaced the page: stop for good.
        if !alive(p) {
            return;
        }
        if document().hidden() {
            schedule(p, 5_000);
            return;
        }
        let id = p.id.get_value();
        leptos::task::spawn_local(async move {
            match order_status(id).await {
                Ok(Some(next)) => {
                    p.failures.set_value(0);
                    p.net.set(Net::Live);
                    let before = p.t.with_untracked(|t| t.status);
                    if next.status != before {
                        let lang = p.ui.lang.get_untracked();
                        let label = next.status.label();
                        p.announce.set(match lang {
                            Lang::En => format!("Order update: {}", label.en),
                            Lang::Ar => format!("تحديث الطلب: {}", label.ar),
                        });
                    }
                    p.t.set(next);
                    if let Some(ms) = wait_ms(&p.t.get_untracked()) {
                        schedule(p, ms);
                    }
                }
                // Gone (expired, or the server restarted): keep what we showed and stop asking.
                Ok(None) => {}
                // Keep the last good status; back off and say so.
                Err(_) => {
                    if !crate::components::net::is_online() {
                        p.net.set(Net::Offline);
                        return;
                    }
                    p.failures.update_value(|f| *f += 1);
                    let wait = backoff_ms(p.failures.get_value());
                    p.clock.set(now_ms());
                    p.net.set(Net::Retrying {
                        at_ms: now_ms() + wait as f64,
                    });
                    schedule(p, wait);
                }
            }
        });
    }
}
