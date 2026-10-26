//! Home page, laid out like Apple's Store page: a large two-tone title, the category rail, then
//! "The latest" — swipeable product tiles built from real catalogue packshots (no stock photography,
//! nothing floating over an image). The title and tiles are plain server HTML in the first chunk
//! (the first tile's packshot is the LCP).
//! Every merchandising rail is its own `<Suspense>` + `<ErrorBoundary>` around its own server
//! function, so with out-of-order streaming each rail arrives the moment its data does — the slow
//! "deals" rail sits *above* "featured" yet lands after it — and a failing rail (`?fail=deals`)
//! degrades to an inline notice without touching the rest of the page.

use crate::api::{self, types::Rail};
use crate::components::{
    icons::Icon,
    layout::{Container, Rail as Row, SectionHeader},
    product_card::{ProductCard, ProductCardSkeleton},
};
use crate::format;
use crate::media::Img;
use crate::prefs::{Lang, Numerals, tr, use_ui};
use crate::request::req;
use domain::L10n;
use leptos::prelude::*;
use leptos_meta::{Link, Meta, Script, Title};

/// Images resolved on the server (the hydrate build never renders this page).
#[derive(Clone, Default)]
struct HomeArt {
    og: Option<Img>,
}

#[cfg(feature = "ssr")]
fn home_art(tenant: &str, lang: Lang) -> HomeArt {
    let st = crate::server::state();
    HomeArt { og: st.media().try_img(&format!("brand/og-{tenant}-{}", lang.code())) }
}

#[cfg(not(feature = "ssr"))]
fn home_art(_: &str, _: Lang) -> HomeArt {
    HomeArt::default()
}

/// One "The latest" tile: a real product, resolved synchronously from the in-memory catalogue so it
/// is in the first chunk (no Suspense, no layout shift).
#[derive(Clone)]
struct Tile {
    slug: String,
    eyebrow: String,
    name: String,
    sub: String,
    price: String,
    img: Img,
}

#[cfg(feature = "ssr")]
fn latest_tiles(lang: Lang, num: Numerals) -> Vec<Tile> {
    let catalog = crate::server::current_catalog();
    let ar = lang.is_rtl();
    let mut seen = std::collections::HashSet::new();
    let mut picks = catalog.rail(Rail::New, 24);
    picks.extend(catalog.rail(Rail::Featured, 24));
    picks
        .into_iter()
        .filter(|c| c.in_stock && !c.img.is_missing())
        .filter_map(|c| catalog.product(c.id).map(|p| (c, p.category, p.subtitle.clone())))
        .filter(|(_, cat, _)| seen.insert(*cat))
        .take(4)
        .map(|(c, _, sub)| {
            let money = format::money(c.price, lang, num);
            Tile {
                slug: c.slug.clone(),
                eyebrow: c.brand.as_ref().map(|b| b.pick(ar).to_owned()).unwrap_or_default(),
                name: c.name.pick(ar).to_owned(),
                sub: sub.pick(ar).to_owned(),
                price: if c.from {
                    match lang {
                        Lang::En => format!("From {money}"),
                        Lang::Ar => format!("من {money}"),
                    }
                } else {
                    money
                },
                img: c.img.clone(),
            }
        })
        .collect()
}

#[cfg(not(feature = "ssr"))]
fn latest_tiles(_: Lang, _: Numerals) -> Vec<Tile> {
    vec![]
}

/// The meta description (in production: CMS content per tenant).
fn description(tenant: &str) -> L10n {
    match tenant {
        "souq" => L10n::new(
            "Fresh produce, Egyptian pantry staples and sweets — packed by people who know a good tomato.",
            "خضار وفاكهة طازة، خزين البيت المصري، وحلويات — بيجهّزها ناس فاهمة يعني إيه طماطم حلوة.",
        ),
        _ => L10n::new(
            "Phones, beauty, fashion and home — genuine products, delivered across Egypt, cash on delivery.",
            "موبايلات وجمال وموضة وبيت — منتجات أصلية، توصيل لكل مصر، والدفع عند الاستلام.",
        ),
    }
}

