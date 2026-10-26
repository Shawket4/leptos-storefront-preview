//! Shared server functions (catalogue reads) and the data types they exchange.
//!
//! Every read: resolves the tenant from the request (`request::init`), waits the widget's
//! simulated backend latency (`server::sim`), honours `?fail=<widget>` for error-boundary demos,
//! and goes through the moka read cache keyed by tenant + catalogue generation + query.

pub mod cart;
pub mod types;

use crate::components::product_card::CardData;
use domain::ProductId;
use leptos::prelude::*;
use types::*;

#[cfg(feature = "ssr")]
use crate::server::{sim::Widget, state};

#[cfg(feature = "ssr")]
async fn read<T, F>(widget: Widget, kind: &str, key: String, f: F) -> Result<std::sync::Arc<T>, ServerFnError>
where
    T: Send + Sync + 'static,
    F: FnOnce(&crate::server::TenantCatalog, &crate::server::Media) -> T,
{
    // Everything from the request's context first: after the `.await` the client may have gone,
    // and its context with it.
    let rq = crate::request::init();
    let st = state();
    crate::server::sim::check_fail(widget)?;
    crate::server::sim::delay(widget).await;
    let catalog = st.catalog(&rq.tenant.id);
    let value = st
        .reads()
        .get(&rq.tenant.id, kind, &key, || async { f(&catalog, st.media()) })
        .await;
    Ok(value)
}

/// Categories in display order, with product counts (header nav, category rail).
#[server]
pub async fn nav_categories() -> Result<Vec<NavCategory>, ServerFnError> {
    Ok((*read(Widget::Nav, "nav", String::new(), |c, _| c.nav().to_vec()).await?).clone())
}

/// A home-page merchandising rail.
#[server]
pub async fn home_rail(rail: Rail, limit: u32) -> Result<Vec<CardData>, ServerFnError> {
    let widget = match rail {
        Rail::Featured => Widget::Featured,
        Rail::Deals => Widget::Deals,
        Rail::New => Widget::New,
        Rail::Bestsellers => Widget::Bestsellers,
        Rail::Express => Widget::Express,
    };
    let limit = limit.clamp(1, 48) as usize;
    let v = read(widget, "rail", format!("{rail:?}:{limit}"), move |c, _| c.rail(rail, limit)).await?;
    Ok((*v).clone())
}

/// One page of a category listing / filtered search, per `ListingQuery`.
#[server]
pub async fn list_products(query: ListingQuery) -> Result<ListingPage, ServerFnError> {
    let key = format!("{}?{}", query.category.as_deref().unwrap_or("*"), query.to_query());
    let v = read(Widget::Listing, "listing", key, move |c, _| c.listing(&query)).await?;
    Ok((*v).clone())
}

/// The product page's data, `None` if the slug doesn't exist for this tenant.
#[server]
pub async fn product_detail(slug: String) -> Result<Option<ProductView>, ServerFnError> {
    let key = slug.clone();
    let v = read(Widget::Product, "product", key, move |c, m| c.product_view(&slug, m)).await?;
    Ok((*v).clone())
}

/// "You may also like" for a product.
#[server]
pub async fn related_products(id: ProductId, limit: u32) -> Result<Vec<CardData>, ServerFnError> {
    let limit = limit.clamp(1, 24) as usize;
    let v = read(Widget::Related, "related", format!("{}:{limit}", id.0), move |c, _| c.related(id, limit)).await?;
    Ok((*v).clone())
}

/// Instant search: top `limit` hits with highlight segments, plus matching categories.
#[server]
pub async fn search_products(q: String, limit: u32) -> Result<SearchResults, ServerFnError> {
    let q = q.trim().chars().take(80).collect::<String>();
    let limit = limit.clamp(1, 48) as usize;
    let key = format!("{limit}:{}", crate::server::search::fold(&q));
    let v = read(Widget::Search, "search", key, move |c, _| c.search(&q, limit)).await?;
    Ok((*v).clone())
}

/// Read-cache statistics for this process (showcase). `GET /api/cache/stats`.
#[server(endpoint = "cache/stats", input = leptos::server_fn::codec::GetUrl)]
pub async fn cache_stats() -> Result<CacheInfo, ServerFnError> {
    let rq = crate::request::init();
    crate::server::cache::private();
    let st = state();
    let s = st.reads().stats();
    Ok(CacheInfo {
        entries: s.entries,
        hits: s.hits,
        misses: s.misses,
        generation: st.reads().generation(&rq.tenant.id),
        base_latency_ms: crate::server::sim::base_ms(),
    })
}

/// Invalidate every cached read of the current tenant by bumping its generation (what a catalogue
/// edit does; in production you would also purge the CDN by `Cache-Tag: t{tenant}`).
#[server(endpoint = "cache/bump")]
pub async fn cache_bump() -> Result<CacheInfo, ServerFnError> {
    let rq = crate::request::init();
    state().reads().bump(&rq.tenant.id);
    cache_stats().await
}
