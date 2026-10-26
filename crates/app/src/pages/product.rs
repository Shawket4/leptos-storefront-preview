//! Product detail page. (Stub — replaced by the owning agent.)

use leptos::prelude::*;

#[component]
pub fn ProductPage() -> impl IntoView {
    view! { <section class="stub"><h1>"Product detail"</h1></section> }
}

/// Provide the product page's client store (e.g. the selected variant shared by the gallery and
/// the purchase panel islands). Called on the server per request and once on the client.
pub fn provide_store() {}
