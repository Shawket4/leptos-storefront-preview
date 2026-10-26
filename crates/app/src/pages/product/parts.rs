//! The rest of the product page: the two-column layout, title block, delivery promise,
//! highlights, specs, description, related products, and the not-found and error views.

use super::gallery::Gallery;
use super::model::{Notice, text};
use super::panel::PurchasePanel;
use crate::api;
use crate::api::types::{ListingQuery, NavCategory, ProductView};
use crate::components::badge::{Badge, BadgeKind};
use crate::components::button::{Button, ButtonKind};
use crate::components::icons::Icon;
use crate::components::layout::{Rail, SectionHeader};
use crate::components::product_card::{CardData, ProductCard};
use crate::components::rating::Rating;
use crate::components::states::{SectionError, StateTone, StateView};
use crate::format;
use crate::prefs::{Lang, Numerals, tr};
use crate::request::req;
use domain::{ProductId, Tag};
use leptos::either::Either;
use leptos::prelude::*;

/// Gallery + info column, for the variant in the URL.
#[component]
pub fn ProductBody(
    pv: ProductView,
    selected: u32,
    start: usize,
    notice: Option<Notice>,
) -> impl IntoView {
    let rq = req();
    let (lang, num) = (rq.prefs.lang, rq.prefs.numerals);
    let p = &pv.product;
    view! {
        <div class="pdp-grid">
            <div class="pdp-media">
                <Gallery product=p.id.0 name=p.name.clone() images=pv.images.clone() start=start lang num/>
            </div>
            <div class="pdp-info">
                <TitleBlock pv=pv.clone() lang=lang numerals=num/>
                <PurchasePanel pv=pv.clone() selected=selected notice=notice lang num/>
                <Promise express=p.has(Tag::Express) lang=lang numerals=num/>
                <Details pv=pv.clone() lang=lang/>
            </div>
        </div>
    }
}

#[component]
fn TitleBlock(pv: ProductView, lang: Lang, numerals: Numerals) -> impl IntoView {
    let p = &pv.product;
    let name = text(&p.name, lang);
    let brand = pv.brand.as_ref().map(|b| {
        let href = ListingQuery {
            brands: vec![b.id.clone()],
            page: 1,
            ..Default::default()
        }
        .href();
        view! { <a class="pdp-brand" href=href>{text(&b.name, lang)}</a> }
    });
    // Catalogue subtitles like "OPPO · Phones" only repeat the kicker and the breadcrumb.
    let boiler = pv
        .brand
        .as_ref()
        .map(|b| format!("{} · {}", b.name.en, pv.category.name.en));
    let subtitle = (!p.subtitle.en.is_empty() && Some(&p.subtitle.en) != boiler.as_ref())
        .then(|| text(&p.subtitle, lang));
    let badges = [
        p.has(Tag::Bestseller).then_some(BadgeKind::Bestseller),
        p.has(Tag::New).then_some(BadgeKind::New),
        p.has(Tag::Express).then_some(BadgeKind::Express),
    ];
    view! {
        <header class="pdp-head">
            {brand}
            <h1 class="pdp-name">{name}</h1>
            {subtitle.map(|s| view! { <p class="pdp-sub">{s}</p> })}
            <div class="pdp-meta">
                <Rating value=p.rating count=p.reviews lang=lang numerals=numerals/>
                <div class="pdp-badges">
                    {badges.into_iter().flatten().map(|k| view! { <Badge kind=k lang=lang numerals=numerals/> }).collect_view()}
                </div>
            </div>
        </header>
    }
}

