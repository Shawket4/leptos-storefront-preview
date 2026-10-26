//! `/showcase/stream`: the one route rendered with `SsrMode::OutOfOrder` (lib.rs), to show what
//! out-of-order streaming does and why the storefront doesn't use it.
//!
//! The frame and the first block go out at once; each `<Suspense>` follows the moment its read
//! resolves, as a `<template>` plus a small inline script that moves it into place. Every block
//! records when the server finished it (`Clock`), so the arrival order is visible on the page.
//! Links into and out of this page carry `data-router="off"`: the stream can only be watched on a
//! full page load (a router that fetches HTML receives it all at once).

use super::demos::LatencyLinks;
use crate::api::{self, types::Rail};
use crate::components::icons::Icon;
use crate::format;
use crate::pages::errors::SectionError;
use crate::prefs::{Lang, Numerals, tr};
use crate::request::req;
use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

/// When each streamed block resolved on the server, relative to the start of the render.
#[derive(Debug)]
struct Clock {
    #[cfg(feature = "ssr")]
    start: std::time::Instant,
    arrivals: AtomicU32,
}

impl Clock {
    fn new() -> Self {
        Clock {
            #[cfg(feature = "ssr")]
            start: std::time::Instant::now(),
            arrivals: AtomicU32::new(0),
        }
    }

    /// (ms since the render started, arrival rank from 1)
    fn arrived(&self) -> (u64, u32) {
        #[cfg(feature = "ssr")]
        let ms = self.start.elapsed().as_millis() as u64;
        #[cfg(not(feature = "ssr"))]
        let ms = 0;
        (ms, self.arrivals.fetch_add(1, Ordering::SeqCst) + 1)
    }
}

