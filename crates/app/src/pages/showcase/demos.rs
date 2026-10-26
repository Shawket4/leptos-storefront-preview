//! The server-rendered part of each card's live demo. Interactive pieces are islands
//! (`islands.rs`); everything here is plain HTML from the server.

use super::Ctx;
use super::islands::{self, Probe, SaveItem, StreamTarget, Unit};
use crate::api::{self, types::Rail};
use crate::components::icons::Icon;
use crate::components::product_card::CardData;
use crate::format;
use crate::pages::errors::SectionError;
use crate::prefs::{Lang, Numerals, Theme, tr};
use crate::tenant::TenantVia;
use leptos::prelude::*;
use std::sync::Arc;

/// The demo for a feature id.
pub fn demo(id: &str, cx: Arc<Ctx>) -> AnyView {
    match id {
        "islands" => view! { <IslandsDemo cx/> }.into_any(),
        "ssr-modes" => view! { <SsrModesDemo cx/> }.into_any(),
        "streaming" => view! { <StreamingDemo cx/> }.into_any(),
        "errors" => view! { <ErrorsDemo cx/> }.into_any(),
        "meta" => view! { <MetaDemo cx/> }.into_any(),
        "reactivity" => view! { <ReactivityDemo cx/> }.into_any(),
        "shared-state" => view! { <SharedStateDemo cx/> }.into_any(),
        "server-fns" => view! { <ServerFnsDemo cx/> }.into_any(),
        "transition" => view! { <islands::ListingPager pages=cx.pager_pages/> }.into_any(),
        "sim" => view! { <SimDemo cx/> }.into_any(),
        "action-form" => view! { <ActionFormDemo cx/> }.into_any(),
        "islands-router" => view! { <RouterTabsDemo cx/> }.into_any(),
        "router" => view! { <RouterDemo cx/> }.into_any(),
        "caching" => view! { <CachingDemo cx/> }.into_any(),
        "tenancy" => view! { <TenancyDemo cx/> }.into_any(),
        "i18n" => view! { <I18nDemo cx/> }.into_any(),
        "view-transitions" => view! { <VtDemo cx/> }.into_any(),
        "images" => view! { <ImagesDemo cx/> }.into_any(),
        "a11y" => view! { <A11yDemo cx/> }.into_any(),
        _ => ().into_any(),
    }
}

fn money(p: domain::Piastres, lang: Lang, n: Numerals) -> String {
    format::money(p, lang, n)
}

/// A small packshot for lists (server HTML). Decorative: the product name sits next to it.
#[component]
fn Thumb(card: CardData) -> impl IntoView {
    let img = card.img;
    (!img.is_missing()).then(|| {
        view! {
            <img class="sc-mini__img" src=img.src_near(240) srcset=img.srcset() sizes="48px"
                alt="" width="48" height="48" loading="lazy" decoding="async"
                style=format!("background-color:{}", img.color)/>
        }
    })
}

/// One product line: thumbnail, name, price.
#[component]
fn MiniRow(card: CardData, lang: Lang, num: Numerals) -> impl IntoView {
    let ar = lang.is_rtl();
    let name = card.name.pick(ar).to_owned();
    let price = money(card.price, lang, num);
    let href = format!("/p/{}", card.slug);
    view! {
        <li class="sc-mini">
            <Thumb card=card.clone()/>
            <a class="sc-mini__name" href=href>{name}</a>
            <span class="sc-mini__price mono">{price}</span>
        </li>
    }
}

// ─── 01 Islands ────────────────────────────────────────────────────────────────────────────────

#[component]
fn IslandsDemo(cx: Arc<Ctx>) -> impl IntoView {
    let lang = cx.lang;
    let num = cx.num;
    let ar = lang.is_rtl();
    view! {
        <div class="sc-split">
            <div class="sc-pane">
                <p class="sc-pane__label">
                    <Icon name="layout-grid" size=16/>
                    <span>{tr(lang, "#[component] — server HTML, 0 bytes of JS", "‏#[component] — HTML من السيرفر، ٠ بايت JS")}</span>
                </p>
                <ul class="sc-list" role="list">
                    {cx.cats.iter().take(4).map(|c| view! {
                        <li>
                            <a href=format!("/c/{}", c.slug)>{c.name.pick(ar).to_owned()}</a>
                            <span class="mono">{format::items(c.count, lang, num)}</span>
                        </li>
                    }).collect_view()}
                </ul>
            </div>
            <div class="sc-pane sc-pane--island">
                <p class="sc-pane__label">
                    <Icon name="sparkles" size=16/>
                    <span>{tr(lang, "#[island] — hydrated in your browser", "‏#[island] — بتصحى في المتصفح عندك")}</span>
                </p>
                <islands::IslandCounter probe=false/>
            </div>
        </div>
        <islands::BundleProbe files=cx.files.clone() debug_build=cx.debug_build/>
    }
}

// ─── SSR modes ─────────────────────────────────────────────────────────────────────────────────

#[component]
fn SsrModesDemo(cx: Arc<Ctx>) -> impl IntoView {
    let lang = cx.lang;
    // (mode, where it's used here, first byte, needs JS to show streamed parts, <head> from data)
    let modes: [(&str, (&str, &str), (&str, &str), bool, bool); 4] = [
        (
            "OutOfOrder",
            ("/ (home), /showcase", "/ (الرئيسية)، ‎/showcase‎"),
            (
                "at once; sections land as their data does",
                "فورًا؛ والأقسام بتوصل مع بياناتها",
            ),
            true,
            false,
        ),
        (
            "InOrder",
            (
                "/nojs, /c/…, /search, /cart, /checkout, /orders/…",
                "‏/nojs و ‎/c/…‎ و ‎/search‎ و ‎/cart‎ و ‎/checkout‎ و ‎/orders/…‎",
            ),
            (
                "at once; then in document order, each section waiting for the one before",
                "فورًا؛ وبعدين بترتيب الصفحة، كل قسم مستني اللي قبله",
            ),
            false,
            false,
        ),
        (
            "PartiallyBlocked",
            ("/p/… (product)", "‏/p/…‎ (المنتج)"),
            (
                "after its blocking resources",
                "بعد الـ blocking resources بتاعته",
            ),
            false,
            true,
        ),
        (
            "Async",
            ("not used", "مش مستخدم"),
            ("after everything", "بعد ما كله يخلص"),
            false,
            true,
        ),
    ];
    let targets = vec![
        StreamTarget {
            en: "Home — OutOfOrder".into(),
            ar: "الرئيسية — OutOfOrder".into(),
            url: "/".into(),
            router: false,
        },
        StreamTarget {
            en: "Home — InOrder twin".into(),
            ar: "الرئيسية — نسخة InOrder".into(),
            url: "/nojs".into(),
            router: false,
        },
        StreamTarget {
            en: "Home — as a client navigation".into(),
            ar: "الرئيسية — كتنقل من غير ريلود".into(),
            url: "/".into(),
            router: true,
        },
    ];
    let yes = tr(lang, "yes", "أيوه");
    let no = tr(lang, "no", "لأ");
    view! {
        <ul class="sc-modes" role="list">
            {modes.into_iter().map(|(m, used, first, js, head)| view! {
                <li class="sc-mode" class:is-unused=m == "Async">
                    <code class="sc-mode__name mono" dir="ltr">{format!("SsrMode::{m}")}</code>
                    <dl class="sc-mode__kv">
                        <div><dt>{tr(lang, "Used for", "مستخدم في")}</dt><dd>{tr(lang, used.0, used.1)}</dd></div>
                        <div><dt>{tr(lang, "First byte", "أول بايت")}</dt><dd>{tr(lang, first.0, first.1)}</dd></div>
                        <div><dt>{tr(lang, "Streamed parts need JS", "الأجزاء المتبعتة محتاجة JS")}</dt><dd class:is-warn=js>{if js { yes } else { no }}</dd></div>
                        <div><dt>{tr(lang, "<head> can use data", "الـ <head> يقدر يستخدم البيانات")}</dt><dd>{if head { yes } else { no }}</dd></div>
                    </dl>
                </li>
            }).collect_view()}
        </ul>
        <p class="sc-note">
            {tr(
                lang,
                "Modes are per route, not per request — a server can't tell whether the visitor has JavaScript. Client navigations by the islands router always get in-order HTML (leptos_axum switches off out-of-order when it sees the Islands-Router header), so such a navigation waits for the page's slowest section.",
                "الطريقة بتتحدد لكل route، مش لكل طلب — السيرفر مايعرفش الزائر عنده JavaScript ولا لأ. والتنقلات اللي بيعملها الـ islands router بتاخد HTML بالترتيب دايمًا (‏leptos_axum بيقفل الـ out-of-order لما يشوف هيدر Islands-Router)، فالتنقل ده بيستنى أبطأ قسم في الصفحة.",
            )}
        </p>
        <islands::StreamInspector targets/>
    }
}

