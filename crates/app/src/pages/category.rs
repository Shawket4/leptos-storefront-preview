//! Category listing: `/c/:slug?min=&max=&brand=&stock=&deal=&sort=&page=`.
//!
//! Everything is decided from the URL. Synchronously, before the first byte: the category (an
//! unknown slug is a 404), the caching headers (`Cache-Tag: t{tenant}, t{tenant}-cat{id}`), title,
//! description, canonical and breadcrumb JSON-LD. Then the page streams in order (the route is
//! `SsrMode::InOrder`, so it is complete without JS) and hands the first page of results to the
//! `CatalogExplorer` island, which takes over filtering, sorting and paging in the browser.

pub(crate) mod explorer;
pub(crate) mod listing;
pub(crate) mod shim;

use crate::api::{
    self,
    types::{ListingQuery, NavCategory},
};
use crate::components::{icons::Icon, product_card::ProductGridSkeleton};
use crate::format;
use crate::prefs::{Lang, tr};
use crate::request::req;
use explorer::CatalogExplorer;
use leptos::either::Either;
use leptos::prelude::*;
use leptos_meta::{Link, Meta, Title};
use leptos_router::hooks::{use_location, use_params_map};
use shim::{Breadcrumbs, EmptyState};

/// The category for a slug, from the in-memory catalogue (synchronous: it decides the status code
/// and the cache tags before anything streams).
fn find_category(slug: &str) -> Option<NavCategory> {
    #[cfg(feature = "ssr")]
    {
        crate::server::current_catalog()
            .category_by_slug(slug)
            .cloned()
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = slug;
        None
    }
}

#[component]
pub fn CategoryPage() -> impl IntoView {
    let rq = req();
    let (lang, num) = (rq.prefs.lang, rq.prefs.numerals);
    let ar = lang.is_rtl();
    let slug = use_params_map()
        .with_untracked(|p| p.get("slug"))
        .unwrap_or_default();
    let query =
        ListingQuery::from_query(Some(slug.clone()), &use_location().search.get_untracked());

    let Some(cat) = find_category(&slug) else {
        return Either::Left(view! { <CategoryNotFound/> });
    };
    #[cfg(feature = "ssr")]
    {
        use crate::server::cache;
        cache::public(&[cache::tag_category(&rq.tenant.id, cat.id)]);
    }

    let site = rq.tenant.name.pick(ar).to_owned();
    let name = cat.name.pick(ar).to_owned();
    let title = listing::doc_title(&query, Some(&cat.name), &site, lang, num);
    let description = listing::category_description(&cat, &site, lang, num);
    // Page 1 canonicalises without `page` (`to_query` drops defaults); filters stay, in canonical order.
    let canonical = rq.absolute(&query.href());
    let crumbs_ld = serde_json::json!({
        "@context": "https://schema.org",
        "@type": "BreadcrumbList",
        "itemListElement": [
            { "@type": "ListItem", "position": 1, "name": tr(lang, "Home", "الرئيسية"), "item": rq.absolute("/") },
            { "@type": "ListItem", "position": 2, "name": name.clone(), "item": rq.absolute(&format!("/c/{}", cat.slug)) },
        ]
    })
    .to_string()
    .replace("</", "<\\/");

    let href = query.href();
    let data = Resource::new(|| (), move |_| api::list_products(query.clone()));

    Either::Right(view! {
        <Title text=title.clone()/>
        <Meta name="description" content=description.clone()/>
        <Link rel="canonical" href=canonical.clone()/>
        <Meta property="og:type" content="website"/>
        <Meta property="og:site_name" content=site.clone()/>
        <Meta property="og:title" content=title/>
        <Meta property="og:description" content=description/>
        <Meta property="og:url" content=canonical/>
        <script type="application/ld+json" inner_html=crumbs_ld></script>

        // `data-listing-href` (explorer::HREF_ATTR) is how the island learns about same-route
        // navigations: the islands router diffs this attribute but never the island itself.
        <div class="listing container" data-listing-href=href>
            <Breadcrumbs items=vec![(tr(lang, "Home", "الرئيسية").to_owned(), Some("/".to_owned())), (name.clone(), None)]/>
            <header class="cat-head">
                <span class="cat-head__icon" aria-hidden="true">
                    <Icon name=cat.icon.clone() size=26/>
                </span>
                <div class="cat-head__text">
                    <h1 class="cat-head__title">{name}</h1>
                    <p class="cat-head__blurb">{cat.blurb.pick(ar).to_owned()}</p>
                </div>
                <p class="cat-head__count">{format::items(cat.count, lang, num)}</p>
            </header>
            <ListingResults data site lang/>
        </div>
    })
}

