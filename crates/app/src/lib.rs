#![recursion_limit = "512"]

pub mod pages;
pub mod prefs;
pub mod request;
pub mod tenant;

#[cfg(feature = "ssr")]
pub mod server;

#[cfg(feature = "hydrate")]
pub mod client;

use leptos::prelude::*;
use leptos_meta::{MetaTags, Title, provide_meta_context};
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
    let brand_css = format!(
        ":root{{--accent-light:{};--accent-dark:{};--on-accent:{}}}",
        t.accent, t.accent_dark, t.on_accent
    );
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
                <meta name="color-scheme" content="light dark"/>
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
        <Title formatter=|t: String| t/>
        <Router>
            <main id="main">
                <Routes fallback=|| view! { <pages::NotFound/> }>
                    <Route path=path!("") view=pages::HomePage/>
                </Routes>
            </main>
        </Router>
    }
}
