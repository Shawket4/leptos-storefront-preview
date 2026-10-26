//! `/showcase`: a guided tour of the storefront as built. What Leptos does on the server, what the
//! TypeScript client does, and why the WASM islands went. Each card has what it is, why it matters
//! here, a live demo, where the app uses it, and short excerpts of the real source, cut at compile
//! time (`showcase/snippets.rs`) so they can't go stale.
//!
//! Everything here is server-rendered (in order). Demos that need the browser are small TypeScript
//! features (`client/src/features/showcase.ts`) mounted on this markup with `data-ts="sc-…"`. The
//! page is `private, no-store`: it shows the visitor's own latency settings and live cache counters.

mod data;
mod demos;
mod snippets;
mod stream;

pub use stream::StreamDemoPage;

use crate::api::types::CacheInfo;
use crate::components::icons::Icon;
use crate::pages::errors::ErrorPage;
use crate::prefs::{Lang, Numerals, tr};
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
    pub cat: Option<Cat>,
    /// `?crash=1|500|503`: the full-page error view (a route-level failure).
    pub crash: Option<u16>,
}

impl Query {
    fn parse(path_and_query: &str) -> Query {
        let raw = path_and_query.split_once('?').map(|(_, q)| q).unwrap_or("");
        let mut q = Query::default();
        for pair in raw.split('&').filter(|p| !p.is_empty()) {
            let (k, v) = pair.split_once('=').unwrap_or((pair, ""));
            match k {
                "cat" => q.cat = Cat::parse(v),
                "crash" => {
                    q.crash = match v {
                        "" | "0" => None,
                        "503" => Some(503),
                        _ => Some(500),
                    }
                }
                _ => {}
            }
        }
        q
    }
}

/// Links that depend on the tenant's catalogue.
#[derive(Clone, Debug, Default)]
pub(crate) struct Links {
    pub category: Option<String>,
    pub product: Option<String>,
}

