//! `/showcase` — every Leptos feature this storefront uses, each with a plain explanation (EN +
//! AR), why it matters here, a live demo, where it is used in the app, and the code that does it.
//!
//! The page is a server component; each demo that needs the browser is one small island
//! (`showcase/islands.rs`). Code excerpts are cut from the repository at compile time
//! (`showcase/snippets.rs`), so they can't go stale. It is `private, no-store`: it shows the
//! visitor's own simulated-latency settings and the server's live cache counters.

mod data;
mod demos;
mod islands;
#[cfg(feature = "hydrate")]
mod js;
mod notes;
mod snippets;

pub use islands::provide_store;

use crate::api::types::{CacheInfo, NavCategory};
use crate::components::icons::Icon;
use crate::components::product_card::CardData;
use crate::media::Img;
use crate::pages::errors::ErrorPage;
use crate::prefs::{Lang, Numerals, Theme, tr};
use crate::request::req;
use crate::tenant::TenantVia;
use data::{Cat, FEATURES, Feature, Href};
use domain::{L10n, Piastres};
use leptos::prelude::*;
use leptos_meta::{Link, Meta, Title};
use snippets::{Code, Cut, Src, cut};
use std::sync::Arc;

// ─── Request-derived data, computed once on the server ─────────────────────────────────────────

/// Query parameters this page understands.
#[derive(Clone, Debug, Default)]
pub(crate) struct Query {
    pub tab: Option<String>,
    pub cat: Option<Cat>,
    /// `?fail=` (per-request simulated faults).
    pub fail: Vec<String>,
    /// `?crash=1|500|503`: show the full-page error view (route-level failure).
    pub crash: Option<u16>,
    /// View-transition demo layout: `list` or grid.
    pub vt_list: bool,
    /// The raw query string (router demo).
    pub raw: String,
    /// `?__path=…&__err=…` from a no-JS ActionForm post that failed.
    pub action_err: Option<(String, String)>,
}

impl Query {
    fn parse(path_and_query: &str) -> Query {
        let raw = path_and_query
            .split_once('?')
            .map(|(_, q)| q)
            .unwrap_or("")
            .to_owned();
        let mut q = Query {
            raw: raw.clone(),
            ..Default::default()
        };
        let mut path = None;
        let mut err = None;
        for pair in raw.split('&').filter(|p| !p.is_empty()) {
            let (k, v) = pair.split_once('=').unwrap_or((pair, ""));
            let v = crate::api::types::url_decode(v);
            match k {
                "tab" if !v.is_empty() => q.tab = Some(v),
                "cat" => q.cat = Cat::parse(&v),
                "fail" => {
                    q.fail = v
                        .split([',', ' ', '+'])
                        .filter(|s| !s.is_empty())
                        .map(str::to_owned)
                        .collect()
                }
                "crash" => {
                    q.crash = match v.as_str() {
                        "" | "0" => None,
                        "503" => Some(503),
                        _ => Some(500),
                    }
                }
                "vt" => q.vt_list = v == "list",
                "__path" => path = Some(v),
                "__err" => err = Some(v),
                _ => {}
            }
        }
        q.action_err = path.zip(err);
        q
    }
}

/// Links that depend on the tenant's catalogue.
#[derive(Clone, Debug, Default)]
pub(crate) struct Links {
    pub category: Option<String>,
    pub product: Option<String>,
    pub variant: Option<u32>,
}

