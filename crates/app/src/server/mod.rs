//! Server-only state: tenants, catalogs, caches, carts and orders. Never compiled to WASM.

use crate::prefs::Lang;
use crate::tenant::Tenant;
use domain::L10n;
use std::sync::Arc;

pub const COOKIE_CART: &str = "cart";

/// Shared application state, provided to every render and server function as context.
#[derive(Clone)]
pub struct AppState(Arc<Inner>);

struct Inner {
    tenants: Vec<Tenant>,
}

impl AppState {
    pub async fn load() -> Self {
        let tenants = vec![
            Tenant {
                id: "nile".into(),
                name: L10n::new("Nile", "نايل"),
                tagline: L10n::new("Everything, delivered across Egypt", "كل حاجة توصلك لحد البيت"),
                default_lang: Lang::En,
                accent: "#0D6273".into(),
                accent_dark: "#2E94A6".into(),
                on_accent: "#FFFFFF".into(),
                logo: "brand/nile-logo".into(),
            },
            Tenant {
                id: "souq".into(),
                name: L10n::new("Souq", "سوق"),
                tagline: L10n::new("Your neighbourhood market, online", "سوق الحتة... أونلاين"),
                default_lang: Lang::Ar,
                accent: "#A8431E".into(),
                accent_dark: "#F08A5D".into(),
                on_accent: "#FFFFFF".into(),
                logo: "brand/souq-logo".into(),
            },
        ];
        AppState(Arc::new(Inner { tenants }))
    }

    pub fn tenant(&self, id: &str) -> &Tenant {
        self.0
            .tenants
            .iter()
            .find(|t| t.id == id)
            .unwrap_or(&self.0.tenants[0])
    }

    pub fn tenants(&self) -> &[Tenant] {
        &self.0.tenants
    }
}