// ─── Streaming: three blocks, three latencies ──────────────────────────────────────────────────

#[component]
fn StreamingDemo(cx: Arc<Ctx>) -> impl IntoView {
    let lang = cx.lang;
    let num = cx.num;
    let next = cx.next_here("streaming");
    let replay = format!("/showcase?replay={}#streaming", cx.now_ms);
    let mode = if cx.router_nav {
        tr(
            lang,
            "This render: an islands-router navigation, so in order — all blocks arrived together.",
            "الرسمة دي: تنقل بالـ islands router، يعني بالترتيب — كل البلوكات وصلت مع بعض.",
        )
    } else {
        tr(
            lang,
            "This render: a full load, out of order — each block arrived the moment its data did.",
            "الرسمة دي: تحميل كامل بأي ترتيب — كل بلوك وصل أول ما بياناته وصلت.",
        )
    };
    view! {
        <div class="sc-row">
            <span class="sc-row__label">{tr(lang, "Base latency", "الزمن الأساسي")}</span>
            <LatencyLinks cx=cx.clone() next=next external=true/>
            <a class="sc-link" href=replay rel="external">
                <Icon name="rotate-ccw" size=16/>
                <span>{tr(lang, "Replay with a full load", "أعد بتحميل كامل")}</span>
            </a>
        </div>
        <p class="sc-note">{mode}</p>
        <div class="sc-sblocks">
            <StreamBlock cx=cx.clone() rail=Rail::Deals label=("Today's deals", "عروض النهارده")/>
            <StreamBlock cx=cx.clone() rail=Rail::Featured label=("Picked for you", "اخترنالك")/>
            <StreamBlock cx=cx.clone() rail=Rail::New label=("New arrivals", "وصل حديثًا")/>
            <BlockingBlock cx=cx.clone()/>
        </div>
        <p class="sc-note">
            {match lang {
                Lang::En => format!(
                    "Blocks are in page order; the badge is the order the server finished them in. Each waits {} ms × its widget's factor of simulated backend time; a cache miss adds the pretend database.",
                    format::int(cx.base_ms as i64, num)
                ),
                Lang::Ar => format!(
                    "البلوكات بترتيب الصفحة؛ والرقم هو ترتيب ما السيرفر خلّصهم. كل واحد بيستنى {} ms × معامل الـ widget بتاعه كزمن باك إند تجريبي؛ والـ cache miss بيزوّد زمن قاعدة البيانات التجريبية.",
                    format::int(cx.base_ms as i64, num)
                ),
            }}
        </p>
    }
}

#[component]
fn StreamBlock(cx: Arc<Ctx>, rail: Rail, label: (&'static str, &'static str)) -> impl IntoView {
    let lang = cx.lang;
    let num = cx.num;
    let clock = cx.clock.clone();
    let data = Resource::new(|| (), move |_| api::home_rail(rail, 3));
    view! {
        <section class="sc-sblock" aria-label=tr(lang, label.0, label.1)>
            <header class="sc-sblock__head">
                <strong>{tr(lang, label.0, label.1)}</strong>
                <code class="sc-tag mono" dir="ltr">{format!("home_rail({rail:?})")}</code>
            </header>
            <Suspense fallback=move || view! { <SBlockSkeleton/> }>
                <ErrorBoundary fallback=|errors| view! { <SectionError errors compact=true/> }>
                    {
                        let clock = clock.clone();
                        move || {
                            let clock = clock.clone();
                            Suspend::new(async move {
                                let r = data.await;
                                let (ms, rank) = clock.arrived();
                                r.map(|cards| view! {
                                    <p class="sc-sblock__meta">
                                        <span class="sc-rank mono">{format!("#{}", format::count(rank, num))}</span>
                                        <span class="mono">{match lang {
                                            Lang::En => format!("resolved after {} ms", format::int(ms as i64, num)),
                                            Lang::Ar => format!("خلص بعد {} ms", format::int(ms as i64, num)),
                                        }}</span>
                                        <i class="sc-sblock__bar" style=format!("--ms:{ms}") aria-hidden="true"></i>
                                    </p>
                                    <ul class="sc-minis" role="list">
                                        {cards.into_iter().map(|c| view! { <MiniRow card=c lang num/> }).collect_view()}
                                    </ul>
                                })
                            })
                        }
                    }
                </ErrorBoundary>
            </Suspense>
        </section>
    }
}

/// The opt-out: a blocking resource. The first byte waits for it, and its HTML is inline in the
/// first chunk — present with JavaScript off.
#[component]
fn BlockingBlock(cx: Arc<Ctx>) -> impl IntoView {
    let lang = cx.lang;
    let num = cx.num;
    let ar = lang.is_rtl();
    let clock = cx.clock.clone();
    let data = Resource::new_blocking(|| (), |_| api::nav_categories());
    view! {
        <section class="sc-sblock sc-sblock--blocking" aria-label=tr(lang, "Blocking resource", "Blocking resource")>
            <header class="sc-sblock__head">
                <strong>{tr(lang, "Categories (blocking)", "الأقسام (blocking)")}</strong>
                <code class="sc-tag mono" dir="ltr">"Resource::new_blocking"</code>
            </header>
            <Suspense fallback=move || view! { <SBlockSkeleton/> }>
                <ErrorBoundary fallback=|errors| view! { <SectionError errors compact=true/> }>
                    {
                        let clock = clock.clone();
                        move || {
                            let clock = clock.clone();
                            Suspend::new(async move {
                                let r = data.await;
                                let (ms, _) = clock.arrived();
                                r.map(|cats| view! {
                                    <p class="sc-sblock__meta">
                                        <span class="sc-rank sc-rank--first mono">{tr(lang, "1st chunk", "أول chunk")}</span>
                                        <span class="mono">{match lang {
                                            Lang::En => format!("ready at {} ms — before the first byte", format::int(ms as i64, num)),
                                            Lang::Ar => format!("جاهز عند {} ms — قبل أول بايت", format::int(ms as i64, num)),
                                        }}</span>
                                    </p>
                                    <ul class="sc-pills" role="list">
                                        {cats.into_iter().take(6).map(|c| view! { <li><a href=format!("/c/{}", c.slug)>{c.name.pick(ar).to_owned()}</a></li> }).collect_view()}
                                    </ul>
                                })
                            })
                        }
                    }
                </ErrorBoundary>
            </Suspense>
        </section>
    }
}

#[component]
fn SBlockSkeleton() -> impl IntoView {
    view! {
        <div class="sc-sblock__skel" aria-hidden="true">
            <span class="skeleton sc-skel-line"></span>
            {(0..3).map(|_| view! {
                <span class="sc-skel-row"><span class="skeleton sc-skel-img"></span><span class="skeleton sc-skel-text"></span></span>
            }).collect_view()}
        </div>
    }
}

/// 0 / 350 / 1200 ms and the server default, as links to `/__sim`.
#[component]
fn LatencyLinks(cx: Arc<Ctx>, next: String, #[prop(optional)] external: bool) -> impl IntoView {
    let lang = cx.lang;
    let num = cx.num;
    let current = cx.base_ms;
    let mut opts: Vec<(Option<u64>, String)> = [0u64, 350, 1200]
        .into_iter()
        .map(|v| (Some(v), format!("{} ms", format::int(v as i64, num))))
        .collect();
    opts.push((None, tr(lang, "Default", "الافتراضي").to_owned()));
    let rel = external.then_some("external");
    view! {
        <span class="sc-seg" role="group" aria-label=tr(lang, "Simulated latency", "الزمن التجريبي")>
            {opts.into_iter().map(|(v, label)| {
                let on = v.map(|v| v == current).unwrap_or(false);
                let href = format!("/__sim?ms={}&next={next}", v.map(|v| v.to_string()).unwrap_or_default());
                view! {
                    <a class="sc-seg__opt" href=href rel=rel aria-current=if on { "true" } else { "false" }>{label}</a>
                }
            }).collect_view()}
        </span>
    }
}

