//! The checkout draft, server-side: what the shopper has entered so far, keyed by tenant + this
//! browser's `visitor` cookie (an opaque HttpOnly id — never the cart, which is the browser's own
//! `cart` cookie), so a no-JS checkout survives every round-trip and a refresh. Plus a little
//! per-order metadata the domain `Order` doesn't carry (who placed it: the same visitor id).

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

pub fn key(tenant: &str, visitor: &str) -> String {
    format!("{tenant}:{visitor}")
}

/// The HttpOnly cookie naming this browser's checkout draft and the orders it placed.
pub const VISITOR_COOKIE: &str = "visitor";

/// This browser's visitor id, if it has one.
pub fn visitor() -> Option<String> {
    let parts = leptos::prelude::use_context::<http::request::Parts>()?;
    parts
        .headers
        .get_all(http::header::COOKIE)
        .iter()
        .filter_map(|v| v.to_str().ok())
        .flat_map(crate::request::parse_cookies)
        .find(|(k, _)| *k == VISITOR_COOKIE)
        .map(|(_, v)| v.to_owned())
        .filter(|v| v.len() == 32 && v.bytes().all(|b| b.is_ascii_hexdigit()))
}

/// The visitor id, created (and set as a cookie on this response) if the browser has none yet.
pub fn ensure_visitor() -> String {
    if let Some(v) = visitor() {
        return v;
    }
    let id = uuid::Uuid::new_v4().simple().to_string();
    if let Some(res) = leptos::prelude::use_context::<leptos_axum::ResponseOptions>() {
        let c = format!("{VISITOR_COOKIE}={id}; Path=/; Max-Age=31536000; HttpOnly; SameSite=Lax");
        if let Ok(v) = http::HeaderValue::from_str(&c) {
            res.append_header(http::header::SET_COOKIE, v);
        }
    }
    id
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

/// Orders placed by this visitor (in no order; callers sort by time).
pub fn orders_of(owner: &str) -> Vec<String> {
    PLACED
        .iter()
        .filter(|(_, m)| m.owner == owner)
        .map(|(id, _)| (*id).clone())
        .collect()
}
