//! Placeholder until the design system lands (agent/design owns this file).

use leptos::prelude::*;

#[component]
pub fn Icon(
    name: &'static str,
    #[prop(optional)] size: Option<u16>,
    #[prop(optional, into)] class: String,
    #[prop(optional)] label: Option<&'static str>,
) -> impl IntoView {
    let s = size.unwrap_or(20);
    view! { <span class=format!("icon icon-{name} {class}") style=format!("inline-size:{s}px;block-size:{s}px") aria-label=label aria-hidden=label.is_none().then_some("true")></span> }
}
