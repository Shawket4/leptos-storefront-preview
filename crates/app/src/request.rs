//! Everything the server learns from the incoming request before rendering: which tenant, which
//! origin to build absolute URLs from, and the visitor's display preferences.

use crate::prefs::Prefs;
use crate::tenant::{Tenant, TenantVia};
use leptos::prelude::*;

/// Per-request facts, provided as context by [`init`] (server only). Plain data, so components
/// that read it still compile into the WASM bundle.
#[derive(Clone, Debug)]
pub struct RequestCtx {
    pub tenant: Tenant,
    pub via: TenantVia,
    /// The `Host` header as received, e.g. `nile.100-123-21-13.sslip.io:3000`.
    pub host: String,
    /// `scheme://host`, for canonical URLs, OpenGraph and JSON-LD.
    pub origin: String,
    /// Path and query of this request, e.g. `/c/coffee?sort=price`.
    pub path_and_query: String,
    pub prefs: Prefs,
}

impl RequestCtx {
    pub fn absolute(&self, path: &str) -> String {
        format!("{}{}", self.origin, path)
    }
}

/// The current request. Panics outside a server render; only call from server components.
pub fn req() -> RequestCtx {
    expect_context::<RequestCtx>()
}

/// Parse a `Cookie` header into `(name, value)` pairs.
pub fn parse_cookies(header: &str) -> impl Iterator<Item = (&str, &str)> {
    header.split(';').filter_map(|kv| {
        let (k, v) = kv.split_once('=')?;
        Some((k.trim(), v.trim()))
    })
}

#[cfg(feature = "ssr")]
pub use server::init;

#[cfg(feature = "ssr")]
mod server {
    use super::*;
    use crate::prefs::{COOKIE_LANG, COOKIE_NUMERALS, COOKIE_THEME, Lang, Numerals, Theme, Ui};
    use crate::tenant::{COOKIE_TENANT, DEFAULT_TENANT, TENANT_IDS, tenant_from_host};
    use http::request::Parts;

    /// Resolve tenant + preferences for this request and provide [`RequestCtx`] and [`Ui`].
    /// Called once at the top of the shell, before anything renders.
    pub fn init() -> RequestCtx {
        if let Some(rq) = use_context::<RequestCtx>() {
            return rq;
        }
        let state = crate::server::state();
        let parts = use_context::<Parts>();

        let header = |name: http::header::HeaderName| {
            parts
                .as_ref()
                .and_then(|p| p.headers.get(&name))
                .and_then(|v| v.to_str().ok())
                .map(str::to_owned)
        };
        let host = header(http::header::HOST)
            .or_else(|| parts.as_ref().and_then(|p| p.uri.authority().map(|a| a.to_string())))
            .unwrap_or_else(|| "localhost:3000".into());
        let scheme = parts
            .as_ref()
            .and_then(|p| p.headers.get("x-forwarded-proto"))
            .and_then(|v| v.to_str().ok())
            .filter(|s| *s == "https" || *s == "http")
            .unwrap_or("http")
            .to_owned();
        let cookie_header = header(http::header::COOKIE).unwrap_or_default();
        let cookie = |name: &str| {
            parse_cookies(&cookie_header)
                .find(|(k, _)| *k == name)
                .map(|(_, v)| v.to_owned())
        };

        let (tenant_id, via) = match tenant_from_host(&host) {
            Some(id) => (id, TenantVia::Subdomain),
            None => match cookie(COOKIE_TENANT)
                .and_then(|c| TENANT_IDS.iter().copied().find(|id| *id == c))
            {
                Some(id) => (id, TenantVia::Cookie),
                None => (DEFAULT_TENANT, TenantVia::Default),
            },
        };
        let tenant = state.tenant(tenant_id).clone();

        let lang = cookie(COOKIE_LANG)
            .and_then(|v| Lang::parse(&v))
            .unwrap_or(tenant.default_lang);
        let numerals = cookie(COOKIE_NUMERALS)
            .and_then(|v| Numerals::parse(&v))
            .unwrap_or_default();
        let theme = cookie(COOKIE_THEME)
            .and_then(|v| Theme::parse(&v))
            .unwrap_or_default();
        let prefs = Prefs { lang, numerals, theme };

        let path_and_query = parts
            .as_ref()
            .and_then(|p| p.uri.path_and_query().map(|pq| pq.as_str().to_owned()))
            .unwrap_or_else(|| "/".into());

        let rq = RequestCtx {
            tenant,
            via,
            origin: format!("{scheme}://{host}"),
            host,
            path_and_query,
            prefs,
        };
        provide_context(rq.clone());
        Ui::provide(prefs);
        rq
    }
}