/// What delivery and payment look like, from the tenant's own terms.
#[component]
fn Promise(express: bool, lang: Lang, numerals: Numerals) -> impl IntoView {
    let t = req().tenant;
    let money = |p| format::money(p, lang, numerals);
    let free = money(t.free_delivery_over);
    let fee = money(t.delivery_fee);
    let fast = t.express_minutes.map(|m| {
        let m = format::count(m, numerals);
        let (title, sub) = match lang {
            Lang::En => (
                format!("At your door in {m} minutes"),
                "From our nearest store".to_owned(),
            ),
            Lang::Ar => (format!("عندك في {m} دقيقة"), "من أقرب فرع لينا".to_owned()),
        };
        (title, sub)
    });
    // Only what the tenant's terms say: the fee, the free-delivery threshold, express minutes.
    let (dtitle, dsub) = match lang {
        Lang::En => (
            format!("Free delivery over {free}"),
            format!("Otherwise {fee} per order"),
        ),
        Lang::Ar => (
            format!("توصيل مجاني فوق {free}"),
            format!("غير كده {fee} للطلب"),
        ),
    };
    // Express-tagged stock ships from a dark store (`Tag::Express`: delivered within hours).
    let express_line = (fast.is_none() && express).then(|| {
        (
            tr(lang, "Express delivery", "توصيل سريع"),
            tr(
                lang,
                "This item arrives within hours",
                "المنتج ده يوصلك خلال ساعات",
            ),
        )
    });
    let row = |icon: &'static str, title: String, sub: String| {
        view! {
            <li class="pdp-promise__row">
                <span class="pdp-promise__icon"><Icon name=icon size=20/></span>
                <span class="pdp-promise__text">
                    <span class="pdp-promise__title">{title}</span>
                    <span class="pdp-promise__sub">{sub}</span>
                </span>
            </li>
        }
    };
    view! {
        <ul class="pdp-promise" role="list" aria-label=tr(lang, "Delivery and payment", "التوصيل والدفع")>
            {fast.map(|(a, b)| row("zap", a, b))}
            {express_line.map(|(a, b)| row("zap", a.to_owned(), b.to_owned()))}
            {row("truck", dtitle, dsub)}
            {row(
                "banknote",
                tr(lang, "Cash on delivery", "الدفع عند الاستلام").to_owned(),
                tr(lang, "Pay when your order arrives", "ادفع لما طلبك يوصل").to_owned(),
            )}
        </ul>
    }
}

#[component]
fn Details(pv: ProductView, lang: Lang) -> impl IntoView {
    let p = &pv.product;
    let pid = p.id.0;
    let highlights = (!p.highlights.is_empty()).then(|| {
        view! {
            <ul class="pdp-highlights" role="list">
                {p.highlights.iter().map(|h| view! {
                    <li class="pdp-highlights__item">
                        <Icon name="check" size=18 class="pdp-highlights__icon"/>
                        <span>{text(h, lang)}</span>
                    </li>
                }).collect_view()}
            </ul>
        }
    });
    let specs = (!p.specs.is_empty()).then(|| {
        let (hid, hid_ref) = (format!("pdp-specs-{pid}"), format!("pdp-specs-{pid}"));
        view! {
            <section class="pdp-sec" aria-labelledby=hid_ref>
                <h2 class="pdp-sec__title" id=hid>{tr(lang, "Specifications", "المواصفات")}</h2>
                <table class="pdp-specs">
                    <tbody>
                        {p.specs.iter().map(|s| view! {
                            <tr>
                                <th scope="row">{text(&s.k, lang)}</th>
                                <td>{text(&s.v, lang)}</td>
                            </tr>
                        }).collect_view()}
                    </tbody>
                </table>
            </section>
        }
    });
    let desc = text(&p.description, lang);
    let paras: Vec<String> = desc
        .split('\n')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
        .collect();
    let (hid, hid_ref) = (format!("pdp-desc-{pid}"), format!("pdp-desc-{pid}"));
    view! {
        {highlights}
        {specs}
        <section class="pdp-sec" aria-labelledby=hid_ref>
            <h2 class="pdp-sec__title" id=hid>{tr(lang, "About this product", "عن المنتج")}</h2>
            <div class="pdp-desc">{paras.into_iter().map(|t| view! { <p>{t}</p> }).collect_view()}</div>
        </section>
    }
}

/// "You may also like": its own data and error boundary (`?fail=related` demos it).
#[component]
pub fn Related(id: u32, category: NavCategory, lang: Lang, numerals: Numerals) -> impl IntoView {
    let title = tr(lang, "You may also like", "ممكن يعجبك كمان").to_owned();
    let (hid, hid_ref) = (format!("pdp-related-{id}"), format!("pdp-related-{id}"));
    // The shelf's own name is the link: short enough to leave the title on one line.
    let more = text(&category.name, lang);
    let href = format!("/c/{}", category.slug);
    view! {
        <section class="pdp-related" aria-labelledby=hid_ref>
            <SectionHeader title=title.clone() id=hid href=href.clone() link_label=more/>
            <CardRail
                fetch=api::related_products(ProductId(id), 12)
                label=title
                what=tr(lang, "Similar products", "المنتجات المشابهة").to_owned()
                empty_href=href
                lang=lang
                numerals=numerals
            />
        </section>
    }
}

