//! The server-rendered part of each card's live demo. Parts that run in the browser are TypeScript
//! features (`client/src/features/showcase.ts`) mounted on this markup by `data-ts="sc-…"`; their
//! inputs are `data-*` attributes, and what they show before JS runs is rendered here.

use super::{Ctx, Sample, nums};
use crate::api::{self, types::Rail};
use crate::components::button::{Button, ButtonKind, ButtonSize};
use crate::components::icons::Icon;
use crate::components::segmented::{SegItem, Segmented, SegmentedToggle};
use crate::format;
use crate::pages::errors::SectionError;
use crate::prefs::{Lang, Numerals, tr};
use crate::tenant::TenantVia;
use leptos::prelude::*;
use leptos::server_fn::ServerFn;
use std::sync::Arc;

/// The demo for a feature id.
pub fn demo(id: &str, cx: Arc<Ctx>) -> AnyView {
    match id {
        "in-order" => view! { <InOrderDemo cx/> }.into_any(),
        "streaming" => view! { <StreamingDemo cx/> }.into_any(),
        "server-fns" => view! { <ServerFnsDemo cx/> }.into_any(),
        "errors" => view! { <ErrorsDemo cx/> }.into_any(),
        "read-cache" => view! { <ReadCacheDemo cx/> }.into_any(),
        "tenancy" => view! { <TenancyDemo cx/> }.into_any(),
        "meta" => view! { <MetaDemo cx/> }.into_any(),
        "sim" => view! { <SimDemo cx/> }.into_any(),
        "router" => view! { <RouterDemo cx/> }.into_any(),
        "stores" => view! { <StoresDemo cx/> }.into_any(),
        "cart" => view! { <CartDemo cx/> }.into_any(),
        "templates" => view! { <TemplatesDemo cx/> }.into_any(),
        "data-cache" => view! { <DataCacheDemo cx/> }.into_any(),
        "springs" => view! { <SpringsDemo cx/> }.into_any(),
        "bundle" => view! { <BundleDemo cx/> }.into_any(),
        "why-not-islands" => view! { <IslandsDemo cx/> }.into_any(),
        _ => ().into_any(),
    }
}

fn stats_path() -> &'static str {
    <api::CacheStats as ServerFn>::PATH
}

fn bump_path() -> &'static str {
    <api::CacheBump as ServerFn>::PATH
}

/// Off / 350 / 1200 ms, as links to `/__sim` that come back to `next` (URL-encoded).
#[component]
pub(super) fn LatencyLinks(
    lang: Lang,
    num: Numerals,
    current: u64,
    next: String,
    /// A full page load afterwards (to watch out-of-order streaming).
    #[prop(optional)]
    router_off: bool,
) -> impl IntoView {
    let items: Vec<SegItem> = [0u64, 350, 1200]
        .into_iter()
        .map(|v| {
            let label = if v == 0 {
                tr(lang, "Off", "مقفول").to_owned()
            } else {
                format!("{} ms", format::int(v as i64, num))
            };
            let item = SegItem::new(label, format!("/__sim?ms={v}&next={next}"), v == current);
            if router_off { item.attr("data-router", "off") } else { item }
        })
        .collect();
    view! { <Segmented items label=tr(lang, "Simulated latency", "التأخير التجريبي")/> }
}

/// A key/value row; with `out`, its value is a `[data-slot]` a TS feature updates (`patch()`).
#[component]
fn Kv(#[prop(into)] k: String, #[prop(into)] v: String, #[prop(optional)] out: &'static str) -> impl IntoView {
    let slot = (!out.is_empty()).then_some(out);
    view! {
        <div class="sc-kv"><dt>{k}</dt><dd class="mono" dir="auto" data-slot=slot>{v}</dd></div>
    }
}

// ─── In-order SSR ──────────────────────────────────────────────────────────────────────────────

/// Every route and its rendering mode, read from lib.rs at compile time (so this can't drift).
fn route_modes() -> Vec<(String, String)> {
    #[cfg(feature = "ssr")]
    {
        include_str!("../../lib.rs")
            .lines()
            .map(str::trim)
            .filter(|l| l.starts_with("<Route path=path!(\""))
            .filter_map(|l| {
                let path = l.split("path!(\"").nth(1)?.split('"').next()?;
                let mode = match l.split("SsrMode::").nth(1) {
                    Some(m) => m.split(|c: char| !c.is_ascii_alphanumeric()).next().unwrap_or("").to_owned(),
                    None => "OutOfOrder (default)".to_owned(),
                };
                Some((format!("/{path}"), mode))
            })
            .collect()
    }
    #[cfg(not(feature = "ssr"))]
    Vec::new()
}

