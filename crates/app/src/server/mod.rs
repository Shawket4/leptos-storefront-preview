//! Server-only state: tenants, catalogues, media, read cache, carts and orders. Never in WASM.

pub mod cache;
pub mod cart;
pub mod catalog;
pub mod media;
pub mod orders;
pub mod pricing;
pub mod search;
pub mod sim;

use crate::prefs::Lang;
use crate::tenant::Tenant;
use domain::{L10n, Piastres};
use leptos::prelude::*;
use std::{collections::HashMap, path::PathBuf, sync::Arc};

pub use catalog::TenantCatalog;
pub use media::Media;

pub const COOKIE_CART: &str = "cart";
/// Readable (not HttpOnly) cart count, so cacheable public pages can show the badge from a cookie
/// without rendering anything personal into their HTML.
pub const COOKIE_CART_COUNT: &str = "cart_n";

/// Shared application state, provided to every render and server function as context.
#[derive(Clone)]
pub struct AppState(Arc<Inner>);

struct Inner {
    tenants: Vec<Tenant>,
    catalogs: HashMap<String, Arc<TenantCatalog>>,
    media: Arc<Media>,
    reads: cache::ReadCache,
    carts: cart::CartStore,
    orders: orders::OrderStore,
}

fn data_dir() -> PathBuf {
    std::env::var("DATA_DIR").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from("data"))
}

fn media_dir() -> PathBuf {
    std::env::var("MEDIA_DIR").map(PathBuf::from).unwrap_or_else(|_| data_dir().join("media"))
}

fn demo_tenants() -> Vec<Tenant> {
    vec![
        Tenant {
            id: "nile".into(),
            name: L10n::new("Nile", "نايل"),
            tagline: L10n::new("Everything, delivered across Egypt", "كل حاجة توصلك لحد البيت"),
            default_lang: Lang::En,
            accent: "#0D6273".into(),
            accent_dark: "#2E94A6".into(),
            on_accent: "#FFFFFF".into(),
            logo: "brand/nile-lockup-en".into(),
            order_prefix: "NIL".into(),
            delivery_fee: Piastres::from_pounds(45),
            free_delivery_over: Piastres::from_pounds(1500),
            express_minutes: None,
        },
        Tenant {
            id: "souq".into(),
            name: L10n::new("Souq", "سوق"),
            tagline: L10n::new("Your neighbourhood market, online", "سوق الحتة... أونلاين"),
            default_lang: Lang::Ar,
            accent: "#A8431E".into(),
            accent_dark: "#F08A5D".into(),
            on_accent: "#FFFFFF".into(),
            logo: "brand/souq-lockup-ar".into(),
            order_prefix: "SOQ".into(),
            delivery_fee: Piastres::from_pounds(25),
            free_delivery_over: Piastres::from_pounds(600),
            express_minutes: Some(30),
        },
    ]
}

impl AppState {
    pub async fn load() -> Self {
        let media = Arc::new(Media::load(&media_dir()));
        let tenants = demo_tenants();
        let mut catalogs = HashMap::new();
        for t in &tenants {
            let path = data_dir().join("catalog").join(format!("{}.json", t.id));
            let catalog = match std::fs::read_to_string(&path)
                .map_err(|e| e.to_string())
                .and_then(|s| serde_json::from_str::<domain::Catalog>(&s).map_err(|e| e.to_string()))
            {
                Ok(c) => c,
                Err(e) => {
                    tracing::warn!("catalogue {} not loaded ({e}); serving an empty one", path.display());
                    domain::Catalog { tenant: t.id.clone(), brands: vec![], categories: vec![], products: vec![] }
                }
            };
            tracing::info!(
                "tenant {}: {} categories, {} products",
                t.id,
                catalog.categories.len(),
                catalog.products.len()
            );
            catalogs.insert(t.id.clone(), Arc::new(TenantCatalog::new(catalog, &media)));
        }
        tracing::info!("media: {} images", media.len());
        AppState(Arc::new(Inner {
            tenants,
            catalogs,
            media,
            reads: cache::ReadCache::new(),
            carts: cart::CartStore::new(),
            orders: orders::OrderStore::new(),
        }))
    }

    pub fn tenant(&self, id: &str) -> &Tenant {
        self.0.tenants.iter().find(|t| t.id == id).unwrap_or(&self.0.tenants[0])
    }

    pub fn tenants(&self) -> &[Tenant] {
        &self.0.tenants
    }

    pub fn catalog(&self, tenant: &str) -> Arc<TenantCatalog> {
        self.0
            .catalogs
            .get(tenant)
            .cloned()
            .unwrap_or_else(|| self.0.catalogs.values().next().cloned().expect("a catalogue"))
    }

    pub fn media(&self) -> &Media {
        &self.0.media
    }

    pub fn reads(&self) -> &cache::ReadCache {
        &self.0.reads
    }

    pub fn carts(&self) -> &cart::CartStore {
        &self.0.carts
    }

    pub fn orders(&self) -> &orders::OrderStore {
        &self.0.orders
    }
}

/// The app state from context (server components and server functions).
pub fn state() -> AppState {
    expect_context::<AppState>()
}

/// The current request's tenant catalogue.
pub fn current_catalog() -> Arc<TenantCatalog> {
    let rq = crate::request::init();
    state().catalog(&rq.tenant.id)
}
