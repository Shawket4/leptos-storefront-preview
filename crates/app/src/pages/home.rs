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

/// One promo panel: a real product, resolved synchronously from the in-memory catalogue so it is in
/// the first chunk (no Suspense, no layout shift).
#[derive(Clone)]
struct Panel {
    slug: String,
    eyebrow: String,
    name: String,
    sub: String,
    price: String,
    img: Img,
}

/// Which products the home page promotes (in production: the tenant's CMS campaign).
#[cfg(feature = "ssr")]
struct Campaign {
    hero: (u32, L10n),
    duo: [(u32, L10n); 2],
}

#[cfg(feature = "ssr")]
fn campaign(tenant: &str) -> Campaign {
    match tenant {
        "souq" => Campaign {
            hero: (2451, L10n::new("Fresh this week", "طازة الأسبوع ده")),
            duo: [
                (2053, L10n::new("Baked this morning", "مخبوز الصبح")),
                (2501, L10n::new("Bestseller", "الأكثر مبيعًا")),
            ],
        },
        _ => Campaign {
            hero: (1202, L10n::new("Featured", "مميز")),
            duo: [(1106, L10n::new("New", "جديد")), (1807, L10n::new("Fragrance", "عطور"))],
        },
    }
}

#[cfg(feature = "ssr")]
fn panels(tenant: &str, lang: Lang, num: Numerals) -> (Option<Panel>, Vec<Panel>) {
    let catalog = crate::server::current_catalog();
    let ar = lang.is_rtl();
    let make = |id: u32, eyebrow: &L10n| -> Option<Panel> {
        let card = catalog.card(domain::ProductId(id))?;
        let p = catalog.product(domain::ProductId(id))?;
        if card.img.is_missing() {
            return None;
        }
        let brand = card.brand.as_ref().map(|b| b.pick(ar).to_owned()).unwrap_or_default();
        // Headline = the product, not the SKU: drop the brand prefix and the " — colour" suffix.
        let full = card.name.pick(ar);
        let name = full
            .strip_prefix(brand.as_str())
            .filter(|rest| !brand.is_empty() && rest.starts_with(' '))
            .map(str::trim_start)
            .unwrap_or(full);
        let name = name.split(" — ").next().unwrap_or(name);
        let money = format::money_short(card.price, lang, num);
        Some(Panel {
            slug: card.slug.clone(),
            eyebrow: eyebrow.pick(ar).to_owned(),
            name: no_break_hyphens(name),
            sub: no_break_hyphens(p.subtitle.pick(ar)),
            price: match (card.from, lang) {
                (true, Lang::En) => format!("From {money}"),
                (true, Lang::Ar) => format!("من {money}"),
                _ => money,
            },
            img: card.img.clone(),
        })
    };
    let c = campaign(tenant);
    let hero = make(c.hero.0, &c.hero.1);
    let duo = c.duo.iter().filter_map(|(id, e)| make(*id, e)).collect();
    (hero, duo)
}

#[cfg(not(feature = "ssr"))]
fn panels(_: &str, _: Lang, _: Numerals) -> (Option<Panel>, Vec<Panel>) {
    (None, vec![])
}

/// "14-inch" must not break after the hyphen: use U+2011 (non-breaking hyphen) between word chars.
#[cfg(feature = "ssr")]
fn no_break_hyphens(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    chars
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let around = i > 0 && i + 1 < chars.len() && chars[i - 1].is_alphanumeric() && chars[i + 1].is_alphanumeric();
            if *c == '-' && around { '\u{2011}' } else { *c }
        })
        .collect()
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
    let (hero, duo) = panels(&t.id, lang, num);

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
    // The ribbon carries figures, so it goes through `format` like every number.
    let ribbon = match t.express_minutes {
        Some(m) => match lang {
            Lang::En => format!("Express delivery in {} minutes · Cash on delivery", format::count(m, num)),
            Lang::Ar => format!("توصيل سريع في {} دقيقة · الدفع عند الاستلام", format::count(m, num)),
        },
        None => match lang {
            Lang::En => format!("Free delivery over {} · Cash on delivery", format::money_short(t.free_delivery_over, lang, num)),
            Lang::Ar => format!("توصيل مجاني فوق {} · الدفع عند الاستلام", format::money_short(t.free_delivery_over, lang, num)),
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
        {hero.map(|p| view! { <PromoPanel panel=p lang hero=true/> })}
        {(!duo.is_empty()).then(|| view! {
            <div class="duo">
                {duo.into_iter().map(|p| view! { <PromoPanel panel=p lang/> }).collect_view()}
            </div>
        })}

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

/// An apple.com-style promo panel: eyebrow, the product as the headline, one line, the price, two
/// actions, then the packshot — big. Type sits on a solid panel; nothing overlaps the image.
#[component]
fn PromoPanel(panel: Panel, lang: Lang, #[prop(optional)] hero: bool) -> impl IntoView {
    let href = format!("/p/{}", panel.slug);
    let heading = if hero { "promo__name promo__name--hero" } else { "promo__name" };
    view! {
        <section class="promo" class:promo--hero=hero aria-label=panel.name.clone()>
            <p class="promo__eyebrow">{panel.eyebrow}</p>
            {if hero {
                view! { <h1 class=heading>{panel.name.clone()}</h1> }.into_any()
            } else {
                view! { <h2 class=heading>{panel.name.clone()}</h2> }.into_any()
            }}
            <p class="promo__sub">{panel.sub}</p>
            <p class="promo__price">{panel.price}</p>
            <p class="promo__ctas">
                <a class="promo__buy" href=href.clone()>{tr(lang, "Buy", "اشتري")}</a>
                <a class="promo__more" href=href.clone()>
                    {tr(lang, "Learn more", "اعرف أكتر")}<Icon name="chevron-right" size=18 class="flip-rtl"/>
                </a>
            </p>
            <a class="promo__media" href=href tabindex="-1" aria-hidden="true">
                <img
                    src=panel.img.src_near(960)
                    srcset=panel.img.srcset()
                    sizes=if hero { "(min-width: 768px) 560px, 86vw" } else { "(min-width: 768px) 380px, 70vw" }
                    alt=""
                    width="960"
                    height="960"
                    loading=if hero { "eager" } else { "lazy" }
                    fetchpriority=if hero { "high" } else { "auto" }
                    decoding="async"
                />
            </a>
        </section>
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