#[component]
pub fn HomePage() -> impl IntoView {
    let rq = req();
    // Headers go out with the first chunk, so the caching decision is made before any Suspense.
    #[cfg(feature = "ssr")]
    crate::server::cache::public(&[]);

    let lang = rq.prefs.lang;
    let num = rq.prefs.numerals;
    let ar = lang.is_rtl();
    let t = rq.tenant.clone();
    let art = home_art(&t.id, lang);
    let tiles = latest_tiles(lang, num);

    let title = format!("{} · {}", t.name.pick(ar), t.tagline.pick(ar));
    let description = description(&t.id).pick(ar).to_owned();
    let og_image = art.og.as_ref().map(|i| rq.absolute(&i.largest()));
    let website_ld = serde_json::json!({
        "@context": "https://schema.org",
        "@type": "WebSite",
        "name": t.name.pick(ar),
        "url": rq.absolute("/"),
        "inLanguage": if ar { "ar-EG" } else { "en-EG" },
        "potentialAction": {
            "@type": "SearchAction",
            "target": { "@type": "EntryPoint", "urlTemplate": rq.absolute("/search?q={search_term_string}") },
            "query-input": "required name=search_term_string"
        }
    })
    .to_string()
    .replace("</", "<\\/");

    let express = t.express_minutes.is_some();
    // The promise line carries figures, so it goes through `format` like every number.
    let promise = match t.express_minutes {
        Some(m) => match lang {
            Lang::En => format!("Express in {} minutes · Cash on delivery", format::count(m, num)),
            Lang::Ar => format!("توصيل سريع في {} دقيقة · الدفع عند الاستلام", format::count(m, num)),
        },
        None => match lang {
            Lang::En => format!(
                "Free delivery over {} · Cash on delivery · 14-day returns",
                format::money_short(t.free_delivery_over, lang, num)
            ),
            Lang::Ar => format!(
                "توصيل مجاني فوق {} · الدفع عند الاستلام · استرجاع خلال ١٤ يوم",
                format::money_short(t.free_delivery_over, lang, num)
            ),
        },
    };
    let name = t.name.pick(ar).to_owned();
    let tagline = t.tagline.pick(ar).to_owned();

    view! {
        <Title text=title.clone()/>
        <Meta name="description" content=description.clone()/>
        <Link rel="canonical" href=rq.absolute("/")/>
        <Meta property="og:type" content="website"/>
        <Meta property="og:site_name" content=t.name.pick(ar).to_owned()/>
        <Meta property="og:title" content=title/>
        <Meta property="og:description" content=description/>
        <Meta property="og:url" content=rq.absolute("/")/>
        <Meta property="og:locale" content=if ar { "ar_EG" } else { "en_EG" }/>
        {og_image.map(|src| view! {
            <Meta property="og:image" content=src/>
            <Meta property="og:image:width" content="1200"/>
            <Meta property="og:image:height" content="630"/>
        })}
        <Meta name="twitter:card" content="summary_large_image"/>
        <Script type_="application/ld+json">{website_ld}</Script>

        <Container>
            <header class="intro">
                <h1 class="intro__title"><strong>{name}"."</strong>" "<span>{tagline}</span></h1>
                <p class="intro__promise">{promise}</p>
            </header>
            <CategoryRail lang/>
            <LatestTiles tiles lang/>
            <RailSection rail=Rail::Deals lang num
                title=tr(lang, "Today's deals", "عروض النهارده")
                subtitle=tr(lang, "Fresh prices, every morning", "أسعار جديدة كل يوم الصبح")
                href="/search?deal=1&sort=discount" countdown=true/>
            <RailSection rail=Rail::Featured lang num
                title=tr(lang, "Picked for you", "اخترنالك")
                subtitle=tr(lang, "What our buyers are excited about this week", "اللي فريقنا متحمس له الأسبوع ده")
                href="/search?sort=rating" priority=true/>
            {express.then(|| view! {
                <RailSection rail=Rail::Express lang num
                    title=tr(lang, "At your door in 30 minutes", "عندك في ٣٠ دقيقة")
                    subtitle=tr(lang, "From our nearest dark store", "من أقرب مخزن لينا")
                    href="/search?sort=relevance"/>
            })}
            <RailSection rail=Rail::Bestsellers lang num
                title=tr(lang, "Bestsellers", "الأكثر مبيعًا")
                subtitle=tr(lang, "What everyone's ordering", "اللي الناس كلها بتطلبه")
                href="/search?sort=rating"/>
        </Container>
    }
}

