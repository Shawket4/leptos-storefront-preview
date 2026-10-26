//! Home page, product-led like the marketplaces people actually use on their phones: a one-line
//! ribbon of store promises, the category rail, then today's deals, picks and bestsellers — real
//! products above the fold, no hero banner (owner's call).
//! Every merchandising rail is its own `Suspend` + `<ErrorBoundary>` around its own server
//! function. The page renders in order (one complete document, no JS needed), and a failing rail
//! (`?fail=deals`) degrades to an inline notice without touching the rest of the page. The deals
//! countdown is rendered by the server and ticked by `features/browse.ts` (`data-ts="countdown"`).

use crate::api::{self, types::Rail};
use crate::components::{
    icons::Icon,
    layout::{Container, Rail as Row, SectionHeader},
    product_card::ProductCard,
};
use crate::format;
use crate::media::Img;
use crate::prefs::{Lang, Numerals, tr};
use crate::request::req;
use domain::L10n;
use leptos::prelude::*;
use leptos_meta::{Link, Meta, Script, Title};

/// The share image for this shop and language.
fn og_image(tenant: &str, lang: Lang) -> Option<Img> {
    crate::server::state()
        .media()
        .try_img(&format!("brand/og-{tenant}-{}", lang.code()))
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
    // Headers go out with the first chunk, so the caching decision is made before any data.
    crate::server::cache::public(&[]);

    let lang = rq.prefs.lang;
    let num = rq.prefs.numerals;
    let ar = lang.is_rtl();
    let t = rq.tenant.clone();

    let title = format!("{} · {}", t.name.pick(ar), t.tagline.pick(ar));
    let description = description(&t.id).pick(ar).to_owned();
    let og_image = og_image(&t.id, lang).map(|i| rq.absolute(&i.largest()));
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
    // The ribbon carries figures, so it goes through `format` like every number.
    let ribbon = match t.express_minutes {
        Some(m) => match lang {
            Lang::En => format!(
                "Express delivery in {} minutes · Cash on delivery",
                format::count(m, num)
            ),
            Lang::Ar => format!(
                "توصيل سريع في {} دقيقة · الدفع عند الاستلام",
                format::count(m, num)
            ),
        },
        None => match lang {
            Lang::En => format!(
                "Free delivery over {} · Cash on delivery",
                format::money_short(t.free_delivery_over, lang, num)
            ),
            Lang::Ar => format!(
                "توصيل مجاني فوق {} · الدفع عند الاستلام",
                format::money_short(t.free_delivery_over, lang, num)
            ),
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

        <p class="ribbon">{ribbon}</p>

        <Container>
            <h1 class="sr-only">{format!("{} — {}", t.name.pick(ar), t.tagline.pick(ar))}</h1>
            <CategoryRail lang/>
            <RailSection rail=Rail::Deals lang num
                title=tr(lang, "Today's deals", "عروض النهارده")
                subtitle=tr(lang, "Fresh prices, every morning", "أسعار جديدة كل يوم الصبح")
                href="/search?deal=1&sort=discount" countdown=true priority=true/>
            <RailSection rail=Rail::Featured lang num
                title=tr(lang, "Picked for you", "اخترنالك")
                subtitle=tr(lang, "What our buyers are excited about this week", "اللي فريقنا متحمس له الأسبوع ده")
                href="/search?sort=rating"/>
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

#[component]
fn CategoryRail(lang: Lang) -> impl IntoView {
    let ar = lang.is_rtl();
    view! {
        <section id="categories" class="home-section home-cats" aria-labelledby="cats-title">
            <SectionHeader title=tr(lang, "Shop by category", "تسوّق حسب القسم") id="cats-title"/>
            <ErrorBoundary fallback=move |_| view! { <WidgetError lang/> }>
                    {Suspend::new(async move {
                        api::nav_categories().await.map(|cats| view! {
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
                                                                alt="" width="96" height="96" loading="lazy" decoding="async"/>
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
        </section>
    }
}

/// One merchandising rail, with its own data and its own error boundary.
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
                {countdown.then(|| view! { <DealsCountdown ends_at_ms=next_cairo_midnight_ms() lang num/> })}
            </div>
            <ErrorBoundary fallback=move |_| view! { <WidgetError lang/> }>
                    {Suspend::new(async move {
                        api::home_rail(rail, 12).await.map(|cards| {
                            if cards.is_empty() {
                                // A real empty state, not a blank strip.
                                return view! {
                                    <p class="rail-empty">
                                        {tr(lang, "Nothing here right now — new picks land every morning.", "مفيش حاجة هنا دلوقتي — بننزّل اختيارات جديدة كل يوم الصبح.")}
                                        " "<a href="/search">{tr(lang, "Browse everything", "تصفّح كل المنتجات")}</a>
                                    </p>
                                }
                                .into_any();
                            }
                            view! {
                                <Row>
                                    {cards.into_iter().enumerate().map(|(i, card)| view! {
                                        <ProductCard card lang numerals=num priority=priority && i < 2/>
                                    }).collect_view()}
                                </Row>
                            }
                            .into_any()
                        })
                    })}
            </ErrorBoundary>
        </section>
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
            if kept.is_empty() {
                p.to_owned()
            } else {
                format!("{p}?{}", kept.join("&"))
            }
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
    let now = crate::server::orders::now_ms();
    let off = format::cairo_offset_hours(now) * 3_600_000;
    let local_day = (now + off).div_euclid(86_400_000);
    (local_day + 1) * 86_400_000 - off
}

/// A live HH:MM:SS countdown to `ends_at_ms`. The server renders the time left, so it is right
/// without JS; the browser ticks it (`data-ts="countdown"`), in the visitor's numerals.
#[component]
fn DealsCountdown(ends_at_ms: i64, lang: Lang, num: Numerals) -> impl IntoView {
    let s = ((ends_at_ms - crate::server::orders::now_ms()) / 1000).max(0);
    let clock = format::digits(
        &format!("{:02}:{:02}:{:02}", s / 3600, (s / 60) % 60, s % 60),
        num,
    );
    view! {
        <p class="countdown" aria-hidden="true" data-ts="countdown" data-ends=ends_at_ms>
            <Icon name="clock" size=16/>
            <span>{tr(lang, "Ends in", "تنتهي خلال")}</span>
            <span class="countdown__clock mono" dir="ltr" data-clock="">{clock}</span>
        </p>
    }
}
