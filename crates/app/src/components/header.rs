//! The site header (translucent material chrome) and the category strip under it.
//!
//! Phones: the logo row scrolls away and the search row stays pinned (one sticky element with a
//! negative `top`, no JS). From 768px the search sits in the bar and the whole bar is pinned.
//! The category strip is not sticky: it scrolls with the page.

use super::assets::TenantLogo;
use super::icons::Icon;
use super::prefs_controls::{PrefsControls, PrefsVariant};
use crate::prefs::{Lang, Theme, tr};
use crate::tenant::Tenant;
use domain::L10n;
use leptos::prelude::*;

/// One entry of the category strip.
#[derive(Clone, Debug, PartialEq)]
pub struct NavItem {
    pub slug: String,
    pub name: L10n,
    /// A lucide icon name.
    pub icon: String,
}

#[component]
pub fn SiteHeader(tenant: Tenant, lang: Lang, theme: Theme) -> impl IntoView {
    let home_label = format!(
        "{} — {}",
        tenant.name.pick(lang.is_rtl()),
        tr(lang, "home", "الرئيسية")
    );
    view! {
        <header class="site-header">
            <div class="container site-header__bar">
                <a class="site-header__logo" href="/" aria-label=home_label>
                    <TenantLogo tenant=tenant lang=lang theme=theme height=30/>
                </a>
                <div class="site-header__search">
                    <crate::pages::search::HeaderSearch/>
                </div>
                <div class="site-header__actions">
                    <PrefsControls variant=PrefsVariant::Menu/>
                    <div class="site-header__cart">
                        <crate::pages::cart::CartButton/>
                    </div>
                </div>
            </div>
        </header>
    }
}

/// The category strip: chips with an icon, the current one filled. Scrolls sideways when it
/// doesn't fit (edges fade). `current` is the slug of the category being viewed.
#[component]
pub fn CatNav(
    categories: Vec<NavItem>,
    lang: Lang,
    #[prop(optional)] current: Option<String>,
) -> impl IntoView {
    let ar = lang.is_rtl();
    let label = tr(lang, "Categories", "الأقسام");
    view! {
        <nav class="cat-nav" aria-label=label>
            <div class="container">
                <ul class="cat-nav__list" role="list">
                    {categories
                        .into_iter()
                        .map(|c| {
                            // Always render aria-current (page/false): the islands router only
                            // ever adds or updates attributes when it patches the next page in.
                            let is_current = current.as_deref() == Some(c.slug.as_str());
                            let aria = if is_current { "page" } else { "false" };
                            let href = format!("/c/{}", c.slug);
                            let name = c.name.pick(ar).to_owned();
                            view! {
                                <li>
                                    <a class="cat-nav__link" href=href aria-current=aria data-press="">
                                        <Icon name=c.icon size=16/>
                                        <span class="cat-nav__label">{name}</span>
                                    </a>
                                </li>
                            }
                        })
                        .collect_view()}
                </ul>
            </div>
        </nav>
    }
}
