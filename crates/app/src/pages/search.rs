//! Search page + the header's instant-search box. (Stub — replaced by the browse agent.)

use leptos::prelude::*;

#[component]
pub fn SearchPage() -> impl IntoView {
    view! { <section class="stub"><h1>"Search"</h1></section> }
}

/// The search box in the site header. Must work as a plain GET form to `/search?q=` without JS.
#[component]
pub fn HeaderSearch() -> impl IntoView {
    view! {
        <form class="header-search" action="/search" method="get" role="search">
            <input type="search" name="q" aria-label="Search"/>
        </form>
    }
}