/// The explorer, once its first page is in. In order (no JS needed) it is simply part of the
/// page; the fallback only shows if the route is ever switched to out-of-order streaming.
#[component]
pub(crate) fn ListingResults(
    data: Resource<Result<crate::api::types::ListingPage, ServerFnError>>,
    site: String,
    lang: Lang,
) -> impl IntoView {
    view! {
        <Suspense fallback=|| view! { <ListingSkeleton/> }>
            <ErrorBoundary fallback=move |_| view! { <ListingFailed lang/> }>
                {move || {
                    let site = site.clone();
                    Suspend::new(async move {
                        data.await.map(|page| view! { <CatalogExplorer initial=page site_name=site/> })
                    })
                }}
            </ErrorBoundary>
        </Suspense>
    }
}

/// The explorer's frame with pulsing cards, the same size as what replaces it.
#[component]
fn ListingSkeleton() -> impl IntoView {
    view! {
        <div class="lx lx--skeleton" aria-hidden="true">
            <div class="lx-bar">
                <span class="skeleton lx-skel-btn"></span>
                <span class="skeleton lx-skel-total"></span>
                <span class="skeleton lx-skel-btn"></span>
            </div>
            <div class="lx-body">
                <div class="lx-filters">
                    <div class="lx-form">
                        <span class="skeleton lx-skel-panel"></span>
                    </div>
                </div>
                <div class="lx-results">
                    <ProductGridSkeleton count=8 class="lx-grid"/>
                </div>
            </div>
        </div>
    }
}

/// The first page of results failed to load (`?fail=listing` demonstrates it). The retry link is
/// the same listing without the `fail` parameter.
#[component]
fn ListingFailed(lang: Lang) -> impl IntoView {
    let rq = req();
    let retry = rq
        .path_and_query
        .split_once('?')
        .map(|(path, qs)| {
            let kept: Vec<&str> = qs
                .split('&')
                .filter(|kv| !kv.starts_with("fail="))
                .collect();
            if kept.is_empty() {
                path.to_owned()
            } else {
                format!("{path}?{}", kept.join("&"))
            }
        })
        .unwrap_or_else(|| rq.path_and_query.clone());
    view! {
        <div class="lx-notice lx-notice--block" role="alert">
            <Icon name="triangle-alert" size=20 class="lx-notice-icon"/>
            <p class="lx-notice-text">
                <strong>{tr(lang, "We couldn't load these products.", "تعذّر تحميل المنتجات.")}</strong>
                " "
                <span>{tr(lang, "The rest of the shop is working normally.", "باقي المتجر يعمل بشكل طبيعي.")}</span>
            </p>
            <a class="btn btn--secondary btn--md" href=retry data-press="">
                <span class="btn__label">{tr(lang, "Try again", "حاول مرة أخرى")}</span>
            </a>
        </div>
    }
}

/// `/c/<unknown>`: a real 404, still useful — every aisle we do have.
#[component]
fn CategoryNotFound() -> impl IntoView {
    let rq = req();
    let lang = rq.prefs.lang;
    let ar = lang.is_rtl();
    #[cfg(feature = "ssr")]
    let cats: Vec<NavCategory> = {
        if let Some(res) = use_context::<leptos_axum::ResponseOptions>() {
            res.set_status(http::StatusCode::NOT_FOUND);
        }
        crate::server::cache::public(&[]);
        crate::server::current_catalog().nav().to_vec()
    };
    #[cfg(not(feature = "ssr"))]
    let cats: Vec<NavCategory> = Vec::new();
    let site = rq.tenant.name.pick(ar).to_owned();
    view! {
        <Title text=format!("{} · {site}", tr(lang, "Aisle not found", "القسم مش موجود"))/>
        <Meta name="robots" content="noindex"/>
        <div class="listing container listing--missing">
            <EmptyState
                art="search"
                title=tr(lang, "We don't have that aisle", "القسم ده مش عندنا")
                body=tr(lang, "It may have moved or been renamed. Here's everything we stock:", "ممكن يكون اتنقل أو اسمه اتغيّر. دي كل الأقسام اللي عندنا:")
            />
            <ul class="cat-pills" role="list">
                {cats
                    .into_iter()
                    .map(|c| view! {
                        <li>
                            <a class="cat-pill" href=format!("/c/{}", c.slug)>
                                <Icon name=c.icon.clone() size=18/>
                                <span>{c.name.pick(ar).to_owned()}</span>
                            </a>
                        </li>
                    })
                    .collect_view()}
            </ul>
        </div>
    }
}
