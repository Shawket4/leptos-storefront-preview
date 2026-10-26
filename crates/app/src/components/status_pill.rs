//! Status pills (Madar's): icon + label, 24px tall, pill-shaped, 12px semibold on a 12–14% tint
//! of the tone. For order states, stock, payment results.

use super::icons::Icon;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Tone {
    #[default]
    Neutral,
    Brand,
    Success,
    Warning,
    Danger,
    Info,
}

impl Tone {
    pub fn class(self) -> &'static str {
        match self {
            Tone::Neutral => "tone-neutral",
            Tone::Brand => "tone-brand",
            Tone::Success => "tone-success",
            Tone::Warning => "tone-warning",
            Tone::Danger => "tone-danger",
            Tone::Info => "tone-info",
        }
    }

    /// The icon a tone wears when the caller doesn't pick one.
    pub fn default_icon(self) -> &'static str {
        match self {
            Tone::Neutral => "info",
            Tone::Brand => "sparkles",
            Tone::Success => "circle-check",
            Tone::Warning => "triangle-alert",
            Tone::Danger => "circle-x",
            Tone::Info => "info",
        }
    }
}

#[component]
pub fn StatusPill(
    #[prop(optional)] tone: Tone,
    /// The label.
    #[prop(into)]
    label: TextProp,
    /// A lucide icon; defaults to the tone's own.
    #[prop(optional)]
    icon: Option<&'static str>,
    /// Label only.
    #[prop(optional)]
    no_icon: bool,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let icon = (!no_icon).then(|| icon.unwrap_or_else(|| tone.default_icon()));
    let cls = if class.is_empty() {
        format!("pill {}", tone.class())
    } else {
        format!("pill {} {class}", tone.class())
    };
    view! {
        <span class=cls>
            {icon.map(|i| view! { <Icon name=i size=14/> })}
            <span class="pill__label">{move || label.get()}</span>
        </span>
    }
}