// ─── Error boundaries ──────────────────────────────────────────────────────────────────────────

#[component]
fn ErrorsDemo(cx: Arc<Ctx>) -> impl IntoView {
    let lang = cx.lang;
    let num = cx.num;
    let next = cx.next_here("errors");
    let faults: Vec<String> =
        cx.q.fail
            .iter()
            .chain(cx.fail_cookie.iter())
            .cloned()
            .collect();
    let state = if faults.is_empty() {
        tr(lang, "No faults on.", "مفيش أعطال شغالة.").to_owned()
    } else {
        format!(
            "{} {}",
            tr(lang, "Faults on:", "الأعطال الشغالة:"),
            faults.join(", ")
        )
    };
    let widget = |rail: Rail, label: (&'static str, &'static str)| {
        let data = Resource::new(|| (), move |_| api::home_rail(rail, 3));
        view! {
            <section class="sc-sblock">
                <header class="sc-sblock__head">
                    <strong>{tr(lang, label.0, label.1)}</strong>
                    <code class="sc-tag mono" dir="ltr">{format!("fail={}", format!("{rail:?}").to_lowercase())}</code>
                </header>
                <Suspense fallback=move || view! { <SBlockSkeleton/> }>
                    <ErrorBoundary fallback=|errors| view! { <SectionError errors/> }>
                        {move || Suspend::new(async move {
                            data.await.map(|cards| view! {
                                <ul class="sc-minis" role="list">
                                    {cards.into_iter().map(|c| view! { <MiniRow card=c lang num/> }).collect_view()}
                                </ul>
                            })
                        })}
                    </ErrorBoundary>
                </Suspense>
            </section>
        }
    };
    view! {
        <div class="sc-row sc-row--wrap">
            <a class="sc-btnlink" href="/showcase?fail=bestsellers#errors">{tr(lang, "Break Bestsellers (this request)", "اكسر «الأكثر مبيعًا» (الطلب ده بس)")}</a>
            <a class="sc-btnlink" href="/showcase?fail=bestsellers,express#errors">{tr(lang, "Break both", "اكسر الاتنين")}</a>
            <a class="sc-btnlink" href=format!("/__sim?fail=bestsellers&next={next}")>{tr(lang, "Break it for my session (cookie)", "اكسره للجلسة بتاعتي (كوكي)")}</a>
            <a class="sc-btnlink sc-btnlink--ok" href=format!("/__sim?fail=&next={}", crate::api::types::url_encode("/showcase#errors"))>{tr(lang, "Repair everything", "صلّح كله")}</a>
        </div>
        <p class="sc-note" role="status">{state}</p>
        <div class="sc-sblocks sc-sblocks--2">
            {widget(Rail::Bestsellers, ("Bestsellers", "الأكثر مبيعًا"))}
            {widget(Rail::Express, ("Express picks", "توصيل سريع"))}
        </div>
        <p class="sc-note">
            {tr(lang, "A whole route can fail too: ", "والـ route كله ممكن يقع برضه: ")}
            <a class="link" href="/showcase?crash=1">{tr(lang, "open the full-page error view", "افتح صفحة الخطأ الكاملة")}</a>
            {tr(lang, " (status 500, from pages/errors.rs).", " (‏status 500، من pages/errors.rs).")}
        </p>
    }
}

// ─── leptos_meta ───────────────────────────────────────────────────────────────────────────────

#[component]
fn MetaDemo(cx: Arc<Ctx>) -> impl IntoView {
    let lang = cx.lang;
    let ar = lang.is_rtl();
    let canonical = format!("{}/showcase", cx.origin);
    let title = format!(
        "{} · {}",
        tr(lang, "How this storefront is built", "إزاي المتجر ده متبني"),
        cx.tenant.name.pick(ar)
    );
    let server = vec![
        ("<title>", title.clone()),
        (
            "meta description",
            tr(
                lang,
                "Every Leptos feature this storefront uses — …",
                "كل خاصية في Leptos المتجر ده بيستخدمها — …",
            )
            .to_owned(),
        ),
        ("meta robots", "noindex".into()),
        ("link rel=canonical", canonical.clone()),
        ("meta og:type", "article".into()),
        ("meta og:title", title),
        ("meta og:url", canonical),
        ("meta og:locale", if ar { "ar_EG" } else { "en_EG" }.into()),
        ("meta twitter:card", "summary".into()),
    ]
    .into_iter()
    .map(|(k, v)| islands::HeadTag {
        key: k.into(),
        value: v,
    })
    .collect::<Vec<_>>();
    view! {
        <islands::HeadReadout server/>
        <p class="sc-note">
            {tr(lang, "The home page adds a WebSite JSON-LD with a site-search action: ", "الرئيسية بتضيف WebSite JSON-LD فيه أكشن للبحث: ")}
            <a class="link" href="/">"/"</a>
            {tr(lang, " (view source). Tags set inside a <Suspense> that resolves after the first byte can't reach <head> — the reason the product page blocks for its data.", " (اعرض الكود). والتاجات اللي جوه <Suspense> بيخلص بعد أول بايت مش بتوصل للـ <head> — وده سبب إن صفحة المنتج بتستنى بياناتها.")}
        </p>
    }
}

// ─── Reactivity ────────────────────────────────────────────────────────────────────────────────

#[component]
fn ReactivityDemo(cx: Arc<Ctx>) -> impl IntoView {
    let free = cx.tenant.free_over.0;
    // Prefer products whose price sits under the free-delivery line, so steps cross it.
    let mut units: Vec<Unit> = cx
        .picks
        .iter()
        .map(|c| Unit {
            en: c.name.en.clone(),
            ar: c.name.ar.clone(),
            price: c.price.0,
        })
        .collect();
    units.sort_by_key(|u| (u.price >= free, u.price));
    let fallback = Unit {
        en: "Coffee beans 250 g".into(),
        ar: "بن ٢٥٠ جم".into(),
        price: free / 3 + 900,
    };
    if units.iter().all(|u| u.price >= free) {
        units.insert(0, fallback);
    }
    view! { <islands::PriceCalc units fee=cx.tenant.delivery_fee.0 free_over=free/> }
}

#[component]
fn SharedStateDemo(cx: Arc<Ctx>) -> impl IntoView {
    let lang = cx.lang;
    let num = cx.num;
    let items: Vec<SaveItem> = cx
        .picks
        .iter()
        .map(|c| SaveItem {
            id: c.id.0,
            en: c.name.en.clone(),
            ar: c.name.ar.clone(),
        })
        .collect();
    view! {
        <islands::SavedTray items=items.clone()/>
        <ul class="sc-minis sc-minis--save" role="list">
            {cx.picks.iter().cloned().zip(items).map(|(c, item)| {
                let name = c.name.pick(lang.is_rtl()).to_owned();
                let price = money(c.price, lang, num);
                view! {
                    <li class="sc-mini">
                        <Thumb card=c/>
                        <span class="sc-mini__name">{name}</span>
                        <span class="sc-mini__price mono">{price}</span>
                        <islands::SaveToggle item/>
                    </li>
                }
            }).collect_view()}
        </ul>
        <p class="sc-note">
            {tr(
                lang,
                "Four separate islands — the tray and three buttons — share one store from context. Navigate with a tab in “islands-router” below and they keep it: the root owner lives as long as the document.",
                "أربع islands منفصلة — اللستة وتلات زراير — بيشاركوا store واحد من الـ context. اتنقل بتاب في «islands-router» تحت وهيفضلوا فاكرينه: الـ owner الرئيسي عايش طول ما الصفحة عايشة.",
            )}
        </p>
    }
}

// ─── Server functions ──────────────────────────────────────────────────────────────────────────