#[component]
fn InOrderDemo(cx: Arc<Ctx>) -> impl IntoView {
    let lang = cx.lang;
    view! {
        <div class="sc-routes" role="table" aria-label=tr(lang, "Routes and how they render", "الـ routes وبتترسم إزاي")>
            <div class="sc-routes__row sc-routes__row--head" role="row">
                <span role="columnheader">{tr(lang, "Route", "الـ route")}</span>
                <span role="columnheader">{tr(lang, "Rendering", "الرسم")}</span>
            </div>
            {route_modes().into_iter().map(|(path, mode)| {
                let ooo = mode != "InOrder";
                view! {
                    <div class="sc-routes__row" role="row" class:is-ooo=ooo>
                        <code role="cell" class="mono" dir="ltr">{path}</code>
                        <code role="cell" class="mono" dir="ltr">{mode}</code>
                    </div>
                }
            }).collect_view()}
        </div>
        <p class="sc-note">{tr(lang, "Read from crates/app/src/lib.rs when the server was built.", "متقري من crates/app/src/lib.rs وقت بناء السيرفر.")}</p>
        <dl class="sc-kvs" data-ts="sc-doc">
            <Kv k=tr(lang, "This page's first byte", "أول بايت من الصفحة دي") v="—" out="ttfb"/>
            <Kv k=tr(lang, "Last byte", "آخر بايت") v="—" out="end"/>
            <Kv k=tr(lang, "HTML, as sent and unpacked", "الـ HTML، مضغوط ومفكوك") v="—" out="bytes"/>
            <Kv k=tr(lang, "Script needed to see the page", "السكريبت اللازم عشان تشوف الصفحة") v=tr(lang, "None", "ولا حاجة")/>
        </dl>
        <p class="sc-note">{tr(lang, "Timings are the browser's own (Navigation Timing) for the document this tab loaded first.", "التوقيتات من المتصفح نفسه (Navigation Timing)، للمستند اللي التاب ده حمّله الأول.")}</p>
    }
}

// ─── Out-of-order streaming ────────────────────────────────────────────────────────────────────

#[component]
fn StreamingDemo(cx: Arc<Ctx>) -> impl IntoView {
    let lang = cx.lang;
    let num = cx.num;
    let next = crate::api::types::url_encode("/showcase/stream");
    view! {
        <div class="sc-callout" class:is-off={cx.base_ms == 0}>
            <p class="sc-callout__text">
                <Icon name="timer" size=18/>
                <span><strong>{latency_state(&cx)}</strong></span>
            </p>
            <LatencyLinks lang num current=cx.base_ms next router_off=true/>
        </div>
        <p class="sc-actions">
            <Button href="/showcase/stream" icon="zap" attr:data-router="off">
                {tr(lang, "Open the streaming demo", "افتح ديمو البث")}
            </Button>
            <a class="sc-btnlink" href="/showcase/stream?fail=deals" data-router="off">
                <Icon name="triangle-alert" size=16/>
                <span>{tr(lang, "With the deals failing", "والعروض فاشلة")}</span>
            </a>
        </p>
        <p class="sc-note">{tr(
            lang,
            "Both open as a full page load (data-router=\"off\"). The client router would fetch the page as HTML and get it only after the slowest block, so you'd see nothing stream.",
            "الاتنين بيفتحوا تحميل كامل للصفحة (data-router=\"off\"). راوتر العميل كان هيجيب الصفحة كـ HTML وياخدها بعد أبطأ بلوك، فمش هتشوف أي بث.",
        )}</p>
    }
}

fn latency_state(cx: &Ctx) -> String {
    let lang = cx.lang;
    if cx.base_ms == 0 {
        return tr(
            lang,
            "Simulated latency is off: the blocks would all arrive at once. Pick a latency first.",
            "التأخير التجريبي مقفول: البلوكات كلها هتوصل مع بعض. اختار تأخير الأول.",
        )
        .to_owned();
    }
    let ms = format::int(cx.base_ms as i64, cx.num);
    let source = if cx.sim_cookie { tr(lang, "your cookie", "الكوكي بتاعتك") } else { tr(lang, "server default", "الافتراضي بتاع السيرفر") };
    match lang {
        Lang::En => format!("Simulated latency: {ms} ms ({source})."),
        Lang::Ar => format!("التأخير التجريبي: {ms} مللي ثانية (‏{source})."),
    }
}

// ─── Server functions ──────────────────────────────────────────────────────────────────────────

#[component]
fn ServerFnsDemo(cx: Arc<Ctx>) -> impl IntoView {
    let lang = cx.lang;
    let stats = stats_path();
    view! {
        <div class="sc-api" data-ts="sc-api" data-url=stats>
            <p class="sc-row">
                <Button kind=ButtonKind::Secondary icon="zap" attr:data-act="call" attr:hidden="">
                    {format!("GET {stats}")}
                </Button>
                <a class="sc-link" href=stats data-router="off" data-nojs="">
                    <span>{tr(lang, "Open the JSON", "افتح الـ JSON")}</span>
                    <Icon name="arrow-up-right" size=14 class="flip-rtl"/>
                </a>
            </p>
            <dl class="sc-kvs">
                <Kv k=tr(lang, "Status", "الحالة") v="—" out="status"/>
                <Kv k=tr(lang, "Time", "الوقت") v="—" out="ms"/>
                <Kv k="Cache-Control" v="—" out="cc"/>
            </dl>
            <pre class="sc-json mono" dir="ltr" data-out="body" tabindex="0">"{ … }"</pre>
            <p class="sc-note">{tr(lang, "The endpoints this page's client may call, from #app-config:", "الـ endpoints اللي عميل الصفحة دي يقدر يكلمها، من ‎#app-config‎:")}</p>
            <ul class="sc-endpoints" role="list" data-out="endpoints">
                <li><code class="mono" dir="ltr">{format!("GET {stats}")}</code></li>
                <li><code class="mono" dir="ltr">{format!("POST {}", bump_path())}</code></li>
            </ul>
        </div>
    }
}