#[component]
pub fn StreamDemoPage() -> impl IntoView {
    // Personal (the visitor's latency cookie): never cached. Headers leave with the first chunk.
    #[cfg(feature = "ssr")]
    crate::server::cache::private();

    let rq = req();
    let lang = rq.prefs.lang;
    let num = rq.prefs.numerals;
    #[cfg(feature = "ssr")]
    let base_ms = crate::server::sim::base_ms();
    #[cfg(not(feature = "ssr"))]
    let base_ms = 0u64;
    let clock = Arc::new(Clock::new());
    let title = format!(
        "{} · {}",
        tr(lang, "Out-of-order streaming", "البث بأي ترتيب"),
        rq.tenant.name.pick(lang.is_rtl())
    );
    let next = crate::api::types::url_encode("/showcase/stream");

    view! {
        <Title text=title/>
        <Meta name="robots" content="noindex"/>
        <article class="sc sc-streampage container" aria-labelledby="scs-title">
            <header class="sc-intro sc-intro--sub">
                <p class="sc-kicker">
                    <code class="mono" dir="ltr">"SsrMode::OutOfOrder"</code>
                    <span aria-hidden="true">" · "</span>
                    <span>{tr(lang, "the only route that streams this way", "الـ route الوحيد اللي بيتبعت بالطريقة دي")}</span>
                </p>
                <h1 id="scs-title" class="sc-title">{tr(lang, "Out-of-order streaming", "البث بأي ترتيب")}</h1>
                <p class="sc-lede">
                    {tr(
                        lang,
                        "The frame of this page left the server at once. Each block below followed when its read finished, in whatever order that was, and a small inline script moved it into place. The badge on each block is the order the server finished it in.",
                        "إطار الصفحة دي خرج من السيرفر في ساعتها. وكل بلوك تحت جه ورا لما قرايته خلصت، بأي ترتيب كان، وسكريبت صغير جوه الصفحة حطه في مكانه. الرقم اللي على كل بلوك هو ترتيب ما السيرفر خلّصه.",
                    )}
                </p>
                <div class="sc-callout" class:is-off={base_ms == 0}>
                    <p class="sc-callout__text">
                        <Icon name="timer" size=18/>
                        <span>
                            <strong>{latency_line(base_ms, lang, num)}</strong>
                            " "
                            {(base_ms == 0).then(|| tr(lang, "Turn it on to see the blocks arrive out of order.", "شغّله عشان تشوف البلوكات بتوصل بأي ترتيب."))}
                        </span>
                    </p>
                    <LatencyLinks lang num current=base_ms next=next.clone() router_off=true/>
                </div>
                <p class="sc-actions">
                    <a class="sc-btnlink" href="/showcase/stream" data-router="off">
                        <Icon name="rotate-ccw" size=16/>
                        <span>{tr(lang, "Reload and watch again", "حمّل تاني واتفرج")}</span>
                    </a>
                    <a class="sc-btnlink" href="/showcase/stream?fail=deals" data-router="off">
                        <Icon name="triangle-alert" size=16/>
                        <span>{tr(lang, "Stream with the deals failing", "ابعت والعروض فاشلة")}</span>
                    </a>
                    <a class="sc-btnlink" href="/showcase#streaming" data-router="off">
                        <Icon name="arrow-left" size=16 class="flip-rtl"/>
                        <span>{tr(lang, "Back to the tour", "ارجع للجولة")}</span>
                    </a>
                </p>
            </header>

            <div class="sc-sblocks">
                <InstantBlock lang num clock=clock.clone()/>
                <StreamBlock lang num clock=clock.clone() rail=Rail::Deals label=("Today's deals", "عروض النهارده")/>
                <StreamBlock lang num clock=clock.clone() rail=Rail::Featured label=("Picked for you", "اخترنالك")/>
                <StreamBlock lang num clock=clock.clone() rail=Rail::New label=("New arrivals", "وصل جديد")/>
                <StreamBlock lang num clock=clock.clone() rail=Rail::Bestsellers label=("Bestsellers", "الأكثر مبيعًا")/>
            </div>

            // Filled by `sc-streamtiming` from the browser's navigation timing once the stream ends.
            <dl class="sc-kvs sc-streamtiming" data-ts="sc-streamtiming">
                <div class="sc-kv">
                    <dt>{tr(lang, "First byte", "أول بايت")}</dt>
                    <dd class="mono" data-slot="ttfb">"—"</dd>
                </div>
                <div class="sc-kv">
                    <dt>{tr(lang, "Last byte (the slowest block)", "آخر بايت (أبطأ بلوك)")}</dt>
                    <dd class="mono" data-slot="end">"—"</dd>
                </div>
                <div class="sc-kv">
                    <dt>{tr(lang, "Streamed blocks placed by script", "بلوكات اتحطت بسكريبت")}</dt>
                    <dd class="mono" data-slot="patched">"—"</dd>
                </div>
            </dl>

            <ul class="sc-steps sc-steps--plain" role="list">
                <li>{tr(lang, "Without JavaScript the streamed blocks stay skeletons: each one arrives as a <template> that only a script can place.", "من غير JavaScript البلوكات اللي بتتبعت ورا بتفضل هياكل: كل واحد بيوصل كـ <template> ومحدش يقدر يحطه في مكانه غير سكريبت.")}</li>
                <li>{tr(lang, "The status code and headers left with the frame, before any read finished: a failure later in the stream can't turn the page into a 500.", "كود الحالة والهيدرز خرجوا مع الإطار، قبل ما أي قراية تخلص: فشل بعد كده في البث مايقدرش يقلب الصفحة 500.")}</li>
                <li>{tr(lang, "A router that fetches this page as HTML gets it only when the slowest block is done, so the shop's pages render in order and the router swaps complete pages.", "الراوتر اللي بيجيب الصفحة دي كـ HTML مش هياخدها غير لما أبطأ بلوك يخلص، عشان كده صفحات المتجر بتترسم بالترتيب والراوتر بيبدّل صفحات كاملة.")}</li>
            </ul>
        </article>
    }
}

fn latency_line(base_ms: u64, lang: Lang, num: Numerals) -> String {
    if base_ms == 0 {
        tr(lang, "Simulated latency is off: every read answers at once.", "التأخير التجريبي مقفول: كل قراية بترد في ساعتها.").to_owned()
    } else {
        let ms = format::int(base_ms as i64, num);
        match lang {
            Lang::En => format!("Simulated latency: {ms} ms as the base for every read."),
            Lang::Ar => format!("التأخير التجريبي: {ms} مللي ثانية أساس لكل قراية."),
        }
    }
}

