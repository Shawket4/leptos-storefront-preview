//! Cart page, cart drawer, header cart button, quick-add, and the client cart store.
//! (Stubs — replaced by the commerce agent.)

use domain::{ProductId, VariantId};
use leptos::prelude::*;

#[component]
pub fn CartPage() -> impl IntoView {
    view! { <section class="stub"><h1>"Cart"</h1></section> }
}

/// Header cart button (link to `/cart`; the commerce agent makes it an island with the badge).
#[component]
pub fn CartButton() -> impl IntoView {
    view! { <a class="cart-button" href="/cart">"Cart"</a> }
}

/// The slide-over cart, mounted once by the app shell.
#[component]
pub fn CartDrawer() -> impl IntoView {}

/// The round "+" on product cards for single-variant, in-stock products.
#[component]
pub fn QuickAdd(product: ProductId, variant: VariantId) -> impl IntoView {
    let _ = (product, variant);
}

/// Provide the client cart store (called on the server per request and once on the client).
pub fn provide_store() {}
