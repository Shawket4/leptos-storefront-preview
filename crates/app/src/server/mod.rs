//! Server-only state: tenants, catalogues, media, read cache and orders. (Carts live in the
//! browser's `cart` cookie — `server::cart` prices them.)

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
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, RwLock},
    time::{Duration, SystemTime},
};

pub use catalog::TenantCatalog;
pub use media::Media;

/// Shared application state, provided to every render and server function as context.
#[derive(Clone)]
pub struct AppState(Arc<Inner>);

struct Inner {
    tenants: Vec<Tenant>,
    catalogs: RwLock<HashMap<String, Arc<TenantCatalog>>>,
    media: Arc<Media>,
    reads: cache::ReadCache,
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
        let catalogs = load_catalogs(&tenants, &media);
        tracing::info!("media: {} images", media.len());
        let state = AppState(Arc::new(Inner {
            tenants,
            catalogs: RwLock::new(catalogs),
            media,
            reads: cache::ReadCache::new(),
            orders: orders::OrderStore::new(),
        }));
        let _ = STATE.set(state.clone());
        state
    }

    /// Reload catalogues and media manifests when their files change (polled every 3 s), then
    /// bump every tenant's cache generation — the same move a catalogue edit makes in production
    /// (plus a CDN purge by `Cache-Tag`).
    pub fn watch_data(&self) {
        let st = self.clone();
        tokio::spawn(async move {
            let mut last = data_fingerprint();
            loop {
                tokio::time::sleep(Duration::from_secs(3)).await;
                let now = data_fingerprint();
                if now != last {
                    last = now;
                    st.0.media.reload(&media_dir());
                    let fresh = load_catalogs(&st.0.tenants, &st.0.media);
                    if let Ok(mut c) = st.0.catalogs.write() {
                        *c = fresh;
                    }
                    for t in &st.0.tenants {
                        st.0.reads.bump(&t.id);
                    }
                    tracing::info!("data changed: reloaded ({} images)", st.0.media.len());
                }
            }
        });
    }
}

/// Sum of (len, mtime) of every catalogue and manifest file: cheap change detection.
fn data_fingerprint() -> u128 {
    let mut acc = 0u128;
    let mut add = |p: &std::path::Path| {
        if let Ok(m) = std::fs::metadata(p) {
            let t = m.modified().ok().and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok()).map(|d| d.as_millis()).unwrap_or(0);
            acc = acc.wrapping_add(t ^ (m.len() as u128) << 64);
        }
    };
    for dir in [data_dir().join("catalog"), data_dir().join("search")] {
        if let Ok(rd) = std::fs::read_dir(&dir) {
            rd.flatten().for_each(|e| add(&e.path()));
        }
    }
    if let Ok(rd) = std::fs::read_dir(media_dir()) {
        rd.flatten().for_each(|e| add(&e.path().join("manifest.json")));
    }
    acc
}

/// The search synonym table for a shop: the shared `data/search/synonyms.txt` plus the shop's own
/// `synonyms.<shop>.txt` (words that are synonyms in one shop only). Missing files are empty.
fn load_synonyms(tenant: &str) -> search::Synonyms {
    let dir = data_dir().join("search");
    let read = |name: String| std::fs::read_to_string(dir.join(name)).unwrap_or_default();
    search::Synonyms::parse(&format!("{}\n{}", read("synonyms.txt".into()), read(format!("synonyms.{tenant}.txt"))))
}

fn load_catalogs(tenants: &[Tenant], media: &Media) -> HashMap<String, Arc<TenantCatalog>> {
    let mut catalogs = HashMap::new();
    {
        for t in tenants {
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
            let synonyms = load_synonyms(&t.id);
            tracing::info!("tenant {}: {} synonym groups", t.id, synonyms.len());
            catalogs.insert(t.id.clone(), Arc::new(TenantCatalog::new(catalog, media, synonyms)));
        }
    }
    catalogs
}

impl AppState {

    pub fn tenant(&self, id: &str) -> &Tenant {
        self.0.tenants.iter().find(|t| t.id == id).unwrap_or(&self.0.tenants[0])
    }

    pub fn tenants(&self) -> &[Tenant] {
        &self.0.tenants
    }

    pub fn catalog(&self, tenant: &str) -> Arc<TenantCatalog> {
        let c = self.0.catalogs.read().expect("catalogue lock");
        c.get(tenant).cloned().unwrap_or_else(|| c.values().next().cloned().expect("a catalogue"))
    }

    pub fn media(&self) -> &Media {
        &self.0.media
    }

    pub fn reads(&self) -> &cache::ReadCache {
        &self.0.reads
    }

    pub fn orders(&self) -> &orders::OrderStore {
        &self.0.orders
    }
}

/// The process's one `AppState`, for when the request's context is gone (below).
static STATE: std::sync::OnceLock<AppState> = std::sync::OnceLock::new();

/// The app state: from the request's context, else the process-wide handle. Leptos runs page
/// reads as spawned tasks that keep going after a client hangs up mid-stream, by which time the
/// request's owner — and its context — is disposed; a context-only lookup then panicked.
pub fn state() -> AppState {
    use_context::<AppState>().or_else(|| STATE.get().cloned()).expect("AppState::load runs at startup")
}

/// The current request's tenant catalogue.
pub fn current_catalog() -> Arc<TenantCatalog> {
    let rq = crate::request::init();
    state().catalog(&rq.tenant.id)
}