/// A rail of product cards from a read: an error that offers a retry, and a quiet line when there
/// is nothing to show. (The page renders in order, so there is no loading state to draw.)
#[component]
pub fn CardRail<F>(
    fetch: F,
    label: String,
    /// The rail's name for its error ("Similar products didn't load").
    what: String,
    /// Where "browse instead" points when the rail comes back empty.
    empty_href: String,
    lang: Lang,
    numerals: Numerals,
) -> impl IntoView
where
    F: std::future::Future<Output = Result<Vec<CardData>, ServerFnError>> + Send + 'static,
{
    view! {
        <ErrorBoundary fallback=move |errors| view! { <SectionError errors=errors what=what.clone()/> }>
                {Suspend::new(async move {
                        fetch.await.map(|cards| {
                            if cards.is_empty() {
                                return Either::Left(view! {
                                    <p class="pdp-rail-empty">
                                        {tr(lang, "Nothing similar right now.", "مفيش حاجة شبهه دلوقتي.")}
                                        " "
                                        <a href=empty_href>{tr(lang, "Browse the shelf", "اتفرج على القسم")}</a>
                                    </p>
                                });
                            }
                            Either::Right(view! {
                                <Rail label=label>
                                    {cards
                                        .into_iter()
                                        .map(|card| view! { <ProductCard card=card lang=lang numerals=numerals/> })
                                        .collect_view()}
                                </Rail>
                            })
                        })
                })}
        </ErrorBoundary>
    }
}

/// The product itself didn't load (backend down, `?fail=product`).
#[component]
pub fn ProductError(lang: Lang) -> impl IntoView {
    view! {
        <div class="pdp-state" role="alert">
            <StateView
                tone=StateTone::Error
                title=tr(lang, "We couldn't load this product", "تعذّر تحميل هذا المنتج")
                body=tr(
                    lang,
                    "Something went wrong on our side. Your cart is safe — please try again.",
                    "حدث خطأ من جهتنا. سلتك محفوظة، يُرجى المحاولة مرة أخرى.",
                )
                page=true
            >
                <Button kind=ButtonKind::Primary href=retry_href() icon="rotate-ccw">
                    {tr(lang, "Try again", "إعادة المحاولة")}
                </Button>
                <Button kind=ButtonKind::Secondary href="/".to_owned()>{tr(lang, "Back to the shop", "العودة للمتجر")}</Button>
            </StateView>
        </div>
    }
}

/// The same page without the `fail` parameter the demo uses.
fn retry_href() -> String {
    let pq = req().path_and_query;
    match pq.split_once('?') {
        Some((p, q)) => {
            let kept: Vec<&str> = q.split('&').filter(|kv| !kv.starts_with("fail=")).collect();
            if kept.is_empty() {
                p.to_owned()
            } else {
                format!("{p}?{}", kept.join("&"))
            }
        }
        None => pq,
    }
}

/// Unknown slug: a 404 that still helps — search for what they were after, or pick from what
/// people are buying.
#[component]
pub fn NotFound(slug: String, lang: Lang, numerals: Numerals) -> impl IntoView {
    let guess = slug.replace(['-', '_'], " ");
    let title = tr(lang, "Popular right now", "الأكثر طلبًا دلوقتي").to_owned();
    view! {
        <div class="container pdp-404">
            <div class="pdp-state">
                <StateView
                    tone=StateTone::NotFound
                    kicker=tr(lang, "Error 404", "خطأ 404")
                    title=tr(lang, "We couldn't find this product", "مش لاقيين المنتج ده")
                    body=tr(
                        lang,
                        "The link may be old, or the product left our shelves. Try searching for it:",
                        "ممكن اللينك قديم، أو المنتج مبقاش عندنا. جرّب تدوّر عليه:",
                    )
                    page=true
                >
                    <form class="pdp-404__search" action="/search" method="get" role="search">
                        <label class="sr-only" for="pdp-404-q">{tr(lang, "Search products", "دوّر على منتج")}</label>
                        <input class="pdp-404__input" id="pdp-404-q" type="search" name="q" value=guess autocomplete="off"/>
                        <Button kind=ButtonKind::Primary button_type="submit" icon="search">{tr(lang, "Search", "دوّر")}</Button>
                    </form>
                </StateView>
            </div>
            <section class="pdp-related" aria-labelledby="pdp-404-popular">
                <SectionHeader title=title.clone() id="pdp-404-popular" href="/search?sort=rating"/>
                <CardRail
                    fetch=api::home_rail(api::types::Rail::Bestsellers, 12)
                    label=title
                    what=tr(lang, "Popular products", "المنتجات الأكثر طلبًا").to_owned()
                    empty_href="/".to_owned()
                    lang=lang
                    numerals=numerals
                />
            </section>
        </div>
    }
}
