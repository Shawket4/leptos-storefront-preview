//! Category listing: `/c/:slug?min=&max=&brand=&stock=&deal=&sort=&page=`.
//!
//! Everything is decided from the URL. Synchronously, before the first byte: the category (an
//! unknown slug is a 404), the caching headers (`Cache-Tag: t{tenant}, t{tenant}-cat{id}`), title,
//! description, canonical and breadcrumb JSON-LD. Then the page renders in order (complete without
//! JS) with the results of `api::list_products` (`view::Listing`). Filtering, sorting and paging
//! are new URLs: the server renders them; the browser code only makes the trip quick.

pub(crate) mod listing;
pub(crate) mod view;

use crate::api::types::ListingPage;
use crate::api::{
    self,
    types::{ListingQuery, NavCategory},
};
use crate::components::button::{Button, ButtonKind};
use crate::components::{breadcrumbs::Breadcrumbs, empty_state::EmptyState, icons::Icon};
use crate::format;
use crate::pages::search::suggest::{CatLink, Suggestions};
use crate::pages::search::{HeaderSearch, SearchBox};
use crate::prefs::{Lang, tr};
use crate::request::req;
use leptos::either::Either;
use leptos::prelude::*;
use leptos_meta::{Link, Meta, Title};
use leptos_router::hooks::{use_location, use_params_map};

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

        <div class="listing container">
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
            <ListingResults data aisles=aisles(Some(&slug))/>
        </div>
    })
}

/// The busiest aisles (except `except`), offered by empty states and the 404.
pub(crate) fn aisles(except: Option<&str>) -> Vec<CatLink> {
    #[cfg(feature = "ssr")]
    {
        crate::pages::search::suggest::popular_categories(6, except)
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = except;
        Vec::new()
    }
}

/// A search that found nothing gets its suggestions with the first page (so no JS is needed).
fn first_suggestions(page: &ListingPage) -> Option<Suggestions> {
    #[cfg(feature = "ssr")]
    {
        page.query
            .q
            .as_deref()
            .filter(|_| page.items.is_empty() && page.query.category.is_none())
            .map(crate::pages::search::suggest::suggestions_now)
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = page;
        None
    }
}

/// The listing, once its first page is in (the route renders in order, so the fallback is never
/// sent; a failed read shows `ListingFailed`).
#[component]
pub(crate) fn ListingResults(
    data: Resource<Result<ListingPage, ServerFnError>>,
    /// Aisles the empty states offer.
    aisles: Vec<CatLink>,
) -> impl IntoView {
    let rq = req();
    let (lang, num) = (rq.prefs.lang, rq.prefs.numerals);
    view! {
        <Suspense fallback=|| ()>
            <ErrorBoundary fallback=move |_| view! { <ListingFailed lang/> }>
                {move || {
                    let aisles = aisles.clone();
                    Suspend::new(async move {
                        data.await.map(|page| {
                            let suggest = first_suggestions(&page);
                            let extras = view::Extras { aisles, suggest };
                            view! { <view::Listing page extras lang num/> }
                        })
                    })
                }}
            </ErrorBoundary>
        </Suspense>
    }
}

/// The first page of results failed to load (`?fail=listing` demonstrates it). Retry is a link to
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
            <Button kind=ButtonKind::Secondary href=retry icon="rotate-ccw">
                {tr(lang, "Try again", "حاول مرة أخرى")}
            </Button>
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
                art="not-found"
                title=tr(lang, "We don't have that aisle", "القسم ده مش عندنا")
                body=tr(lang, "It may have moved or been renamed. Here's everything we stock:", "ممكن يكون اتنقل أو اسمه اتغيّر. دي كل الأقسام اللي عندنا:")
                page=true
            />
            <div class="listing-missing__search">
                <HeaderSearch kind=SearchBox::Page/>
            </div>
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
