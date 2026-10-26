//! The page frame around every route: skip link, header, category strip, main, footer, phone tab
//! bar, and the global mounts (cart drawer, toaster), plus the shell's small script.
//!
//! A server component: it reads the request (`RequestCtx`) and the catalogue's categories
//! synchronously, so the frame is in the first streamed chunk. During route-list generation there
//! is no request; then it renders the routes alone.

use super::bottom_nav::{BottomNav, Tab};
use super::footer::SiteFooter;
use super::header::{CatNav, NavItem, SiteHeader};
use super::toast::Toaster;
use crate::prefs::tr;
use crate::request::RequestCtx;
use leptos::prelude::*;

/// Progressive enhancement for the whole frame (see shell.js; cargo-leptos only rebuilds on .rs
/// edits, so touch this comment after changing it).
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
            <script inner_html=SHELL_JS></script>
        </div>
    }
    .into_any()
}
