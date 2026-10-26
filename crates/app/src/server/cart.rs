//! The cart on the server. The browser owns it — the `cart` cookie (`domain::cart`, the same
//! format as `client/src/stores/cart.ts`) — so the server *reads* it to price it (the drawer's
//! quote, /cart, checkout) and *writes* it in two cases only: the no-JS cart forms
//! (`POST /cart/add|set|remove`, the axum handlers in [`forms`]) and placing an order (cleared).
//!
//! Every line is priced with Madar's rules (`pricing::unit_price` → `madar_catalog::price_line`),
//! the way Madar re-prices every order at intake instead of trusting the client.

use super::{TenantCatalog, media::Media, pricing};
use crate::api::types::{CartLineView, CartView};
use crate::tenant::Tenant;
use domain::cart::{self as dc, CartLine};
use domain::{Piastres, ProductId, VariantId};
use http::{HeaderMap, HeaderValue, header};

// ── The cookie ─────────────────────────────────────────────────────────────────────────────────

/// A cookie's value by name, from every `Cookie` header of a request.
fn cookie<'a>(headers: &'a HeaderMap, name: &str) -> Option<&'a str> {
    headers
        .get_all(header::COOKIE)
        .iter()
        .filter_map(|v| v.to_str().ok())
        .flat_map(crate::request::parse_cookies)
        .find(|(k, _)| *k == name)
        .map(|(_, v)| v)
}

/// The cart in a request's `cart` cookie: percent-decoded, parsed and hardened (`domain::cart`).
pub fn from_headers(headers: &HeaderMap) -> Vec<CartLine> {
    cookie(headers, dc::COOKIE)
        .map(|v| dc::parse(&percent_decode(v)))
        .unwrap_or_default()
}

/// This request's cart (empty without the cookie, or outside a request).
pub fn from_request() -> Vec<CartLine> {
    leptos::prelude::use_context::<http::request::Parts>()
        .map(|p| from_headers(&p.headers))
        .unwrap_or_default()
}