#[component]
fn ServerFnsDemo(cx: Arc<Ctx>) -> impl IntoView {
    let lang = cx.lang;
    let t = cx.tenant.id.clone();
    view! {
        <islands::CacheLab initial=cx.cache.clone() tenant=t.clone()/>
        <ul class="sc-steps" role="list">
            <li>{tr(lang, "Every catalogue read goes through a moka cache keyed by tenant + generation + query (the shape of Madar's menu cache).", "كل قراءة من الكتالوج بتعدّي على كاش moka بمفتاح المتجر + الجيل + الاستعلام (نفس شكل كاش المنيو في مدار).")}</li>
            <li>
                {tr(lang, "Bumping the generation makes every key of this shop unreachable at once — no scan. In production the same edit purges the CDN: ", "زيادة الجيل بتخلي كل مفاتيح المتجر ده مش بتتلقي مرة واحدة — من غير ما نلف عليها. وفي الإنتاج نفس التعديل بيمسح الـ CDN: ")}
                <code class="mono" dir="ltr">{format!("Cache-Tag: t{t}")}</code>
                {tr(lang, " for the shop, ", " للمتجر، ")}
                <code class="mono" dir="ltr">{format!("t{t}-p{{id}}")}</code>
                {tr(lang, " for one product.", " لمنتج واحد.")}
            </li>
        </ul>
    }
}

// ─── Simulated latency and faults ──────────────────────────────────────────────────────────────

#[component]
fn SimDemo(cx: Arc<Ctx>) -> impl IntoView {
    let lang = cx.lang;
    let num = cx.num;
    let next = cx.next_here("sim");
    let widgets = [
        "deals",
        "featured",
        "new",
        "bestsellers",
        "express",
        "listing",
        "search",
        "product",
        "related",
        "cart",
        "nav",
    ];
    let on: Vec<String> = cx.fail_cookie.clone();
    let source = if cx.base_ms == cx.default_ms {
        tr(lang, "the server default", "الافتراضي بتاع السيرفر")
    } else {
        tr(lang, "your cookie", "الكوكي بتاعتك")
    };
    view! {
        <div class="sc-row">
            <span class="sc-row__label">{tr(lang, "Base latency", "الزمن الأساسي")}</span>
            <LatencyLinks cx=cx.clone() next=next.clone()/>
        </div>
        <p class="sc-note">
            {match lang {
                Lang::En => format!("Now {} ms ({source}). Applies to every page of this shop until you change it.", format::int(cx.base_ms as i64, num)),
                Lang::Ar => format!("دلوقتي {} ms (‏{source}). بيتطبق على كل صفحات المتجر لحد ما تغيّره.", format::int(cx.base_ms as i64, num)),
            }}
        </p>
        <div class="sc-row sc-row--wrap">
            <span class="sc-row__label">{tr(lang, "Faults (session)", "الأعطال (للجلسة)")}</span>
            <span class="sc-toggles">
                {widgets.into_iter().map(|w| {
                    let is_on = on.iter().any(|x| x == w);
                    let list: Vec<&str> = if is_on {
                        on.iter().map(String::as_str).filter(|x| *x != w).collect()
                    } else {
                        on.iter().map(String::as_str).chain([w]).collect()
                    };
                    let href = format!("/__sim?fail={}&next={next}", list.join(","));
                    view! {
                        <a class="sc-toggle mono" href=href aria-pressed=if is_on { "true" } else { "false" }>{w}</a>
                    }
                }).collect_view()}
            </span>
            <a class="sc-link" href=format!("/__sim?fail=&next={next}")>{tr(lang, "Clear", "امسح")}</a>
        </div>
        <p class="sc-note">
            {tr(lang, "A fault makes that widget's read return an error everywhere — try “deals”, then open the home page.", "العطل بيخلي قراءة الـ widget ده ترجّع خطأ في كل حتة — جرّب «deals» وافتح الرئيسية.")}
        </p>
    }
}

// ─── ActionForm ────────────────────────────────────────────────────────────────────────────────

#[component]
fn ActionFormDemo(cx: Arc<Ctx>) -> impl IntoView {
    let lang = cx.lang;
    view! {
        <islands::NoteBoard board=cx.board.clone() error=cx.note_error.clone()/>
        <ol class="sc-steps" role="list">
            <li>{tr(lang, "With JavaScript: the island prevents the native submit, dispatches the ServerAction (a fetch), and swaps in the Board it returns. pending() drives the button's spinner.", "مع JavaScript: الـ island بتمنع الإرسال العادي، وتشغّل الـ ServerAction (‏fetch)، وتحط الـ Board اللي راجع. و pending()‎ هو اللي مشغّل اللودر في الزرار.")}</li>
            <li>{tr(lang, "Without JavaScript: the browser posts the form to the server function's URL; server_fn sees Accept: text/html and answers 302 back to this page, which renders the new board. A validation error comes back as ?__err=… and shows the same message.", "من غير JavaScript: المتصفح بيبعت الفورم لـ URL دالة السيرفر؛ و server_fn بيشوف Accept: text/html فيرد بـ 302 لنفس الصفحة، اللي بترسم اللستة الجديدة. ولو فيه خطأ بيرجع كـ ‎?__err=…‎ ويظهر نفس الرسالة.")}</li>
        </ol>
    }
}

// ─── islands-router: tabs that keep island state ───────────────────────────────────────────────

/// `HH:MM:SS.mmm` in Cairo time.
fn clock_ms(utc_ms: i64, num: Numerals) -> String {
    let local = utc_ms + format::cairo_offset_hours(utc_ms) * 3_600_000;
    let ms = local.rem_euclid(1000);
    let s = local.div_euclid(1000).rem_euclid(86_400);
    format::digits(
        &format!("{:02}:{:02}:{:02}.{ms:03}", s / 3600, (s / 60) % 60, s % 60),
        num,
    )
}

#[component]
fn RouterTabsDemo(cx: Arc<Ctx>) -> impl IntoView {
    let lang = cx.lang;
    let num = cx.num;
    let ar = lang.is_rtl();
    let active = cx
        .tab
        .as_ref()
        .map(|(c, _)| c.slug.clone())
        .unwrap_or_default();
    let tabs: Vec<_> = cx
        .cats
        .iter()
        .filter(|c| c.count > 0)
        .take(3)
        .cloned()
        .collect();
    let stamp = format!(
        "{} {} · {}",
        tr(lang, "Server-rendered at", "اترسم على السيرفر الساعة"),
        clock_ms(cx.now_ms, num),
        if cx.router_nav {
            tr(
                lang,
                "islands-router navigation (in order)",
                "تنقل بالـ islands router (بالترتيب)",
            )
        } else {
            tr(lang, "full page load", "تحميل كامل للصفحة")
        }
    );
    view! {
        <div class="sc-tabs">
            <nav class="sc-tabs__list" aria-label=tr(lang, "Categories", "الأقسام")>
                {tabs.into_iter().map(|c| {
                    let on = c.slug == active;
                    view! {
                        <a class="sc-tab" href=format!("/showcase?tab={}#islands-router", c.slug) aria-current=if on { "true" } else { "false" }>
                            {c.name.pick(ar).to_owned()}
                        </a>
                    }
                }).collect_view()}
            </nav>
            <div class="sc-tabs__panel">
                <p id="router-stamp" class="sc-stamp mono">{stamp}</p>
                {cx.tab.clone().map(|(c, cards)| view! {
                    <p class="sc-tabs__blurb">
                        <strong>{c.name.pick(ar).to_owned()}</strong>
                        " — "
                        <span>{c.blurb.pick(ar).to_owned()}</span>
                    </p>
                    <ul class="sc-minis" role="list">
                        {cards.into_iter().map(|card| view! { <MiniRow card lang num/> }).collect_view()}
                    </ul>
                })}
            </div>
            <div class="sc-tabs__island">
                <p class="sc-pane__label">
                    <Icon name="sparkles" size=16/>
                    <span>{tr(lang, "An island: count, then switch tabs", "‏island: عدّ وبعدين غيّر التاب")}</span>
                </p>
                <islands::IslandCounter probe=true/>
            </div>
        </div>
    }
}

// ─── Router: query-driven server rendering ─────────────────────────────────────────────────────

