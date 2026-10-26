//! 404 and error pages.

use leptos::prelude::*;

#[component]
pub fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        if let Some(resp) = use_context::<leptos_axum::ResponseOptions>() {
            resp.set_status(http::StatusCode::NOT_FOUND);
        }
    }
    view! { <section class="stub"><h1>"404"</h1></section> }
}
