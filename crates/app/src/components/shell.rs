//! The page frame around every route: skip link, header, category strip, main, footer, phone tab
//! bar, and the global mounts (cart drawer, toaster), plus the shell's small script.
//!
//! A server component: it reads the request (`RequestCtx`) and the catalogue's categories
//! synchronously, so the frame is in the first streamed chunk. During route-list generation there
//! is no request; then it renders the routes alone.

use super::bottom_nav::{BottomNav, Tab};
use super::footer::SiteFooter;
use super::header::{CatNav, NavItem, SiteHeader};
use super::icons::Icon;
use super::offline::OfflineBanner;
use super::toast::Toaster;
use crate::prefs::tr;
use crate::request::RequestCtx;
use leptos::prelude::*;

/// Progressive enhancement for the whole frame (see shell.js; cargo-leptos only rebuilds on .rs
/// edits, so touch this comment after changing it: prefetch, progress, failures, prefs next,
/// previews, submitter, scroll, bare ?).
const SHELL_JS: &str = include_str!("shell.js");

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
            // target="_self" keeps the skip link native: the islands router would otherwise
            // re-fetch the page instead of moving focus to the content.
            <a class="skip-link" href="#main" target="_self">
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
            <script inner_html=SHELL_JS></script>
        </div>
    }
    .into_any()
}

/// What shell.js drives during router navigations: the progress bar (shown only past 150 ms) and
/// the alert for a navigation that failed (offline, 5xx, 12 s timeout), its words rendered here in
/// the visitor's language so the script needs none of its own.
#[component]
fn NavFeedback(lang: crate::prefs::Lang) -> impl IntoView {
    view! {
        <div class="nav-progress" data-state="idle" aria-hidden="true">
            <span class="nav-progress__bar"></span>
        </div>
        <div
            class="nav-alert"
            data-open="false"
            role="alert"
            data-why-offline=tr(lang, "You're offline.", "أنت غير متصل بالإنترنت.")
            data-why-timeout=tr(lang, "It took too long to answer.", "استغرق الرد وقتًا طويلًا.")
            data-why-network=tr(lang, "The connection dropped.", "انقطع الاتصال.")
            data-why-server=tr(lang, "The store had a problem.", "حدثت مشكلة في المتجر.")
        >
            <Icon name="triangle-alert" size=20 class="nav-alert__icon"/>
            <p class="nav-alert__text">
                <strong>{tr(lang, "Couldn't open that page", "تعذّر فتح هذه الصفحة")}</strong>
                " "
                <span class="nav-alert__why"></span>
            </p>
            <button type="button" class="nav-alert__retry">{tr(lang, "Try again", "إعادة المحاولة")}</button>
            <button type="button" class="nav-alert__close" aria-label=tr(lang, "Dismiss", "إغلاق")>
                <Icon name="x" size=16/>
            </button>
        </div>
    }
}
