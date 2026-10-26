//! Placeholder until the design system lands (agent/design owns this file).

use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonKind {
    #[default]
    Primary,
    Secondary,
    Outline,
    Ghost,
    Danger,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonSize {
    Sm,
    #[default]
    Md,
    Lg,
}

#[component]
pub fn Button(
    #[prop(optional)] kind: ButtonKind,
    #[prop(optional)] size: ButtonSize,
    #[prop(optional)] href: Option<String>,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let _ = size;
    let cls = format!("btn btn--{kind:?} {class}").to_lowercase();
    match href {
        Some(h) => view! { <a class=cls href=h>{children()}</a> }.into_any(),
        None => view! { <button class=cls type="button">{children()}</button> }.into_any(),
    }
}
