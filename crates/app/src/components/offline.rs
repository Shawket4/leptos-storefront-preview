//! The offline banner: when the device loses its connection (or a navigation fails because of it),
//! a calm bar at the top says so and offers a retry; when the connection comes back it leaves with
//! a "back online" toast, and a navigation that failed meanwhile is tried again. The current page
//! stays as it is throughout. Mounted once by the shell; the behaviour is `features/core.ts`.

use super::icons::Icon;
use crate::prefs::lang_now;
use crate::strings;
use leptos::prelude::*;

#[component]
pub fn OfflineBanner() -> impl IntoView {
    let lang = lang_now();
    let s = |k| strings::get(k, lang);
    view! {
        <div class="offline" data-open="false" role="status">
            <Icon name="wifi-off" size=20 class="offline__icon"/>
            <p class="offline__text">
                <strong hidden>{s("net.offline")}</strong>
                <span class="offline__hint" hidden>{s("net.offline_hint")}</span>
                <span class="offline__still" hidden>{s("net.offline_still")}</span>
            </p>
            <button type="button" class="offline__retry">{s("net.retry")}</button>
        </div>
    }
}