impl Links {
    pub fn resolve(&self, h: Href) -> Option<String> {
        match h {
            Href::Path(p) => Some(p.to_owned()),
            Href::Category => self.category.as_ref().map(|c| format!("/c/{c}")),
            Href::CategoryQuery => self
                .category
                .as_ref()
                .map(|c| format!("/c/{c}?stock=1&sort=price-asc")),
            Href::Product => self.product.as_ref().map(|p| format!("/p/{p}")),
            Href::ProductVariant => match (&self.product, self.variant) {
                (Some(p), Some(v)) => Some(format!("/p/{p}?v={v}")),
                _ => None,
            },
            Href::ProductQuery(q) => self.product.as_ref().map(|p| format!("/p/{p}?{q}")),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct TenantFacts {
    pub id: String,
    pub name: L10n,
    pub default_lang: Lang,
    pub via: Option<TenantVia>,
    pub host: String,
    pub order_prefix: String,
    pub delivery_fee: Piastres,
    pub free_over: Piastres,
    pub express_minutes: Option<u32>,
    pub categories: usize,
    pub products: usize,
    /// (id, name, href, other origin?)
    pub other: Option<(String, L10n, String, bool)>,
}

/// Measures when each streamed block resolved on the server, relative to the page's start.
#[derive(Debug)]
pub(crate) struct Clock {
    #[cfg(feature = "ssr")]
    pub start: std::time::Instant,
    pub arrivals: std::sync::atomic::AtomicU32,
}

impl Clock {
    fn new() -> Self {
        Clock {
            #[cfg(feature = "ssr")]
            start: std::time::Instant::now(),
            arrivals: std::sync::atomic::AtomicU32::new(0),
        }
    }

    /// (ms since the page started rendering, arrival rank starting at 1)
    pub fn arrived(&self) -> (u64, u32) {
        #[cfg(feature = "ssr")]
        let ms = self.start.elapsed().as_millis() as u64;
        #[cfg(not(feature = "ssr"))]
        let ms = 0;
        (
            ms,
            self.arrivals
                .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
                + 1,
        )
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Ctx {
    pub lang: Lang,
    pub num: Numerals,
    pub theme: Theme,
    pub path_and_query: String,
    pub origin: String,
    pub q: Query,
    pub tenant: TenantFacts,
    pub base_ms: u64,
    /// The visitor set the latency with the `sim` cookie (else it's the server default).
    pub sim_cookie: bool,
    pub fail_cookie: Vec<String>,
    pub cats: Vec<NavCategory>,
    pub links: Links,
    /// Three products for the calculator and the wishlist.
    pub picks: Vec<CardData>,
    /// The active tab's category slug and its first products (islands-router demo).
    pub tab: Option<(NavCategory, Vec<CardData>)>,
    pub image: Option<Img>,
    pub vt_cards: Vec<CardData>,
    pub cache: CacheInfo,
    pub board: notes::Board,
    pub note_error: Option<String>,
    pub files: Vec<islands::DiskFile>,
    pub debug_build: bool,
    /// This HTML is for an islands-router navigation (in-order) rather than a full load.
    pub router_nav: bool,
    pub now_ms: i64,
    pub clock: Arc<Clock>,
    /// Router demo: the parsed listing query and its first results.
    pub listing: Option<(crate::api::types::ListingQuery, Vec<CardData>, u32)>,
    /// Pages in the Transition demo's listing (all products, 24 per page), and its first page.
    pub pager_pages: u32,
    pub pager_first: Vec<CardData>,
    /// In-stock products priced under the free-delivery line (price calculator).
    pub calc_units: Vec<CardData>,
}

impl Ctx {
    /// `/showcase…` with `#anchor`, for `next=` parameters (URL-encoded).
    pub fn next_here(&self, anchor: &str) -> String {
        crate::api::types::url_encode(&format!("{}#{anchor}", self.path_and_query))
    }
}

#[cfg(feature = "ssr")]
fn build_ctx(q: Query) -> Ctx {
    use crate::api::types::ListingQuery;
    use crate::server::{current_catalog, state};

    let rq = req();
    let st = state();
    let cat = current_catalog();
    let t = &rq.tenant;
    let cats = cat.nav().to_vec();
    let first_cat = cats.iter().find(|c| c.count > 0).map(|c| c.slug.clone());
    let multi = cat
        .products()
        .iter()
        .find(|p| p.variants.len() > 1 && p.variants.iter().filter(|v| v.stock > 0).count() > 1);
    let any_product = cat.products().first();
    let links = Links {
        category: first_cat.clone(),
        product: multi.or(any_product).map(|p| p.slug.clone()),
        variant: multi.and_then(|p| p.variants.get(1)).map(|v| v.id.0),
    };
    // Three in-stock products with images, from different categories when possible.
    let mut picks: Vec<CardData> = Vec::new();
    for c in &cats {
        if picks.len() == 3 {
            break;
        }
        let page = cat.listing(&ListingQuery {
            category: Some(c.slug.clone()),
            page: 1,
            ..Default::default()
        });
        if let Some(card) = page
            .items
            .into_iter()
            .find(|c| c.in_stock && !c.img.is_missing())
        {
            picks.push(card);
        }
    }
    let tab = q
        .tab
        .as_deref()
        .and_then(|s| cats.iter().find(|c| c.slug == s))
        .or_else(|| cats.iter().find(|c| c.count > 0))
        .cloned()
        .map(|c| {
            let page = cat.listing(&ListingQuery {
                category: Some(c.slug.clone()),
                page: 1,
                ..Default::default()
            });
            (c, page.items.into_iter().take(4).collect::<Vec<_>>())
        });
    let image = picks
        .first()
        .map(|c| c.img.clone())
        .filter(|i| i.widths.len() > 2);
    let vt_cards: Vec<CardData> = cat
        .listing(&ListingQuery {
            page: 1,
            deal: true,
            ..Default::default()
        })
        .items
        .into_iter()
        .filter(|c| !c.img.is_missing())
        .take(4)
        .collect();

    let fail_cookie = rq_cookie("fail")
        .map(|v| {
            v.split(',')
                .filter(|s| !s.is_empty())
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default();
    let stats = st.reads().stats();
    let cache = CacheInfo {
        entries: stats.entries,
        hits: stats.hits,
        misses: stats.misses,
        generation: st.reads().generation(&t.id),
        base_latency_ms: crate::server::sim::base_ms(),
    };
    let note_error = q.action_err.as_ref().and_then(|(path, err)| {
        use leptos::server_fn::ServerFn;
        (path.as_str() == <notes::LeaveNote as ServerFn>::PATH).then(|| {
            let e = leptos::server_fn::error::ServerFnUrlError::<ServerFnError>::decode_err(err);
            match e {
                ServerFnError::ServerError(m) => m,
                other => other.to_string(),
            }
        })
    });
    let site = leptos::config::get_configuration(None)
        .ok()
        .map(|c| c.leptos_options);
    let files = site
        .map(|o| {
            let dir = std::path::Path::new(o.site_root.as_ref()).join(o.site_pkg_dir.as_ref());
            let name = o.output_name.to_string();
            [
                format!("{name}.wasm"),
                format!("{name}_bg.wasm"),
                format!("{name}.js"),
            ]
            .into_iter()
            .filter_map(|f| {
                std::fs::metadata(dir.join(&f))
                    .ok()
                    .map(|m| islands::DiskFile {
                        name: f,
                        bytes: m.len(),
                    })
            })
            .collect()
        })
        .unwrap_or_default();

    let other = crate::tenant::TENANT_IDS
        .iter()
        .copied()
        .find(|id| *id != t.id)
        .map(|id| {
            let other = st.tenant(id);
            match (rq.via, crate::tenant::swap_tenant_label(&rq.host, id)) {
                (TenantVia::Subdomain, Some(host)) => {
                    let scheme = rq.origin.split("://").next().unwrap_or("http");
                    (
                        id.to_owned(),
                        other.name.clone(),
                        format!("{scheme}://{host}/showcase#tenancy"),
                        true,
                    )
                }
                _ => (
                    id.to_owned(),
                    other.name.clone(),
                    format!("/__tenant/{id}?next=%2Fshowcase%23tenancy"),
                    false,
                ),
            }
        });

    // Router demo: read the query the way a listing page does (use_query_map), parse it with the
    // listing's own parser, and render its first results.
    let listing = {
        let qm = leptos_router::hooks::use_query_map();
        let c = qm
            .with_untracked(|m| m.get("c"))
            .filter(|c| cats.iter().any(|x| &x.slug == c))
            .or(first_cat);
        c.map(|slug| {
            let lq = ListingQuery::from_query(Some(slug), &q.raw);
            let page = cat.listing(&lq);
            let total = page.total;
            (lq, page.items.into_iter().take(3).collect(), total)
        })
    };

    let pager = cat.listing(&ListingQuery {
        page: 1,
        sort: crate::api::types::Sort::Newest,
        ..Default::default()
    });
    let pager_pages = pager.pages;
    // The thumbnails don't use the blurred placeholder; keep the island's props small.
    let pager_first: Vec<CardData> = pager
        .items
        .into_iter()
        .map(|mut c| {
            c.img.lqip = String::new();
            c
        })
        .collect();
    // Calculator: real products under the free-delivery line, so a few steps cross it.
    let free = t.free_delivery_over.0;
    let mut calc_units: Vec<CardData> = Vec::new();
    for c in &cats {
        if calc_units.len() == 3 {
            break;
        }
        let page = cat.listing(&ListingQuery {
            category: Some(c.slug.clone()),
            page: 1,
            ..Default::default()
        });
        if let Some(card) = page
            .items
            .into_iter()
            .find(|x| x.in_stock && x.price.0 >= free / 6 && x.price.0 < free * 9 / 10)
        {
            calc_units.push(card);
        }
    }

    Ctx {
        lang: rq.prefs.lang,
        num: rq.prefs.numerals,
        theme: rq.prefs.theme,
        path_and_query: crate::pages::errors::strip_param(&rq.path_and_query, "__err"),
        origin: rq.origin.clone(),
        tenant: TenantFacts {
            id: t.id.clone(),
            name: t.name.clone(),
            default_lang: t.default_lang,
            via: Some(rq.via),
            host: rq.host.clone(),
            order_prefix: t.order_prefix.clone(),
            delivery_fee: t.delivery_fee,
            free_over: t.free_delivery_over,
            express_minutes: t.express_minutes,
            categories: cats.len(),
            products: cat.products().len(),
            other,
        },
        base_ms: crate::server::sim::base_ms(),
        sim_cookie: rq_cookie("sim").is_some(),
        fail_cookie,
        cats,
        links,
        picks,
        tab,
        image,
        vt_cards,
        cache,
        board: notes::board_now(),
        note_error,
        files,
        debug_build: cfg!(debug_assertions),
        router_nav: false,
        now_ms: crate::server::orders::now_ms(),
        clock: Arc::new(Clock::new()),
        listing,
        pager_pages,
        pager_first,
        calc_units,
        q,
    }
}

#[cfg(feature = "ssr")]
fn rq_cookie(name: &str) -> Option<String> {
    let parts = use_context::<http::request::Parts>()?;
    let header = parts.headers.get(http::header::COOKIE)?.to_str().ok()?;
    crate::request::parse_cookies(header)
        .find(|(k, _)| *k == name)
        .map(|(_, v)| v.to_owned())
}

/// The hydrate build never renders this page (it is a server component); this only has to
/// compile.
#[cfg(not(feature = "ssr"))]
fn build_ctx(q: Query) -> Ctx {
    let rq = req();
    Ctx {
        lang: rq.prefs.lang,
        num: rq.prefs.numerals,
        theme: rq.prefs.theme,
        path_and_query: rq.path_and_query.clone(),
        origin: rq.origin.clone(),
        q,
        tenant: TenantFacts::default(),
        base_ms: 0,
        sim_cookie: false,
        fail_cookie: vec![],
        cats: vec![],
        links: Links::default(),
        picks: vec![],
        tab: None,
        image: None,
        vt_cards: vec![],
        cache: CacheInfo::default(),
        board: notes::Board::default(),
        note_error: None,
        files: vec![],
        debug_build: false,
        router_nav: false,
        now_ms: 0,
        clock: Arc::new(Clock::new()),
        listing: None,
        pager_pages: 1,
        pager_first: vec![],
        calc_units: vec![],
    }
}

// ─── The page ──────────────────────────────────────────────────────────────────────────────────

#[component]
pub fn ShowcasePage() -> impl IntoView {
    // Personal (sim cookies, live counters): never cached. Set before anything streams.
    #[cfg(feature = "ssr")]
    crate::server::cache::private();

    let rq = req();
    let q = Query::parse(&rq.path_and_query);
    let lang = rq.prefs.lang;
    let ar = lang.is_rtl();
    let title = format!(
        "{} · {}",
        tr(lang, "How this storefront is built", "إزاي المتجر ده متبني"),
        rq.tenant.name.pick(ar)
    );
    let description = tr(
        lang,
        "Every Leptos feature this storefront uses — islands, streaming SSR, server functions, the islands router — each with a live demo, where it's used, and the code.",
        "كل خاصية في Leptos المتجر ده بيستخدمها — الجزر، والبث من السيرفر، ودوال السيرفر، والـ islands router — كل واحدة بديمو حي، وفين مستخدمة، والكود بتاعها.",
    );

    // A route-level error view, demonstrated without wrapping the streamed page: an
    // <ErrorBoundary> that *contains* <Suspense> waits for all of it before sending anything
    // (see "Sharp edges"), so the boundary here holds only the synchronous failure.
    if let Some(code) = q.crash {
        let unavailable = code == 503;
        return view! {
            <ErrorBoundary fallback=move |errors| view! { <ErrorPage errors unavailable/> }>
                {Err::<(), _>(ServerFnError::new(format!("simulated route-level failure (/showcase?crash={code})")))}
            </ErrorBoundary>
        }
        .into_any();
    }

    let cx = Arc::new(build_ctx(q));
    let canonical = rq.absolute("/showcase");
    let filter = cx.q.cat.map(|c| c.code());

    view! {
        <Title text=title.clone()/>
        <Meta name="description" content=description/>
        <Meta name="robots" content="noindex"/>
        <Link rel="canonical" href=canonical.clone()/>
        <Meta property="og:type" content="article"/>
        <Meta property="og:title" content=title/>
        <Meta property="og:description" content=description/>
        <Meta property="og:url" content=canonical/>
        <Meta property="og:locale" content=if ar { "ar_EG" } else { "en_EG" }/>
        <Meta name="twitter:card" content="summary"/>

        <article id="sc" class="sc" data-filter=filter>
            <Intro cx=cx.clone()/>
            <div class="sc-layout container">
                <aside class="sc-toc" aria-labelledby="sc-toc-title">
                    <Toc cx=cx.clone()/>
                </aside>
                <div id="features" class="sc-features">
                    {Cat::ALL.into_iter().map(|cat| view! { <Category cat cx=cx.clone()/> }).collect_view()}
                    <demos::SharpEdges cx=cx.clone()/>
                </div>
            </div>
        </article>
    }
    .into_any()
}

/// Title, lede, a few numbers, and the request's path through the stack.
#[component]
fn Intro(cx: Arc<Ctx>) -> impl IntoView {
    let lang = cx.lang;
    let num = cx.num;
    let n_features = crate::format::count(FEATURES.len() as u32, num);
    let steps: [(&str, (&str, &str), (&str, &str), &str); 5] = [
        (
            "globe",
            ("Browser", "المتصفح"),
            (
                "Asks for /showcase with its Host, cookies and Accept-Encoding.",
                "بيطلب ‎/showcase‎ ومعاه الـ Host والكوكيز و Accept-Encoding.",
            ),
            "GET /showcase",
        ),
        (
            "store",
            ("Axum", "Axum"),
            (
                "Resolves the tenant from Host, serves /media and the /__prefs, /__tenant, /__sim helpers, compresses.",
                "بيحدد المتجر من الـ Host، وبيقدّم ‎/media‎ و ‎/__prefs‎ و ‎/__tenant‎ و ‎/__sim‎، وبيضغط.",
            ),
            "tower-http · moka",
        ),
        (
            "layout-grid",
            ("leptos_axum", "leptos_axum"),
            (
                "Matches the route, runs server functions, provides context: AppState, RequestCtx, Ui.",
                "بيطابق الـ route، ويشغّل دوال السيرفر، ويوفّر الـ context: ‏AppState و RequestCtx و Ui.",
            ),
            "/api/* · SsrMode",
        ),
        (
            "zap",
            ("SSR stream", "بث الـ SSR"),
            (
                "Shell and <head> first; each <Suspense> streams in as its data resolves.",
                "الهيكل والـ <head> الأول؛ وكل <Suspense> بيوصل أول ما بياناته تخلص.",
            ),
            "OutOfOrder · InOrder",
        ),
        (
            "sparkles",
            ("Islands hydrate", "الجزر تصحى"),
            (
                "WASM loads on idle; only #[island]s wake up; the islands router takes over links and forms.",
                "الـ WASM بيحمّل لما المتصفح يفضى؛ والـ ‎#[island]‎ بس اللي بتصحى؛ والـ islands router بيمسك اللينكات والفورمز.",
            ),
            "hydrate_islands()",
        ),
    ];
    let mode = if cx.router_nav {
        tr(
            lang,
            "This HTML: an islands-router navigation (in-order).",
            "الـ HTML ده: تنقل بالـ islands router (بالترتيب).",
        )
    } else {
        tr(
            lang,
            "This HTML: a full page load (out-of-order stream).",
            "الـ HTML ده: تحميل كامل للصفحة (بث بأي ترتيب).",
        )
    };
    view! {
        <header class="sc-intro container">
            <p class="sc-kicker">
                <span class="mono" dir="ltr">"Leptos 0.8.22"</span>
                <span aria-hidden="true">" · "</span>
                <span>{tr(lang, "SSR + islands + islands router, on Axum", "SSR مع islands و islands router، على Axum")}</span>
            </p>
            <h1 class="sc-title">{tr(lang, "How this storefront is built", "إزاي المتجر ده متبني")}</h1>
            <p class="sc-lede">
                {tr(
                    lang,
                    "This page is the evaluation: every Leptos feature the storefront uses, explained in a sentence, demonstrated live, linked to where the app uses it, and quoted from the actual code. Nothing here is a mock-up.",
                    "الصفحة دي هي التقييم: كل خاصية في Leptos المتجر بيستخدمها، متشرحة في جملة، وليها ديمو حي، ولينك للمكان اللي الأبلكيشن بيستخدمها فيه، ومقتبسة من الكود الحقيقي. مفيش هنا حاجة شكلية.",
                )}
            </p>
            <ul class="sc-facts" role="list">
                <li><strong class="mono">{n_features}</strong><span>{tr(lang, "features, each with a demo", "خاصية، لكل واحدة ديمو")}</span></li>
                <li><strong class="mono">"0"</strong><span>{tr(lang, "bytes of JS for server components", "بايت JS لكومبوننتس السيرفر")}</span></li>
                <li>
                    {if cx.base_ms == 0 {
                        view! { <strong>{tr(lang, "Off", "مقفول")}</strong> }.into_any()
                    } else {
                        view! { <strong class="mono">{crate::format::count(cx.base_ms as u32, num)}<small>" ms"</small></strong> }.into_any()
                    }}
                    <span>{tr(lang, "simulated backend latency (yours — see Streaming)", "زمن الباك إند التجريبي (بتاعك — شوف البث)")}</span>
                </li>
                <li><strong>{cx.tenant.name.pick(lang.is_rtl()).to_owned()}</strong><span>{tr(lang, "tenant, from the Host header", "المتجر، من الـ Host")}</span></li>
            </ul>
            <figure class="sc-arch" aria-labelledby="sc-arch-cap">
                <figcaption id="sc-arch-cap" class="sc-arch__cap">
                    <span>{tr(lang, "One request, end to end", "طلب واحد من أوله لآخره")}</span>
                    <span class="sc-arch__mode">{mode}</span>
                </figcaption>
                <ol class="sc-arch__steps" role="list">
                    {steps.into_iter().enumerate().map(|(i, (icon, t, d, code))| view! {
                        <li class="sc-arch__step">
                            <span class="sc-arch__n mono">{crate::format::count(i as u32 + 1, num)}</span>
                            <span class="sc-arch__icon"><Icon name=icon size=20/></span>
                            <strong class="sc-arch__t">{tr(lang, t.0, t.1)}</strong>
                            <span class="sc-arch__d">{tr(lang, d.0, d.1)}</span>
                            <code class="sc-arch__code mono" dir="ltr">{code}</code>
                        </li>
                    }).collect_view()}
                </ol>
            </figure>
            <noscript>
                <p class="sc-noscript">
                    <Icon name="info" size=18/>
                    <span>{tr(
                        lang,
                        "JavaScript is off. Links, filters and the forms still work; the out-of-order streamed demos stay as placeholders (that is the trade-off the “SSR modes” card explains), and the browser-measured numbers can't be read.",
                        "الـ JavaScript مقفول. اللينكات والفلاتر والفورمز شغالة؛ بس الديموهات اللي بتتبعت بأي ترتيب هتفضل placeholders (ودي المقايضة اللي كارت «طرق الـ SSR» بيشرحها)، والأرقام اللي بتتقاس في المتصفح مش هتظهر.",
                    )}</span>
                </p>
            </noscript>
        </header>
    }
}

/// Sticky contents: category filter chips (island) + the feature list (server HTML, plain anchors).
#[component]
fn Toc(cx: Arc<Ctx>) -> impl IntoView {
    let lang = cx.lang;
    let num = cx.num;
    let mut chips = vec![islands::Chip {
        code: "all".into(),
        en: "All".into(),
        ar: "الكل".into(),
        count: FEATURES.len() as u32,
    }];
    chips.extend(Cat::ALL.into_iter().map(|c| {
        let (en, ar) = c.short();
        islands::Chip {
            code: c.code().into(),
            en: en.into(),
            ar: ar.into(),
            count: FEATURES.iter().filter(|f| f.cat == c).count() as u32,
        }
    }));
    let active =
        cx.q.cat
            .map(|c| c.code().to_owned())
            .unwrap_or_else(|| "all".into());
    let entries = move || {
        FEATURES
            .iter()
            .enumerate()
            .map(|(i, f)| {
                view! {
                    <li class="sc-toc__item" data-cat=f.cat.code()>
                        <a href=format!("#{}", f.id)>
                            <span class="sc-toc__n mono">{crate::format::digits(&format!("{:02}", i + 1), num)}</span>
                            <span>{f.title(lang)}</span>
                        </a>
                    </li>
                }
            })
            .collect_view()
    };
    let edges = move || {
        view! {
            <li class="sc-toc__item sc-toc__item--edges">
                <a href="#sharp-edges">
                    <Icon name="triangle-alert" size=16/>
                    <span>{tr(lang, "Sharp edges we hit", "حاجات جرحتنا")}</span>
                </a>
            </li>
        }
    };
    view! {
        <h2 id="sc-toc-title" class="sc-toc__title">{tr(lang, "Contents", "المحتويات")}</h2>
        <div class="sc-toc__bar">
            // Phones: a dropdown next to the chips. Wide screens use the list below instead.
            <details class="sc-toc__menu">
                <summary class="sc-toc__sum">
                    <Icon name="list" size=18/>
                    <span>{tr(lang, "Contents", "المحتويات")}</span>
                    <Icon name="chevron-down" size=16 class="sc-toc__chev"/>
                </summary>
                <ol class="sc-toc__ol" role="list">{entries()}{edges()}</ol>
            </details>
            <islands::TocFilter chips active/>
        </div>
        <ol class="sc-toc__ol sc-toc__ol--side" role="list">{entries()}{edges()}</ol>
    }
}

/// One category: heading, a line, then its feature cards.
#[component]
fn Category(cat: Cat, cx: Arc<Ctx>) -> impl IntoView {
    let lang = cx.lang;
    let heading_id = format!("cat-{}", cat.code());
    let labelled_by = heading_id.clone();
    let cards = FEATURES
        .iter()
        .filter(|f| f.cat == cat)
        .map(|f| view! { <Card f cx=cx.clone()/> })
        .collect_view();
    view! {
        <section class="sc-cat" data-cat=cat.code() aria-labelledby=labelled_by>
            <header class="sc-cat__head">
                <h2 id=heading_id class="sc-cat__title">{cat.label(lang)}</h2>
                <p class="sc-cat__blurb">{cat.blurb(lang)}</p>
            </header>
            {cards}
        </section>
    }
}

/// The code each card quotes (one or two short excerpts).
fn cuts_for(id: &str) -> Vec<Cut> {
    match id {
        "islands" => vec![
            cut(Src::Home, "#[island]", "let left", 14),
            cut(Src::Frontend, "pub fn hydrate()", "^}", 6),
        ],
        "ssr-modes" => vec![cut(Src::Lib, "<Routes fallback", "</Routes>", 26)],
        "streaming" => vec![
            cut(
                Src::Home,
                "let data = Resource::new(|| (), move |_| api::home_rail",
                "</Suspense>",
                28,
            ),
            cut(Src::Product, "let data = Resource::new_blocking", "", 1),
        ],
        "errors" => vec![cut(
            Src::Errors,
            "pub fn SectionError(",
            ") -> impl IntoView",
            10,
        )],
        "meta" => vec![
            cut(
                Src::Home,
                "<Title text=title.clone()/>",
                "<Script type_",
                18,
            ),
            cut(
                Src::ProductHead,
                "<Meta property=\"product:price:amount\"",
                "<Script type_",
                8,
            ),
        ],
        "reactivity" => vec![
            cut(
                Src::ProductPanel,
                "let sel = RwSignal::new(data.selected);",
                "let image = Memo",
                8,
            ),
            cut(Src::Islands, "// A derived signal", "let total = Memo", 8),
        ],
        "shared-state" => vec![
            cut(
                Src::Client,
                "pub fn init()",
                "crate::stores::provide_all();",
                6,
            ),
            cut(Src::Stores, "pub struct CartStore", "^}", 10),
        ],
        "server-fns" => vec![cut(
            Src::ApiMod,
            "/// Invalidate every cached read",
            "^}",
            8,
        )],
        "transition" => vec![
            cut(
                Src::Explorer,
                "let page = AsyncDerived::new(move || {",
                "^    });",
                18,
            ),
            cut(Src::Explorer, "^<Transition", "})}", 9),
        ],
        "sim" => vec![cut(Src::Sim, "pub fn check_fail", "^}", 12)],
        "action-form" => vec![
            cut(Src::ApiCart, "/// Remove a line.", "^}", 9),
            cut(
                Src::CartPage,
                "<ActionForm action=remove",
                "</ActionForm>",
                6,
            ),
            cut(
                Src::Islands,
                "let action = ServerAction::<notes::LeaveNote>::new();",
                "let pending",
                16,
            ),
        ],
        "islands-router" => vec![
            cut(Src::Lib, "<HydrationScripts", "<HydrationScripts", 1),
            cut(
                Src::CartPage,
                "<For each=move || lines.clone() key=|l| l.key.clone()",
                "</For>",
                3,
            ),
            cut(Src::Product, "^<For", "key=|id| *id", 3),
        ],
        "router" => vec![
            cut(
                Src::Product,
                "let slug = use_params_map()",
                "let v = q.get(\"v\")",
                3,
            ),
            cut(Src::ApiTypes, "pub fn from_query", "\"sort\" =>", 14),
        ],
        "caching" => vec![cut(Src::Cache, "pub fn public(tags", "^}", 24)],
        "states" => vec![
            cut(
                Src::Errors,
                "pub fn NotFound()",
                "let back = same_origin_referer",
                16,
            ),
            cut(Src::Net, "// Offline: don't spend a try", "^}", 18),
        ],
        "router-patches" => vec![
            cut(
                Src::ShellJs,
                "// ── The button that submitted a form",
                "}, true);",
                16,
            ),
            cut(
                Src::ShellJs,
                "const doc = new DOMParser().parseFromString(html",
                "} catch (_) {}",
                12,
            ),
        ],
        "motion" => vec![
            cut(
                Src::Motion,
                "pub fn project(",
                "(overshoot * d * constant)",
                12,
            ),
            cut(
                Src::Gallery,
                "// A sample at release: a finger that rested",
                "Release::Swipe",
                26,
            ),
        ],
        "tenancy" => vec![
            cut(Src::Tenant, "pub fn tenant_from_host", "^}", 8),
            cut(
                Src::Request,
                "let (tenant_id, via) = match tenant_from_host",
                "};",
                10,
            ),
        ],
        "i18n" => vec![
            cut(Src::ServerMain, "async fn set_prefs", "^}", 18),
            cut(
                Src::Client,
                "// The language/numeral/theme controls",
                "on_change.forget();",
                40,
            ),
        ],
        "view-transitions" => vec![
            cut(
                Src::ShellJs,
                "// A product card: name only its image",
                "media.setAttribute(\"data-vt-named\", \"\");",
                9,
            ),
            cut(
                Src::Demos,
                "// Each tile keeps one `view-transition-name`",
                "}).collect_view()}",
                13,
            ),
        ],
        "images" => vec![cut(Src::Media, "pub fn srcset(&self)", "^    }", 9)],
        "a11y" => vec![
            cut(Src::Steps, "pub fn StepFocus()", "^}", 8),
            cut(Src::Islands, "<dialog id=\"sc-sheet\"", "</dialog>", 30),
        ],
        _ => vec![],
    }
}

/// One feature: number, title, what, why, the live demo, where it's used, sources, code.
#[component]
fn Card(f: &'static Feature, cx: Arc<Ctx>) -> impl IntoView {
    let lang = cx.lang;
    let num = cx.num;
    let n = data::number(f.id);
    let title_id = format!("{}-title", f.id);
    let labelled_by = title_id.clone();
    let used: Vec<(String, &'static str)> = f
        .used_in
        .iter()
        .filter_map(|u| cx.links.resolve(u.href).map(|h| (h, tr(lang, u.en, u.ar))))
        .collect();
    let demo = demos::demo(f.id, cx.clone());
    view! {
        <article id=f.id class="sc-card" data-cat=f.cat.code() aria-labelledby=labelled_by>
            <header class="sc-card__head">
                <span class="sc-card__n mono">{crate::format::digits(&format!("{n:02}"), num)}</span>
                <h3 id=title_id class="sc-card__title">{f.title(lang)}</h3>
                <span class="sc-card__cat">{f.cat.label(lang)}</span>
            </header>
            <p class="sc-card__what">{tr(lang, f.what.0, f.what.1)}</p>
            <p class="sc-card__why">
                <strong>{tr(lang, "Why it matters here", "ليه مهمة هنا")}</strong>
                <span>{tr(lang, f.why.0, f.why.1)}</span>
            </p>
            <div class="sc-demo">
                <p class="sc-demo__label">
                    <span class="sc-live" aria-hidden="true"></span>
                    {tr(lang, "Live demo", "ديمو حي")}
                </p>
                {demo}
            </div>
            <footer class="sc-card__foot">
                <div class="sc-used">
                    <span class="sc-foot__label">{tr(lang, "Used in", "مستخدمة في")}</span>
                    {if used.is_empty() && f.note.is_some() {
                        // The note says why the demo is the only usage.
                        ().into_any()
                    } else if used.is_empty() {
                        view! { <span class="sc-used__none">{tr(lang, "This demo is its only usage so far.", "الديمو ده هو الاستخدام الوحيد لحد دلوقتي.")}</span> }.into_any()
                    } else {
                        view! {
                            <ul class="sc-used__list" role="list">
                                {used.into_iter().map(|(href, label)| view! {
                                    <li><a class="sc-used__link" href=href>
                                        <span>{label}</span>
                                        <Icon name="arrow-up-right" size=14 class="flip-rtl"/>
                                    </a></li>
                                }).collect_view()}
                            </ul>
                        }.into_any()
                    }}
                    {f.note.map(|(en, ar)| view! { <p class="sc-used__note">{tr(lang, en, ar)}</p> })}
                </div>
                <div class="sc-src">
                    <span class="sc-foot__label">{tr(lang, "Source", "الكود")}</span>
                    <ul class="sc-src__list" role="list">
                        {f.sources.iter().map(|s| view! { <li><code class="mono" dir="ltr">{*s}</code></li> }).collect_view()}
                    </ul>
                </div>
                {cuts_for(f.id).into_iter().map(|c| view! { <Code cut=c/> }).collect_view()}
            </footer>
        </article>
    }
}