// ─── Error boundaries ──────────────────────────────────────────────────────────────────────────

#[component]
fn ErrorsDemo(cx: Arc<Ctx>) -> impl IntoView {
    let lang = cx.lang;
    let num = cx.num;
    let here = |q: &str| format!("/showcase{q}#errors");
    view! {
        <div class="sc-sblocks sc-sblocks--2">
            <RailBlock lang num rail=Rail::Deals label=("Today's deals", "عروض النهارده")/>
            <RailBlock lang num rail=Rail::Bestsellers label=("Bestsellers", "الأكثر مبيعًا")/>
        </div>
        <div class="sc-row sc-row--wrap">
            <span class="sc-row__label">{tr(lang, "Break a read", "بوّظ قراية")}</span>
            <span class="sc-actions">
                <a class="sc-btnlink" href=here("?fail=deals")>"?fail=deals"</a>
                <a class="sc-btnlink" href=here("?fail=deals,bestsellers")>"?fail=deals,bestsellers"</a>
                <a class="sc-btnlink sc-btnlink--ok" href=here("")>{tr(lang, "Recover", "رجّعها")}</a>
            </span>
        </div>
        <div class="sc-row sc-row--wrap">
            <span class="sc-row__label">{tr(lang, "Whole page", "الصفحة كلها")}</span>
            <span class="sc-actions">
                <a class="sc-btnlink" href="/showcase/no-such-page">"404"</a>
                <a class="sc-btnlink" href="/showcase?crash=1">"500"</a>
                <a class="sc-btnlink" href="/showcase?crash=503">"503"</a>
            </span>
        </div>
        <p class="sc-note">{tr(
            lang,
            "Each block reads through its own <Suspense> and <ErrorBoundary>, rendered in order. With one failing, the other still shows, and so does the rest of this page.",
            "كل بلوك بيقرا جوه <Suspense> و <ErrorBoundary> خاصين بيه، ومترسومين بالترتيب. لو واحد فشل، التاني بيظهر عادي، وباقي الصفحة كمان.",
        )}</p>
    }
}

/// A small rail read through `api::home_rail` in its own boundary (honours `?fail=`).
#[component]
fn RailBlock(lang: Lang, num: Numerals, rail: Rail, label: (&'static str, &'static str)) -> impl IntoView {
    let name = tr(lang, label.0, label.1);
    view! {
        <section class="sc-sblock" aria-label=name>
            <header class="sc-sblock__head">
                <strong>{name}</strong>
                <code class="sc-tag mono" dir="ltr">{format!("home_rail({rail:?})")}</code>
            </header>
            // In order, so no Resource: the read is awaited where it stands and nothing is
            // serialized for a browser that would never pick it up.
            <Suspense fallback=|| ()>
                <ErrorBoundary fallback=move |errors| view! { <SectionError errors what=name.to_owned() compact=true/> }>
                    {move || Suspend::new(async move {
                        api::home_rail(rail, 3).await.map(|cards| view! {
                            <ul class="sc-minis" role="list">
                                {cards.into_iter().map(|c| view! {
                                    <li class="sc-mini">
                                        <img class="sc-mini__img" src=c.img.src_near(96) alt="" width="48" height="48" loading="lazy" decoding="async"/>
                                        <a class="sc-mini__name" href=format!("/p/{}", c.slug)>{c.name.pick(lang.is_rtl()).to_owned()}</a>
                                        <span class="sc-mini__price mono">{format::money(c.price, lang, num)}</span>
                                    </li>
                                }).collect_view()}
                            </ul>
                        })
                    })}
                </ErrorBoundary>
            </Suspense>
        </section>
    }
}

// ─── Read cache ────────────────────────────────────────────────────────────────────────────────

#[component]
fn ReadCacheDemo(cx: Arc<Ctx>) -> impl IntoView {
    let lang = cx.lang;
    let num = cx.num;
    let c = cx.cache.clone();
    let total = c.hits + c.misses;
    let rate = if total == 0 { 0 } else { (c.hits * 100 + total / 2) / total };
    view! {
        <div class="sc-moka" data-ts="sc-moka" data-stats=stats_path() data-bump=bump_path()>
            <dl class="sc-kvs sc-kvs--grid">
                <Kv k=tr(lang, "Entries", "العناصر") v=format::int(c.entries as i64, num) out="entries"/>
                <Kv k=tr(lang, "Hits", "مرات لقاها") v=format::int(c.hits as i64, num) out="hits"/>
                <Kv k=tr(lang, "Misses", "مرات ملقاهاش") v=format::int(c.misses as i64, num) out="misses"/>
                <Kv k=tr(lang, "Hit rate", "نسبة اللقيان") v=format!("{}%", format::int(rate as i64, num)) out="rate"/>
                <Kv k=tr(lang, "This shop's generation", "الـ generation بتاع المحل ده") v=format::int(c.generation as i64, num) out="gen"/>
            </dl>
            <i class="sc-meter" style=format!("--p:{rate}") aria-hidden="true" data-out-meter=""><b></b></i>
            // Without JS this form posts to the server function, which redirects back here.
            <form class="sc-actions" method="post" action=bump_path()>
                <Button kind=ButtonKind::Secondary icon="refresh-cw" attr:data-act="refresh" attr:hidden="">
                    {tr(lang, "Read the stats again", "اقرا الأرقام تاني")}
                </Button>
                <Button kind=ButtonKind::Outline button_type="submit" icon="rotate-ccw">
                    {tr(lang, "Bump the generation", "زوّد الـ generation")}
                </Button>
            </form>
            <p class="sc-note" data-slot="note">{tr(
                lang,
                "Bumping drops every cached read of this shop: the next page you open misses, then hits again.",
                "الزيادة بتشيل كل القرايات المتخزنة للمحل ده: أول صفحة تفتحها بعدها مش هتلاقي، وبعدين هتلاقي تاني.",
            )}</p>
        </div>
    }
}