impl Links {
    pub fn resolve(&self, h: Href) -> Option<String> {
        match h {
            Href::Path(p) => Some(p.to_owned()),
            Href::Category => self.category.as_ref().map(|c| format!("/c/{c}")),
            Href::Product => self.product.as_ref().map(|p| format!("/p/{p}")),
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
    /// (name, href, another origin?)
    pub other: Option<(L10n, String, bool)>,
}

/// A real product, with everything the cart and template demos show already formatted.
#[derive(Clone, Debug, Default)]
pub(crate) struct Sample {
    pub product: u32,
    pub variant: u32,
    pub name: String,
    pub price: String,
    pub img: String,
    pub href: String,
}

#[derive(Clone, Debug)]
pub(crate) struct Ctx {
    pub lang: Lang,
    pub num: Numerals,
    pub path_and_query: String,
    pub q: Query,
    pub tenant: TenantFacts,
    pub base_ms: u64,
    /// The visitor set the latency with the `sim` cookie (else it's the server default).
    pub sim_cookie: bool,
    pub fail_cookie: Vec<String>,
    pub links: Links,
    pub cache: CacheInfo,
    /// The cart demo's product (in stock, one variant).
    pub sample: Option<Sample>,
    /// Rows for the template demo.
    pub rows: Vec<Sample>,
}

impl Ctx {
    /// `/showcase…#anchor`, URL-encoded for `next=` parameters.
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
    let (lang, num) = (rq.prefs.lang, rq.prefs.numerals);
    let ar = lang.is_rtl();
    let cats = cat.nav().to_vec();
    let multi = cat
        .products()
        .iter()
        .find(|p| p.variants.len() > 1 && p.variants.iter().filter(|v| v.stock > 0).count() > 1);
    let links = Links {
        category: cats.iter().find(|c| c.count > 0).map(|c| c.slug.clone()),
        product: multi.or(cat.products().first()).map(|p| p.slug.clone()),
    };
    // In-stock, single-variant products with images, one per category when possible.
    let mut rows: Vec<Sample> = Vec::new();
    for c in &cats {
        if rows.len() == 3 {
            break;
        }
        let page = cat.listing(&ListingQuery { category: Some(c.slug.clone()), page: 1, ..Default::default() });
        if let Some(card) = page.items.into_iter().find(|c| c.in_stock && c.single_variant && !c.img.is_missing()) {
            rows.push(Sample {
                product: card.id.0,
                variant: card.default_variant.0,
                name: card.name.pick(ar).to_owned(),
                price: crate::format::money(card.price, lang, num),
                img: card.img.src_near(160),
                href: format!("/p/{}", card.slug),
            });
        }
    }
    let stats = st.reads().stats();
    let other = crate::tenant::TENANT_IDS.iter().copied().find(|id| *id != t.id).map(|id| {
        let other = st.tenant(id);
        match (rq.via, crate::tenant::swap_tenant_label(&rq.host, id)) {
            (TenantVia::Subdomain, Some(host)) => {
                let scheme = rq.origin.split("://").next().unwrap_or("http");
                (other.name.clone(), format!("{scheme}://{host}/showcase#tenancy"), true)
            }
            _ => (other.name.clone(), format!("/__tenant/{id}?next=%2Fshowcase%23tenancy"), false),
        }
    });
    Ctx {
        lang,
        num,
        path_and_query: crate::pages::errors::strip_param(&rq.path_and_query, "__err"),
        q,
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
        fail_cookie: rq_cookie("fail")
            .map(|v| v.split(',').filter(|s| !s.is_empty()).map(str::to_owned).collect())
            .unwrap_or_default(),
        links,
        cache: CacheInfo {
            entries: stats.entries,
            hits: stats.hits,
            misses: stats.misses,
            generation: st.reads().generation(&t.id),
            base_latency_ms: crate::server::sim::base_ms(),
        },
        sample: rows.first().cloned(),
        rows,
    }
}

#[cfg(feature = "ssr")]
fn rq_cookie(name: &str) -> Option<String> {
    let parts = use_context::<http::request::Parts>()?;
    let header = parts.headers.get(http::header::COOKIE)?.to_str().ok()?;
    crate::request::parse_cookies(header).find(|(k, _)| *k == name).map(|(_, v)| v.to_owned())
}

/// Only the server renders this page; this keeps `cargo check -p app` (no features) compiling.
#[cfg(not(feature = "ssr"))]
fn build_ctx(q: Query) -> Ctx {
    let rq = req();
    Ctx {
        lang: rq.prefs.lang,
        num: rq.prefs.numerals,
        path_and_query: rq.path_and_query.clone(),
        q,
        tenant: TenantFacts::default(),
        base_ms: 0,
        sim_cookie: false,
        fail_cookie: vec![],
        links: Links::default(),
        cache: CacheInfo::default(),
        sample: None,
        rows: vec![],
    }
}

// ─── The page ──────────────────────────────────────────────────────────────────────────────────

#[component]
pub fn ShowcasePage() -> impl IntoView {
    // Personal (sim cookies, live counters): never cached. Set before the first byte.
    #[cfg(feature = "ssr")]
    crate::server::cache::private();

    let rq = req();
    let q = Query::parse(&rq.path_and_query);
    let lang = rq.prefs.lang;
    let ar = lang.is_rtl();
    let title = format!(
        "{} · {}",
        tr(lang, "How this storefront is built", "المتجر ده متبني إزاي"),
        rq.tenant.name.pick(ar)
    );
    let description = tr(
        lang,
        "A guided tour of the storefront as built: Leptos rendering every page on the server, a small TypeScript client for state, and why the WASM islands went. Live demos, real code.",
        "جولة في المتجر زي ما هو متبني: Leptos بيرسم كل صفحة على السيرفر، وعميل TypeScript صغير للحالة، وليه جزر الـ WASM اتشالت. ديموهات حية وكود حقيقي.",
    );

    // A route-level failure, decided before the first byte: the status is a real 500 / 503.
    if let Some(code) = q.crash {
        return view! {
            <ErrorBoundary fallback=move |errors| view! { <ErrorPage errors unavailable={code == 503}/> }>
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

        // Rows the demos add (filled by TypeScript, see features/showcase.ts).
        <template id="tpl-sc-logline"><li data-slot="text"></li></template>
        <template id="tpl-sc-kv">
            <div class="sc-kv"><dt class="mono" dir="ltr" data-slot="k"></dt><dd dir="auto" data-slot="v"></dd></div>
        </template>
        <article id="sc" class="sc" data-filter=filter data-ts="sc-filter">
            <Intro cx=cx.clone()/>
            <div class="sc-layout container">
                <aside class="sc-toc" aria-labelledby="sc-toc-title">
                    <Toc cx=cx.clone()/>
                </aside>
                <div id="features" class="sc-features">
                    {Cat::ALL.into_iter().map(|cat| view! { <Category cat cx=cx.clone()/> }).collect_view()}
                </div>
            </div>
        </article>
    }
    .into_any()
}

/// Title, lede, a few numbers, and one request's path through the stack.
#[component]
fn Intro(cx: Arc<Ctx>) -> impl IntoView {
    let lang = cx.lang;
    let num = cx.num;
    let js_kb = crate::format::dec1(crate::client_js::APP_JS.len() as f32 / 1024.0, num);
    let steps: [(&str, (&str, &str), (&str, &str), &str); 5] = [
        (
            "globe",
            ("Browser", "المتصفح"),
            ("Asks for /showcase with its Host and cookies.", "بيطلب ‎/showcase‎ ومعاه الـ Host والكوكيز."),
            "GET /showcase",
        ),
        (
            "store",
            ("Axum", "Axum"),
            (
                "Picks the shop from the Host, serves /js, /media and the /__prefs and /__sim helpers, and compresses.",
                "بيحدد المحل من الـ Host، وبيقدّم ‎/js‎ و ‎/media‎ وأدوات ‎/__prefs‎ و ‎/__sim‎، وبيضغط.",
            ),
            "tower-http · moka",
        ),
        (
            "layout-grid",
            ("Leptos SSR", "Leptos SSR"),
            (
                "Renders the route in order into one complete document; status and headers go first.",
                "بيرسم الـ route بالترتيب في مستند واحد كامل؛ وكود الحالة والهيدرز بيخرجوا الأول.",
            ),
            "SsrMode::InOrder",
        ),
        (
            "package",
            ("Page data", "بيانات الصفحة"),
            (
                "#app-config and #app-strings: the shop, the language, the endpoints and every UI string the client needs.",
                "‏#app-config و #app-strings: المحل واللغة والـ endpoints وكل نصوص الواجهة اللي العميل محتاجها.",
            ),
            "application/json",
        ),
        (
            "zap",
            ("TypeScript", "TypeScript"),
            (
                "One module boots: read the page, sync the stores, mount the features, start the router.",
                "ملف واحد بيبدأ: يقرا الصفحة، ويظبط الـ stores، ويركّب الـ features، ويشغّل الراوتر.",
            ),
            "/js/app-….js",
        ),
    ];
    let n_cards = crate::format::count(FEATURES.len() as u32, num);
    view! {
        <header class="sc-intro container">
            <p class="sc-kicker">
                <span class="mono" dir="ltr">"Leptos 0.8.22"</span>
                <span aria-hidden="true">" · "</span>
                <span>{tr(lang, "SSR on Axum, a TypeScript client, no WASM", "SSR على Axum، وعميل TypeScript، ومن غير WASM")}</span>
            </p>
            <h1 class="sc-title">{tr(lang, "How this storefront is built", "المتجر ده متبني إزاي")}</h1>
            <p class="sc-lede">
                {tr(
                    lang,
                    "Leptos renders every page on the server. A small TypeScript client changes state in the browser and asks the server only for data it doesn't have. Each card below says what a piece does and why, runs it live, links to where the shop uses it, and quotes the code.",
                    "‏Leptos بيرسم كل صفحة على السيرفر. وعميل TypeScript صغير بيغيّر الحالة في المتصفح ومابيطلبش من السيرفر غير البيانات اللي مش عنده. كل كارت تحت بيقول الجزء ده بيعمل إيه وليه، وبيشغّله قدامك، وبيوديك للمكان اللي المتجر بيستخدمه فيه، وبيعرض الكود.",
                )}
            </p>
            <ul class="sc-facts" role="list">
                <li><strong class="mono">{n_cards}</strong><span>{tr(lang, "cards, each with a live demo", "كارت، وفي كل واحد ديمو حي")}</span></li>
                <li><strong class="mono">{crate::format::count(0, num)}</strong><span>{tr(lang, "bytes of WebAssembly", "بايت WebAssembly")}</span></li>
                <li>
                    <strong class="mono">{js_kb}<small>" KB"</small></strong>
                    <span>{if cfg!(debug_assertions) {
                        tr(lang, "of JavaScript, raw (this dev build keeps it readable, with a source map)", "JavaScript من غير ضغط (نسخة التطوير دي مقروءة ومعاها source map)")
                    } else {
                        tr(lang, "of JavaScript, raw, minified", "JavaScript من غير ضغط، مصغّر")
                    }}</span>
                </li>
                <li><strong>{cx.tenant.name.pick(lang.is_rtl()).to_owned()}</strong><span>{tr(lang, "shop, picked from the Host header", "المحل، من الـ Host")}</span></li>
            </ul>
            <figure class="sc-arch" aria-labelledby="sc-arch-cap">
                <figcaption id="sc-arch-cap" class="sc-arch__cap">
                    <span>{tr(lang, "One request, end to end", "طلب واحد من أوله لآخره")}</span>
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
                        "JavaScript is off. Every page of the shop still works, and so does every link and form here. The live demos that run in the browser show their server-rendered state only.",
                        "الـ JavaScript مقفول. كل صفحات المتجر شغالة، وكل اللينكات والفورمز هنا كمان. بس الديموهات اللي بتشتغل في المتصفح هتعرض اللي السيرفر رسمه بس.",
                    )}</span>
                </p>
            </noscript>
        </header>
    }
}

/// Sticky contents: part filter chips (links; `sc-filter` applies them in place) and the list.
#[component]
fn Toc(cx: Arc<Ctx>) -> impl IntoView {
    let lang = cx.lang;
    let num = cx.num;
    let active = cx.q.cat.map(|c| c.code()).unwrap_or("all");
    let mut chips: Vec<(&'static str, &'static str, u32)> =
        vec![("all", tr(lang, "All", "الكل"), FEATURES.len() as u32)];
    chips.extend(Cat::ALL.into_iter().map(|c| {
        (c.code(), c.label(lang), FEATURES.iter().filter(|f| f.cat == c).count() as u32)
    }));
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
    view! {
        <h2 id="sc-toc-title" class="sc-toc__title">{tr(lang, "Contents", "المحتويات")}</h2>
        <div class="sc-toc__bar">
            // Phones: a dropdown beside the chips. Wide screens use the list below instead.
            <details class="sc-toc__menu">
                <summary class="sc-toc__sum">
                    <Icon name="list" size=18/>
                    <span>{tr(lang, "Contents", "المحتويات")}</span>
                    <Icon name="chevron-down" size=16 class="sc-toc__chev"/>
                </summary>
                <ol class="sc-toc__ol" role="list">{entries()}</ol>
            </details>
            <nav class="sc-filter" aria-label=tr(lang, "Show a part", "اعرض جزء")>
                {chips.into_iter().map(|(code, label, n)| {
                    let href = if code == "all" { "/showcase#features".to_owned() } else { format!("/showcase?cat={code}#features") };
                    view! {
                        <a class="sc-chip" href=href data-cat-chip=code aria-current=if code == active { "true" } else { "false" }>
                            <span>{label}</span>
                            <span class="sc-chip__n mono">{crate::format::count(n, num)}</span>
                        </a>
                    }
                }).collect_view()}
            </nav>
        </div>
        <ol class="sc-toc__ol sc-toc__ol--side" role="list">{entries()}</ol>
    }
}

/// One part: heading, a line, then its cards.
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

/// The code each card quotes (one or two short excerpts of the real files).
fn cuts_for(id: &str) -> Vec<Cut> {
    match id {
        "in-order" => vec![
            cut(Src::Lib, "<Routes fallback", "</Routes>", 22),
            cut(Src::Shell, "let config = serde_json::to_string(&config)", "^    }", 6),
        ],
        "streaming" => vec![
            cut(Src::Lib, "path!(\"showcase/stream\")", "", 1),
            cut(Src::Stream, "let data = Resource::new(", "</Suspense>", 26),
        ],
        "server-fns" => vec![
            cut(Src::ApiMod, "#[server(endpoint = \"cache/stats\"", "^}", 14),
            cut(Src::Net, "export async function getJSON", "^}", 8),
        ],
        "errors" => vec![
            cut(Src::Errors, "pub fn SectionError(", ") -> impl IntoView", 8),
            cut(Src::Sim, "pub fn check_fail", "^}", 10),
        ],
        "read-cache" => vec![
            cut(Src::Cache, "let full = format!(", ".await;", 12),
            cut(Src::Cache, "pub fn bump(", "^    }", 3),
        ],
        "tenancy" => vec![
            cut(Src::Tenant, "pub fn tenant_from_host", "^}", 8),
            cut(Src::Request, "let (tenant_id, via) = match tenant_from_host", "};", 12),
        ],
        "meta" => vec![cut(Src::ProductHead, "<Title", "<Script", 14)],
        "sim" => vec![
            cut(Src::Sim, "pub fn base_ms()", "^}", 7),
            cut(Src::Sim, "pub async fn delay(", "^}", 6),
        ],
        "router" => vec![
            cut(Src::Router, "const PAGE_CACHE = {", "", 1),
            cut(Src::Router, "// ── Prefetch: Madar's gesture model", "const HOVER_DELAY_MS", 12),
            cut(Src::Main, "readPage();", "});", 11),
        ],
        "stores" => vec![
            cut(Src::Store, "export function createStore", "^}", 19),
            cut(Src::Toasts, "export function toast(", "^}", 6),
        ],
        "cart" => vec![
            cut(Src::CartTs, "function commit(", "^}", 8),
            cut(Src::DomainCart, "pub fn parse(raw: &str)", "^}", 22),
            cut(Src::CartTs, "export function quote(", "^}", 4),
        ],
        "templates" => vec![
            cut(Src::Dom, "export function fill(", "^}", 19),
            cut(Src::Toast, "<template id=\"tpl-toast\">", "</template>", 16),
        ],
        "data-cache" => vec![cut(Src::ClientCache, "export async function cached", "^}", 15)],
        "springs" => vec![
            cut(Src::Motion, "export const project", "};", 9),
            cut(Src::ShowcaseTs, "// Release: hand the finger's velocity", "^  };", 12),
        ],
        "bundle" => vec![
            cut(Src::BuildRs, "node(&[\"node_modules/typescript/bin/tsc\"", "node(&args);", 7),
            cut(Src::BuildMjs, "await build({", "});", 13),
            cut(Src::ClientJs, "pub const APP_JS", "", 2),
        ],
        _ => vec![],
    }
}

/// Prose with `{{n}}` marks: the numbers in the visitor's numerals, the rest as written.
pub(crate) fn nums(text: &str, num: Numerals) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(i) = rest.find("{{") {
        out.push_str(&rest[..i]);
        let after = &rest[i + 2..];
        match after.find("}}") {
            Some(j) => {
                out.push_str(&crate::format::digits(&after[..j], num));
                rest = &after[j + 2..];
            }
            None => {
                out.push_str(&rest[i..]);
                rest = "";
            }
        }
    }
    out.push_str(rest);
    out
}

