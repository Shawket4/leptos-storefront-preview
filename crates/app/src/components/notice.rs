//! Notices: an inline message in a tone, inside the flow of a page — a load that failed, a rail
//! that didn't come, "Added to your cart", why an add didn't work, the search panel's offline,
//! error and empty states, checkout's problems. An icon, a title, an optional line under it, and
//! optional actions (children: a `Button`, a `.link`), which wrap under the text on phones.
//!
//! Announced by tone: danger and warning interrupt (`role="alert"`), the rest wait their turn
//! (`role="status"`); `announce` overrides that. Browser code that reveals a notice later renders
//! it with `attr:hidden=true` (and `attr:data-state=…` to find it); `slots` marks the title and
//! the line `data-slot="title"` / `"body"` so `patch()` (core/dom.ts) can write them.
//!
//! ```ignore
//! <Notice tone=Tone::Warning title=… body=…>
//!     <Button kind=ButtonKind::Secondary size=ButtonSize::Sm href=retry icon="rotate-ccw">…</Button>
//! </Notice>
//! <Notice tone=Tone::Danger title=strings::get("cart.err.full", lang) attr:tabindex="-1"/>
//! ```
//!
//! Its class is `.callout` (`.notice` is still a cart page class).

pub use super::status_pill::Tone;

use super::icons::Icon;
use leptos::prelude::*;

/// How a notice is announced to screen readers.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Announce {
    /// By tone: danger and warning `role="alert"`, the others `role="status"`.
    #[default]
    Auto,
    /// `role="status"`: read when the reader is free (a section that didn't load).
    Polite,
    /// `role="alert"`: read at once.
    Assertive,
    /// No live role: a standing note, or one another live region already reads out.
    Off,
}

fn tone_parts(tone: Tone) -> (&'static str, &'static str) {
    match tone {
        Tone::Neutral => ("callout--neutral", "info"),
        Tone::Brand => ("callout--brand", "sparkles"),
        Tone::Success => ("callout--success", "circle-check"),
        Tone::Warning => ("callout--warning", "triangle-alert"),
        Tone::Danger => ("callout--danger", "circle-alert"),
        Tone::Info => ("callout--info", "info"),
    }
}

#[component]
pub fn Notice(
    /// `Info`, `Success`, `Warning`, `Danger` (also `Neutral`, the default, and `Brand`).
    #[prop(optional)]
    tone: Tone,
    /// The message, in one sentence.
    #[prop(into)]
    title: TextProp,
    /// A quieter line under it: what it means, or what to do.
    #[prop(optional, into)]
    body: Option<TextProp>,
    /// A lucide icon; defaults to the tone's own.
    #[prop(optional)]
    icon: Option<&'static str>,
    #[prop(optional)] announce: Announce,
    /// Mark the title and the line `data-slot="title"` / `"body"` for browser code that writes
    /// them (`patch()`); the line is rendered (hidden) even without `body`.
    #[prop(optional)]
    slots: bool,
    #[prop(optional, into)] class: String,
    /// Actions at the end: a small `Button`, a link.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let (tone_class, tone_icon) = tone_parts(tone);
    let role = match announce {
        Announce::Auto if matches!(tone, Tone::Danger | Tone::Warning) => Some("alert"),
        Announce::Auto | Announce::Polite => Some("status"),
        Announce::Assertive => Some("alert"),
        Announce::Off => None,
    };
    let cls = if class.is_empty() {
        format!("callout {tone_class}")
    } else {
        format!("callout {tone_class} {class}")
    };
    let slot = move |name: &'static str| slots.then_some(name);
    let body = match body {
        Some(b) => Some(
            view! { <p class="callout__body" data-slot=slot("body")>{move || b.get()}</p> }
                .into_any(),
        ),
        None => slots
            .then(|| view! { <p class="callout__body" data-slot="body" hidden></p> }.into_any()),
    };
    view! {
        <div class=cls role=role>
            <span class="callout__icon">
                <Icon name=icon.unwrap_or(tone_icon) size=20/>
            </span>
            <div class="callout__main">
                <div class="callout__text">
                    <p class="callout__title" data-slot=slot("title")>{move || title.get()}</p>
                    {body}
                </div>
                {children.map(|c| view! { <div class="callout__actions">{c()}</div> })}
            </div>
        </div>
    }
}