/// "The latest": swipeable tiles on phones, a row on desktop. Each tile is one link to the product
/// (the "Buy" and "Learn more" labels are part of it, not nested controls).
#[component]
fn LatestTiles(tiles: Vec<Tile>, lang: Lang) -> impl IntoView {
    (!tiles.is_empty()).then(|| {
        view! {
            <section class="home-section latest" aria-labelledby="latest-title">
                <h2 id="latest-title" class="two-tone">
                    <strong>{tr(lang, "The latest.", "الجديد.")}</strong>" "
                    <span>{tr(lang, "Fresh arrivals worth a look.", "وصل حديثًا ويستاهل نظرة.")}</span>
                </h2>
                <ul class="latest__rail">
                    {tiles.into_iter().enumerate().map(|(i, t)| view! {
                        <li>
                            <a class="tile" href=format!("/p/{}", t.slug)>
                                <span class="tile__text">
                                    {(!t.eyebrow.is_empty()).then(|| view! { <span class="tile__eyebrow">{t.eyebrow}</span> })}
                                    <span class="tile__name">{t.name}</span>
                                    {(!t.sub.is_empty()).then(|| view! { <span class="tile__sub">{t.sub}</span> })}
                                    <span class="tile__price">{t.price}</span>
                                    <span class="tile__ctas">
                                        <span class="tile__buy">{tr(lang, "Buy", "اشتري")}</span>
                                        <span class="tile__more">{tr(lang, "Learn more", "اعرف أكتر")}<Icon name="chevron-right" size=16 class="flip-rtl"/></span>
                                    </span>
                                </span>
                                <span class="tile__media" style=format!("background-image:url({})", t.img.lqip)>
                                    <img
                                        src=t.img.src_near(640)
                                        srcset=t.img.srcset()
                                        sizes="(min-width: 1100px) 300px, (min-width: 768px) 45vw, 78vw"
                                        alt=t.img.alt(lang).to_owned()
                                        width="640"
                                        height="640"
                                        loading=if i == 0 { "eager" } else { "lazy" }
                                        fetchpriority=if i == 0 { "high" } else { "auto" }
                                        decoding="async"
                                    />
                                </span>
                            </a>
                        </li>
                    }).collect_view()}
                </ul>
            </section>
        }
    })
}

#[component]
fn CategoryRail(lang: Lang) -> impl IntoView {
    let cats = Resource::new(|| (), |_| api::nav_categories());
    let ar = lang.is_rtl();
    view! {
        <section id="categories" class="home-section home-cats" aria-labelledby="cats-title">
            <SectionHeader title=tr(lang, "Shop by category", "تسوّق حسب القسم") id="cats-title"/>
            <Suspense fallback=move || view! { <CategoryRailSkeleton/> }>
                <ErrorBoundary fallback=move |_| view! { <WidgetError lang/> }>
                    {move || Suspend::new(async move {
                        cats.await.map(|cats| view! {
                            <ul class="cat-rail">
                                {cats.into_iter().map(|c| {
                                    let name = c.name.pick(ar).to_owned();
                                    view! {
                                        <li>
                                            <a class="cat-tile" href=format!("/c/{}", c.slug)>
                                                <span class="cat-tile__well">
                                                    {match c.img {
                                                        Some(img) => view! {
                                                            <img src=img.src_near(240) srcset=img.srcset() sizes="96px"
                                                                alt="" width="96" height="96" loading="lazy" decoding="async"
                                                                style=format!("background-image:url({})", img.lqip)/>
                                                        }.into_any(),
                                                        None => view! { <Icon name=c.icon.clone() size=28/> }.into_any(),
                                                    }}
                                                </span>
                                                <span class="cat-tile__name">{name}</span>
                                            </a>
                                        </li>
                                    }
                                }).collect_view()}
                            </ul>
                        })
                    })}
                </ErrorBoundary>
            </Suspense>
        </section>
    }
}

#[component]
fn CategoryRailSkeleton() -> impl IntoView {
    view! {
        <ul class="cat-rail" aria-hidden="true">
            {(0..10).map(|_| view! {
                <li><span class="cat-tile"><span class="cat-tile__well skeleton"></span><span class="cat-tile__name skeleton skeleton--text"></span></span></li>
            }).collect_view()}
        </ul>
    }
}