// ─── Tenancy ───────────────────────────────────────────────────────────────────────────────────

#[component]
fn TenancyDemo(cx: Arc<Ctx>) -> impl IntoView {
    let lang = cx.lang;
    let num = cx.num;
    let t = &cx.tenant;
    let via = match t.via {
        Some(TenantVia::Subdomain) => tr(lang, "the Host's first label (the production path)", "أول جزء في الـ Host (طريقة الإنتاج)"),
        Some(TenantVia::Cookie) => tr(lang, "a tenant cookie (this host has no shop label)", "كوكي المحل (الـ host ده مافيهوش اسم محل)"),
        _ => tr(lang, "the default (nothing matched)", "الافتراضي (مفيش حاجة طابقت)"),
    };
    let catalogue = match lang {
        Lang::En => format!("{} categories · {}", format::count(t.categories as u32, num), format::items(t.products as u32, lang, num)),
        Lang::Ar => format!("{} قسم · {}", format::count(t.categories as u32, num), format::items(t.products as u32, lang, num)),
    };
    let delivery = match lang {
        Lang::En => format!("{} · free over {}", format::money(t.delivery_fee, lang, num), format::money(t.free_over, lang, num)),
        Lang::Ar => format!("{} · مجاني فوق {}", format::money(t.delivery_fee, lang, num), format::money(t.free_over, lang, num)),
    };
    let express = t.express_minutes.map(|m| match lang {
        Lang::En => format!(" · express in {} min", format::count(m, num)),
        Lang::Ar => format!(" · سريع في {} دقيقة", format::count(m, num)),
    });
    view! {
        <dl class="sc-kvs">
            <Kv k="Host" v=t.host.clone()/>
            <Kv k=tr(lang, "Picked by", "اتحدد من") v=via/>
            <Kv k=tr(lang, "Shop", "المحل") v=format!("{} · {} / {}", t.id, t.name.en, t.name.ar)/>
            <Kv k=tr(lang, "Default language", "اللغة الافتراضية") v=t.default_lang.endonym()/>
            <Kv k=tr(lang, "Catalogue", "الكتالوج") v=catalogue/>
            <Kv k=tr(lang, "Delivery", "التوصيل") v=format!("{delivery}{}", express.unwrap_or_default())/>
            <Kv k=tr(lang, "Order codes", "أكواد الطلبات") v=format!("{}-48213", t.order_prefix)/>
            <Kv k=tr(lang, "Cache-Tag prefix", "بادئة الـ Cache-Tag") v=format!("t{}", t.id)/>
        </dl>
        {t.other.clone().map(|(name, href, cross)| view! {
            <p class="sc-row">
                <Button kind=ButtonKind::Secondary size=ButtonSize::Sm href=href icon="store" attr:data-router=cross.then_some("off")>
                    {match lang {
                        Lang::En => format!("This page at {}", name.en),
                        Lang::Ar => format!("الصفحة دي في {}", name.ar),
                    }}
                </Button>
                <span class="sc-note">{if cross {
                    tr(lang, "Another subdomain is another origin: a full page load, with its own cookies.", "‏subdomain تاني يعني origin تاني: تحميل كامل للصفحة، وكوكيز خاصة بيه.")
                } else {
                    tr(lang, "Sets the tenant cookie and comes back (hosts without a shop label).", "بيحط كوكي المحل ويرجع (للـ hosts اللي من غير اسم محل).")
                }}</span>
            </p>
        })}
    }
}

// ─── Head and meta ─────────────────────────────────────────────────────────────────────────────

#[component]
fn MetaDemo(cx: Arc<Ctx>) -> impl IntoView {
    let lang = cx.lang;
    view! {
        <div data-ts="sc-head">
            <dl class="sc-kvs" data-out="head">
                <Kv k="<title>" v=tr(lang, "How this storefront is built", "المتجر ده متبني إزاي")/>
                <Kv k="robots" v="noindex"/>
            </dl>
            <p class="sc-note">{tr(
                lang,
                "Read live from this document's <head>. After an in-app navigation it shows the new page's head, which the router brings along.",
                "متقري دلوقتي من الـ <head> بتاع المستند ده. وبعد التنقل جوه الموقع بيعرض الـ head بتاع الصفحة الجديدة، اللي الراوتر بيجيبه معاها.",
            )}</p>
        </div>
    }
}

// ─── Simulated latency and faults ──────────────────────────────────────────────────────────────