/// No `<Suspense>`: an in-memory read rendered inline, so it is in the very first chunk.
#[component]
fn InstantBlock(lang: Lang, num: Numerals, clock: Arc<Clock>) -> impl IntoView {
    #[cfg(feature = "ssr")]
    let cats: Vec<(String, String)> = crate::server::current_catalog()
        .nav()
        .iter()
        .take(6)
        .map(|c| (c.slug.clone(), c.name.pick(lang.is_rtl()).to_owned()))
        .collect();
    #[cfg(not(feature = "ssr"))]
    let cats: Vec<(String, String)> = Vec::new();
    let (ms, _) = clock.arrived();
    view! {
        <section class="sc-sblock sc-sblock--first" aria-label=tr(lang, "Categories", "الأقسام")>
            <header class="sc-sblock__head">
                <strong>{tr(lang, "Categories", "الأقسام")}</strong>
                <code class="sc-tag mono" dir="ltr">"current_catalog()"</code>
            </header>
            <p class="sc-sblock__meta">
                <span class="sc-rank sc-rank--first">{tr(lang, "frame", "الإطار")}</span>
                <span class="mono">{match lang {
                    Lang::En => format!("in the first chunk, at {} ms", format::int(ms as i64, num)),
                    Lang::Ar => format!("في أول chunk، عند {} مللي ثانية", format::int(ms as i64, num)),
                }}</span>
            </p>
            <ul class="sc-pills" role="list">
                {cats.into_iter().map(|(slug, name)| view! { <li><a href=format!("/c/{slug}")>{name}</a></li> }).collect_view()}
            </ul>
        </section>
    }
}

#[component]
fn StreamBlock(
    lang: Lang,
    num: Numerals,
    clock: Arc<Clock>,
    rail: Rail,
    label: (&'static str, &'static str),
) -> impl IntoView {
    let data = Resource::new(move || (), move |_| api::home_rail(rail, 3));
    let name = tr(lang, label.0, label.1);
    // The Suspend below may run more than once; the block takes its arrival rank once.
    let seen: Arc<std::sync::OnceLock<(u64, u32)>> = Arc::new(std::sync::OnceLock::new());
    view! {
        <section class="sc-sblock" aria-label=name data-stream-block="">
            <header class="sc-sblock__head">
                <strong>{name}</strong>
                <code class="sc-tag mono" dir="ltr">{format!("home_rail({rail:?})")}</code>
            </header>
            <Suspense fallback=|| view! { <BlockSkeleton/> }>
                <ErrorBoundary fallback=|errors| view! { <SectionError errors compact=true/> }>
                    {
                        let clock = clock.clone();
                        move || {
                            let clock = clock.clone();
                            let seen = seen.clone();
                            Suspend::new(async move {
                                let r = data.await;
                                let (ms, rank) = *seen.get_or_init(|| clock.arrived());
                                r.map(|cards| view! {
                                    <p class="sc-sblock__meta" data-streamed="">
                                        <span class="sc-rank mono">{format!("#{}", format::count(rank, num))}</span>
                                        <span class="mono">{match lang {
                                            Lang::En => format!("finished after {} ms", format::int(ms as i64, num)),
                                            Lang::Ar => format!("خلص بعد {} مللي ثانية", format::int(ms as i64, num)),
                                        }}</span>
                                        <i class="sc-sblock__bar" style=format!("--ms:{ms}") aria-hidden="true"></i>
                                    </p>
                                    <ul class="sc-minis" role="list">
                                        {cards.into_iter().map(|c| {
                                            let price = format::money(c.price, lang, num);
                                            view! {
                                                <li class="sc-mini">
                                                    <img class="sc-mini__img" src=c.img.src_near(96) alt="" width="48" height="48" loading="lazy" decoding="async"/>
                                                    <a class="sc-mini__name" href=format!("/p/{}", c.slug)>{c.name.pick(lang.is_rtl()).to_owned()}</a>
                                                    <span class="sc-mini__price mono">{price}</span>
                                                </li>
                                            }
                                        }).collect_view()}
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
fn BlockSkeleton() -> impl IntoView {
    view! {
        <div class="sc-sblock__skel" aria-hidden="true">
            <span class="skeleton sc-skel-line"></span>
            {(0..3).map(|_| view! {
                <span class="sc-skel-row"><span class="skeleton sc-skel-img"></span><span class="skeleton sc-skel-text"></span></span>
            }).collect_view()}
        </div>
    }
}