#[component]
fn RouterDemo(cx: Arc<Ctx>) -> impl IntoView {
    let lang = cx.lang;
    let num = cx.num;
    let ar = lang.is_rtl();
    let Some((lq, cards, total)) = cx.listing.clone() else {
        return ().into_any();
    };
    let slug = lq.category.clone().unwrap_or_default();
    let sorts = crate::api::types::Sort::ALL;
    let canonical = lq.to_query();
    let href = lq.href();
    let variant = cx.links.resolve(super::data::Href::ProductVariant);
    view! {
        <form class="sc-form" action="/showcase#router" method="get">
            <label class="sc-field">
                <span class="sc-field__label">{tr(lang, "Category", "القسم")}</span>
                <select class="sc-select" name="c">
                    {cx.cats.iter().map(|c| view! { <option value=c.slug.clone() selected=c.slug == slug>{c.name.pick(ar).to_owned()}</option> }).collect_view()}
                </select>
            </label>
            <label class="sc-field">
                <span class="sc-field__label">{tr(lang, "Sort", "الترتيب")}</span>
                <select class="sc-select" name="sort">
                    {sorts.into_iter().map(|s| view! { <option value=s.code() selected=s == lq.sort>{s.label().pick(ar).to_owned()}</option> }).collect_view()}
                </select>
            </label>
            <label class="sc-field">
                <span class="sc-field__label">{tr(lang, "Max price (EGP)", "أقصى سعر (ج.م)")}</span>
                <input class="sc-input" type="number" name="max" min="1" inputmode="numeric" value=lq.max.map(|m| m.to_string())/>
            </label>
            <label class="sc-check"><input type="checkbox" name="stock" value="1" checked=lq.in_stock/><span>{tr(lang, "In stock", "متاح")}</span></label>
            <label class="sc-check"><input type="checkbox" name="deal" value="1" checked=lq.deal/><span>{tr(lang, "Deals", "عروض")}</span></label>
            <button class="btn btn--primary btn--sm" type="submit"><span class="btn__label">{tr(lang, "Render on the server", "ارسم على السيرفر")}</span></button>
        </form>
        <dl class="sc-kvs">
            <div class="sc-kv">
                <dt>"use_query_map()"</dt>
                <dd class="mono" dir="ltr">{if cx.q.raw.is_empty() { "—".to_owned() } else { cx.q.raw.clone() }}</dd>
            </div>
            <div class="sc-kv">
                <dt>"ListingQuery::to_query()"</dt>
                <dd class="mono" dir="ltr">{if canonical.is_empty() { tr(lang, "(defaults — empty)", "(الافتراضي — فاضي)").to_owned() } else { canonical }}</dd>
            </div>
            <div class="sc-kv">
                <dt>{tr(lang, "Rendered here", "اترسم هنا")}</dt>
                <dd>{match lang {
                    Lang::En => format!("{} — first three:", format::items(total, lang, num)),
                    Lang::Ar => format!("{} — أول تلاتة:", format::items(total, lang, num)),
                }}</dd>
            </div>
        </dl>
        <ul class="sc-minis" role="list">
            {cards.into_iter().map(|card| view! { <MiniRow card lang num/> }).collect_view()}
        </ul>
        <p class="sc-note">
            {tr(lang, "The same query on the real listing: ", "نفس الاستعلام على صفحة القسم الحقيقية: ")}
            <a class="link mono" dir="ltr" href=href.clone()>{href.clone()}</a>
            {variant.map(|v| view! {
                <span>{tr(lang, " · a variant in the URL: ", " · واختيار في الـ URL: ")}</span>
                <a class="link mono" dir="ltr" href=v.clone()>{v.clone()}</a>
            })}
        </p>
    }
    .into_any()
}

// ─── Caching ───────────────────────────────────────────────────────────────────────────────────

#[component]
fn CachingDemo(cx: Arc<Ctx>) -> impl IntoView {
    use leptos::server_fn::ServerFn;
    let lang = cx.lang;
    let mut targets = vec![Probe {
        en: "Home".into(),
        ar: "الرئيسية".into(),
        url: "/".into(),
        method: "HEAD".into(),
    }];
    if let Some(c) = &cx.links.category {
        targets.push(Probe {
            en: "Category".into(),
            ar: "قسم".into(),
            url: format!("/c/{c}"),
            method: "HEAD".into(),
        });
    }
    if let Some(p) = &cx.links.product {
        targets.push(Probe {
            en: "Product".into(),
            ar: "منتج".into(),
            url: format!("/p/{p}"),
            method: "HEAD".into(),
        });
    }
    targets.push(Probe {
        en: "Cart (personal)".into(),
        ar: "السلة (شخصي)".into(),
        url: "/cart".into(),
        method: "HEAD".into(),
    });
    targets.push(Probe {
        en: "This page (personal)".into(),
        ar: "الصفحة دي (شخصي)".into(),
        url: "/showcase".into(),
        method: "HEAD".into(),
    });
    targets.push(Probe {
        en: "A server function".into(),
        ar: "دالة سيرفر".into(),
        url: <crate::api::NavCategories as ServerFn>::PATH.to_owned(),
        method: "POST".into(),
    });
    let via_cookie = cx.tenant.via == Some(TenantVia::Cookie);
    view! {
        <islands::HeaderProbe targets/>
        <ul class="sc-steps" role="list">
            <li>{tr(lang, "Catalogue pages: public, s-maxage=300 (5 min at the edge), stale-while-revalidate=60; tagged t{tenant}, t{tenant}-p{id}, t{tenant}-cat{id}.", "صفحات الكتالوج: ‏public و s-maxage=300 (٥ دقايق على الـ CDN) و stale-while-revalidate=60؛ ومتاجة بـ t{tenant} و t{tenant}-p{id} و t{tenant}-cat{id}.")}</li>
            <li>{tr(lang, "Cart, checkout, orders, this page: private, no-store — personal data never reaches a shared cache.", "السلة والدفع والطلبات والصفحة دي: ‏private, no-store — البيانات الشخصية عمرها ما بتوصل لكاش مشترك.")}</li>
            <li>{tr(lang, "x-read-cache is set by the read cache. On a streamed page it only appears when the read finished before the headers left — headers go out with the first chunk.", "‏x-read-cache بيتحط من كاش القراءة. وفي الصفحة المتبعتة بيظهر بس لو القراءة خلصت قبل ما الهيدرز تطلع — الهيدرز بتطلع مع أول chunk.")}</li>
            {via_cookie.then(|| view! {
                <li class="is-warn">{tr(lang, "You reached this shop through a cookie (no tenant in the host), so catalogue pages fall back to private: the URL alone wouldn't identify the shop for a CDN.", "انت وصلت للمتجر ده عن طريق كوكي (مفيش اسم متجر في الـ host)، فصفحات الكتالوج بقت private: الـ URL لوحده مش هيعرّف الـ CDN بالمتجر.")}</li>
            })}
        </ul>
    }
}

// ─── Multi-tenancy ─────────────────────────────────────────────────────────────────────────────