/// One card: number, title, what, why, the live demo, where it's used, sources, code.
#[component]
fn Card(f: &'static Feature, cx: Arc<Ctx>) -> impl IntoView {
    let lang = cx.lang;
    let num = cx.num;
    let n = data::number(f.id);
    let title_id = format!("{}-title", f.id);
    let labelled_by = title_id.clone();
    let used: Vec<(String, String)> = f
        .used_in
        .iter()
        .filter_map(|u| cx.links.resolve(u.href).map(|h| (h, nums(tr(lang, u.en, u.ar), num))))
        .collect();
    let demo = demos::demo(f.id, cx.clone());
    view! {
        <article id=f.id class="sc-card" data-cat=f.cat.code() aria-labelledby=labelled_by>
            <header class="sc-card__head">
                <span class="sc-card__n mono">{crate::format::digits(&format!("{n:02}"), num)}</span>
                <h3 id=title_id class="sc-card__title">{f.title(lang)}</h3>
                <span class="sc-card__cat">{f.cat.label(lang)}</span>
            </header>
            <p class="sc-card__what">{nums(tr(lang, f.what.0, f.what.1), num)}</p>
            <p class="sc-card__why">
                <strong>{tr(lang, "Why it matters here", "ليه ده مهم هنا")}</strong>
                <span>{nums(tr(lang, f.why.0, f.why.1), num)}</span>
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
                    <span class="sc-foot__label">{tr(lang, "Used in", "مستخدم في")}</span>
                    {(!used.is_empty()).then(|| view! {
                        <ul class="sc-used__list" role="list">
                            {used.into_iter().map(|(href, label)| view! {
                                <li><a class="sc-used__link" href=href>
                                    <span>{label}</span>
                                    <Icon name="arrow-up-right" size=14 class="flip-rtl"/>
                                </a></li>
                            }).collect_view()}
                        </ul>
                    })}
                    {f.note.map(|(en, ar)| view! { <p class="sc-used__note">{nums(tr(lang, en, ar), num)}</p> })}
                </div>
                <div class="sc-src">
                    <span class="sc-foot__label">{tr(lang, "Source", "الكود")}</span>
                    <ul class="sc-src__list" role="list">
                        {f.sources.iter().map(|s| view! { <li><code class="mono" dir="ltr">{*s}</code></li> }).collect_view()}
                    </ul>
                </div>
                {cuts_for(f.id).into_iter().map(|c| view! { <Code cut=c lang/> }).collect_view()}
            </footer>
        </article>
    }
}
