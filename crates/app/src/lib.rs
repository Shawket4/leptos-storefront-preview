#![recursion_limit = "512"]

pub mod client_js;
pub mod api;
pub mod components;
pub mod format;
pub mod media;
pub mod pages;
pub mod prefs;
pub mod request;
pub mod strings;
pub mod tenant;

#[cfg(feature = "ssr")]
pub mod server;


use leptos::prelude::*;
use leptos_meta::provide_meta_context;
#[cfg(feature = "ssr")]
use leptos_meta::{HashedStylesheet, MetaTags};
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
                // The two faces the first screen needs, per language (Arabic pages set Arabic words).
                {match p.lang {
                    prefs::Lang::Ar => view! {
                        <link rel="preload" href="/fonts/cairo-arabic-wght-normal.woff2" r#as="font" r#type="font/woff2" crossorigin=""/>
                        <link rel="preload" href="/fonts/cairo-latin-wght-normal.woff2" r#as="font" r#type="font/woff2" crossorigin=""/>
                    }.into_any(),
                    prefs::Lang::En => view! {
                        <link rel="preload" href="/fonts/cairo-latin-wght-normal.woff2" r#as="font" r#type="font/woff2" crossorigin=""/>
                        <link rel="preload" href="/fonts/ibm-plex-mono-latin-600-normal.woff2" r#as="font" r#type="font/woff2" crossorigin=""/>
                    }.into_any(),
                }}
                // Content-hashed by cargo-leptos (hash-files), so it is served immutable.
                <HashedStylesheet options=options.clone() id="leptos"/>
                // Per-tenant icons (WebP, like every image asset here).
                <link rel="icon" type="image/webp" sizes="64x64" href=format!("/media/brand/{}-icon-64.webp", t.id)/>
                <link rel="icon" type="image/webp" sizes="192x192" href=format!("/media/brand/{}-icon-192.webp", t.id)/>
                <link rel="apple-touch-icon" href=format!("/media/brand/{}-icon-192.webp", t.id)/>
                <meta name="theme-color" content=if p.theme == prefs::Theme::Dark { "#0B0B0C" } else { "#FAFAF9" }/>
                <AutoReload options=options.clone()/>
                // The browser code: TypeScript bundled into one ES module (client/, build.rs).
                <script type="module" src=client_js::APP_JS_PATH></script>
                // The chunks the entry imports, fetched in parallel with it (no waterfall).
                {client_js::PRELOAD.iter().map(|href| view! { <link rel="modulepreload" href=*href/> }).collect_view()}
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
                    // Every page renders in order: one complete document that works without JS
                    // and that the client router can swap in (out-of-order chunks need scripts a
                    // fetched document never runs). The out-of-order demo has its own route.
                    <Route path=path!("") view=pages::HomePage ssr=SsrMode::InOrder/>
                    <Route path=path!("c/:slug") view=pages::CategoryPage ssr=SsrMode::InOrder/>
                    <Route path=path!("search") view=pages::SearchPage ssr=SsrMode::InOrder/>
                    <Route path=path!("p/:slug") view=pages::ProductPage ssr=SsrMode::InOrder/>
                    <Route path=path!("cart") view=pages::CartPage ssr=SsrMode::InOrder/>
                    <Route path=path!("checkout") view=pages::CheckoutPage ssr=SsrMode::InOrder/>
                    <Route path=path!("orders") view=pages::OrdersLookupPage ssr=SsrMode::InOrder/>
                    <Route path=path!("orders/:id") view=pages::OrderPage ssr=SsrMode::InOrder/>
                    <Route path=path!("showcase") view=pages::ShowcasePage ssr=SsrMode::InOrder/>
                    <Route path=path!("ui") view=pages::UiKitPage ssr=SsrMode::InOrder/>
                    // The core agent's scratch route (dev checks of core behaviour and components).
                    <Route path=path!("core-scratch") view=pages::core_scratch::CoreScratchPage ssr=SsrMode::InOrder/>
                    // Showcase agent (agreed with the lead): the one out-of-order route, the demo.
                    <Route path=path!("showcase/stream") view=pages::showcase::StreamDemoPage ssr=SsrMode::OutOfOrder/>
                </Routes>
            </components::shell::AppShell>
        </Router>
    }
}
