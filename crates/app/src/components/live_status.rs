//! The state of a region that keeps itself up to date (order tracking; any polling region): a
//! breathing dot and "Live updates" while it works, a warning with "Retrying in 8 s" and a Retry
//! button after a failed update, "You're offline…" without a connection, or a plain line when
//! there's nothing left to watch (`Idle`: "This order is complete.").
//!
//! Browser code drives it with `client/src/core/live-status.ts`: `setLiveStatus(el, state, msg?)`
//! and `retryIn(el, ms, retry)` (the countdown, and the button calls `retry`). The visible line is
//! `data-slot="msg"`; screen readers hear a separate polite line (`data-slot="said"`) that changes
//! with the state, not with every tick of the countdown.

use super::icons::Icon;
use crate::prefs::lang_now;
use crate::strings;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LiveState {
    /// Up to date and watching.
    #[default]
    Live,
    /// The last update failed; trying again soon.
    Retrying,
    /// No connection.
    Offline,
    /// Nothing more to watch: just the message.
    Idle,
}

impl LiveState {
    /// `data-state` (and `setLiveStatus`'s states).
    pub fn code(self) -> &'static str {
        match self {
            LiveState::Live => "live",
            LiveState::Retrying => "retrying",
            LiveState::Offline => "offline",
            LiveState::Idle => "idle",
        }
    }
}

#[component]
pub fn LiveStatus(
    #[prop(optional)] state: LiveState,
    /// The line to show; defaults to the state's own ("Live updates", "You're offline…").
    #[prop(optional, into)]
    message: Option<String>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let lang = lang_now();
    let g = move |k: &str| strings::get(k, lang);
    let message = message.unwrap_or_else(|| {
        match state {
            LiveState::Live => g("live.on"),
            LiveState::Retrying => g("live.failed"),
            LiveState::Offline => g("live.offline"),
            LiveState::Idle => "",
        }
        .to_owned()
    });
    let cls = if class.is_empty() {
        "live-status".to_owned()
    } else {
        format!("live-status {class}")
    };
    view! {
        <div class=cls data-state=state.code()>
            <span class="live-status__dot" aria-hidden="true"></span>
            <Icon name="triangle-alert" size=16 class="live-status__icon live-status__icon--retrying"/>
            <Icon name="wifi-off" size=16 class="live-status__icon live-status__icon--offline"/>
            <span class="live-status__msg" data-slot="msg" aria-hidden="true">{message.clone()}</span>
            <span class="sr-only" role="status" data-slot="said">{message}</span>
            <button type="button" class="live-status__retry" data-live-retry="">{g("net.retry")}</button>
        </div>
    }
}
