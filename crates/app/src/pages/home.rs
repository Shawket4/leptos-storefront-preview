//! Home page.
//!
//! The hero, trust strip and promos are plain server HTML in the first chunk (they are the LCP).
//! Every merchandising rail is its own `<Suspense>` + `<ErrorBoundary>` around its own server
//! function, so with out-of-order streaming each rail arrives the moment its data does — the slow
//! "deals" rail sits *above* "featured" yet lands after it — and a failing rail (`?fail=deals`)
//! degrades to an inline notice without touching the rest of the page.

use crate::api::{self, types::Rail};
use crate::components::{
    button::{Button, ButtonKind, ButtonSize},
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
    hero: Option<Img>,
    hero_m: Option<Img>,
    promos: Vec<Img>,
    og: Option<Img>,
}

#[cfg(feature = "ssr")]
fn home_art(tenant: &str, lang: Lang) -> HomeArt {
    let st = crate::server::state();
    let m = st.media();
    HomeArt {
        hero: m.try_img(&format!("{tenant}/hero-1")),
        hero_m: m.try_img(&format!("{tenant}/hero-1-m")),
        promos: (1..=2).filter_map(|i| m.try_img(&format!("{tenant}/promo-{i}"))).collect(),
        og: m.try_img(&format!("brand/og-{tenant}-{}", lang.code())),
    }
}

#[cfg(not(feature = "ssr"))]
fn home_art(_: &str, _: Lang) -> HomeArt {
    HomeArt::default()
}