#[component]
fn SimDemo(cx: Arc<Ctx>) -> impl IntoView {
    let lang = cx.lang;
    let num = cx.num;
    let next = cx.next_here("sim");
    let widgets = ["deals", "featured", "new", "bestsellers", "express", "listing", "search", "product", "related", "nav"];
    let on = cx.fail_cookie.clone();
    view! {
        <div class="sc-row">
            <span class="sc-row__label">{tr(lang, "Base latency", "التأخير الأساسي")}</span>
            <LatencyLinks lang num current=cx.base_ms next=next.clone()/>
        </div>
        <p class="sc-note">{latency_state(&cx)}</p>
        <div class="sc-row sc-row--wrap">
            <span class="sc-row__label">{tr(lang, "Faults for this session", "أعطال للجلسة دي")}</span>
            <span class="sc-toggles">
                {widgets.into_iter().map(|w| {
                    let is_on = on.iter().any(|x| x == w);
                    let list: Vec<&str> = if is_on {
                        on.iter().map(String::as_str).filter(|x| *x != w).collect()
                    } else {
                        on.iter().map(String::as_str).chain([w]).collect()
                    };
                    let href = format!("/__sim?fail={}&next={next}", list.join(","));
                    view! { <a class="sc-toggle mono" href=href aria-pressed=if is_on { "true" } else { "false" }>{w}</a> }
                }).collect_view()}
            </span>
            <a class="sc-link" href=format!("/__sim?fail=&next={next}")>{tr(lang, "Clear", "امسح")}</a>
        </div>
        <p class="sc-note">{tr(
            lang,
            "A fault makes that widget's read fail on every page until you clear it. Try “deals”, then open the home page.",
            "العطل بيخلّي قراية الـ widget ده تفشل في كل الصفحات لحد ما تمسحه. جرّب «deals» وافتح الرئيسية.",
        )}</p>
    }
}

// ─── Router ────────────────────────────────────────────────────────────────────────────────────

#[component]
fn RouterDemo(cx: Arc<Ctx>) -> impl IntoView {
    let lang = cx.lang;
    let target = cx.links.category.as_ref().map(|c| format!("/c/{c}")).unwrap_or_else(|| "/".into());
    view! {
        <div data-ts="sc-router" data-href=target.clone()>
            <p class="sc-actions">
                <Button kind=ButtonKind::Secondary icon="zap" attr:data-act="prefetch" attr:hidden="">
                    {format!("prefetch(\"{target}\")")}
                </Button>
                <Button kind=ButtonKind::Outline href=target.clone() icon_end="arrow-right">
                    {tr(lang, "Open it", "افتحها")}
                </Button>
                <Button kind=ButtonKind::Ghost icon="refresh-cw" attr:data-act="refresh" attr:hidden="">
                    "refresh()"
                </Button>
            </p>
            <dl class="sc-kvs">
                <Kv k=tr(lang, "routing.pending", "routing.pending") v="false" out="pending"/>
                <Kv k=tr(lang, "routing.to", "routing.to") v="null" out="to"/>
                <Kv k=tr(lang, "Navigations in this tab", "التنقلات في التاب ده") v="0" out="navs"/>
            </dl>
            <ol class="sc-log mono" dir="ltr" data-out="log" aria-live="polite"></ol>
            <p class="sc-note">{tr(
                lang,
                "Prefetch, then open: the page comes out of the page cache. Hovering a link for 65 ms or putting a finger on it does the same.",
                "اعمل prefetch وبعدين افتح: الصفحة بتطلع من كاش الصفحات. والوقوف على لينك 65 مللي ثانية أو حط صباعك عليه بيعمل نفس الحاجة.",
            )}</p>
        </div>
    }
}

// ─── Stores and the toast queue ────────────────────────────────────────────────────────────────

#[component]
fn StoresDemo(cx: Arc<Ctx>) -> impl IntoView {
    let lang = cx.lang;
    view! {
        <div data-ts="sc-stores">
            <dl class="sc-kvs">
                <Kv k="cart" v="—" out="cart"/>
                <Kv k="toasts" v="—" out="toasts"/>
                <Kv k="routing" v="—" out="routing"/>
            </dl>
            <p class="sc-actions" data-needs-js="" hidden>
                <Button kind=ButtonKind::Secondary size=ButtonSize::Sm icon="circle-check" attr:data-toast="success">
                    {tr(lang, "Success", "تمام")}
                </Button>
                <Button kind=ButtonKind::Secondary size=ButtonSize::Sm icon="info" attr:data-toast="info">
                    {tr(lang, "Info", "معلومة")}
                </Button>
                <Button kind=ButtonKind::Secondary size=ButtonSize::Sm icon="circle-x" attr:data-toast="error">
                    {tr(lang, "Error", "خطأ")}
                </Button>
                <Button kind=ButtonKind::Secondary size=ButtonSize::Sm icon="rotate-ccw" attr:data-toast="action">
                    {tr(lang, "With Undo", "ومعاه تراجع")}
                </Button>
            </p>
            <p class="sc-note">{tr(
                lang,
                "Each line is a live subscription. Fire a few toasts: the queue keeps three, newest in front, and the toaster draws them from the same store.",
                "كل سطر اشتراك حي. اعمل كام تنبيه: الطابور بيحتفظ بتلاتة، والأحدث قدام، والـ toaster بيرسمهم من نفس الـ store.",
            )}</p>
        </div>
    }
}

// ─── The cart cookie ───────────────────────────────────────────────────────────────────────────

