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
use leptos::either::Either;
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

#[island]
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
    #[cfg(feature = "hydrate")]
    poll::start(poll::Poll {
        id: StoredValue::new(id),
        t,
        announce,
        root,
        ui,
    });
    #[cfg(not(feature = "hydrate"))]
    let _ = id;

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
            <p class="order-live__foot">
                {move || {
                    let lang = ui.lang.get();
                    if !mounted.get() {
                        // Server HTML (and no-JS): when this was rendered, and a way to look again.
                        let at = t.with(|t| format::time_cairo(t.now_ms, lang, ui.numerals.get()));
                        Either::Left(view! {
                            <span>{format!("{} {at}", tr(lang, "Updated", "آخر تحديث"))}</span>
                            <a class="link" href="">{tr(lang, "Refresh", "تحديث")}</a>
                        })
                    } else if terminal() {
                        Either::Right(Either::Left(view! { <span>{tr(lang, "This order is complete.", "الطلب ده خلص.")}</span> }))
                    } else {
                        Either::Right(Either::Right(view! {
                            <span class="live"><span class="live__dot" aria-hidden="true"></span>{tr(lang, "Live updates", "تحديثات مباشرة")}</span>
                        }))
                    }
                }}
            </p>
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

    #[derive(Clone, Copy)]
    pub struct Poll {
        pub id: StoredValue<String>,
        pub t: RwSignal<OrderTrack>,
        pub announce: RwSignal<String>,
        pub root: NodeRef<leptos::html::Div>,
        pub ui: Ui,
    }

    /// Poll at most every 5 s, at least every 1 s; right after the next step is due if sooner.
    fn wait_ms(t: &OrderTrack) -> Option<u64> {
        if t.status.is_terminal() {
            return None;
        }
        let due = t.next.map(|(_, at)| at - t.now_ms + 400).unwrap_or(5_000);
        Some(due.clamp(1_000, 5_000) as u64)
    }

    pub fn start(p: Poll) {
        if let Some(ms) = wait_ms(&p.t.get_untracked()) {
            schedule(p, ms);
        }
    }

    fn schedule(p: Poll, ms: u64) {
        set_timeout(move || tick(p), std::time::Duration::from_millis(ms));
    }

    fn tick(p: Poll) {
        // The islands router replaced the page: stop for good.
        let alive = p.root.get_untracked().is_some_and(|el| el.is_connected());
        if !alive {
            return;
        }
        if document().hidden() {
            schedule(p, 5_000);
            return;
        }
        let id = p.id.get_value();
        leptos::task::spawn_local(async move {
            if let Ok(Some(next)) = order_status(id).await {
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
            }
            // On a failed poll, keep the last good status and try again later.
            if let Some(ms) = wait_ms(&p.t.get_untracked()) {
                schedule(p, ms);
            }
        });
    }
}