/// Copy that differs per tenant (in production this is CMS content).
struct Copy {
    sub: L10n,
    cta: L10n,
    promos: [(L10n, L10n, &'static str); 2],
}

fn copy(tenant: &str) -> Copy {
    match tenant {
        "souq" => Copy {
            sub: L10n::new(
                "Fresh produce, Egyptian pantry staples and sweets — packed by people who know a good tomato.",
                "خضار وفاكهة طازة، خزين البيت المصري، وحلويات — بيجهّزها ناس فاهمة يعني إيه طماطم حلوة.",
            ),
            cta: L10n::new("Shop today's deals", "شوف عروض النهارده"),
            promos: [
                (L10n::new("The weekly basket", "سلة الأسبوع"), L10n::new("Seasonal fruit & veg, picked this morning", "خضار وفاكهة الموسم، متقطفة الصبح"), "/search?sort=newest"),
                (L10n::new("Pantry, sorted", "الخزين كله"), L10n::new("Rice, pulses, tahini and spices", "رز وبقوليات وطحينة وتوابل"), "/search?deal=1&sort=discount"),
            ],
        },
        _ => Copy {
            sub: L10n::new(
                "Phones, beauty, fashion and home — genuine products, delivered across Egypt, cash on delivery.",
                "موبايلات وجمال وموضة وبيت — منتجات أصلية، توصيل لكل مصر، والدفع عند الاستلام.",
            ),
            cta: L10n::new("Shop the deals", "تسوّق العروض"),
            promos: [
                (L10n::new("New this week", "جديد الأسبوع"), L10n::new("The latest phones, audio and wearables", "أحدث الموبايلات والسماعات والساعات"), "/search?sort=newest"),
                (L10n::new("Up to 35% off", "خصومات لحد ٣٥٪"), L10n::new("Limited stock — while it lasts", "كميات محدودة — لحد ما تخلص"), "/search?deal=1&sort=discount"),
            ],
        },
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
    let c = copy(&t.id);

    let title = format!("{} · {}", t.name.pick(ar), t.tagline.pick(ar));
    let description = c.sub.pick(ar).to_owned();
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
    // The hero kicker carries figures, so it goes through `format` like every number.
    let kicker = match t.express_minutes {
        Some(m) => match lang {
            Lang::En => format!("Express in {} minutes", format::count(m, num)),
            Lang::Ar => format!("توصيل سريع في {} دقيقة", format::count(m, num)),
        },
        None => match lang {
            Lang::En => format!("Free delivery over {}", format::money(t.free_delivery_over, lang, num)),
            Lang::Ar => format!("توصيل مجاني فوق {}", format::money(t.free_delivery_over, lang, num)),
        },
    };

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

        <Hero art=art.clone() kicker=kicker headline=t.tagline.pick(ar).to_owned()
            sub=c.sub.pick(ar).to_owned() cta=c.cta.pick(ar).to_owned() lang/>
        <TrustStrip lang num express_minutes=t.express_minutes free_over=t.free_delivery_over/>

        <Container>
            <CategoryRail lang/>
            <RailSection rail=Rail::Deals lang num
                title=tr(lang, "Today's deals", "عروض النهارده")
                subtitle=tr(lang, "Fresh prices, every morning", "أسعار جديدة كل يوم الصبح")
                href="/search?deal=1&sort=discount" countdown=true/>
            <RailSection rail=Rail::Featured lang num
                title=tr(lang, "Picked for you", "اخترنالك")
                subtitle=tr(lang, "What our buyers are excited about this week", "اللي فريقنا متحمس له الأسبوع ده")
                href="/search?sort=rating" priority=true/>
            <Promos promos=c.promos.iter().map(|(h, s, href)| (h.pick(ar).to_owned(), s.pick(ar).to_owned(), href.to_string())).collect() imgs=art.promos.clone() lang/>
            {express.then(|| view! {
                <RailSection rail=Rail::Express lang num
                    title=tr(lang, "At your door in 30 minutes", "عندك في ٣٠ دقيقة")
                    subtitle=tr(lang, "From our nearest dark store", "من أقرب مخزن لينا")
                    href="/search?sort=relevance"/>
            })}
            <RailSection rail=Rail::New lang num
                title=tr(lang, "New arrivals", "وصل حديثًا")
                subtitle=tr(lang, "Just landed on our shelves", "لسه نازلين الرفوف")
                href="/search?sort=newest"/>
            <RailSection rail=Rail::Bestsellers lang num
                title=tr(lang, "Bestsellers", "الأكثر مبيعًا")
                subtitle=tr(lang, "What everyone's ordering", "اللي الناس كلها بتطلبه")
                href="/search?sort=rating"/>
        </Container>
    }
}

/// Art-directed hero: a 4:5 crop on phones, 21:9 from 768px, rendered in the first chunk with
/// `fetchpriority=high` and its box reserved — it is the LCP element. The copy sits on a solid card
/// that overlaps the photo (no scrim, no gradient), so contrast never depends on the image.
#[component]
fn Hero(art: HomeArt, kicker: String, headline: String, sub: String, cta: String, lang: Lang) -> impl IntoView {
    let alt = art.hero.as_ref().map(|i| i.alt(lang).to_owned()).unwrap_or_default();
    let has_img = art.hero.is_some();
    let lqip = art
        .hero_m
        .as_ref()
        .or(art.hero.as_ref())
        .map(|i| format!("background-image:url({});--img-color:{}", i.lqip, i.color));
    view! {
        <section class="hero" class:hero--plain=!has_img aria-labelledby="hero-title">
            <div class="hero__frame">
                <div class="hero__media" style=lqip>
                    {art.hero.clone().map(|wide| {
                        let mobile = art.hero_m.clone().unwrap_or_else(|| wide.clone());
                        view! {
                            <picture>
                                <source media="(max-width: 767px)" srcset=mobile.srcset() sizes="100vw"/>
                                <img
                                    src=wide.src_near(1280)
                                    srcset=wide.srcset()
                                    sizes="(max-width: 1320px) 100vw, 1320px"
                                    alt=alt.clone()
                                    width="1920"
                                    height="823"
                                    fetchpriority="high"
                                    decoding="async"
                                />
                            </picture>
                        }
                    })}
                </div>
                <div class="hero__card">
                    <p class="hero__kicker">{kicker}</p>
                    <h1 id="hero-title" class="hero__title">{headline}</h1>
                    <p class="hero__sub">{sub}</p>
                    <div class="hero__ctas">
                        <Button kind=ButtonKind::Primary size=ButtonSize::Lg href="/search?deal=1&sort=discount".to_string()>
                            {cta}
                            <Icon name="arrow-right" class="flip-rtl"/>
                        </Button>
                        <Button kind=ButtonKind::Outline size=ButtonSize::Lg href="#categories".to_string()>
                            {tr(lang, "Browse categories", "تصفّح الأقسام")}
                        </Button>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn TrustStrip(lang: Lang, num: Numerals, express_minutes: Option<u32>, free_over: domain::Piastres) -> impl IntoView {
    let free = format::money(free_over, lang, num);
    let items: Vec<(&'static str, String, String)> = vec![
        match express_minutes {
            Some(m) => (
                "zap",
                tr(lang, "Express delivery", "توصيل سريع").to_owned(),
                match lang {
                    Lang::En => format!("In {} minutes, 9am–11pm", format::count(m, num)),
                    Lang::Ar => format!("في {} دقيقة، من ٩ص لـ ١١م", format::count(m, num)),
                },
            ),
            None => (
                "truck",
                tr(lang, "Delivery across Egypt", "توصيل لكل مصر").to_owned(),
                tr(lang, "Cairo & Giza next day", "القاهرة والجيزة تاني يوم").to_owned(),
            ),
        },
        (
            "package-check",
            tr(lang, "Free delivery", "توصيل مجاني").to_owned(),
            match lang {
                Lang::En => format!("On orders over {free}"),
                Lang::Ar => format!("للطلبات فوق {free}"),
            },
        ),
        ("banknote", tr(lang, "Cash on delivery", "الدفع عند الاستلام").to_owned(), tr(lang, "Or card, wallet, InstaPay", "أو كارت أو محفظة أو إنستاباي").to_owned()),
        ("rotate-ccw", tr(lang, "Easy returns", "استرجاع سهل").to_owned(), tr(lang, "14 days, no questions", "خلال ١٤ يوم من غير أسئلة").to_owned()),
    ];
    view! {
        <ul class="trust container" aria-label=tr(lang, "Why shop with us", "ليه تشتري مننا")>
            {items.into_iter().map(|(icon, title, body)| view! {
                <li class="trust__item">
                    <span class="trust__icon"><Icon name=icon size=20/></span>
                    <span class="trust__text"><strong>{title}</strong><span>{body}</span></span>
                </li>
            }).collect_view()}
        </ul>
    }
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

#[component]
fn Promos(promos: Vec<(String, String, String)>, imgs: Vec<Img>, lang: Lang) -> impl IntoView {
    view! {
        <section class="home-section promos" aria-label=tr(lang, "Promotions", "عروض")>
            {promos.into_iter().enumerate().map(|(i, (headline, sub, href))| {
                let img = imgs.get(i).cloned();
                view! {
                    <a class="promo" href=href>
                        <span class="promo__media">
                            {img.map(|img| view! {
                                <img src=img.src_near(640) srcset=img.srcset()
                                    sizes="(max-width: 767px) 100vw, 50vw" alt="" width="640" height="480"
                                    loading="lazy" decoding="async" style=format!("background-image:url({})", img.lqip)/>
                            })}
                        </span>
                        <span class="promo__label">
                            <span class="promo__text">
                                <strong>{headline}</strong>
                                <span>{sub}</span>
                            </span>
                            <span class="promo__go"><Icon name="arrow-right" class="flip-rtl" size=18/></span>
                        </span>
                    </a>
                }
            }).collect_view()}
        </section>
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
