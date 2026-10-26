//! Tenants and how a request picks one.
//!
//! Mirrors Madar's per-shop subdomains: the tenant is the first label of the hostname
//! (`MadarDashboard/src/features/public-shell/use-brand.ts::hostSlug`, `MadarRust/src/tenant_shell.rs`).
//! It is resolved on the server before rendering, so the logo, name and default language are in
//! the very first byte of HTML (no flash).
//!
//! Dev/phone hosts that can't carry a subdomain (a bare tailnet IP, a MagicDNS name) fall back to a
//! `tenant` cookie and then to the default tenant. Production would only ever use the Host.

use crate::prefs::Lang;
use domain::L10n;
use serde::{Deserialize, Serialize};

pub const COOKIE_TENANT: &str = "tenant";

/// Static configuration of one tenant (in production: a row in `organizations`, see Madar's
/// `PublicBrand { name, slug, logo_url, … }`). Only what really differs per shop: name, tagline,
/// default language, logo, catalogue and delivery terms. Colours are platform-wide.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tenant {
    /// Stable id, also the subdomain label and the cache-tag prefix (`t{id}`).
    pub id: String,
    pub name: L10n,
    pub tagline: L10n,
    pub default_lang: Lang,
    /// Logo asset key under /media (see data/media). Colours are global (style/tokens.css), not
    /// per tenant — owner's call.
    pub logo: String,
    /// Order reference prefix (`NIL-48213`), like Madar's branch `code` in `delivery_ref`.
    pub order_prefix: String,
    /// Flat delivery fee and the subtotal from which delivery is free (Madar keeps the fee
    /// outside the tax base: `delivery_fee` on `delivery_orders`).
    pub delivery_fee: domain::Piastres,
    pub free_delivery_over: domain::Piastres,
    /// Minutes for an express slot, if the tenant offers one.
    pub express_minutes: Option<u32>,
}

/// How the current request chose its tenant.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TenantVia {
    /// `nile.localhost:3000`, `souq.100-123-21-13.sslip.io:3000`, … (the production path).
    Subdomain,
    /// A `tenant` cookie set by `/__tenant/{id}` on a host with no tenant label.
    Cookie,
    /// Nothing matched; the default tenant.
    Default,
}

/// Every demo tenant id, in display order.
pub const TENANT_IDS: [&str; 2] = ["nile", "souq"];
pub const DEFAULT_TENANT: &str = "nile";

/// The tenant label in a `Host` header value, if its first label is a known tenant.
///
/// Like Madar's `hostSlug`, a lone label (`localhost`) or an IP never counts; unlike it, two labels
/// are enough so `nile.localhost` works in dev.
pub fn tenant_from_host(host: &str) -> Option<&'static str> {
    let hostname = strip_port(host.trim());
    let mut labels = hostname.split('.');
    let first = labels.next()?.to_ascii_lowercase();
    labels.next()?;
    TENANT_IDS.iter().copied().find(|id| *id == first)
}

/// `host[:port]` → `host`, handling bracketed IPv6.
pub fn strip_port(host: &str) -> &str {
    if let Some(rest) = host.strip_prefix('[') {
        return rest.split(']').next().unwrap_or(rest);
    }
    host.rsplit_once(':').map(|(h, _)| h).unwrap_or(host)
}

/// The same host with its tenant label swapped (`nile.x.sslip.io:3000` → `souq.x.sslip.io:3000`).
pub fn swap_tenant_label(host: &str, to: &str) -> Option<String> {
    tenant_from_host(host)?;
    let (_, rest) = host.split_once('.')?;
    Some(format!("{to}.{rest}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_resolution() {
        assert_eq!(tenant_from_host("nile.localhost:3000"), Some("nile"));
        assert_eq!(tenant_from_host("SOUQ.100-123-21-13.sslip.io:3000"), Some("souq"));
        assert_eq!(tenant_from_host("localhost:3000"), None);
        assert_eq!(tenant_from_host("100.123.21.13:3000"), None);
        assert_eq!(tenant_from_host("[::1]:3000"), None);
        assert_eq!(tenant_from_host("apex-optiplex-3060.tailedc2b4.ts.net"), None);
        assert_eq!(
            swap_tenant_label("nile.100-123-21-13.sslip.io:3000", "souq").as_deref(),
            Some("souq.100-123-21-13.sslip.io:3000")
        );
    }
}