/// `%7C` → `|` (the browser writes the cookie with `encodeURIComponent`). Bad escapes stay as-is.
pub fn percent_decode(s: &str) -> String {
    let b = s.as_bytes();
    let hex = |c: u8| (c as char).to_digit(16).map(|d| d as u8);
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'%' && i + 2 < b.len() {
            if let (Some(h), Some(l)) = (hex(b[i + 1]), hex(b[i + 2])) {
                out.push(h * 16 + l);
                i += 3;
                continue;
            }
        }
        out.push(b[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// `encodeURIComponent`, so a cookie the server writes reads the same in the browser.
fn uri_component(s: &str) -> String {
    let mut out = String::with_capacity(s.len() * 3);
    for b in s.bytes() {
        if b.is_ascii_alphanumeric() || b"-_.!~*'()".contains(&b) {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

const MAX_AGE: u32 = 60 * 60 * 24 * 30;

/// `Set-Cookie` for these lines, cleared when there are none. Readable by the browser code (it
/// owns the cart), so not HttpOnly; no `Secure`: the preview is served over plain http.
pub fn set_cookie(lines: &[CartLine]) -> HeaderValue {
    let value = dc::serialize(lines);
    let c = if value.is_empty() {
        format!("{}=; Path=/; Max-Age=0; SameSite=Lax", dc::COOKIE)
    } else {
        format!("{}={}; Path=/; Max-Age={MAX_AGE}; SameSite=Lax", dc::COOKIE, uri_component(&value))
    };
    HeaderValue::from_str(&c).expect("cookie header")
}

// ── Pricing ────────────────────────────────────────────────────────────────────────────────────

/// The cart priced now.
pub struct Priced {
    /// The lines whose product still exists, priced, in cart order.
    pub view: CartView,
    /// Lines whose product, variant or add-on is gone from the catalogue (they can't be ordered).
    pub missing: Vec<CartLine>,
}

impl Priced {
    /// Something can't be sent as it stands (checkout waits until it's fixed).
    pub fn blocked(&self) -> bool {
        !self.missing.is_empty() || self.view.lines.iter().any(|l| issue(l).is_some())
    }
}

/// What's wrong with a line now: stock moves after an item is added.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Issue {
    OutOfStock,
    /// Fewer left than the line asks for: at most this many.
    Capped(u32),
}

pub fn issue(l: &CartLineView) -> Option<Issue> {
    if l.max_qty == 0 {
        Some(Issue::OutOfStock)
    } else if l.qty > l.max_qty {
        Some(Issue::Capped(l.max_qty))
    } else {
        None
    }
}

pub fn price(lines: &[CartLine], tenant: &Tenant, catalog: &TenantCatalog, media: &Media) -> Priced {
    let mut out = Vec::new();
    let mut missing = Vec::new();
    let mut subtotal = Piastres::ZERO;
    let mut savings = Piastres::ZERO;
    for l in lines {
        let Some((p, v)) = catalog.variant(l.product, l.variant) else {
            missing.push(l.clone());
            continue;
        };
        // An add-on the product no longer offers can't be priced: the line is unavailable.
        let Ok(unit) = pricing::unit_price(p, v, &l.addons) else {
            missing.push(l.clone());
            continue;
        };
        let line_total = unit * l.qty;
        subtotal += line_total;
        let was = v.compare_at.filter(|c| *c > v.price);
        if let Some(was) = was {
            savings += (was - v.price) * l.qty;
        }
        let addons = p
            .modifier_groups
            .iter()
            .flat_map(|g| g.options.iter())
            .filter(|o| l.addons.contains(&o.id))
            .map(|o| o.name.clone())
            .collect();
        let img_key = v.image.and_then(|i| p.images.get(i)).or_else(|| p.images.first());
        out.push(CartLineView {
            key: l.key(),
            product: p.id,
            variant: v.id,
            slug: p.slug.clone(),
            name: p.name.clone(),
            variant_label: TenantCatalog::variant_label(p, v),
            addons,
            img: img_key.map(|k| media.img(k)).unwrap_or_else(|| crate::media::Img::missing("")),
            qty: l.qty,
            max_qty: v.stock.min(dc::MAX_QTY),
            unit_price: unit,
            // The struck price carries the same add-ons as the unit price beside it.
            compare_at: was.map(|c| c + (unit - v.price)),
            line_total,
        });
    }
    let count = out.iter().map(|l| l.qty).sum();
    let delivery_fee = if subtotal.0 == 0 || subtotal >= tenant.free_delivery_over {
        Piastres::ZERO
    } else {
        tenant.delivery_fee
    };
    let view = CartView {
        lines: out,
        count,
        subtotal,
        savings,
        delivery_fee,
        free_delivery_over: tenant.free_delivery_over,
        // Madar keeps the delivery fee outside the tax base.
        tax_included: pricing::vat_included(subtotal),
        total: subtotal + delivery_fee,
    };
    Priced { view, missing }
}

/// This request's cart, priced (tenant from the request).
pub fn priced_for_request() -> (Vec<CartLine>, Priced) {
    let rq = crate::request::init();
    let st = super::state();
    let tenant = st.tenant(&rq.tenant.id).clone();
    let lines = from_request();
    let priced = price(&lines, &tenant, &st.catalog(&tenant.id), st.media());
    (lines, priced)
}

// ── Changes (no-JS forms only: with JS the browser changes its own cookie) ─────────────────────

/// Why a cart form couldn't do what it asked: `?cart_err=<code>` (AGENTS.md, "The cart").
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormError {
    OutOfStock,
    MaxQty,
    Full,
    AddonRequired,
    BadAddon,
    UnknownProduct,
    BadRequest,
}

impl FormError {
    pub fn code(self) -> &'static str {
        match self {
            FormError::OutOfStock => "out_of_stock",
            FormError::MaxQty => "max_qty",
            FormError::Full => "full",
            FormError::AddonRequired => "addon_required",
            FormError::BadAddon => "bad_addon",
            FormError::UnknownProduct => "unknown_product",
            FormError::BadRequest => "bad_request",
        }
    }
}

/// Picked add-ons against the product's Madar-style modifier groups.
pub fn check_addons(p: &domain::Product, addons: &[String]) -> Result<(), FormError> {
    for a in addons {
        if !p.modifier_groups.iter().any(|g| g.options.iter().any(|o| &o.id == a)) {
            return Err(FormError::BadAddon);
        }
    }
    for g in &p.modifier_groups {
        let picked = g.options.iter().filter(|o| addons.contains(&o.id)).count() as u32;
        let min = if g.required { g.min.max(1) } else { g.min };
        let max = match g.selection {
            domain::SelectionType::Single => 1,
            domain::SelectionType::Multi => g.max.unwrap_or(u32::MAX),
        };
        if picked < min {
            return Err(FormError::AddonRequired);
        }
        if picked > max {
            return Err(FormError::BadAddon);
        }
    }
    Ok(())
}

/// Add `qty` (≥ 1) of a variant, merging with an identical line, capped at stock and `MAX_QTY`.
/// Returns how many were added.
pub fn add(
    lines: &mut Vec<CartLine>,
    catalog: &TenantCatalog,
    product: ProductId,
    variant: VariantId,
    qty: u32,
    mut addons: Vec<String>,
) -> Result<u32, FormError> {
    let (p, v) = catalog.variant(product, variant).ok_or(FormError::UnknownProduct)?;
    if v.stock == 0 {
        return Err(FormError::OutOfStock);
    }
    addons.sort();
    addons.dedup();
    check_addons(p, &addons)?;
    let cap = v.stock.min(dc::MAX_QTY);
    let line = CartLine { product, variant, qty: 0, addons };
    let key = line.key();
    if let Some(l) = lines.iter_mut().find(|l| l.key() == key) {
        if l.qty >= cap {
            return Err(FormError::MaxQty);
        }
        let next = (l.qty + qty).min(cap);
        let added = next - l.qty;
        l.qty = next;
        return Ok(added);
    }
    if lines.len() >= dc::MAX_LINES {
        return Err(FormError::Full);
    }
    let qty = qty.min(cap);
    lines.push(CartLine { qty, ..line });
    Ok(qty)
}

/// Set a line's quantity (0 removes it), at most `MAX_QTY`. Stock is the quote's to report.
pub fn set(lines: &mut Vec<CartLine>, key: &str, qty: u32) -> Result<(), FormError> {
    let at = lines.iter().position(|l| l.key() == key).ok_or(FormError::BadRequest)?;
    if qty == 0 {
        lines.remove(at);
        return Ok(());
    }
    lines[at].qty = qty.min(dc::MAX_QTY);
    if qty > dc::MAX_QTY { Err(FormError::MaxQty) } else { Ok(()) }
}

pub fn remove(lines: &mut Vec<CartLine>, key: &str) {
    lines.retain(|l| l.key() != key);
}

/// `POST /cart/add|set|remove`: plain `application/x-www-form-urlencoded` forms (repeated keys —
/// `addon` — rule out the server-function decoder). Each answers `303` to `next` (a same-site
/// path), else the Referer, else `/cart`, with `?added=<qty>` or `?cart_err=<code>`.
pub mod forms {
    use super::*;
    use axum::body::Bytes;
    use axum::response::{IntoResponse, Response};
    use http::StatusCode;

    struct Form(Vec<(String, String)>);

    impl Form {
        fn parse(body: &[u8]) -> Form {
            let dec = |s: &str| percent_decode(&s.replace('+', " "));
            Form(
                String::from_utf8_lossy(body)
                    .split('&')
                    .filter(|kv| !kv.is_empty())
                    .map(|kv| {
                        let (k, v) = kv.split_once('=').unwrap_or((kv, ""));
                        (dec(k), dec(v))
                    })
                    .collect(),
            )
        }

        fn get(&self, k: &str) -> Option<&str> {
            self.0.iter().find(|(a, _)| a == k).map(|(_, v)| v.as_str())
        }

        fn all(&self, k: &str) -> Vec<String> {
            self.0
                .iter()
                .filter(|(a, _)| a == k)
                .map(|(_, v)| v.trim().to_owned())
                .filter(|v| !v.is_empty())
                .collect()
        }

        /// A number field: absent/empty → `None`, unparsable → `bad_request`.
        fn num(&self, k: &str) -> Result<Option<u32>, FormError> {
            match self.get(k).map(str::trim).filter(|s| !s.is_empty()) {
                None => Ok(None),
                Some(s) => s.parse().map(Some).map_err(|_| FormError::BadRequest),
            }
        }
    }

    /// The tenant, resolved like `request::init` (host, then the `tenant` cookie).
    fn tenant_id(headers: &HeaderMap) -> &'static str {
        use crate::tenant::{COOKIE_TENANT, DEFAULT_TENANT, TENANT_IDS, tenant_from_host};
        let host = headers.get(header::HOST).and_then(|v| v.to_str().ok()).unwrap_or_default();
        tenant_from_host(host)
            .or_else(|| cookie(headers, COOKIE_TENANT).and_then(|c| TENANT_IDS.iter().copied().find(|id| *id == c)))
            .unwrap_or(DEFAULT_TENANT)
    }

    /// Same rule as the server's `safe_next`: a path on this site, never `//host` or `\`.
    fn same_site(p: &str) -> bool {
        p.starts_with('/') && !p.starts_with("//") && !p.contains('\\') && !p.chars().any(char::is_control)
    }

    /// Where the form goes back to: `next`, else the Referer's path (same host), else `/cart`.
    fn back(headers: &HeaderMap, next: Option<&str>) -> String {
        if let Some(n) = next.filter(|n| same_site(n)) {
            return n.to_owned();
        }
        let host = headers.get(header::HOST).and_then(|v| v.to_str().ok()).unwrap_or_default();
        headers
            .get(header::REFERER)
            .and_then(|v| v.to_str().ok())
            .and_then(|r| r.strip_prefix("http://").or_else(|| r.strip_prefix("https://")))
            .and_then(|r| {
                let (h, path) = r.split_once('/').unwrap_or((r, ""));
                (h == host).then(|| format!("/{path}"))
            })
            .filter(|p| same_site(p))
            .unwrap_or_else(|| "/cart".into())
    }

    /// `target` with our `added` / `cart_err` replaced by `param` (before any `#fragment`), and
    /// anything outside visible ASCII percent-encoded so it fits a `Location` header.
    fn with_param(target: &str, param: Option<(&str, &str)>) -> String {
        let (base, frag) = match target.split_once('#') {
            Some((b, f)) => (b, Some(f)),
            None => (target, None),
        };
        let (path, query) = base.split_once('?').unwrap_or((base, ""));
        let mut q: Vec<String> = query
            .split('&')
            .filter(|kv| !kv.is_empty() && !matches!(kv.split('=').next(), Some("added" | "cart_err")))
            .map(str::to_owned)
            .collect();
        if let Some((k, v)) = param {
            q.push(format!("{k}={v}"));
        }
        let mut out = path.to_owned();
        if !q.is_empty() {
            out.push('?');
            out.push_str(&q.join("&"));
        }
        if let Some(f) = frag {
            out.push('#');
            out.push_str(f);
        }
        out.chars()
            .map(|c| {
                if c.is_ascii_graphic() {
                    c.to_string()
                } else {
                    let mut b = [0u8; 4];
                    c.encode_utf8(&mut b).bytes().map(|x| format!("%{x:02X}")).collect()
                }
            })
            .collect()
    }

    fn see_other(to: String, cookie: Option<HeaderValue>) -> Response {
        let mut res = StatusCode::SEE_OTHER.into_response();
        let h = res.headers_mut();
        if let Ok(v) = HeaderValue::from_str(&to) {
            h.insert(header::LOCATION, v);
        } else {
            h.insert(header::LOCATION, HeaderValue::from_static("/cart"));
        }
        if let Some(c) = cookie {
            h.append(header::SET_COOKIE, c);
        }
        h.insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
        res
    }

    /// Answer a form: the cookie rewritten on success, the outcome in the query.
    fn answer(headers: &HeaderMap, f: &Form, lines: &[CartLine], r: Result<Option<u32>, FormError>) -> Response {
        let back = back(headers, f.get("next"));
        match r {
            Ok(added) => {
                let n = added.map(|n| n.to_string());
                see_other(with_param(&back, n.as_deref().map(|n| ("added", n))), Some(set_cookie(lines)))
            }
            Err(e) => see_other(with_param(&back, Some(("cart_err", e.code()))), None),
        }
    }

    pub async fn add(headers: HeaderMap, body: Bytes) -> Response {
        let f = Form::parse(&body);
        let catalog = crate::server::state().catalog(tenant_id(&headers));
        let mut lines = from_headers(&headers);
        let r = (|| {
            let product = f.num("product")?.ok_or(FormError::BadRequest)?;
            let variant = f.num("variant")?.ok_or(FormError::BadRequest)?;
            let qty = f.num("qty")?.unwrap_or(1);
            if qty == 0 {
                return Err(FormError::BadRequest);
            }
            super::add(&mut lines, &catalog, ProductId(product), VariantId(variant), qty, f.all("addon")).map(Some)
        })();
        answer(&headers, &f, &lines, r)
    }

    pub async fn set(headers: HeaderMap, body: Bytes) -> Response {
        let f = Form::parse(&body);
        let mut lines = from_headers(&headers);
        let r = (|| {
            let key = f.get("key").ok_or(FormError::BadRequest)?;
            let qty = f.num("qty")?.ok_or(FormError::BadRequest)?;
            super::set(&mut lines, key, qty).map(|_| None)
        })();
        answer(&headers, &f, &lines, r)
    }

    pub async fn remove(headers: HeaderMap, body: Bytes) -> Response {
        let f = Form::parse(&body);
        let mut lines = from_headers(&headers);
        let r = match f.get("key") {
            Some(key) => {
                super::remove(&mut lines, key);
                Ok(None)
            }
            None => Err(FormError::BadRequest),
        };
        answer(&headers, &f, &lines, r)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn params_replace_and_keep_fragments() {
            assert_eq!(with_param("/c/x?sort=p&added=1#top", Some(("added", "2"))), "/c/x?sort=p&added=2#top");
            assert_eq!(with_param("/cart?cart_err=full", None), "/cart");
            assert_eq!(with_param("/search?q=قهوة", Some(("cart_err", "full"))), "/search?q=%D9%82%D9%87%D9%88%D8%A9&cart_err=full");
            assert!(!same_site("//evil.example/") && !same_site("https://x/") && same_site("/p/a?b=c"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cookie_round_trip() {
        let lines = dc::parse("2307:1:2|2501:3:1:as-is+pro-install");
        let v = set_cookie(&lines);
        let enc = v.to_str().unwrap().split(';').next().unwrap().trim_start_matches("cart=").to_owned();
        assert_eq!(enc, "2307%3A1%3A2%7C2501%3A3%3A1%3Aas-is%2Bpro-install");
        assert_eq!(dc::parse(&percent_decode(&enc)), lines);
    }
}
