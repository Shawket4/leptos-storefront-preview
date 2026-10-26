//! The server-rendered part of each card's live demo. Interactive pieces are islands
//! (`islands.rs`); everything here is plain HTML from the server.

use super::Ctx;
use super::islands::{self, Probe, SaveItem, StreamTarget, Unit};
use crate::api::{self, types::Rail};
use crate::components::icons::Icon;
use crate::components::product_card::CardData;
use crate::components::states::{ComingSoon, StateTone, StateView};
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
        "states" => view! { <StatesDemo cx/> }.into_any(),
        "meta" => view! { <MetaDemo cx/> }.into_any(),
        "reactivity" => view! { <ReactivityDemo cx/> }.into_any(),
        "shared-state" => view! { <SharedStateDemo cx/> }.into_any(),
        "server-fns" => view! { <ServerFnsDemo cx/> }.into_any(),
        "transition" => view! { <TransitionDemo cx/> }.into_any(),
        "sim" => view! { <SimDemo cx/> }.into_any(),
        "action-form" => view! { <ActionFormDemo cx/> }.into_any(),
        "islands-router" => view! { <RouterTabsDemo cx/> }.into_any(),
        "router" => view! { <RouterDemo cx/> }.into_any(),
        "router-patches" => view! { <PatchesDemo cx/> }.into_any(),
        "motion" => view! { <MotionDemo cx/> }.into_any(),
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
    let yes = ("yes", "أيوه");
    let no = ("no", "لأ");
    // (mode, where it's used here, first byte, needs JS to show streamed parts, <head> from data)
    let modes: [(&str, (&str, &str), (&str, &str), bool, (&str, &str)); 4] = [
        (
            "OutOfOrder",
            ("/ (home), /showcase", "/ (الرئيسية)، ‎/showcase‎"),
            (
                "at once; sections land as their data does",
                "فورًا؛ والأقسام بتوصل مع بياناتها",
            ),
            true,
            no,
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
            no,
        ),
        (
            "PartiallyBlocked",
            ("/p/… (product)", "‏/p/…‎ (المنتج)"),
            (
                "after its blocking resources",
                "بعد الـ blocking resources بتاعته",
            ),
            // leptos_axum 0.8.22: `_ = replace_blocks; // TODO` — blocks aren't spliced in.
            true,
            ("not in 0.8.22", "لأ، مش في 0.8.22"),
        ),
        (
            "Async",
            ("not used", "مش مستخدم"),
            ("after everything", "بعد ما كله يخلص"),
            false,
            yes,
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
    view! {
        <ul class="sc-modes" role="list">
            {modes.into_iter().map(|(m, used, first, js, head)| view! {
                <li class="sc-mode" class:is-unused=m == "Async">
                    <code class="sc-mode__name mono" dir="ltr">{format!("SsrMode::{m}")}</code>
                    <dl class="sc-mode__kv">
                        <div><dt>{tr(lang, "Used for", "مستخدم في")}</dt><dd>{tr(lang, used.0, used.1)}</dd></div>
                        <div><dt>{tr(lang, "First byte", "أول بايت")}</dt><dd>{tr(lang, first.0, first.1)}</dd></div>
                        <div><dt>{tr(lang, "Streamed parts need JS", "الأجزاء المتبعتة محتاجة JS")}</dt><dd class:is-warn=js>{if js { tr(lang, yes.0, yes.1) } else { tr(lang, no.0, no.1) }}</dd></div>
                        <div><dt>{tr(lang, "<head> can use data", "الـ <head> يقدر يستخدم البيانات")}</dt><dd>{tr(lang, head.0, head.1)}</dd></div>
                    </dl>
                </li>
            }).collect_view()}
        </ul>
        <p class="sc-note">
            {tr(
                lang,
                "Modes are per route, not per request — a server can't tell whether the visitor has JavaScript. Client navigations by the islands router always get in-order HTML (leptos_axum switches off out-of-order when it sees the Islands-Router header), so such a navigation waits for the page's slowest section. PartiallyBlocked in 0.8.22 only delays the first byte: leptos_axum leaves block replacement as a TODO, so the product page keeps its product inside a keyed <For> (resolved before it renders) and builds its <head> without waiting for data.",
                "الطريقة بتتحدد لكل route، مش لكل طلب — السيرفر مايعرفش الزائر عنده JavaScript ولا لأ. والتنقلات اللي بيعملها الـ islands router بتاخد HTML بالترتيب دايمًا (‏leptos_axum بيقفل الـ out-of-order لما يشوف هيدر Islands-Router)، فالتنقل ده بيستنى أبطأ قسم في الصفحة. و PartiallyBlocked في 0.8.22 بيأخر أول بايت وبس: ‏leptos_axum سايب استبدال البلوكات TODO، فصفحة المنتج حاطة المنتج جوه <For> بمفتاح (بيخلص قبل ما يترسم) وبتبني الـ <head> من غير ما تستنى البيانات.",
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
        <LatencyCallout cx=cx.clone() anchor="streaming" external=true
            see=("Turn it on to watch the blocks arrive out of order.", "شغّله عشان تشوف البلوكات بتوصل بأي ترتيب.")/>
        <p class="sc-note">
            {mode}
            " "
            <a class="sc-link" href=replay rel="external">
                <Icon name="rotate-ccw" size=16/>
                <span>{tr(lang, "Replay with a full load", "أعد بتحميل كامل")}</span>
            </a>
        </p>
        <div class="sc-sblocks">
            <StreamBlock cx=cx.clone() rail=Rail::Deals label=("Today's deals", "عروض النهارده")/>
            <StreamBlock cx=cx.clone() rail=Rail::Featured label=("Picked for you", "اخترنالك")/>
            <StreamBlock cx=cx.clone() rail=Rail::New label=("New arrivals", "وصل حديثًا")/>
            <BlockingBlock cx=cx.clone()/>
        </div>
        <p class="sc-note">
            {match lang {
                Lang::En => format!(
                    "Blocks are in page order; the badge is the order the server finished them in. Each waits {} ms × its widget's factor of simulated backend time.",
                    format::int(cx.base_ms as i64, num)
                ),
                Lang::Ar => format!(
                    "البلوكات بترتيب الصفحة؛ والرقم هو ترتيب ما السيرفر خلّصهم. كل واحد بيستنى {} ms × معامل الـ widget بتاعه كزمن باك إند تجريبي.",
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
    let opts: Vec<(Option<u64>, String)> = [0u64, 350, 1200]
        .into_iter()
        .map(|v| {
            let label = if v == 0 {
                tr(lang, "Off", "مقفول").to_owned()
            } else {
                format!("{} ms", format::int(v as i64, num))
            };
            (Some(v), label)
        })
        .collect();
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

/// "Simulated latency is off — turn it on to see this", with the control right there. Latency is
/// off by default (owner); it is a per-visitor cookie set through `/__sim`.
#[component]
fn LatencyCallout(
    cx: Arc<Ctx>,
    /// The anchor to come back to.
    anchor: &'static str,
    /// What becomes visible with latency on (EN, AR).
    see: (&'static str, &'static str),
    /// Full page load after switching (needed to watch out-of-order streaming).
    #[prop(optional)]
    external: bool,
) -> impl IntoView {
    let lang = cx.lang;
    let num = cx.num;
    let next = cx.next_here(anchor);
    let off = cx.base_ms == 0;
    let state = if off {
        tr(
            lang,
            "Simulated latency is off: every read answers at once.",
            "الزمن التجريبي مقفول: كل قراءة بترد على طول.",
        )
        .to_owned()
    } else {
        let source = if cx.sim_cookie {
            tr(lang, "your cookie", "الكوكي بتاعتك")
        } else {
            tr(lang, "server default", "الافتراضي بتاع السيرفر")
        };
        match lang {
            Lang::En => format!(
                "Simulated latency: {} ms ({source}).",
                format::int(cx.base_ms as i64, num)
            ),
            Lang::Ar => format!(
                "الزمن التجريبي: {} ms (‏{source}).",
                format::int(cx.base_ms as i64, num)
            ),
        }
    };
    view! {
        <div class="sc-callout" class:is-off=off>
            <p class="sc-callout__text">
                <Icon name="timer" size=18/>
                <span>
                    <strong>{state}</strong>
                    " "
                    {if off { tr(lang, see.0, see.1) } else { "" }}
                </span>
            </p>
            <LatencyLinks cx=cx.clone() next=next external/>
        </div>
    }
}

// ─── <Transition> ──────────────────────────────────────────────────────────────────────────────

#[component]
fn TransitionDemo(cx: Arc<Ctx>) -> impl IntoView {
    view! {
        <LatencyCallout cx=cx.clone() anchor="transition"
            see=("Turn it on to watch the old page stay, dimmed, while the next one loads.", "شغّله عشان تشوف الصفحة القديمة فاضلة ومعتّمة لحد ما الجديدة تحمّل.")/>
        <islands::ListingPager pages=cx.pager_pages first=cx.pager_first.clone()/>
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
    // Products from this shop's catalogue priced under the free-delivery line (see build_ctx).
    let source = if cx.calc_units.is_empty() {
        &cx.picks
    } else {
        &cx.calc_units
    };
    let units: Vec<Unit> = source
        .iter()
        .map(|c| Unit {
            en: c.name.en.clone(),
            ar: c.name.ar.clone(),
            price: c.price.0,
        })
        .collect();
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
        <LatencyCallout cx=cx.clone() anchor="server-fns"
            see=("Turn it on to feel each round trip (a cache hit skips only the pretend database, CACHE_MISS_MS, which is off here too).", "شغّله عشان تحس بكل رحلة للسيرفر (الـ hit بيوفّر بس وقت قاعدة البيانات التجريبية CACHE_MISS_MS، وده مقفول هنا كمان).")/>
        <islands::CacheLab initial=cx.cache.clone() tenant=t.clone()/>
        <ul class="sc-steps">
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
    let source = if cx.sim_cookie {
        tr(lang, "your cookie", "الكوكي بتاعتك")
    } else {
        tr(lang, "the server default", "الافتراضي بتاع السيرفر")
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
        <ol class="sc-steps">
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
        <ul class="sc-steps">
            <li>{tr(lang, "Catalogue pages, for browsers: Cache-Control: no-cache (always revalidate — a stale page after a language switch would be a bug) and Vary: Cookie.", "صفحات الكتالوج للمتصفح: ‏Cache-Control: no-cache (اتأكد كل مرة — صفحة قديمة بعد تغيير اللغة تبقى غلطة) و Vary: Cookie.")}</li>
            <li>{tr(lang, "For the edge: CDN-Cache-Control: public, s-maxage=300, stale-while-revalidate=60, tagged t{tenant}, t{tenant}-p{id}, t{tenant}-cat{id}. Cloudflare ignores Vary: Cookie, so its cache key must include the lang, num and theme cookies; the Host already separates shops.", "للـ CDN: ‏CDN-Cache-Control: public, s-maxage=300, stale-while-revalidate=60 ومتاجة بـ t{tenant} و t{tenant}-p{id} و t{tenant}-cat{id}. و Cloudflare بيتجاهل Vary: Cookie، فمفتاح الكاش لازم يشمل كوكيز lang و num و theme؛ والـ Host أصلًا بيفصل المحلات.")}</li>
            <li>{tr(lang, "Cart, checkout, orders, this page: private, no-store — personal data never reaches a shared cache.", "السلة والدفع والطلبات والصفحة دي: ‏private, no-store — البيانات الشخصية عمرها ما بتوصل لكاش مشترك.")}</li>
            <li>{tr(lang, "x-read-cache is set by the read cache. On a streamed page it only appears when the read finished before the headers left — headers go out with the first chunk.", "‏x-read-cache بيتحط من كاش القراءة. وفي الصفحة المتبعتة بيظهر بس لو القراءة خلصت قبل ما الهيدرز تطلع — الهيدرز بتطلع مع أول chunk.")}</li>
            {via_cookie.then(|| view! {
                <li class="is-warn">{tr(lang, "You reached this shop through a cookie (no tenant in the host), so catalogue pages get no CDN-Cache-Control: the URL alone wouldn't identify the shop for a CDN.", "انت وصلت للمتجر ده عن طريق كوكي (مفيش اسم متجر في الـ host)، فصفحات الكتالوج مابتاخدش CDN-Cache-Control: الـ URL لوحده مش هيعرّف الـ CDN بالمتجر.")}</li>
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
        <ol class="sc-steps">
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
        <ul class="sc-steps">
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
    let items: [((&str, &str), (&str, &str), (&str, &str)); 18] = [
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
                "PartiallyBlocked only delays the first byte",
                "الـ PartiallyBlocked بيأخر أول بايت وبس",
            ),
            (
                "leptos_axum 0.8.22 doesn't splice blocking resources into the first chunk (render_app_to_stream_with_context_and_replace_blocks opens with `_ = replace_blocks; // TODO`): the stream only waits for them, and a <Suspense> over one still ships as an out-of-order template + script — without JavaScript it stays on its fallback.",
                "‏leptos_axum 0.8.22 مابيحطش الـ blocking resources جوه أول chunk (الدالة render_app_to_stream_with_context_and_replace_blocks بتبدأ بـ ‎`_ = replace_blocks; // TODO`‎): البث بيستناهم وبس، والـ <Suspense> اللي فوق واحد منهم بيتبعت برضه كـ template و script بأي ترتيب — ومن غير JavaScript بيفضل على الـ fallback.",
            ),
            (
                "The product page keeps its product inside a keyed <For>, whose items resolve before they render, so it arrives inline; related products stream on their own.",
                "صفحة المنتج حاطة المنتج جوه <For> بمفتاح، وعناصره بتخلص قبل ما تترسم، فبيوصل جوه الصفحة على طول؛ والمنتجات المشابهة بتتبعت لوحدها.",
            ),
        ),
        (
            (
                "Head tags inside <Suspense> never reach <head>",
                "تاجات الـ head جوه <Suspense> عمرها ما بتوصل للـ <head>",
            ),
            (
                "leptos_meta writes into <head> only what was registered when the first chunk is ready (inject_meta_context). A <Title> or <Meta> rendered inside a <Suspense> — even over a blocking resource — registers later and is silently dropped.",
                "‏leptos_meta بيكتب في الـ <head> اللي اتسجل لحد ما أول chunk يجهز بس (‏inject_meta_context). و <Title> أو <Meta> جوه <Suspense> — حتى لو فوق blocking resource — بيتسجل بعدها وبيضيع من غير أي رسالة.",
            ),
            (
                "Head tags are built synchronously at the top of each page from in-memory data (product, category), never from a resource.",
                "تاجات الـ head بتتبني على طول في أول كل صفحة من بيانات في الذاكرة (المنتج والقسم)، ومش من resource أبدًا.",
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
                "The router intercepts every same-origin <a>, including href=\"#…\": it re-fetches the page and never scrolls. Leaving the click to the browser isn't enough either — the hash change fires popstate, and the router answers that with a full re-fetch too.",
                "الراوتر بيعترض أي <a> على نفس الموقع، حتى href=\"#…\": بيجيب الصفحة تاني ومابيعملش scroll. وإنك تسيب الضغطة للمتصفح مش كفاية — تغيير الـ hash بيطلّع popstate، والراوتر بيرد عليه بإعادة تحميل الصفحة من السيرفر برضه.",
            ),
            (
                "This page handles in-page links itself (scroll, focus, replaceState — no popstate) before the router sees them; the app shell should do the same globally (the skip link too).",
                "الصفحة دي بتتعامل مع اللينكات الداخلية بنفسها (scroll وفوكس و replaceState — من غير popstate) قبل ما الراوتر يشوفها؛ والـ shell المفروض يعمل كده لكل الصفحات (ولينك «تخطّى للمحتوى» كمان).",
            ),
        ),
        (
            (
                "Cross-page diffs can slip inside <head>",
                "الـ diff بين صفحتين ممكن يتلخبط في الـ <head>",
            ),
            (
                "The diff walks the old and new documents in lockstep, <head> included. Two pages with a different number of head tags (title, meta, JSON-LD) knock the walk out of step, and the next page's content never makes it to the screen — the URL changes, the page doesn't.",
                "الـ diff بيمشي على المستند القديم والجديد خطوة بخطوة، والـ <head> منهم. صفحتين فيهم عدد مختلف من تاجات الـ head (العنوان و meta و JSON-LD) بيلخبطوا الخطوات، ومحتوى الصفحة الجديدة مابيوصلش للشاشة — الـ URL بيتغير والصفحة لأ.",
            ),
            (
                "shell.js applies the new page's head tags itself and hands the router HTML whose <head> matches the live one.",
                "‏shell.js بيطبّق تاجات الـ head بتاعة الصفحة الجديدة بنفسه، ويدي الراوتر HTML الـ <head> بتاعه مطابق للي على الشاشة.",
            ),
        ),
        (
            (
                "<noscript> comes alive in fetched pages",
                "الـ <noscript> بيصحى في الصفحات اللي بتتجاب",
            ),
            (
                "The router parses fetched HTML with DOMParser, where scripting is off, so <noscript> content is real markup — and the diff moves it into the live page. A <noscript><meta http-equiv=refresh> sent every JS visitor clicking “Home” to the no-JS page with a full reload.",
                "الراوتر بيقرا الـ HTML اللي جابه بـ DOMParser، واللي الـ scripting فيه مقفول، فمحتوى الـ <noscript> بيبقى عناصر حقيقية — والـ diff بينقلها للصفحة. ‏<noscript><meta http-equiv=refresh> كان بيبعت أي حد عنده JS داس على «الرئيسية» لصفحة الـ no-JS بتحميل كامل.",
            ),
            (
                "The home page's no-JS fallback is now a <noscript> link to /nojs, never sent on router responses, and shell.js empties <noscript> on both sides before diffing.",
                "البديل بتاع الـ no-JS في الرئيسية بقى <noscript> فيه لينك لـ ‎/nojs‎، وعمره ما بيتبعت في ردود الراوتر، و shell.js بيفضّي الـ <noscript> في الناحيتين قبل الـ diff.",
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
                "Without help, a client navigation opens the new page at the old scroll position — the router diffs and calls pushState, nothing else.",
                "من غير مساعدة، التنقل بيفتح الصفحة الجديدة في نفس مكان الـ scroll القديم — الراوتر بيعمل diff و pushState وبس.",
            ),
            (
                "shell.js: a new path scrolls to the top, #hash to its section, Back/Forward to the saved position (history.scrollRestoration = manual).",
                "‏shell.js: المسار الجديد بيطلع لفوق، والـ ‎#hash‎ بيروح لقسمه، والرجوع والتقدم بيرجعوا للمكان المحفوظ (‏history.scrollRestoration = manual).",
            ),
        ),
        (
            (
                "Router form posts drop the pressed button",
                "فورمز الراوتر بتنسى الزرار اللي اتداس",
            ),
            (
                "The islands router builds the request from new FormData(form) without the submitter (islands_routing.js, submitToReq), so <button name=\"op\" value=\"remove\"> never reaches the server with JavaScript on — it does without.",
                "الـ islands router بيبني الطلب من new FormData(form)‎ من غير الـ submitter (‏islands_routing.js، ‏submitToReq)، فـ <button name=\"op\" value=\"remove\"> عمره ما بيوصل للسيرفر والـ JavaScript شغال — مع إنه بيوصل من غيره.",
            ),
            (
                "shell.js adds the pressed button as a hidden field for that one submission (found on the cart page).",
                "‏shell.js بيضيف الزرار اللي اتداس كخانة مخفية للإرسال ده بس (اتلقت في صفحة السلة).",
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
                "Cart mutations go through one serialized queue with optimistic updates (CartOps in pages/cart/store.rs).",
                "تعديلات السلة بتعدّي على طابور واحد بالترتيب مع تحديث متفائل (‏CartOps في pages/cart/store.rs).",
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
                "#[serde(default)] on the struct — ListingQuery has it now — or a JSON input encoding, or flat arguments.",
                "‏#[serde(default)] على الـ struct — و ListingQuery بقى عليه — أو JSON encoding، أو وسايط بسيطة.",
            ),
        ),
        (
            (
                "Dev builds aren't release builds",
                "نسخة التطوير مش نسخة الإصدار",
            ),
            (
                "cargo leptos watch compiles with --cfg erase_components (every view becomes an AnyView, so nearly every node gets an islands-router branch marker) and adds hot-reload comments. Measured on 26 Oct: the home page is ~650–750 KB of HTML with ~7,600–9,000 markers in dev, but 213 KB with 1,014 markers (20 KB, 9%) in release; the WASM is ~12 MB in dev and 1.13 MB raw / 326 KB brotli in release.",
                "‏cargo leptos watch بيبني بـ ‎--cfg erase_components‎ (كل view بيبقى AnyView، فتقريبًا كل عنصر بياخد branch marker للـ islands router) وبيضيف تعليقات hot-reload. قسناها يوم ٢٦ أكتوبر: الرئيسية حوالي ٦٥٠–٧٥٠ KB HTML وفيها ٧٦٠٠–٩٠٠٠ marker في التطوير، لكن ٢١٣ KB و ١٠١٤ marker (٢٠ KB، ٩٪) في الإصدار؛ والـ WASM حوالي ١٢ MB في التطوير و ١٫١٣ MB خام / ٣٢٦ KB بـ brotli في الإصدار.",
            ),
            (
                "Judge size and speed on cargo leptos build --release (tools/wasm-size.mjs for the bundle).",
                "احكم على الحجم والسرعة من cargo leptos build --release (‏tools/wasm-size.mjs للـ bundle).",
            ),
        ),
        (
            (
                "A pending <Suspense> inside an island breaks client navigation",
                "‏<Suspense> مستني جوه island بيكسر التنقل",
            ),
            (
                "An island whose <Suspense>/<Transition> isn't ready when it hydrates (a Resource or LocalResource inside it) hydrates fine on a full load, but after an islands-router navigation into the page it panics: its hydration depends on inline scripts (serialized resources, incomplete-chunk markers) that the router never runs, and on the page-wide hydration-id sequence. The panic then takes down every island hydrated after it on that page.",
                "الـ island اللي الـ <Suspense>/<Transition> بتاعها مش جاهز وقت الـ hydration (‏Resource أو LocalResource جواها) بتصحى عادي في التحميل الكامل، لكن بعد التنقل للصفحة بالـ islands router بتعمل panic: الـ hydration بتاعتها معتمدة على سكريبتات جوه الصفحة (الـ resources المتسلسلة وعلامات الـ chunks الناقصة) والراوتر عمره ما بيشغّلها، ومعتمدة كمان على تسلسل أرقام الـ hydration في الصفحة كلها. والـ panic بيوقّع كل الـ islands اللي بعدها في نفس الصفحة.",
            ),
            (
                "Seed from props: an AsyncDerived that is ready on its first poll (the listing explorer and this page's pager), so server and client render the same branch with no hydration data.",
                "ابدأ من الـ props: ‏AsyncDerived جاهز من أول poll (زي explorer القوايم والـ pager في الصفحة دي)، فالسيرفر والمتصفح بيرسموا نفس الفرع من غير بيانات hydration.",
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

// ─── islands-router patches ────────────────────────────────────────────────────────────────────

#[component]
fn PatchesDemo(cx: Arc<Ctx>) -> impl IntoView {
    let lang = cx.lang;
    view! {
        <islands::PatchProbe/>
        <ol class="sc-steps">
            <li>{tr(lang, "Head realignment: before the router diffs, shell.js applies the new page's title/meta/JSON-LD itself and splices the live <head> into the fetched HTML, so the lockstep walk reaches <body> in step.", "ظبط الـ head: قبل الـ diff، ‏shell.js بيطبّق العنوان و meta و JSON-LD بتوع الصفحة الجديدة بنفسه، ويحط الـ <head> الحالي في الـ HTML اللي اتجاب، فالمشي خطوة بخطوة يوصل للـ <body> مظبوط.")}</li>
            <li>{tr(lang, "Inert <noscript>: emptied in the page and in fetched HTML; the server also leaves the home page's no-JS link out when the request carries Islands-Router (lib.rs).", "‏<noscript> مالوش أثر: بيتفضّى في الصفحة وفي الـ HTML اللي بيتجاب؛ والسيرفر كمان مابيحطش لينك الـ no-JS بتاع الرئيسية لما الطلب فيه Islands-Router (‏lib.rs).")}</li>
            <li>{tr(lang, "Scroll: new path → top, #hash → its section, Back/Forward → the saved position, all applied inside the view transition right after the swap.", "الـ scroll: مسار جديد → لفوق، و ‎#hash‎ → قسمه، والرجوع والتقدم → المكان المحفوظ، وكله بيتطبق جوه الـ view transition بعد التبديل على طول.")}</li>
            <li>{tr(lang, "Revalidation: router fetches use cache: no-cache — pages vary by the language/theme cookies, and a stale copy would answer in the old language.", "التأكد من السيرفر: طلبات الراوتر بتستخدم cache: no-cache — الصفحات بتختلف حسب كوكيز اللغة والثيم، والنسخة القديمة هترد باللغة القديمة.")}</li>
            <li>{tr(lang, "The pressed button: the router posts new FormData(form), which leaves out the clicked <button name value>; shell.js adds it as a hidden field for that one submission.", "الزرار اللي اتداس: الراوتر بيبعت new FormData(form)‎ اللي مافيهوش الـ <button name value> اللي اتداس؛ و shell.js بيضيفه كخانة مخفية للإرسال ده بس.")}</li>
            <li>{tr(lang, "Prefetch on intent: a hover (after a short pause), keyboard focus or a press warms the router's own GET, used once by the click that follows; nothing on Save-Data or 2G, and never a /__ preference link.", "التجهيز قبل الضغط: الـ hover (بعد وقفة قصيرة) أو فوكس الكيبورد أو اللمس بيجهّز نفس طلب الراوتر، والضغطة اللي بعدها بتستخدمه مرة واحدة؛ ومفيش تجهيز مع Save-Data أو 2G، ولا للينكات التفضيلات ‎/__‎.")}</li>
            <li>{tr(lang, "Progress and failure: a thin bar only past 150 ms; offline, a 5xx or 12 s without an answer shows a notice with Retry, and a second failure falls back to a full page load.", "التقدم والفشل: شريط رفيع بعد ١٥٠ مللي ثانية بس؛ ولو مفيش نت أو 5xx أو ١٢ ثانية من غير رد بيظهر تنبيه فيه «إعادة المحاولة»، والفشل التاني بيعمل تحميل كامل للصفحة.")}</li>
            <li>{tr(lang, "Clean addresses: server_fn redirects end in a bare “?” (/cart?); the history entry is committed without it.", "عناوين نضيفة: تحويلات server_fn بتخلص بـ «?» لوحدها (‎/cart?‎)؛ والعنوان بيتسجل في الـ history من غيرها.")}</li>
            <li>{tr(lang, "Preference links follow you: at press time their next= points at the live address, so a language switch keeps the listing's filters.", "لينكات التفضيلات بتمشي وراك: وقت الضغط الـ next=‎ بيتوجه للعنوان الحالي، فتبديل اللغة بيحافظ على فلاتر القايمة.")}</li>
            <li>{tr(lang, "Server markup shaped for the diff: attributes always present, keyed <For> lists, Option/Either for parts that come and go.", "‏HTML السيرفر متظبط على مقاس الـ diff: الـ attributes موجودة دايمًا، و <For> بمفاتيح، و Option/Either للأجزاء اللي بتظهر وتختفي.")}</li>
        </ol>
        <p class="sc-note">
            {tr(lang, "Try a cross-page round trip: ", "جرّب رحلة بين صفحتين: ")}
            <a class="link" href="/ui#overlays">{tr(lang, "open the UI kit", "افتح الـ UI kit")}</a>
            {tr(lang, ", then press Back — the title changes, the new page starts at its top, and Back lands where you were.", "، وبعدين ارجع — العنوان بيتغير، والصفحة الجديدة بتبدأ من فوق، والرجوع بيرجّعك لمكانك.")}
        </p>
    }
}

// ─── Motion ────────────────────────────────────────────────────────────────────────────────────

#[component]
fn MotionDemo(cx: Arc<Ctx>) -> impl IntoView {
    let lang = cx.lang;
    view! {
        <islands::SpringLab/>
        <ul class="sc-steps">
            <li>{tr(lang, "Drag the puck: it follows 1:1 from where you grabbed it; pull past an end and it resists progressively (rubberband, Apple's constant 0.55).", "اسحب الكرة: بتمشي مع صباعك بالظبط من المكان اللي مسكتها منه؛ ولو شديتها بعد الحافة بتقاوم بالتدريج (‏rubberband بمعامل Apple ‏0.55).")}</li>
            <li>{tr(lang, "Let go mid-flick: the release velocity is handed to the spring, the projection (v/1000)·d/(1−d) picks the resting place, and a flick earns a little bounce.", "سيبها وهي مقذوفة: السرعة لحظة الترك بتتسلم للسبرينج، والتوقع ‏(v/1000)·d/(1−d) بيختار مكان الوقوف، والقذفة بتاخد ارتداد خفيف.")}</li>
            <li>{tr(lang, "Grab it while it moves: it stops where it is on screen and follows you — no jump, no waiting for the animation to finish.", "امسكها وهي بتتحرك: بتقف مكانها على الشاشة وتمشي معاك — من غير نطة ولا استنى الحركة تخلص.")}</li>
            <li>{tr(lang, "Reduced motion: springs land at once.", "لو مفعّل «حركة أقل»: السبرينج بيوصل على طول.")}</li>
        </ul>
    }
}

// ─── States ────────────────────────────────────────────────────────────────────────────────────

#[component]
fn StatesDemo(cx: Arc<Ctx>) -> impl IntoView {
    let lang = cx.lang;
    let slow = format!("/__sim?ms=1200&next={}", cx.next_here("streaming"));
    view! {
        <div class="sc-states">
            <section class="sc-state">
                <h4 class="sc-state__title">{tr(lang, "Loading", "بيحمّل")}</h4>
                <SBlockSkeleton/>
                <p class="sc-note">{tr(lang, "Solid blocks that pulse, sized like what replaces them — nothing moves when the data lands.", "بلوكات مصمتة بتنبض، بنفس مقاس اللي هيجي مكانها — ومفيش حاجة بتتحرك لما البيانات توصل.")}</p>
                <a class="sc-link" href=slow rel="external">{tr(lang, "Slow every read to 1.2 s and watch", "بطّأ كل قراءة لـ ١٫٢ ث وشوف")}</a>
            </section>
            <section class="sc-state">
                <h4 class="sc-state__title">{tr(lang, "A section failed", "قسم واقع")}</h4>
                <SectionError what=tr(lang, "Bestsellers", "الأكثر مبيعًا") retry_href="/showcase#states" compact=true/>
                <p class="sc-note">{tr(lang, "Only that section; the page around it keeps working. Retry drops the fault.", "القسم ده بس؛ والصفحة اللي حواليه شغالة. وإعادة المحاولة بتشيل العطل.")}</p>
                <a class="sc-link" href="/showcase?fail=bestsellers#errors">{tr(lang, "Break a real one", "اكسر قسم حقيقي")}</a>
            </section>
            <section class="sc-state">
                <h4 class="sc-state__title">{tr(lang, "Nothing to show", "مفيش حاجة تتعرض")}</h4>
                <StateView compact=true tone=StateTone::Empty
                    title=tr(lang, "No results for “galaxy s99”", "مفيش نتايج لـ «galaxy s99»")
                    body=tr(lang, "Check the spelling, or try fewer words.", "اتأكد من الكتابة، أو جرّب كلمات أقل.")/>
                <a class="sc-link" href="/search?q=galaxy+s99">{tr(lang, "Run that search", "جرّب البحث ده")}</a>
            </section>
            <section class="sc-state">
                <h4 class="sc-state__title">{tr(lang, "Offline", "من غير نت")}</h4>
                <p class="sc-note">{tr(lang, "The shell mounts one OfflineBanner on every page: a notice with Retry while the connection is down; “Back online” and an in-place refresh when it returns. These buttons fire the browser's own events — DevTools → Network → Offline does it for real.", "الـ shell بيركّب OfflineBanner واحد في كل صفحة: تنبيه فيه «إعادة المحاولة» طول ما النت مقطوع؛ و«عاد الاتصال» وتحديث للصفحة مكانها لما يرجع. الزراير دي بتطلق نفس أحداث المتصفح — و DevTools ← Network ← Offline بيعملها بجد.")}</p>
                <islands::OfflineDemo/>
            </section>
            <section class="sc-state">
                <h4 class="sc-state__title">{tr(lang, "A read that retries", "قراية بتعيد المحاولة")}</h4>
                <p class="sc-note">{tr(lang, "net::retry tries again after 300 ms, 600 ms, then 1.2 s — four tries in all — and spends none while offline.", "‏net::retry بيعيد المحاولة بعد ٣٠٠ مللي ثانية، وبعدين ٦٠٠، وبعدين ١٫٢ ثانية — أربع محاولات في المجموع — ومن غير نت مابيضيعش ولا محاولة.")}</p>
                <islands::RetryDemo/>
            </section>
            <section class="sc-state">
                <h4 class="sc-state__title">{tr(lang, "Not found", "مش موجود")}</h4>
                <p class="sc-note">{tr(lang, "A designed 404 with status 404: illustration, search, popular categories, back and home links. Works without JavaScript.", "صفحة 404 متصممة وبـ status 404: رسمة، وبحث، وأشهر الأقسام، ولينكات للرجوع وللرئيسية. وشغالة من غير JavaScript.")}</p>
                <a class="sc-link" href="/showcase/no-such-thing">{tr(lang, "Open a 404", "افتح صفحة 404")}</a>
            </section>
            <section class="sc-state">
                <h4 class="sc-state__title">{tr(lang, "Something broke", "حاجة وقعت")}</h4>
                <p class="sc-note">{tr(lang, "A route-level failure renders the full-page error: 500 for a bug, 503 (with Retry-After) when the shop is briefly unavailable.", "لما الـ route كله يقع بتظهر صفحة الخطأ الكاملة: 500 لو فيه bug، و 503 (ومعاها Retry-After) لما المتجر مش متاح لفترة قصيرة.")}</p>
                <span class="sc-state__links">
                    <a class="sc-link" href="/showcase?crash=1">"500"</a>
                    <a class="sc-link" href="/showcase?crash=503">"503"</a>
                </span>
            </section>
            <section class="sc-state">
                <h4 class="sc-state__title">{tr(lang, "Not built yet", "لسه ما اتعملش")}</h4>
                <ComingSoon compact=true
                    feature=tr(lang, "Card payments", "الدفع بالكارت")
                    description=tr(lang, "Pay by card or mobile wallet at checkout. Until then, cash on delivery.", "ادفع بالكارت أو المحفظة وانت بتطلب. ولحد ده، الدفع عند الاستلام.")/>
            </section>
        </div>
    }
}