#[component]
fn CartDemo(cx: Arc<Ctx>) -> impl IntoView {
    let lang = cx.lang;
    let Some(s) = cx.sample.clone() else {
        return view! { <p class="sc-note">{tr(lang, "This shop has no product to try it with.", "المحل ده مافيهوش منتج نجرّب بيه.")}</p> }.into_any();
    };
    let Sample { product, variant, name, price, img, href } = s;
    let (d_name, d_price, d_img, d_href) = (name.clone(), price.clone(), img.clone(), href.clone());
    view! {
        <div class="sc-cartdemo" data-ts="sc-cart" data-product=product data-variant=variant data-name=d_name data-price=d_price data-img=d_img data-href=d_href>
            <div class="sc-mini sc-mini--wide">
                <img class="sc-mini__img" src=img alt="" width="48" height="48" loading="lazy" decoding="async"/>
                <a class="sc-mini__name" href=href>{name}</a>
                <span class="sc-mini__price mono">{price}</span>
            </div>
            <p class="sc-actions" data-needs-js="" hidden>
                <Button kind=ButtonKind::Primary size=ButtonSize::Sm icon="plus" attr:data-act="add">
                    {tr(lang, "Add one", "زوّد واحد")}
                </Button>
                <Button kind=ButtonKind::Outline size=ButtonSize::Sm icon="trash-2" attr:data-act="remove">
                    {tr(lang, "Remove the line", "شيل السطر")}
                </Button>
                <Button kind=ButtonKind::Secondary size=ButtonSize::Sm icon="receipt" attr:data-act="quote">
                    {tr(lang, "Ask the server for prices", "اسأل السيرفر عن الأسعار")}
                </Button>
            </p>
            <dl class="sc-kvs">
                <Kv k=tr(lang, "cart cookie", "كوكي cart") v="—" out="cookie"/>
                <Kv k=tr(lang, "Units in the cart", "عدد القطع في السلة") v="—" out="count"/>
                <Kv k=tr(lang, "Requests for that change", "طلبات عشان التغيير ده") v="—" out="requests"/>
                <Kv k=tr(lang, "Server's quote", "حساب السيرفر") v="—" out="quote"/>
            </dl>
            <p class="sc-note">{tr(
                lang,
                "This is your real cart: the header badge and the drawer follow it. Adding and removing make no request; only “Ask the server” does, and it sends nothing but the cookie.",
                "دي سلتك الحقيقية: علامة الهيدر والدرج ماشيين معاها. الإضافة والشيل مابيعملوش أي طلب؛ «اسأل السيرفر» بس هو اللي بيعمل، ومابيبعتش غير الكوكي.",
            )}</p>
        </div>
    }
    .into_any()
}

// ─── Templates ─────────────────────────────────────────────────────────────────────────────────

#[component]
fn TemplatesDemo(cx: Arc<Ctx>) -> impl IntoView {
    let lang = cx.lang;
    #[derive(serde::Serialize)]
    struct Row<'a> {
        name: &'a str,
        price: &'a str,
        img: &'a str,
        href: &'a str,
    }
    let rows: Vec<Row> = cx.rows.iter().map(|r| Row { name: &r.name, price: &r.price, img: &r.img, href: &r.href }).collect();
    let json = serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into()).replace('<', "\\u003c");
    view! {
        <div data-ts="sc-tpl">
            <script type="application/json" data-ts-json="" inner_html=json></script>
            <template id="tpl-sc-row">
                <li class="sc-mini">
                    <img class="sc-mini__img" data-attr-src="img" alt="" width="48" height="48" decoding="async"/>
                    <a class="sc-mini__name" data-attr-href="href" data-slot="name"></a>
                    <span class="sc-mini__price mono" data-slot="price"></span>
                </li>
            </template>
            <p class="sc-actions" data-needs-js="" hidden>
                <Button kind=ButtonKind::Secondary size=ButtonSize::Sm icon="copy" attr:data-act="fill">
                    {"fill(\"sc-row\", row)"}
                </Button>
                <Button kind=ButtonKind::Ghost size=ButtonSize::Sm icon="x" attr:data-act="clear">
                    {tr(lang, "Clear", "امسح")}
                </Button>
            </p>
            <ul class="sc-minis" role="list" data-out="rows"></ul>
            <p class="sc-note" data-slot="count">{tr(
                lang,
                "The row's markup is the <template> above, rendered by Rust in your language. The client only puts the data in, as text.",
                "شكل السطر هو الـ <template> اللي فوق، Rust راسمه بلغتك. والعميل بس بيحط البيانات، كنص.",
            )}</p>
        </div>
    }
}

// ─── The client data cache ─────────────────────────────────────────────────────────────────────

#[component]
fn DataCacheDemo(cx: Arc<Ctx>) -> impl IntoView {
    let lang = cx.lang;
    view! {
        <div data-ts="sc-cache" data-url=stats_path()>
            <p class="sc-actions" data-needs-js="" hidden>
                <Button kind=ButtonKind::Secondary icon="zap" attr:data-act="read">
                    {tr(lang, "Read through cached()", "اقرا من خلال cached()")}
                </Button>
                <Button kind=ButtonKind::Ghost icon="trash-2" attr:data-act="invalidate">
                    "invalidate()"
                </Button>
            </p>
            <i class="sc-ttl" aria-hidden="true" data-out-ttl=""><b class="sc-ttl__fresh"></b><b class="sc-ttl__stale"></b><b class="sc-ttl__now"></b></i>
            <dl class="sc-kvs">
                <Kv k=tr(lang, "Key", "المفتاح") v="—" out="key"/>
                <Kv k=tr(lang, "Age", "العمر") v="—" out="age"/>
                <Kv k=tr(lang, "cache.ts stats()", "stats()‎ في cache.ts") v="—" out="counts"/>
            </dl>
            <ol class="sc-log mono" dir="ltr" data-out="log" aria-live="polite"></ol>
            <p class="sc-note">{tr(
                lang,
                "The value is the server's cache stats, cached here for 5 s fresh and 10 s more stale. Click a few times and watch the three cases.",
                "القيمة هي أرقام كاش السيرفر، ومتخزنة هنا 5 ثواني طازة و10 ثواني كمان قديمة. دوس كذا مرة وشوف الحالات التلاتة.",
            )}</p>
        </div>
    }
}

