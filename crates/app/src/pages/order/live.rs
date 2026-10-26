//! The tracking block: the headline, the expected time and the status timeline. The server renders
//! it current (with a Refresh link, so it's complete without JS); `features/checkout.ts`
//! (`data-ts="order-track"`) then polls `GET /api/orders/track?id=` — paced by when the next step
//! is due, backing off after failures, paused while the tab is hidden — fills the same slots from
//! the answer, announces each new status politely, and stops at a final status.

use super::TrackJson;
use crate::components::icons::Icon;
use crate::prefs::{Lang, tr};
use domain::OrderStatus;
use leptos::prelude::*;

#[component]
pub fn OrderLive(t: TrackJson, lang: Lang) -> impl IntoView {
    let ar = lang.is_rtl();
    view! {
        <div class="order-live" data-ts="order-track" data-id=t.id.clone() data-status=t.status.clone()
            data-final=t.done.to_string() data-next-in=t.next_in_ms.map(|n| n.to_string())>
            <h1 id="order-headline" class="order-live__headline" tabindex="-1" data-slot="headline">{t.headline.clone()}</h1>
            <p class="order-live__eta" data-slot="eta">{t.eta.clone()}</p>
            <ol class="timeline" style=format!("--p:{:.4}", t.progress) data-cancelled=t.cancelled.to_string()>
                {OrderStatus::STEPS.into_iter().enumerate().map(|(i, step)| {
                    let s = t.steps.get(i).cloned().unwrap_or_default();
                    view! {
                        <li class=format!("timeline__step is-{}", s.state) data-step=i
                            aria-current=if s.state == "current" { "step" } else { "false" }>
                            <span class="timeline__dot" aria-hidden="true"><Icon name="check" size=12/></span>
                            <span class="timeline__text">
                                <span class="timeline__label">{step.label().pick(ar).to_owned()}</span>
                                <span class="timeline__time" data-slot=format!("t{i}") hidden=s.time.is_none()>{s.time.clone()}</span>
                            </span>
                        </li>
                    }
                }).collect_view()}
            </ol>
            <div class="order-live__foot">
                // Without JS: when this was rendered, and a way to look again.
                <span class="order-live__static" data-static="">
                    <span data-slot="updated">{t.updated.clone()}</span>
                    " "
                    <a class="link" href="">{tr(lang, "Refresh", "تحديث")}</a>
                </span>
                <span class="live" data-live="" hidden="">
                    <span class="live__dot" aria-hidden="true"></span>{tr(lang, "Live updates", "تحديثات مباشرة")}
                </span>
                <span class="order-live__done" data-complete="" hidden="">{tr(lang, "This order is complete.", "الطلب ده خلص.")}</span>
                <span class="net-state" data-net="" role="status" hidden="">
                    <Icon name="triangle-alert" size=16/>
                    <span data-slot="net"></span>
                    <button type="button" class="link-btn" data-retry="">{tr(lang, "Retry now", "حاول دلوقتي")}</button>
                </span>
            </div>
            <p class="sr-only" aria-live="polite" aria-atomic="true" data-announce=""></p>
        </div>
    }
}
