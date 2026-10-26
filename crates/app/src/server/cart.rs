//! Server-side carts, keyed by an opaque id in an HttpOnly cookie, one cart per tenant.
//!
//! Lines are stored as ids only; names and prices are resolved at read time, and every line is
//! priced with Madar's rules (`pricing::unit_price` → `madar_catalog::price_line`), the way
//! Madar re-prices every order at intake instead of trusting the client.

use super::{TenantCatalog, media::Media, pricing};
use crate::api::types::{CartLineView, CartView};
use crate::tenant::Tenant;
use domain::{Piastres, ProductId, VariantId};
use http::HeaderValue;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CartLine {
    pub product: ProductId,
    pub variant: VariantId,
    pub addons: Vec<String>,
    pub qty: u32,
}

impl CartLine {
    pub fn key(&self) -> String {
        line_key(self.product, self.variant, &self.addons)
    }
}

pub fn line_key(product: ProductId, variant: VariantId, addons: &[String]) -> String {
    let mut k = format!("{}-{}", product.0, variant.0);
    let mut a = addons.to_vec();
    a.sort();
    for x in a {
        k.push('+');
        k.push_str(&x);
    }
    k
}

#[derive(Clone, Debug, Default)]
pub struct Cart {
    pub lines: Vec<CartLine>,
}

pub const MAX_PER_LINE: u32 = 20;

impl Cart {
    /// Add `qty` (merging with an identical line), capped at stock and `MAX_PER_LINE`.
    /// Returns the line's new quantity.
    pub fn add(&mut self, product: ProductId, variant: VariantId, addons: Vec<String>, qty: u32, stock: u32) -> u32 {
        let cap = stock.min(MAX_PER_LINE);
        let key = line_key(product, variant, &addons);
        if let Some(l) = self.lines.iter_mut().find(|l| l.key() == key) {
            l.qty = (l.qty + qty).min(cap);
            return l.qty;
        }
        let qty = qty.min(cap);
        if qty > 0 {
            self.lines.push(CartLine { product, variant, addons, qty });
        }
        qty
    }

    /// Set a line's quantity (0 removes it), capped at stock.
    pub fn set_qty(&mut self, key: &str, qty: u32, stock: u32) {
        let cap = stock.min(MAX_PER_LINE);
        if qty == 0 {
            self.lines.retain(|l| l.key() != key);
        } else if let Some(l) = self.lines.iter_mut().find(|l| l.key() == key) {
            l.qty = qty.min(cap);
        }
    }

    pub fn remove(&mut self, key: &str) {
        self.lines.retain(|l| l.key() != key);
    }

    pub fn clear(&mut self) {
        self.lines.clear();
    }

    pub fn count(&self) -> u32 {
        self.lines.iter().map(|l| l.qty).sum()
    }
}

pub struct CartStore {
    carts: moka::sync::Cache<String, Arc<Mutex<Cart>>>,
}

impl CartStore {
    pub fn new() -> Self {
        CartStore {
            carts: moka::sync::Cache::builder()
                .max_capacity(100_000)
                .time_to_idle(Duration::from_secs(30 * 24 * 3600))
                .build(),
        }
    }

    fn key(tenant: &str, id: &str) -> String {
        format!("{tenant}:{id}")
    }

    pub fn get(&self, tenant: &str, id: &str) -> Option<Arc<Mutex<Cart>>> {
        self.carts.get(&Self::key(tenant, id))
    }

    /// The cart for this cookie value, creating it (and a new id) if needed.
    /// Returns `(id, cart, created)`.
    pub fn get_or_create(&self, tenant: &str, id: Option<&str>) -> (String, Arc<Mutex<Cart>>, bool) {
        if let Some(id) = id.filter(|s| s.len() >= 16 && s.len() <= 64) {
            let k = Self::key(tenant, id);
            if let Some(c) = self.carts.get(&k) {
                return (id.to_owned(), c, false);
            }
            // Unknown id (server restarted, or another tenant on the same host): adopt it.
            let c = Arc::new(Mutex::new(Cart::default()));
            self.carts.insert(k, c.clone());
            return (id.to_owned(), c, false);
        }
        let id = uuid::Uuid::new_v4().simple().to_string();
        let c = Arc::new(Mutex::new(Cart::default()));
        self.carts.insert(Self::key(tenant, &id), c.clone());
        (id, c, true)
    }
}

impl Default for CartStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Resolve and price a cart for display. Lines whose product vanished are skipped.
pub fn view(cart: &Cart, tenant: &Tenant, catalog: &TenantCatalog, media: &Media) -> CartView {
    let mut lines = Vec::new();
    let mut subtotal = Piastres::ZERO;
    let mut savings = Piastres::ZERO;
    for l in &cart.lines {
        let Some((p, v)) = catalog.variant(l.product, l.variant) else { continue };
        let unit = pricing::unit_price(p, v, &l.addons).unwrap_or(v.price);
        let line_total = unit * l.qty;
        subtotal += line_total;
        if let Some(was) = v.compare_at.filter(|c| *c > v.price) {
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
        lines.push(CartLineView {
            key: l.key(),
            product: p.id,
            variant: v.id,
            slug: p.slug.clone(),
            name: p.name.clone(),
            variant_label: TenantCatalog::variant_label(p, v),
            addons,
            img: img_key.map(|k| media.img(k)).unwrap_or_else(|| crate::media::Img::missing("")),
            qty: l.qty,
            max_qty: v.stock.min(MAX_PER_LINE),
            unit_price: unit,
            compare_at: v.compare_at.filter(|c| *c > v.price),
            line_total,
        });
    }
    let count = lines.iter().map(|l| l.qty).sum();
    let delivery_fee = if subtotal.0 == 0 || subtotal >= tenant.free_delivery_over {
        Piastres::ZERO
    } else {
        tenant.delivery_fee
    };
    CartView {
        lines,
        count,
        subtotal,
        savings,
        delivery_fee,
        free_delivery_over: tenant.free_delivery_over,
        // Madar keeps the delivery fee outside the tax base.
        tax_included: pricing::vat_included(subtotal),
        total: subtotal + delivery_fee,
    }
}

/// `Set-Cookie` values for the cart id (HttpOnly) and the readable count used by the badge.
/// No `Secure` flag: the preview is served over plain http on the tailnet.
pub fn cookies(id: &str, count: u32) -> [HeaderValue; 2] {
    let age = 30 * 24 * 3600;
    [
        HeaderValue::from_str(&format!("{}={id}; Path=/; Max-Age={age}; HttpOnly; SameSite=Lax", super::COOKIE_CART))
            .expect("cookie"),
        HeaderValue::from_str(&format!("{}={count}; Path=/; Max-Age={age}; SameSite=Lax", super::COOKIE_CART_COUNT))
            .expect("cookie"),
    ]
}