#[component]
fn TenancyDemo(cx: Arc<Ctx>) -> impl IntoView {
    let lang = cx.lang;
    let num = cx.num;
    let t = &cx.tenant;
    let via = match t.via {
        Some(TenantVia::Subdomain) => tr(
            lang,
            "the Host's first label (production path)",
            "أول جزء في الـ Host (طريقة الإنتاج)",
        ),
        Some(TenantVia::Cookie) => tr(
            lang,
            "a tenant cookie (this host has no tenant label)",
            "كوكي المتجر (الـ host ده مافيهوش اسم متجر)",
        ),
        _ => tr(
            lang,
            "the default (nothing matched)",
            "الافتراضي (مفيش حاجة طابقت)",
        ),
    };
    let express = t.express_minutes.map(|m| match lang {
        Lang::En => format!("express in {} min", format::count(m, num)),
        Lang::Ar => format!("توصيل سريع في {} دقيقة", format::count(m, num)),
    });
    let rows: Vec<(&str, String)> = vec![
        (tr(lang, "Host", "الـ Host"), t.host.clone()),
        (tr(lang, "Resolved by", "اتحدد عن طريق"), via.to_owned()),
        (
            tr(lang, "Tenant", "المتجر"),
            format!("{} · {} / {}", t.id, t.name.en, t.name.ar),
        ),
        (
            tr(lang, "Default language", "اللغة الافتراضية"),
            t.default_lang.endonym().to_owned(),
        ),
        (
            tr(lang, "Catalogue", "الكتالوج"),
            match lang {
                Lang::En => format!(
                    "{} categories · {}",
                    format::count(t.categories as u32, num),
                    format::items(t.products as u32, lang, num)
                ),
                Lang::Ar => format!(
                    "{} قسم · {}",
                    format::count(t.categories as u32, num),
                    format::items(t.products as u32, lang, num)
                ),
            },
        ),
        (tr(lang, "Delivery", "التوصيل"), {
            let mut s = match lang {
                Lang::En => format!(
                    "{} · free over {}",
                    money(t.delivery_fee, lang, num),
                    money(t.free_over, lang, num)
                ),
                Lang::Ar => format!(
                    "{} · مجاني فوق {}",
                    money(t.delivery_fee, lang, num),
                    money(t.free_over, lang, num)
                ),
            };
            if let Some(e) = &express {
                s.push_str(" · ");
                s.push_str(e);
            }
            s
        }),
        (
            tr(lang, "Order codes", "أكواد الطلبات"),
            format!("{}-48213", t.order_prefix),
        ),
        (
            tr(lang, "Cache-Tag prefix", "بادئة الـ Cache-Tag"),
            format!("t{}", t.id),
        ),
    ];
    view! {
        <dl class="sc-kvs">
            {rows.into_iter().map(|(k, v)| view! {
                <div class="sc-kv"><dt>{k}</dt><dd dir="auto">{v}</dd></div>
            }).collect_view()}
        </dl>
        {t.other.clone().map(|(_, name, href, cross)| {
            let rel = cross.then_some("external");
            view! {
                <p class="sc-row">
                    <a class="btn btn--secondary btn--sm" href=href rel=rel>
                        <Icon name="store" size=16/>
                        <span class="btn__label">{match lang {
                            Lang::En => format!("This page on {}", name.en),
                            Lang::Ar => format!("الصفحة دي على {}", name.ar),
                        }}</span>
                    </a>
                    <span class="sc-note">{if cross {
                        tr(lang, "Another subdomain is another origin: a full page load, its own cookies.", "‏subdomain تاني يعني origin تاني: تحميل كامل للصفحة، وكوكيز خاصة بيه.")
                    } else {
                        tr(lang, "Sets the tenant cookie and comes back (hosts without a tenant label).", "بيحط كوكي المتجر ويرجع (للـ hosts اللي من غير اسم متجر).")
                    }}</span>
                </p>
            }
        })}
        <p class="sc-note">{tr(lang, "Colours are one platform-wide palette; shops differ by name, logo, language and catalogue.", "الألوان باليتة واحدة للمنصة كلها؛ والمحلات بتختلف في الاسم واللوجو واللغة والكتالوج.")}</p>
    }
}

// ─── i18n ──────────────────────────────────────────────────────────────────────────────────────

#[component]
fn I18nDemo(cx: Arc<Ctx>) -> impl IntoView {
    let lang = cx.lang;
    let num = cx.num;
    let next = cx.next_here("i18n");
    let link = |key: &str, val: &str| format!("/__prefs?{key}={val}&next={next}");
    let groups: Vec<(&str, Vec<(String, String, bool)>)> = vec![
        (
            tr(lang, "Language", "اللغة"),
            vec![
                ("English".into(), link("lang", "en"), lang == Lang::En),
                ("العربية".into(), link("lang", "ar"), lang == Lang::Ar),
            ],
        ),
        (
            tr(lang, "Numerals", "الأرقام"),
            vec![
                ("123".into(), link("num", "latn"), num == Numerals::Latn),
                ("١٢٣".into(), link("num", "arab"), num == Numerals::Arab),
            ],
        ),
        (
            tr(lang, "Theme", "الثيم"),
            vec![
                (
                    tr(lang, "Light (default)", "فاتح (الافتراضي)").into(),
                    link("theme", "light"),
                    cx.theme == Theme::Light,
                ),
                (
                    tr(lang, "Dark", "غامق").into(),
                    link("theme", "dark"),
                    cx.theme == Theme::Dark,
                ),
                (
                    tr(lang, "System", "حسب الجهاز").into(),
                    link("theme", "system"),
                    cx.theme == Theme::System,
                ),
            ],
        ),
    ];
    let at = cx.now_ms + 26 * 3_600_000;
    view! {
        <div class="sc-prefs">
            {groups.into_iter().map(|(label, opts)| view! {
                <div class="sc-row">
                    <span class="sc-row__label">{label}</span>
                    <span class="sc-seg" role="group" aria-label=label>
                        {opts.into_iter().map(|(text, href, on)| view! {
                            <a class="sc-seg__opt" href=href aria-current=if on { "true" } else { "false" }>{text}</a>
                        }).collect_view()}
                    </span>
                </div>
            }).collect_view()}
        </div>
        <islands::LiveLang amount=129_950 items=3 at_ms=at/>
        <ol class="sc-steps" role="list">
            <li>{tr(lang, "A plain link to /__prefs?lang=ar&next=… sets a cookie and redirects back.", "لينك عادي لـ ‎/__prefs?lang=ar&next=…‎ بيحط كوكي ويرجّعك.")}</li>
            <li>{tr(lang, "The islands router follows the redirect and diffs the server's Arabic HTML into the page: <html lang dir> flip, the layout mirrors (logical CSS only).", "الـ islands router بيتبع التحويل ويعمل diff للـ HTML العربي: ‏<html lang dir> بيتقلبوا، والتصميم بيتعكس (CSS منطقي بس).")}</li>
            <li>{tr(lang, "Islands are skipped by the diff, so a MutationObserver on <html> copies lang/data-numerals/data-theme into the Ui store and every island re-renders its own text.", "الـ islands مش بيتعملها diff، فـ MutationObserver على <html> بينقل lang و data-numerals و data-theme للـ store بتاع الـ Ui وكل island بتعيد رسم نصها.")}</li>
            <li>{tr(lang, "Light is the default even on a dark-mode phone (owner's call); Dark and System are opt-in.", "الفاتح هو الافتراضي حتى على موبايل الوضع الغامق (قرار صاحب المنتج)؛ والغامق و«حسب الجهاز» اختياريين.")}</li>
        </ol>
    }
}

// ─── View transitions ──────────────────────────────────────────────────────────────────────────

#[component]
fn VtDemo(cx: Arc<Ctx>) -> impl IntoView {
    let lang = cx.lang;
    let num = cx.num;
    let ar = lang.is_rtl();
    let list = cx.q.vt_list;
    let opts = [
        (
            tr(lang, "Grid", "شبكة"),
            "/showcase?vt=grid#view-transitions",
            !list,
        ),
        (
            tr(lang, "List", "قائمة"),
            "/showcase?vt=list#view-transitions",
            list,
        ),
    ];
    view! {
        <div class="sc-row">
            <span class="sc-row__label">{tr(lang, "Layout", "الشكل")}</span>
            <span class="sc-seg" role="group" aria-label=tr(lang, "Layout", "الشكل")>
                {opts.into_iter().map(|(label, href, on)| view! {
                    <a class="sc-seg__opt" href=href aria-current=if on { "true" } else { "false" }>{label}</a>
                }).collect_view()}
            </span>
        </div>
        // Each tile keeps one `view-transition-name` across both layouts, so the browser morphs it
        // from its old box to its new one while the router swaps the HTML.
        <ul class=if list { "sc-vt sc-vt--list" } else { "sc-vt sc-vt--grid" } role="list">
            {cx.vt_cards.iter().enumerate().map(|(i, c)| {
                let img = c.img.clone();
                view! {
                    <li class="sc-vt__tile" style=format!("view-transition-name: sc-vt-{i}")>
                        <img src=img.src_near(240) srcset=img.srcset() sizes="96px" alt="" width="96" height="96" loading="lazy" decoding="async"/>
                        <span class="sc-vt__name">{c.name.pick(ar).to_owned()}</span>
                        <span class="sc-vt__price mono">{money(c.price, lang, num)}</span>
                    </li>
                }
            }).collect_view()}
        </ul>
        <islands::VtSupport/>
    }
}

