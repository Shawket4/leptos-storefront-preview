//! The page frame around every route: header, main, footer, global islands. (Stub.)

use leptos::prelude::*;

#[component]
pub fn AppShell(children: Children) -> impl IntoView {
    view! { <main id="main">{children()}</main> }
}