// ─── Springs ───────────────────────────────────────────────────────────────────────────────────

#[component]
fn SpringsDemo(cx: Arc<Ctx>) -> impl IntoView {
    let lang = cx.lang;
    let presets = [("default", true), ("snappy", false), ("gentle", false), ("bouncy", false)];
    view! {
        <div class="sc-spring" data-ts="sc-spring">
            <div class="sc-spring__track" data-track="">
                <span class="sc-spring__stop" style="--at:0"></span>
                <span class="sc-spring__stop" style="--at:0.5"></span>
                <span class="sc-spring__stop" style="--at:1"></span>
                <span class="sc-spring__ghost" data-ghost="" aria-hidden="true"></span>
                <button type="button" class="sc-spring__puck" data-puck="" aria-label=tr(lang, "Drag or throw me", "اسحبني أو ارميني")></button>
            </div>
            <p class="sc-row sc-row--wrap" data-needs-js="" hidden>
                <SegmentedToggle
                    items=presets.into_iter().map(|(p, on)| SegItem::toggle(p, p, on).attr("lang", "en")).collect()
                    label=tr(lang, "Spring", "السبرينج")
                    class="sc-spring__presets"
                />
                <button type="button" class="sc-link" data-act="interrupt">{tr(lang, "Send it, then reverse mid-way", "ابعته وارجّعه في النص")}</button>
            </p>
            <dl class="sc-kvs sc-kvs--grid">
                <Kv k=tr(lang, "Value", "القيمة") v="0" out="x"/>
                <Kv k=tr(lang, "Velocity", "السرعة") v="0" out="v"/>
                <Kv k=tr(lang, "Projected rest", "هيقف عند") v="—" out="proj"/>
            </dl>
            <p class="sc-note">{tr(
                lang,
                "Drag the dot and let go. A slow release settles on the nearest stop; a flick is projected and lands where it would have come to rest. Past either end it rubber-bands.",
                "اسحب النقطة وسيبها. لو سبتها براحة هتقف عند أقرب علامة؛ ولو زقيتها هتتحسب وهتقف مكان ما كانت هتهدى. وبعد أي طرف بتتمط زي الأستك.",
            )}</p>
        </div>
    }
}

// ─── Bundle size ───────────────────────────────────────────────────────────────────────────────

#[component]
fn BundleDemo(cx: Arc<Ctx>) -> impl IntoView {
    let lang = cx.lang;
    let num = cx.num;
    let raw = crate::client_js::APP_JS.len();
    let raw_kb = format::dec1(raw as f32 / 1024.0, num);
    view! {
        <div data-ts="sc-bundle" data-src=crate::client_js::APP_JS_PATH data-budget="30">
            <dl class="sc-kvs">
                <Kv k=tr(lang, "File", "الملف") v=crate::client_js::APP_JS_PATH/>
                <Kv k=tr(lang, "Raw, as built", "من غير ضغط، زي ما اتبنى") v=format!("{raw_kb} KB") out="raw"/>
                <Kv k=tr(lang, "Code only (no source map), gzip", "الكود بس (من غير source map)، gzip") v="—" out="gzip"/>
                <Kv k=tr(lang, "As your browser received it", "زي ما المتصفح بتاعك استلمه") v="—" out="wire"/>
            </dl>
            <div class="sc-bars" role="list">
                <div class="sc-bar sc-bar--now" role="listitem" style="--kb:0" data-out-bar="">
                    <span class="sc-bar__label">{tr(lang, "This client, gzip", "العميل ده، gzip")}</span>
                    <i class="sc-bar__fill" aria-hidden="true"></i>
                    <span class="sc-bar__n mono" data-slot="bar">"—"</span>
                </div>
                <div class="sc-bar sc-bar--budget" role="listitem" style="--kb:30">
                    <span class="sc-bar__label">{tr(lang, "The budget, gzip", "الميزانية، gzip")}</span>
                    <i class="sc-bar__fill" aria-hidden="true"></i>
                    <span class="sc-bar__n mono">{format!("{} KB", format::count(30, num))}</span>
                </div>
                <div class="sc-bar sc-bar--old" role="listitem" style="--kb:379">
                    <span class="sc-bar__label">{tr(lang, "WASM islands, split build, brotli", "جزر WASM، نسخة مقسّمة، brotli")}</span>
                    <i class="sc-bar__fill" aria-hidden="true"></i>
                    <span class="sc-bar__n mono">{format!("{} KB", format::count(379, num))}</span>
                </div>
                <div class="sc-bar sc-bar--old" role="listitem" style="--kb:769">
                    <span class="sc-bar__label">{tr(lang, "WASM islands, one bundle, brotli", "جزر WASM، ملف واحد، brotli")}</span>
                    <i class="sc-bar__fill" aria-hidden="true"></i>
                    <span class="sc-bar__n mono">{format!("{} KB", format::count(769, num))}</span>
                </div>
            </div>
            <p class="sc-note">{if cfg!(debug_assertions) {
                tr(lang, "This is a dev build: readable code with an inline source map. The measurement strips the map; a release build is minified and smaller still.", "دي نسخة تطوير: كود مقروء ومعاه source map جوه الملف. القياس بيشيل الـ map؛ ونسخة الـ release مصغّرة وأصغر كمان.")
            } else {
                tr(lang, "A release build: minified, no source map.", "نسخة release: مصغّرة ومن غير source map.")
            }}</p>
        </div>
    }
}

