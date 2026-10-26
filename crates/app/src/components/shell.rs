//! The page frame around every route: skip link, header, category strip, main, footer, phone tab
//! bar, and the page-wide singletons (cart drawer, toaster, offline banner, navigation feedback,
//! pull-to-refresh), plus the JSON the browser code reads (`#app-config`, `#app-strings`).
//!
//! A server component: it reads the request (`RequestCtx`) and the catalogue's categories
//! synchronously, so the frame is in the first streamed chunk. During route-list generation there
//! is no request; then it renders the routes alone.
//!
//! Client-side navigations (client/src/core/router.ts) swap `<main>` and every
//! `[data-region=<name>]` (category strip, footer, tab bar); the header with its search box, the
//! cart drawer and the toaster stay. A language, numerals or theme change swaps the whole `.app`.

use super::bottom_nav::{BottomNav, Tab};
use super::footer::SiteFooter;
use super::header::{CatNav, NavItem, SiteHeader};
use super::icons::Icon;
use super::offline::OfflineBanner;
use super::toast::Toaster;
use crate::prefs::tr;
use crate::request::RequestCtx;
use crate::strings;
use leptos::prelude::*;

/// The tenant's categories for the header strip and the footer, in display order.
fn nav_items() -> Vec<NavItem> {
    #[cfg(feature = "ssr")]
    {
        if use_context::<crate::server::AppState>().is_none() {
            return Vec::new();
        }
        crate::server::current_catalog()
            .nav()
            .iter()
            .map(|c| NavItem {
                slug: c.slug.clone(),
                name: c.name.clone(),
                icon: c.icon.clone(),
            })
            .collect()
    }
    #[cfg(not(feature = "ssr"))]
    {
        Vec::new()
    }
}

#[component]
pub fn AppShell(children: Children) -> impl IntoView {
    let Some(rq) = use_context::<RequestCtx>() else {
        // Route-list generation (no request): just the routes.
        return view! { <main id="main">{children()}</main> }.into_any();
    };
    let p = rq.prefs;
    let lang = p.lang;
    let path = rq
        .path_and_query
        .split('?')
        .next()
        .unwrap_or("/")
        .to_owned();
    let current_cat = path
        .strip_prefix("/c/")
        .map(|s| s.trim_end_matches('/').to_owned());
    let tab = Tab::for_path(&rq.path_and_query);
    let cats = nav_items();

    view! {
        <div class="app">
            <a class="skip-link" href="#main">
                {tr(lang, "Skip to content", "انتقل للمحتوى")}
            </a>
            <SiteHeader tenant=rq.tenant.clone() lang=lang theme=p.theme/>
            <CatNav categories=cats.clone() lang=lang current=current_cat.unwrap_or_default()/>
            <main id="main" tabindex="-1">{children()}</main>
            <SiteFooter rq=rq.clone() categories=cats/>
            <BottomNav lang=lang current=tab/>
            <crate::pages::cart::CartDrawer/>
            <Toaster/>
            <OfflineBanner/>
            <NavFeedback lang=lang/>
            <PullToRefresh/>
            <AppData rq=rq.clone()/>
        </div>
    }
    .into_any()
}

/// What the router's feedback drives (features/core.ts): the progress bar (only past 150 ms) and
/// the alert for a navigation that failed (5xx, 12 s timeout, a dropped connection; offline shows
/// the offline banner instead). The reason (`nav.why.*`) is put in by the browser code.
#[component]
fn NavFeedback(lang: crate::prefs::Lang) -> impl IntoView {
    let s = |k| strings::get(k, lang);
    view! {
        <div class="nav-progress" data-state="idle" aria-hidden="true">
            <span class="nav-progress__bar"></span>
        </div>
        <div class="nav-alert" data-open="false" role="alert">
            <Icon name="triangle-alert" size=20 class="nav-alert__icon"/>
            <p class="nav-alert__text">
                <strong>{s("nav.failed")}</strong>
                " "
                <span class="nav-alert__why"></span>
            </p>
            <button type="button" class="nav-alert__retry">{s("net.retry")}</button>
            <button type="button" class="nav-alert__close" aria-label=s("toast.dismiss")>
                <Icon name="x" size=16/>
            </button>
        </div>
    }
}

/// The pull-to-refresh disc (touch, at the top of the page): follows the finger down, turns its
/// arrow when letting go would refresh, spins while the page reloads in place.
#[component]
fn PullToRefresh() -> impl IntoView {
    view! {
        <div class="ptr" aria-hidden="true">
            <span class="ptr__disc">
                <Icon name="arrow-down" size=18 class="ptr__arrow"/>
                <Icon name="loader-circle" size=18 class="ptr__spin"/>
            </span>
        </div>
    }
}

/// What the browser code reads about this page (`client/src/core/page.ts`): `#app-config` (tenant,
/// display prefs, the endpoints it may call for data, the cart limits) and `#app-strings` (the UI
/// strings in this page's language, from `crate::strings`).
#[component]
fn AppData(rq: RequestCtx) -> impl IntoView {
    #[derive(serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Api {
        cart_quote: &'static str,
        search_suggest: &'static str,
        order_track: &'static str,
    }
    #[derive(serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    struct CartCfg {
        max_lines: usize,
        max_qty: u32,
        cookie: &'static str,
        /// The tenant's delivery fee and free-delivery threshold, piastres (the client computes
        /// the cart: `features/cart.ts`).
        delivery_fee: i64,
        free_delivery_over: i64,
        /// The money label in this language (`EGP` / `ج.م`), for `core/money.ts`.
        currency: &'static str,
    }
    #[derive(serde::Serialize)]
    struct Config<'a> {
        tenant: &'a str,
        lang: &'a str,
        numerals: &'a str,
        api: Api,
        cart: CartCfg,
    }
    let config = Config {
        tenant: &rq.tenant.id,
        lang: rq.prefs.lang.code(),
        numerals: rq.prefs.numerals.code(),
        api: Api {
            cart_quote: "/api/cart/quote",
            search_suggest: "/api/search/suggest",
            order_track: "/api/orders/track",
        },
        cart: CartCfg {
            max_lines: domain::cart::MAX_LINES,
            max_qty: domain::cart::MAX_QTY,
            cookie: domain::cart::COOKIE,
            delivery_fee: rq.tenant.delivery_fee.0,
            free_delivery_over: rq.tenant.free_delivery_over.0,
            currency: crate::format::currency_label(rq.prefs.lang),
        },
    };
    let config = serde_json::to_string(&config)
        .unwrap_or_default()
        .replace('<', "\\u003c");
    view! {
        <script type="application/json" id="app-config" inner_html=config></script>
        <script type="application/json" id="app-strings" inner_html=crate::strings::client_json(rq.prefs.lang, &rq.path_and_query)></script>
    }
}
