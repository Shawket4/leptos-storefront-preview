//! Cart server functions. Shaped for `<ActionForm>` (flat, form-encodable arguments) so the same
//! endpoints serve islands (JSON) and plain no-JS `<form method=post>` (server_fn then answers
//! with a redirect back to the Referer, and encodes any error into that URL).
//!
//! Every response is `Cache-Control: private, no-store` and refreshes two cookies: the HttpOnly
//! cart id and the readable `cart_n` count that cacheable pages use for the header badge.
//!
//! Error codes (map them to copy in the UI): `out_of_stock`, `unknown_product`,
//! `addon_required`, `bad_addon`.

use super::types::CartView;
use leptos::prelude::*;

#[cfg(feature = "ssr")]
mod imp {
    use super::*;
    use crate::server::{cart, sim, state};
    use domain::{ProductId, VariantId};

    pub fn empty(tenant: &crate::tenant::Tenant) -> CartView {
        CartView { free_delivery_over: tenant.free_delivery_over, ..Default::default() }
    }

    /// Run `f` on this visitor's cart (creating it if needed), then price it and set cookies.
    pub async fn mutate(
        f: impl FnOnce(&mut cart::Cart, &crate::server::TenantCatalog) -> Result<(), ServerFnError>,
    ) -> Result<CartView, ServerFnError> {
        let rq = crate::request::init();
        crate::server::cache::private();
        sim::check_fail(sim::Widget::Cart)?;
        sim::delay(sim::Widget::Cart).await;
        let st = state();
        let tenant = st.tenant(&rq.tenant.id).clone();
        let catalog = st.catalog(&tenant.id);
        let (id, handle, _) = st.carts().get_or_create(&tenant.id, rq.cart_id.as_deref());
        let view = {
            let mut c = handle.lock().map_err(|_| ServerFnError::new("cart lock"))?;
            f(&mut c, &catalog)?;
            cart::view(&c, &tenant, &catalog, st.media())
        };
        if let Some(res) = use_context::<leptos_axum::ResponseOptions>() {
            for v in cart::cookies(&id, view.count) {
                res.append_header(http::header::SET_COOKIE, v);
            }
        }
        Ok(view)
    }

    pub fn parse_addons(raw: Option<String>) -> Vec<String> {
        raw.unwrap_or_default()
            .split([',', ' '])
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_owned)
            .collect()
    }

    /// Check picked add-ons against the product's Madar-style modifier groups.
    pub fn check_addons(p: &domain::Product, addons: &[String]) -> Result<(), ServerFnError> {
        for a in addons {
            if !p.modifier_groups.iter().any(|g| g.options.iter().any(|o| &o.id == a)) {
                return Err(ServerFnError::new("bad_addon"));
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
                return Err(ServerFnError::new("addon_required"));
            }
            if picked > max {
                return Err(ServerFnError::new("bad_addon"));
            }
        }
        Ok(())
    }

    pub fn ids(product: u32, variant: u32) -> (ProductId, VariantId) {
        (ProductId(product), VariantId(variant))
    }
}

/// The visitor's cart (empty if they have none yet).
#[server]
pub async fn cart_get() -> Result<CartView, ServerFnError> {
    let rq = crate::request::init();
    crate::server::cache::private();
    crate::server::sim::delay(crate::server::sim::Widget::Cart).await;
    let st = crate::server::state();
    let tenant = st.tenant(&rq.tenant.id).clone();
    let Some(handle) = rq.cart_id.as_deref().and_then(|id| st.carts().get(&tenant.id, id)) else {
        return Ok(imp::empty(&tenant));
    };
    let c = handle.lock().map_err(|_| ServerFnError::new("cart lock"))?;
    Ok(crate::server::cart::view(&c, &tenant, &st.catalog(&tenant.id), st.media()))
}

/// Add a variant (with optional comma-separated add-on ids). `qty` defaults to 1.
#[server]
pub async fn cart_add(product: u32, variant: u32, qty: Option<u32>, addons: Option<String>) -> Result<CartView, ServerFnError> {
    let (pid, vid) = imp::ids(product, variant);
    let addons = imp::parse_addons(addons);
    imp::mutate(move |cart, catalog| {
        let (p, v) = catalog.variant(pid, vid).ok_or_else(|| ServerFnError::new("unknown_product"))?;
        if v.stock == 0 {
            return Err(ServerFnError::new("out_of_stock"));
        }
        imp::check_addons(p, &addons)?;
        cart.add(pid, vid, addons, qty.unwrap_or(1).max(1), v.stock);
        Ok(())
    })
    .await
}

/// Set a line's quantity; 0 removes it. `key` is `CartLineView::key`.
#[server]
pub async fn cart_set_qty(key: String, qty: u32) -> Result<CartView, ServerFnError> {
    imp::mutate(move |cart, catalog| {
        let stock = cart
            .lines
            .iter()
            .find(|l| l.key() == key)
            .and_then(|l| catalog.variant(l.product, l.variant))
            .map(|(_, v)| v.stock)
            .unwrap_or(0);
        cart.set_qty(&key, qty, stock);
        Ok(())
    })
    .await
}

/// Remove a line.
#[server]
pub async fn cart_remove(key: String) -> Result<CartView, ServerFnError> {
    imp::mutate(move |cart, _| {
        cart.remove(&key);
        Ok(())
    })
    .await
}
