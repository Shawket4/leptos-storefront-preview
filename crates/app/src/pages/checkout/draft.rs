//! The checkout draft, server-side: what the shopper has entered so far, keyed by tenant + cart
//! cookie (the same key the cart uses), so a no-JS checkout survives every round-trip and a
//! refresh. Plus a little per-order metadata the domain `Order` doesn't carry.

use super::rules::AddressInput;
use domain::PaymentMethod;
use moka::sync::Cache;
use std::sync::LazyLock;
use std::time::Duration;

#[derive(Clone, Debug, Default)]
pub struct Draft {
    /// The last valid address (trimmed, phone normalised).
    pub address: Option<AddressInput>,
    /// The last *invalid* submission, to refill the form after a no-JS error round-trip (the
    /// errors themselves travel in the redirect URL; personal data never does).
    pub attempt: Option<AddressInput>,
    pub slot: Option<String>,
    /// The window picked on a submit that failed (kept so the page shows the same choice).
    pub attempt_slot: Option<String>,
    pub payment: Option<PaymentMethod>,
    /// The order this cart already became (a second "place order" — double click, back button,
    /// no-JS resubmit — lands on it instead of failing).
    pub placed: Option<String>,
}

/// Per-order facts kept beside the domain `Order`: who placed it (only that visitor sees the
/// full address on the tracking page), the delivery notes, and the city/area ids.
#[derive(Clone, Debug)]
pub struct PlacedMeta {
    pub owner: String,
    pub notes: String,
    pub city: String,
    pub area: String,
}

static DRAFTS: LazyLock<Cache<String, Draft>> = LazyLock::new(|| {
    Cache::builder()
        .max_capacity(100_000)
        .time_to_idle(Duration::from_secs(24 * 3600))
        .build()
});

static PLACED: LazyLock<Cache<String, PlacedMeta>> = LazyLock::new(|| {
    Cache::builder()
        .max_capacity(50_000)
        .time_to_live(Duration::from_secs(7 * 24 * 3600))
        .build()
});

pub fn key(tenant: &str, cart_id: &str) -> String {
    format!("{tenant}:{cart_id}")
}

pub fn get(key: &str) -> Draft {
    DRAFTS.get(key).unwrap_or_default()
}

pub fn update(key: &str, f: impl FnOnce(&mut Draft)) -> Draft {
    let mut d = get(key);
    f(&mut d);
    DRAFTS.insert(key.to_owned(), d.clone());
    d
}

pub fn remember_order(id: &str, meta: PlacedMeta) {
    PLACED.insert(id.to_owned(), meta);
}

pub fn order_meta(id: &str) -> Option<PlacedMeta> {
    PLACED.get(&id.to_ascii_uppercase())
}

/// Orders placed from this cart cookie (newest last as stored; callers sort by time).
pub fn orders_of(owner: &str) -> Vec<String> {
    PLACED
        .iter()
        .filter(|(_, m)| m.owner == owner)
        .map(|(id, _)| (*id).clone())
        .collect()
}
