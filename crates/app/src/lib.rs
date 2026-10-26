#![recursion_limit = "512"]

pub mod api;
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
    SsrMode,
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
                {home_nojs_fallback(&rq.path_and_query)}
                <link rel="stylesheet" id="leptos" href="/pkg/storefront.css"/>
                // Per-tenant icons (WebP, like every image asset here).
                <link rel="icon" type="image/webp" sizes="64x64" href=format!("/media/brand/{}-icon-64.webp", t.id)/>
                <link rel="icon" type="image/webp" sizes="192x192" href=format!("/media/brand/{}-icon-192.webp", t.id)/>
                <link rel="apple-touch-icon" href=format!("/media/brand/{}-icon-192.webp", t.id)/>
                <meta name="theme-color" content=if p.theme == prefs::Theme::Dark { "#0B0B0C" } else { "#FAFAF9" }/>
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
                    // Out-of-order streaming: fastest first paint, sections land as their data
                    // does — but placing them needs JS (each arrives as a <template> + script).
                    <Route path=path!("") view=pages::HomePage/>
                    // The same home page streamed in order, for visitors without JS (see shell).
                    <Route path=path!("nojs") view=pages::HomePage ssr=SsrMode::InOrder/>
                    // Must work fully without JS → in-order streaming (no script needed).
                    <Route path=path!("c/:slug") view=pages::CategoryPage ssr=SsrMode::InOrder/>
                    <Route path=path!("search") view=pages::SearchPage ssr=SsrMode::InOrder/>
                    // Blocking product resource: <head> waits for it (title/OG/JSON-LD are data),
                    // and its HTML is rendered inline; the rest still streams.
                    <Route path=path!("p/:slug") view=pages::ProductPage ssr=SsrMode::PartiallyBlocked/>
                    <Route path=path!("cart") view=pages::CartPage ssr=SsrMode::InOrder/>
                    <Route path=path!("checkout") view=pages::CheckoutPage ssr=SsrMode::InOrder/>
                    <Route path=path!("orders/:id") view=pages::OrderPage ssr=SsrMode::InOrder/>
                    <Route path=path!("showcase") view=pages::ShowcasePage/>
                    <Route path=path!("ui") view=pages::UiKitPage/>
                </Routes>
            </components::shell::AppShell>
        </Router>
    }
}

/// Out-of-order streamed sections only appear with JS. On the home page, visitors without JS are
/// sent to `/nojs`, the same page streamed in order. (A `<noscript>` in `<head>` only runs when
/// scripting is off, so everyone else never sees it.)
#[cfg(feature = "ssr")]
fn home_nojs_fallback(path_and_query: &str) -> Option<impl IntoView + use<>> {
    let query = match path_and_query.split_once('?') {
        Some(("/", q)) => format!("?{q}"),
        None if path_and_query == "/" => String::new(),
        _ => return None,
    };
    let content = format!("0; url=/nojs{query}");
    Some(view! { <noscript><meta http-equiv="refresh" content=content/></noscript> })
}
