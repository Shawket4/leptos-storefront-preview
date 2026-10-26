//! The phone tab bar (below 960px): Home, Categories, Deals, Cart, Account — plain links on a
//! translucent material, the current tab in ink. The cart tab carries a count slot for the cart
//! island (`.bottom-nav__badge`).

use super::icons::Icon;
use crate::prefs::{Lang, tr};
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tab {
    Home,
    Categories,
    Deals,
    Cart,
    Account,
}

impl Tab {
    /// Which tab a request path belongs to, if any.
    pub fn for_path(path_and_query: &str) -> Option<Tab> {
        let (path, query) = path_and_query
            .split_once('?')
            .unwrap_or((path_and_query, ""));
        match path {
            "/" | "/nojs" => Some(Tab::Home),
            "/search" if query.split('&').any(|kv| kv == "deal=1") => Some(Tab::Deals),
            "/cart" | "/checkout" => Some(Tab::Cart),
            p if p.starts_with("/c/") => Some(Tab::Categories),
            p if p == "/orders" || p.starts_with("/orders/") => Some(Tab::Account),
            _ => None,
        }
    }
}

/// Where each tab goes.
pub const DEALS_HREF: &str = "/search?deal=1&sort=discount";
pub const CATEGORIES_HREF: &str = "/#categories";
pub const ACCOUNT_HREF: &str = "/orders";

#[component]
pub fn BottomNav(
    lang: Lang,
    current: Option<Tab>,
    /// Rendered inside the cart tab (the commerce agent's count island), if any.
    #[prop(optional)]
    cart_badge: Option<Children>,
) -> impl IntoView {
    let tabs = [
        (Tab::Home, "/", "house", tr(lang, "Home", "الرئيسية")),
        (
            Tab::Categories,
            CATEGORIES_HREF,
            "layout-grid",
            tr(lang, "Categories", "الأقسام"),
        ),
        (
            Tab::Deals,
            DEALS_HREF,
            "badge-percent",
            tr(lang, "Deals", "العروض"),
        ),
        (
            Tab::Cart,
            "/cart",
            "shopping-bag",
            tr(lang, "Cart", "السلة"),
        ),
        (
            Tab::Account,
            ACCOUNT_HREF,
            "package",
            tr(lang, "Orders", "طلباتي"),
        ),
    ];
    let mut cart_badge = cart_badge;
    view! {
        <nav class="bottom-nav" aria-label=tr(lang, "Main", "التنقل الرئيسي")>
            <ul class="bottom-nav__list" role="list">
                {tabs
                    .into_iter()
                    .map(|(tab, href, icon, label)| {
                        let aria = if current == Some(tab) { "page" } else { "false" };
                        let badge = (tab == Tab::Cart)
                            .then(|| view! { <span class="bottom-nav__badge">{cart_badge.take().map(|c| c())}</span> });
                        view! {
                            <li>
                                <a class="bottom-nav__link" href=href aria-current=aria data-press="">
                                    <span class="bottom-nav__icon">
                                        <Icon name=icon size=24/>
                                        {badge}
                                    </span>
                                    <span class="bottom-nav__label">{label}</span>
                                </a>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </nav>
    }
}