// ─── Responsive images ─────────────────────────────────────────────────────────────────────────

#[component]
fn ImagesDemo(cx: Arc<Ctx>) -> impl IntoView {
    let lang = cx.lang;
    let num = cx.num;
    let Some(img) = cx.image.clone() else {
        return ().into_any();
    };
    let sizes = "(min-width: 768px) 320px, 72vw".to_owned();
    let ladder = img.widths.clone();
    let srcset = img.srcset();
    view! {
        <div class="sc-split sc-split--img">
            <islands::CurrentSrc img=img.clone() sizes=sizes.clone()/>
            <dl class="sc-kvs">
                <div class="sc-kv"><dt>{tr(lang, "Key", "المفتاح")}</dt><dd class="mono" dir="ltr">{img.key.clone()}</dd></div>
                <div class="sc-kv">
                    <dt>{tr(lang, "Ladder", "المقاسات")}</dt>
                    <dd class="sc-chips">{ladder.into_iter().map(|w| view! { <span class="sc-tag mono">{format!("{}w", format::count(w as u32, num))}</span> }).collect_view()}</dd>
                </div>
                <div class="sc-kv"><dt>"sizes"</dt><dd class="mono" dir="ltr">{sizes}</dd></div>
                <div class="sc-kv"><dt>"srcset"</dt><dd class="mono sc-wrap" dir="ltr">{srcset}</dd></div>
                <div class="sc-kv">
                    <dt>{tr(lang, "Aspect · alpha", "النسبة · شفافية")}</dt>
                    <dd class="mono">{format!("{} · {}", format::dec1(img.aspect, num), if img.alpha { "transparent" } else { "opaque" })}</dd>
                </div>
                <div class="sc-kv">
                    <dt>{tr(lang, "Placeholder", "الصورة المؤقتة")}</dt>
                    <dd class="sc-lqip">
                        <span class="sc-lqip__swatch" style=format!("background-color:{}", img.color) title=img.color.clone()></span>
                        <span class="sc-lqip__img" style=format!("background-image:url({})", img.lqip)></span>
                        <span class="sc-note">{match lang {
                            Lang::En => format!("{} color + a {} B blurred WebP, inline", img.color, format::int(img.lqip.len() as i64, num)),
                            Lang::Ar => format!("لون {} + صورة WebP مموهة {} بايت جوه الصفحة", img.color, format::int(img.lqip.len() as i64, num)),
                        }}</span>
                    </dd>
                </div>
            </dl>
        </div>
    }
    .into_any()
}

// ─── Accessibility ─────────────────────────────────────────────────────────────────────────────

#[component]
fn A11yDemo(cx: Arc<Ctx>) -> impl IntoView {
    let lang = cx.lang;
    view! {
        <islands::SheetDemo/>
        <ul class="sc-steps" role="list">
            <li>{tr(lang, "Modal sheets: a native <dialog> with showModal() — focus moves in, the page behind is inert, Esc closes, focus returns to the trigger. No focus-trap library.", "الشاشات المنبثقة: ‏<dialog> أصلي مع showModal()‎ — الفوكس بيدخل جواه، والصفحة اللي ورا بتتقفل، وEsc بيقفل، والفوكس بيرجع للزرار. من غير مكتبة focus-trap.")}</li>
            <li>{tr(lang, "Status messages: role=status / aria-live=polite regions (the wishlist tray above announces each change; failed sections use role=status).", "رسايل الحالة: مناطق role=status و aria-live=polite (لستة المحفوظ فوق بتعلن كل تغيير؛ والأقسام الواقعة بتستخدم role=status).")}</li>
            <li>{tr(lang, "Toggle buttons use aria-pressed, links in a set use aria-current, every icon-only button has a name, and the visible focus ring is a 3px outline.", "زراير التبديل بتستخدم aria-pressed، واللينكات في مجموعة بتستخدم aria-current، وكل زرار أيقونة ليه اسم، وحلقة الفوكس الظاهرة outline بعرض ٣ بكسل.")}</li>
            <li>{tr(lang, "Semantic structure first: one <h1>, sections named by their headings, lists as lists — which is also what search engines read.", "الهيكل الدلالي الأول: ‏<h1> واحد، وأقسام بأسماء عناوينها، والقوايم قوايم — ودي نفس الحاجة اللي محركات البحث بتقراها.")}</li>
        </ul>
    }
}

// ─── Sharp edges ───────────────────────────────────────────────────────────────────────────────

