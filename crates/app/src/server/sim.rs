//! Simulated upstream latency (so streaming and `<Suspense>` are visible) and fault injection
//! (so per-section error boundaries can be demonstrated).
//!
//! - `SIM_LATENCY_MS` (env, default 350) is the base round-trip to a pretend catalogue backend.
//!   Each widget waits `base × factor`, so home sections resolve at different times and stream
//!   in out of order.
//! - A `sim` cookie overrides the base per visitor (`/showcase` sets it: 0 / 350 / 1200).
//! - `?fail=<widget>` on the page URL (or a `fail` cookie) makes that widget's read fail.

use leptos::prelude::*;
use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Widget {
    Nav,
    Hero,
    Featured,
    Deals,
    New,
    Bestsellers,
    Express,
    Listing,
    Product,
    Related,
    Search,
    Cart,
    Checkout,
    Order,
}

impl Widget {
    pub fn code(self) -> &'static str {
        match self {
            Widget::Nav => "nav",
            Widget::Hero => "hero",
            Widget::Featured => "featured",
            Widget::Deals => "deals",
            Widget::New => "new",
            Widget::Bestsellers => "bestsellers",
            Widget::Express => "express",
            Widget::Listing => "listing",
            Widget::Product => "product",
            Widget::Related => "related",
            Widget::Search => "search",
            Widget::Cart => "cart",
            Widget::Checkout => "checkout",
            Widget::Order => "order",
        }
    }

    fn factor(self) -> f64 {
        match self {
            Widget::Nav | Widget::Hero => 0.0,
            Widget::Featured => 1.0,
            Widget::Deals => 2.6,
            Widget::New => 1.6,
            Widget::Bestsellers => 2.0,
            Widget::Express => 1.3,
            Widget::Listing => 0.8,
            Widget::Product => 0.6,
            Widget::Related => 1.8,
            Widget::Search => 0.25,
            Widget::Cart | Widget::Checkout | Widget::Order => 0.3,
        }
    }
}

fn cookie(name: &str) -> Option<String> {
    let parts = use_context::<http::request::Parts>()?;
    let header = parts.headers.get(http::header::COOKIE)?.to_str().ok()?;
    crate::request::parse_cookies(header).find(|(k, _)| *k == name).map(|(_, v)| v.to_owned())
}

fn query_param(name: &str) -> Option<String> {
    let parts = use_context::<http::request::Parts>()?;
    let q = parts.uri.query()?;
    q.split('&').find_map(|kv| {
        let (k, v) = kv.split_once('=').unwrap_or((kv, ""));
        (k == name).then(|| v.to_owned())
    })
}

/// The visitor's base latency in ms.
pub fn base_ms() -> u64 {
    cookie("sim")
        .and_then(|v| v.parse().ok())
        .or_else(|| std::env::var("SIM_LATENCY_MS").ok().and_then(|v| v.parse().ok()))
        .unwrap_or(350)
        .min(10_000)
}

/// Wait as long as this widget's pretend backend call takes.
pub async fn delay(w: Widget) {
    let ms = (base_ms() as f64 * w.factor()) as u64;
    if ms > 0 {
        tokio::time::sleep(Duration::from_millis(ms)).await;
    }
}

/// Fail this widget if the page asked for it (`?fail=deals` / `fail` cookie). Comma lists work.
pub fn check_fail(w: Widget) -> Result<(), ServerFnError> {
    let wanted = query_param("fail").or_else(|| cookie("fail")).unwrap_or_default();
    if wanted.split([',', '+', ' ']).any(|x| x == w.code() || x == "all") {
        return Err(ServerFnError::ServerError(format!(
            "simulated failure in '{}' (remove ?fail= to recover)",
            w.code()
        )));
    }
    Ok(())
}
