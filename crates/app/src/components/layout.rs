//! Placeholder until the design system lands (agent/design owns this file).

use leptos::prelude::*;

#[component]
pub fn Container(children: Children) -> impl IntoView {
    view! { <div class="container">{children()}</div> }
}

#[component]
pub fn SectionHeader(
    title: String,
    #[prop(optional)] subtitle: Option<String>,
    #[prop(optional)] href: Option<String>,
    #[prop(optional)] link_label: Option<String>,
) -> impl IntoView {
    view! {
        <header class="section-header">
            <div><p class="section-header__title">{title}</p>{subtitle.map(|s| view! { <p class="section-header__sub">{s}</p> })}</div>
            {href.map(|h| view! { <a class="section-header__link" href=h>{link_label}</a> })}
        </header>
    }
}

#[component]
pub fn Rail(children: Children) -> impl IntoView {
    view! { <div class="rail">{children()}</div> }
}