/// One merchandising rail, streamed on its own.
#[component]
fn RailSection(
    rail: Rail,
    lang: Lang,
    num: Numerals,
    title: &'static str,
    subtitle: &'static str,
    href: &'static str,
    #[prop(optional)] countdown: bool,
    #[prop(optional)] priority: bool,
) -> impl IntoView {
    let data = Resource::new(|| (), move |_| api::home_rail(rail, 12));
    let id = format!("rail-{rail:?}").to_lowercase();
    let labelled_by = id.clone();
    view! {
        <section class="home-section home-rail" aria-labelledby=labelled_by>
            <div class="home-rail__head">
                <SectionHeader
                    title=title
                    subtitle=subtitle
                    href=href
                    link_label=tr(lang, "See all", "عرض الكل")
                    id=id
                />
                {countdown.then(|| view! { <DealsCountdown ends_at_ms=next_cairo_midnight_ms()/> })}
            </div>
            <Suspense fallback=move || view! { <RailSkeleton/> }>
                <ErrorBoundary fallback=move |_| view! { <WidgetError lang/> }>
                    {move || Suspend::new(async move {
                        data.await.map(|cards| view! {
                            <Row>
                                {cards.into_iter().enumerate().map(|(i, card)| view! {
                                    <ProductCard card lang numerals=num priority=priority && i < 2/>
                                }).collect_view()}
                            </Row>
                        })
                    })}
                </ErrorBoundary>
            </Suspense>
        </section>
    }
}

#[component]
fn RailSkeleton() -> impl IntoView {
    view! {
        <div aria-hidden="true">
            <Row>{(0..6).map(|_| view! { <ProductCardSkeleton/> }).collect_view()}</Row>
        </div>
    }
}

/// A section's data failed: say so, offer a retry, keep the rest of the page.
#[component]
fn WidgetError(lang: Lang) -> impl IntoView {
    let rq = req();
    // Retry = the same page without the `fail` parameter that the demo uses.
    let retry = rq
        .path_and_query
        .split_once('?')
        .map(|(p, q)| {
            let kept: Vec<&str> = q.split('&').filter(|kv| !kv.starts_with("fail=")).collect();
            if kept.is_empty() { p.to_owned() } else { format!("{p}?{}", kept.join("&")) }
        })
        .unwrap_or_else(|| rq.path_and_query.clone());
    view! {
        <div class="widget-error" role="status">
            <Icon name="triangle-alert" size=20/>
            <p>
                <strong>{tr(lang, "This section didn't load.", "القسم ده ما اتحمّلش.")}</strong>
                " "
                {tr(lang, "Everything else on the page is fine.", "باقي الصفحة شغالة عادي.")}
            </p>
            <a class="widget-error__retry" href=retry>{tr(lang, "Try again", "جرّب تاني")}</a>
        </div>
    }
}

/// Midnight Cairo time, as Unix ms (deals "end" daily).
fn next_cairo_midnight_ms() -> i64 {
    #[cfg(feature = "ssr")]
    {
        let now = crate::server::orders::now_ms();
        let off = format::cairo_offset_hours(now) * 3_600_000;
        let local_day = (now + off).div_euclid(86_400_000);
        (local_day + 1) * 86_400_000 - off
    }
    #[cfg(not(feature = "ssr"))]
    {
        0
    }
}

/// A live HH:MM:SS countdown. The server renders the starting value, so it is correct without JS;
/// the island only ticks it. Re-renders on language/numeral changes.
#[island]
fn DealsCountdown(ends_at_ms: i64) -> impl IntoView {
    let ui = use_ui();
    let now = RwSignal::new(now_ms());
    #[cfg(feature = "hydrate")]
    {
        let handle = set_interval_with_handle(move || now.set(now_ms()), std::time::Duration::from_secs(1));
        on_cleanup(move || {
            if let Ok(h) = handle {
                h.clear();
            }
        });
    }
    let left = move || ((ends_at_ms - now.get()) / 1000).max(0);
    let clock = move || {
        let s = left();
        format::digits(&format!("{:02}:{:02}:{:02}", s / 3600, (s / 60) % 60, s % 60), ui.numerals.get())
    };
    view! {
        <p class="countdown" aria-hidden="true">
            <Icon name="clock" size=16/>
            <span>{move || tr(ui.lang.get(), "Ends in", "تنتهي خلال")}</span>
            <span class="countdown__clock mono" dir="ltr">{clock}</span>
        </p>
    }
}

fn now_ms() -> i64 {
    #[cfg(feature = "hydrate")]
    {
        js_sys::Date::now() as i64
    }
    #[cfg(not(feature = "hydrate"))]
    {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as i64)
            .unwrap_or(0)
    }
}
