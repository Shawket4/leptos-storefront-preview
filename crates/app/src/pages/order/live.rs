//! The tracking block: the headline, the expected time and the status timeline. The server renders
//! it current (with a Refresh link, so it's complete without JS); `features/checkout.ts`
//! (`data-ts="order-track"`) then polls `GET /api/orders/track?id=` — paced by when the next step
//! is due, backing off after failures, paused while the tab is hidden — and moves the same
//! `Timeline` and `LiveStatus` (`core/timeline.ts`, `core/live-status.ts`), announcing each new
//! status politely, until a final status.

use super::TrackJson;
use crate::components::live_status::{LiveState, LiveStatus};
use crate::components::timeline::{StepState, Timeline, TimelineStep};
use crate::prefs::{Lang, tr};
use domain::OrderStatus;
use leptos::prelude::*;

#[component]
pub fn OrderLive(t: TrackJson, lang: Lang) -> impl IntoView {
    let ar = lang.is_rtl();
    let steps = OrderStatus::STEPS
        .into_iter()
        .enumerate()
        .map(|(i, step)| {
            let s = t.steps.get(i).cloned().unwrap_or_default();
            let state = match s.state.as_str() {
                "done" => StepState::Done,
                "current" => StepState::Current,
                _ => StepState::Todo,
            };
            TimelineStep::new(step.label().pick(ar), state).time(s.time)
        })
        .collect::<Vec<_>>();
    view! {
        <div class="order-live" data-ts="order-track" data-id=t.id.clone() data-status=t.status.clone()
            data-final=t.done.to_string() data-next-in=t.next_in_ms.map(|n| n.to_string())>
            <h1 id="order-headline" class="order-live__headline" tabindex="-1" data-slot="headline">{t.headline.clone()}</h1>
            <p class="order-live__eta" data-slot="eta">{t.eta.clone()}</p>
            <Timeline steps progress=t.progress stopped=t.cancelled slots=true label=tr(lang, "Order progress", "مراحل الطلب")/>
            <div class="order-live__foot">
                // Without JS: when this was rendered, and a way to look again.
                <span class="order-live__static" data-static="">
                    <span data-slot="updated">{t.updated.clone()}</span>
                    " "
                    <a class="link" href="">{tr(lang, "Refresh", "تحديث")}</a>
                </span>
                // With JS: live, retrying, offline, or complete (the browser code shows it).
                <LiveStatus state=LiveState::Idle attr:hidden=true/>
            </div>
            <p class="sr-only" aria-live="polite" aria-atomic="true" data-announce=""></p>
        </div>
    }
}