// ─── Why we left WASM islands ──────────────────────────────────────────────────────────────────

#[component]
fn IslandsDemo(cx: Arc<Ctx>) -> impl IntoView {
    let lang = cx.lang;
    let num = cx.num;
    let kb = |n: u32| format!("{} KB", format::int(n as i64, num));
    // README.md, "WASM size report" (release builds, brotli as sent, 2026-10-26).
    let pages: [((&str, &str), u32, u32); 6] = [
        (("Home", "الرئيسية"), 1, 379),
        (("Cart, checkout", "السلة والدفع"), 1, 379),
        (("Listing, search", "الأقسام والبحث"), 83, 578),
        (("Product", "المنتج"), 107, 581),
        (("/ui", "/ui"), 37, 431),
        (("/showcase", "/showcase"), 210, 713),
    ];
    let bugs: [(&str, &str); 4] = [
        (
            "An island still waiting for data when hydration ran panicked after an in-app navigation, and the panic stopped every island hydrated after it, the header's cart badge included.",
            "جزيرة لسه مستنية بيانات وقت الـ hydration كانت بتعمل panic بعد التنقل جوه الموقع، والـ panic كان بيوقف كل الجزر اللي بعدها، ومعاهم علامة السلة في الهيدر.",
        ),
        (
            "A lazy island built without --split made the JS glue import a placeholder module, so no island on any page hydrated.",
            "جزيرة lazy اتبنت من غير ‎--split‎ خلّت كود الـ JS يستورد module وهمي، فمفيش ولا جزيرة في أي صفحة صحيت.",
        ),
        (
            "Values created for the browser inside a component body could be dropped on another server thread, which panicked and cut the HTML short, at random.",
            "قيم معمولة للمتصفح جوه جسم الكومبوننت كانت ممكن تتشال على thread تاني في السيرفر، فيحصل panic والـ HTML يتقطع، بشكل عشوائي.",
        ),
        (
            "The islands router needed about 400 lines of shim: head alignment, <noscript>, the browser cache, the pressed submit button, scroll and back.",
            "الـ islands router كان محتاج حوالي 400 سطر ترقيع: ظبط الـ head، والـ <noscript>، وكاش المتصفح، وزرار الإرسال اللي اتداس، والـ scroll والرجوع.",
        ),
    ];
    view! {
        <div class="sc-islands">
            <dl class="sc-kvs sc-kvs--grid">
                <Kv k=tr(lang, "One bundle, brotli", "ملف واحد، brotli") v=kb(769)/>
                <Kv k=tr(lang, "Split build, main module", "نسخة مقسّمة، الملف الأساسي") v=kb(379)/>
                <Kv k=tr(lang, "Rebuild after a one-line change", "إعادة البناء بعد تعديل سطر") v=match lang {
                    Lang::En => format!("{}–{} s (server alone: {} s)", format::count(36, num), format::count(48, num), format::count(15, num)),
                    Lang::Ar => format!("{}–{} ثانية (السيرفر لوحده: {} ثانية)", format::count(36, num), format::count(48, num), format::count(15, num)),
                }/>
            </dl>
            <div class="sc-routes" role="table" aria-label=tr(lang, "WASM per page, split build", "الـ WASM لكل صفحة، نسخة مقسّمة")>
                <div class="sc-routes__row sc-routes__row--head sc-routes__row--3" role="row">
                    <span role="columnheader">{tr(lang, "Page", "الصفحة")}</span>
                    <span role="columnheader">{tr(lang, "WASM files", "ملفات WASM")}</span>
                    <span role="columnheader">{tr(lang, "WASM, brotli", "WASM، brotli")}</span>
                </div>
                {pages.into_iter().map(|(name, files, size)| view! {
                    <div class="sc-routes__row sc-routes__row--3" role="row">
                        <span role="cell">{tr(lang, name.0, name.1)}</span>
                        <span role="cell" class="mono">{format::count(files, num)}</span>
                        <span role="cell" class="mono">{kb(size)}</span>
                    </div>
                }).collect_view()}
            </div>
            <ul class="sc-steps" role="list">
                {bugs.into_iter().map(|(en, ar)| view! { <li>{nums(tr(lang, en, ar), num)}</li> }).collect_view()}
            </ul>
            <p class="sc-note">
                {tr(lang, "What replaced it is measured live in “The size of the client”.", "اللي جه مكانها متقاس دلوقتي في «حجم العميل».")}
                " "
                <a class="sc-link" href="#bundle">{tr(lang, "See it", "شوفه")}</a>
            </p>
        </div>
    }
}
