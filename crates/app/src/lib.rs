#![recursion_limit = "512"]

pub mod api;
pub mod brand;
pub mod components;
pub mod format;
pub mod media;
pub mod pages;
pub mod prefs;
pub mod request;
pub mod stores;
pub mod tenant;

#[cfg(feature = "ssr")]
pub mod server;

#[cfg(feature = "hydrate")]
pub mod client;

use leptos::prelude::*;
use leptos_meta::provide_meta_context;
#[cfg(feature = "ssr")]
use leptos_meta::MetaTags;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

/// The HTML document. Server only: resolves the tenant and preferences first, so `<html>` already
/// carries the right `lang`, `dir`, theme and tenant accent in the first streamed chunk.
#[cfg(feature = "ssr")]
pub fn shell(options: LeptosOptions) -> impl IntoView {
    let rq = request::init();
    let p = rq.prefs;
    let t = &rq.tenant;
    let brand_css = brand::tenant_css(&t.accent, &t.accent_dark);
    // Native controls/scrollbars follow the chosen theme; only "system" lets the device decide.
    let color_scheme = match p.theme {
        prefs::Theme::Light => "light",
        prefs::Theme::Dark => "dark",
        prefs::Theme::System => "light dark",
    };
    view! {
        <!DOCTYPE html>
        <html
            lang=p.lang.code()
            dir=p.lang.dir()
            data-theme=p.theme.code()
            data-numerals=p.numerals.code()
            data-tenant=t.id.clone()
        >
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1, viewport-fit=cover"/>
                <meta name="color-scheme" content=color_scheme/>
                <style id="tenant-brand" inner_html=brand_css></style>
                <link rel="stylesheet" id="leptos" href="/pkg/storefront.css"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options islands=true islands_router=true/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Router>
            <components::shell::AppShell>
                <Routes fallback=|| view! { <pages::NotFound/> }>
                    <Route path=path!("") view=pages::HomePage/>
                    <Route path=path!("c/:slug") view=pages::CategoryPage/>
                    <Route path=path!("p/:slug") view=pages::ProductPage/>
                    <Route path=path!("search") view=pages::SearchPage/>
                    <Route path=path!("cart") view=pages::CartPage/>
                    <Route path=path!("checkout") view=pages::CheckoutPage/>
                    <Route path=path!("orders/:id") view=pages::OrderPage/>
                    <Route path=path!("showcase") view=pages::ShowcasePage/>
                    <Route path=path!("ui") view=pages::UiKitPage/>
                </Routes>
            </components::shell::AppShell>
        </Router>
    }
}
