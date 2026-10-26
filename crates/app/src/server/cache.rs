//! Two kinds of caching.
//!
//! **HTTP / CDN** — what a page tells Cloudflare. Catalogue pages are the same for every visitor
//! *with the same display preferences* (language, numerals, theme live in cookies), so:
//! - browsers get `Cache-Control: no-cache` (always revalidate — a stale page after a language
//!   switch is a bug), plus `Vary: Cookie` for any cache that honours it;
//! - the edge gets `CDN-Cache-Control: public, s-maxage=300, stale-while-revalidate=60` and
//!   `Cache-Tag`s for targeted purges: `t{tenant}`, `t{tenant}-p{product}`, `t{tenant}-cat{category}`.
//!   Cloudflare ignores `Vary: Cookie`, so its cache key must include the `lang`, `num` and `theme`
//!   cookies (a Cache Rule with a custom key) — the Host already separates tenants.
//! Personalised responses (cart, checkout, orders) are `private, no-store`.
//! Headers must be set before the first streamed chunk, i.e. synchronously at the top of a page.
//!
//! **Read cache** — a moka cache in front of catalogue reads, keyed by tenant + catalogue
//! generation + query (the same shape as Madar's `MenuCache` key `"{org}:{version}:{variant}"`).
//! Bumping a tenant's generation invalidates all its entries without a scan.

use http::{HeaderName, HeaderValue, header::CACHE_CONTROL};
use leptos::prelude::*;
use moka::future::Cache;
use std::{
    any::Any,
    future::Future,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    time::Duration,
};

pub const CACHE_TAG: HeaderName = HeaderName::from_static("cache-tag");
pub const CDN_CACHE_CONTROL: HeaderName = HeaderName::from_static("cdn-cache-control");

pub fn tag_tenant(tenant: &str) -> String {
    format!("t{tenant}")
}
pub fn tag_product(tenant: &str, id: u32) -> String {
    format!("t{tenant}-p{id}")
}
pub fn tag_category(tenant: &str, id: u32) -> String {
    format!("t{tenant}-cat{id}")
}

fn response() -> Option<leptos_axum::ResponseOptions> {
    use_context::<leptos_axum::ResponseOptions>()
}

/// Mark this response cacheable at the edge, with the tenant tag plus `tags`.
///
/// Cookie-resolved tenants (bare-IP dev hosts) can't be cached publicly — the URL alone doesn't
/// determine the tenant — so those responses fall back to `private`.
pub fn public(tags: &[String]) {
    let Some(res) = response() else { return };
    let rq = crate::request::init();
    // Browsers always revalidate: preferences are cookies, so the URL alone doesn't pin the HTML.
    res.insert_header(CACHE_CONTROL, HeaderValue::from_static("no-cache"));
    res.insert_header(http::header::VARY, HeaderValue::from_static("Cookie, Accept-Encoding"));
    if rq.via != crate::tenant::TenantVia::Cookie {
        // Only the edge keeps a shared copy (keyed by Host + the lang/num/theme cookies).
        res.insert_header(
            CDN_CACHE_CONTROL,
            HeaderValue::from_static("public, s-maxage=300, stale-while-revalidate=60"),
        );
    }
    let mut all = vec![tag_tenant(&rq.tenant.id)];
    all.extend(tags.iter().cloned());
    if let Ok(v) = HeaderValue::from_str(&all.join(",")) {
        res.insert_header(CACHE_TAG, v);
    }
}

/// Personalised response: never cache anywhere.
pub fn private() {
    if let Some(res) = response() {
        res.insert_header(CACHE_CONTROL, HeaderValue::from_static("private, no-store"));
    }
}

type Value = Arc<dyn Any + Send + Sync>;

/// Read-through cache for computed catalogue views (listings, rails, search, product views).
pub struct ReadCache {
    cache: Cache<String, Value>,
    generations: moka::sync::Cache<String, u64>,
    hits: AtomicU64,
    misses: AtomicU64,
}

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub struct CacheStats {
    pub entries: u64,
    pub hits: u64,
    pub misses: u64,
}

impl ReadCache {
    pub fn new() -> Self {
        let ttl = std::env::var("CACHE_TTL_SECS").ok().and_then(|v| v.parse().ok()).unwrap_or(120);
        ReadCache {
            cache: Cache::builder()
                .max_capacity(20_000)
                .time_to_live(Duration::from_secs(ttl))
                .build(),
            generations: moka::sync::Cache::new(64),
            hits: AtomicU64::new(0),
            misses: AtomicU64::new(0),
        }
    }

    pub fn generation(&self, tenant: &str) -> u64 {
        self.generations.get(tenant).unwrap_or(0)
    }

    /// Invalidate every cached read of a tenant (e.g. after a catalogue edit).
    pub fn bump(&self, tenant: &str) {
        self.generations.insert(tenant.to_owned(), self.generation(tenant) + 1);
    }

    /// Get `kind:key` for `tenant`, computing it with `load` on a miss (concurrent misses for the
    /// same key share one computation). Misses pay `CACHE_MISS_MS` (default 0 = off) of pretend DB
    /// time, to make the cache's effect visible in demos.
    pub async fn get<T, F, Fut>(&self, tenant: &str, kind: &str, key: &str, load: F) -> Arc<T>
    where
        T: Send + Sync + 'static,
        F: FnOnce() -> Fut,
        Fut: Future<Output = T>,
    {
        let full = format!("{tenant}:{}:{kind}:{key}", self.generation(tenant));
        let mut missed = false;
        let value = self
            .cache
            .get_with(full, async {
                missed = true;
                let miss_ms: u64 = std::env::var("CACHE_MISS_MS").ok().and_then(|v| v.parse().ok()).unwrap_or(0);
                if miss_ms > 0 {
                    tokio::time::sleep(Duration::from_millis(miss_ms)).await;
                }
                Arc::new(load().await) as Value
            })
            .await;
        if missed {
            self.misses.fetch_add(1, Ordering::Relaxed);
        } else {
            self.hits.fetch_add(1, Ordering::Relaxed);
        }
        if let Some(res) = response() {
            res.insert_header(
                HeaderName::from_static("x-read-cache"),
                HeaderValue::from_static(if missed { "miss" } else { "hit" }),
            );
        }
        value.downcast::<T>().expect("read cache type matches its key")
    }

    pub fn stats(&self) -> CacheStats {
        CacheStats {
            entries: self.cache.entry_count(),
            hits: self.hits.load(Ordering::Relaxed),
            misses: self.misses.load(Ordering::Relaxed),
        }
    }
}

impl Default for ReadCache {
    fn default() -> Self {
        Self::new()
    }
}