/// Things that surprised us, each verified in the Leptos 0.8.22 source or measured here, with what
/// the storefront does about it.
#[component]
pub fn SharpEdges(cx: Arc<Ctx>) -> impl IntoView {
    let lang = cx.lang;
    let items: [((&str, &str), (&str, &str), (&str, &str)); 12] = [
        (
            (
                "Out-of-order streaming needs JavaScript",
                "البث بأي ترتيب محتاج JavaScript",
            ),
            (
                "Each resolved <Suspense> arrives as a <template> plus an inline <script> that moves it into place (tachys ssr::OooChunk). With JS off those sections stay fallbacks forever.",
                "كل <Suspense> بيخلص بيوصل كـ <template> ومعاه <script> بيحطه مكانه (‏tachys ssr::OooChunk). ولو الـ JS مقفول الأقسام دي بتفضل placeholders للأبد.",
            ),
            (
                "Per-route modes (see SSR modes); /nojs serves the home page in order via a <noscript> refresh.",
                "طريقة لكل route (شوف «طرق الـ SSR»)؛ و ‎/nojs‎ بيقدّم الرئيسية بالترتيب عن طريق <noscript> refresh.",
            ),
        ),
        (
            (
                "A boundary around Suspense blocks the stream",
                "الـ boundary حوالين Suspense بيوقف البث",
            ),
            (
                "An <ErrorBoundary> that contains a <Suspense> waits for it before sending anything (it must know whether to show the fallback), so everything inside — and after it — streams in order. A route-level boundary would hold the whole page.",
                "‏<ErrorBoundary> فيه <Suspense> بيستناه قبل ما يبعت أي حاجة (لازم يعرف هيعرض البديل ولا لأ)، فكل اللي جواه — واللي بعده — بيتبعت بالترتيب. والـ boundary على مستوى الـ route هيمسك الصفحة كلها.",
            ),
            (
                "Always <Suspense> outside, <ErrorBoundary> inside, per section; no route-level boundary around streamed content.",
                "دايمًا <Suspense> برّه و <ErrorBoundary> جوه، لكل قسم؛ ومفيش boundary على مستوى الـ route حوالين محتوى متبعت.",
            ),
        ),
        (
            (
                "Client navigations wait for the slowest section",
                "التنقل بيستنى أبطأ قسم",
            ),
            (
                "The islands router fetches the whole next page; leptos_axum renders it in order for that request, and the diff runs after the last byte. One slow widget slows every navigation to that page.",
                "الـ islands router بيجيب الصفحة الجاية كلها؛ و leptos_axum بيرسمها بالترتيب للطلب ده، والـ diff بيحصل بعد آخر بايت. ‏widget واحد بطيء بيبطّأ كل تنقل للصفحة دي.",
            ),
            (
                "Keep slow, non-essential data in islands that fetch after load, or behind caches; the islands-router card measures each navigation.",
                "خلي البيانات البطيئة اللي مش أساسية في islands بتجيبها بعد التحميل، أو ورا كاش؛ وكارت الـ islands-router بيقيس كل تنقل.",
            ),
        ),
        (
            (
                "Same-page #anchors become navigations",
                "لينكات ‎#anchor‎ بتتحول لتنقل",
            ),
            (
                "The router intercepts every same-origin <a>, including href=\"#…\": it re-fetches the page and never scrolls.",
                "الراوتر بيعترض أي <a> على نفس الموقع، حتى href=\"#…\": بيجيب الصفحة تاني ومابيعملش scroll.",
            ),
            (
                "This page stops those clicks before they reach the router (TocFilter); the app shell should do it globally (skip link too).",
                "الصفحة دي بتوقف الضغطات دي قبل ما توصل للراوتر (‏TocFilter)؛ والـ shell المفروض يعملها لكل الصفحات (ولينك «تخطّى للمحتوى» كمان).",
            ),
        ),
        (
            (
                "The diff adds attributes, never removes them",
                "الـ diff بيضيف attributes ومابيشيلهاش",
            ),
            (
                "diffElement copies the new page's attributes onto the old element but doesn't delete missing ones: render aria-current=\"false\", not nothing.",
                "‏diffElement بينسخ attributes الصفحة الجديدة على العنصر القديم بس مابيمسحش اللي ناقص: اكتب aria-current=\"false\" مش تسيبه.",
            ),
            (
                "Stateful attributes here always render a value.",
                "الـ attributes اللي ليها حالة هنا دايمًا ليها قيمة.",
            ),
        ),
        (
            ("No scroll handling on navigation", "مفيش scroll مع التنقل"),
            (
                "After a client navigation the new page opens at the old scroll position — the router diffs and calls pushState, nothing else.",
                "بعد التنقل الصفحة الجديدة بتفتح في نفس مكان الـ scroll القديم — الراوتر بيعمل diff و pushState وبس.",
            ),
            (
                "A small shell script is needed: scroll to the top on a new path, restore on Back.",
                "محتاجين سكريبت صغير في الـ shell: يطلع لفوق مع مسار جديد، ويرجّع المكان مع الرجوع.",
            ),
        ),
        (
            (
                "Action results land in completion order",
                "نتايج الـ Action بتيجي بترتيب ما بتخلص",
            ),
            (
                "Action::dispatch never advances its own 'latest' counter (reactive_graph actions/action.rs), so with two requests in flight the slower one's result wins — even if it was sent first. A fast quantity stepper can end on a stale cart.",
                "‏Action::dispatch عمره ما بيزوّد عدّاد «الأحدث» بتاعه (‏reactive_graph actions/action.rs)، فلو فيه طلبين شغالين اللي يخلص متأخر هو اللي بيكسب — حتى لو كان اتبعت الأول. وعدّاد الكمية لو اتداس بسرعة ممكن يقف على سلة قديمة.",
            ),
            (
                "Cart mutations go through one serialized queue with optimistic updates (the commerce branch's CartOps).",
                "تعديلات السلة بتعدّي على طابور واحد بالترتيب مع تحديث متفائل (‏CartOps في فرع الشراء).",
            ),
        ),
        (
            (
                "Server-function arguments: empty lists vanish",
                "وسايط دوال السيرفر: اللستات الفاضية بتختفي",
            ),
            (
                "With the default URL encoding (serde_qs), an empty Vec inside a struct argument isn't sent at all, and the server rejects the call: api::list_products(ListingQuery { brands: vec![], .. }) fails from the browser with “missing field brands”, while the same call works during SSR, where nothing is encoded.",
                "مع الـ URL encoding الافتراضي (‏serde_qs)، الـ Vec الفاضي جوه struct مش بيتبعت خالص، والسيرفر بيرفض الطلب: ‏api::list_products(ListingQuery { brands: vec![], .. }) بيفشل من المتصفح بـ «missing field brands»، مع إن نفس النداء بيشتغل وقت الـ SSR لإن مفيش encoding.",
            ),
            (
                "#[serde(default)] on such fields (reported for ListingQuery), a JSON input encoding, or flat arguments — this page's pager uses pager_page(page).",
                "‏#[serde(default)] على الحقول دي (اتبلّغ عنه في ListingQuery)، أو JSON encoding، أو وسايط بسيطة — والـ pager في الصفحة دي بيستخدم pager_page(page).",
            ),
        ),
        (
            (
                "Dev builds aren't release builds",
                "نسخة التطوير مش نسخة الإصدار",
            ),
            (
                "cargo leptos watch compiles with --cfg erase_components (every view becomes an AnyView, so every node gets an islands-router branch marker) and adds hot-reload comments. Dev HTML and WASM are several times larger than release; the numbers on this dev server overstate both.",
                "‏cargo leptos watch بيبني بـ ‎--cfg erase_components‎ (كل view بيبقى AnyView، فكل عنصر بياخد branch marker للـ islands router) وبيضيف تعليقات hot-reload. الـ HTML والـ WASM في التطوير أكبر بكتير من الإصدار؛ والأرقام على السيرفر ده مبالغ فيها.",
            ),
            (
                "Judge size and speed on cargo leptos build --release (tools/wasm-size.mjs for the bundle).",
                "احكم على الحجم والسرعة من cargo leptos build --release (‏tools/wasm-size.mjs للـ bundle).",
            ),
        ),
        (
            (
                "Islands live outside the router",
                "الـ islands عايشة برّه الراوتر",
            ),
            (
                "Inside an island there is no router context (use_query_map, use_navigate) and the diff never updates its contents — text inside an island won't follow a server-side language switch by itself.",
                "جوه الـ island مفيش context للراوتر (‏use_query_map و use_navigate) والـ diff مابيحدّثش محتواها — فالنص اللي جواها مش هيتبع تغيير اللغة من السيرفر لوحده.",
            ),
            (
                "Islands read window.location / history directly, and render text from the shared Ui store (fed by the <html> MutationObserver).",
                "الـ islands بتقرا window.location و history مباشرة، وبترسم نصها من store الـ Ui المشترك (اللي بيتغذى من الـ MutationObserver على <html>).",
            ),
        ),
        (
            ("Version churn ahead", "تغييرات جاية في الإصدارات"),
            (
                "0.8.22 (5 Oct 2026) is the latest stable; 0.9.0-rc0 shipped on 15 Oct, and the islands_router example's README still opens with “Work in Progress”.",
                "‏0.8.22 (٥ أكتوبر ٢٠٢٦) هو آخر إصدار مستقر؛ و 0.9.0-rc0 نزل يوم ١٥ أكتوبر، والـ README بتاع مثال islands_router لسه بيبدأ بـ «Work in Progress».",
            ),
            (
                "Pin exact versions; budget an upgrade pass for 0.9.",
                "ثبّت الإصدارات بالظبط؛ وخلي فيه وقت للترقية لـ 0.9.",
            ),
        ),
        (
            (
                "Tooling: one build cache, many worktrees",
                "الأدوات: كاش بناء واحد لكذا worktree",
            ),
            (
                "With a shared Cargo build-dir, parallel git worktrees produced the same artifact names and linked each other's app code — a server could run another branch's pages.",
                "لما الـ build-dir بتاع Cargo كان مشترك، الـ worktrees اللي شغالة مع بعض طلّعت نفس أسماء الملفات واتربطت بكود بعض — يعني سيرفر ممكن يشغّل صفحات فرع تاني.",
            ),
            (
                "build-dir = \".../lsp-build/{workspace-path-hash}\" in .cargo/config.toml: one cache per worktree.",
                "‏build-dir = \".../lsp-build/{workspace-path-hash}\" في ‎.cargo/config.toml‎: كاش لكل worktree.",
            ),
        ),
    ];
    view! {
        <section id="sharp-edges" class="sc-edges" aria-labelledby="sharp-edges-title">
            <header class="sc-cat__head">
                <h2 id="sharp-edges-title" class="sc-cat__title">{tr(lang, "Sharp edges we hit", "حاجات جرحتنا")}</h2>
                <p class="sc-cat__blurb">{tr(lang, "What to know before betting on it — each one checked against the Leptos 0.8.22 source or measured on this app.", "اللي لازم تعرفه قبل ما تراهن عليه — كل واحدة متشيّكة على كود Leptos 0.8.22 أو متقاسة على الأبلكيشن ده.")}</p>
            </header>
            <ol class="sc-edges__list" role="list">
                {items.into_iter().map(|(t, what, fix)| view! {
                    <li class="sc-edge">
                        <h3 class="sc-edge__title">{tr(lang, t.0, t.1)}</h3>
                        <p>{tr(lang, what.0, what.1)}</p>
                        <p class="sc-edge__fix"><Icon name="check" size=16/><span>{tr(lang, fix.0, fix.1)}</span></p>
                    </li>
                }).collect_view()}
            </ol>
        </section>
    }
}
