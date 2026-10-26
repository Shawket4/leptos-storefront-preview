//! The listing (a category, or `/search`), rendered on the server from its `ListingQuery`: the
//! URL is the state. Bar (Filters, Sort), result count, active-filter chips, the filter form, the
//! grid and pagination.
//!
//! **Without JS** the filters are one GET form with an Apply button (on phones the Filters link
//! opens it in place via `:target`); sort, chips and pages are plain links.
//!
//! **With JS** (`features/browse.ts`): `data-ts="listing"` and `data-filters=""`. A filter or sort
//! change navigates (`replace`, scroll kept) after a short quiet, and the server renders the new
//! results; on phones the same fields sit in the one bottom sheet (`components::drawer`), which
//! the router keeps open across that navigation. Nothing here is rendered by the client.

use super::listing::*;
use crate::api::types::{BrandFacet, ListingPage, ListingQuery, Sort};
use crate::components::{
    button::{Button, ButtonKind},
    drawer::{Drawer, DrawerSide},
    empty_state::EmptyState,
    icons::Icon,
    product_card::{ProductCard, ProductGrid},
};
use crate::format;
use crate::pages::search::suggest::{CatLink, Suggestions};
use crate::prefs::{Lang, Numerals, tr};
use crate::strings;
use leptos::either::{Either, EitherOf3};
use leptos::prelude::*;

/// Brands listed before "Show all".
const BRANDS_SHOWN: usize = 8;
/// The phone sheet's id (`data-drawer-open`).
const SHEET: &str = "lx-sheet";

/// What the empty states can offer.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Extras {
    /// The busiest other aisles.
    pub aisles: Vec<CatLink>,
    /// Suggestions for a search that found nothing ("did you mean", popular searches).
    pub suggest: Option<Suggestions>,
}

#[component]
pub fn Listing(page: ListingPage, extras: Extras, lang: Lang, num: Numerals) -> impl IntoView {
    let q = page.query.clone();
    let is_search = q.category.is_none();
    // Nothing to filter or sort (a search whose base set is empty): the empty state alone.
    let bare = page.total == 0 && page.facets.price_max == 0 && !q.has_filters();
    let filters = panel_filter_count(&q);
    let filters_label = strings::get("browse.filters", lang);
    let shown = format::items(page.total, lang, num);
    let cls = match (is_search, bare) {
        (true, true) => "lx lx--search lx--bare",
        (true, false) => "lx lx--search",
        (false, true) => "lx lx--bare",
        (false, false) => "lx",
    };
    let sheet_title = filters_label.to_owned();

    view! {
        <div class=cls data-ts="listing">
            {is_search.then(|| search_head(&q, lang))}
            <div class="lx-bar" id="lx-bar">
                <Button
                    kind=ButtonKind::Secondary
                    href="#lx-filters"
                    icon="sliders-horizontal"
                    class="lx-filter-btn"
                    attr:data-drawer-open=SHEET
                    attr:aria-haspopup="dialog"
                >
                    {filters_label}
                    {(filters > 0).then(|| view! { <span class="lx-badge">{format::count(filters, num)}</span> })}
                </Button>
                {sort_menu(&q, lang)}
            </div>
            <p class="lx-total" role="status">{range_label(page.page, page.total, lang, num)}</p>
            {chips(&page, lang, num)}
            <div class="lx-body">
                <aside class="lx-filters" id="lx-filters" aria-labelledby="lx-filters-title">
                    <form class="lx-form" method="get" action=base_href(&q) data-filters="">
                        <div class="lx-form-head">
                            <h2 class="lx-form-title" id="lx-filters-title">{filters_label}</h2>
                            <a class="lx-sheet-close nojs-only" href="#lx-bar">{tr(lang, "Close", "إغلاق")}</a>
                        </div>
                        {fields(&page, "f", lang, num)}
                        <div class="lx-form-foot nojs-only">
                            <Button kind=ButtonKind::Secondary href=cleared(&q).href()>
                                {strings::get("browse.clear", lang)}
                            </Button>
                            <Button kind=ButtonKind::Primary button_type="submit">
                                {tr(lang, "Apply", "طبّق")}
                            </Button>
                        </div>
                    </form>
                </aside>
                <section class="lx-results" id="lx-results" tabindex="-1" aria-label=tr(lang, "Products", "المنتجات")>
                    <div class="lx-progress" aria-hidden="true"><span></span></div>
                    <div class="lx-results-body">
                        {if page.items.is_empty() {
                            Either::Left(empty_view(&page, &extras, lang))
                        } else {
                            Either::Right(results(&page, lang, num))
                        }}
                    </div>
                </section>
            </div>
            {(!bare).then(|| {
                let foot = format!("{} {shown}", tr(lang, "Show", "اعرض"));
                let clear = strings::get("browse.clear", lang);
                view! {
                    <Drawer
                        id=SHEET
                        side=DrawerSide::Bottom
                        title=Signal::stored(sheet_title)
                        class="lx-sheet"
                        footer=Box::new(move || view! {
                            <div class="lx-sheet-foot">
                                <button type="button" class="btn btn--ghost btn--md" data-clear="all" data-press="">
                                    <span class="btn__label">{clear}</span>
                                </button>
                                <button type="button" class="btn btn--primary btn--md lx-show" data-drawer-close="" data-press="">
                                    <Icon name="loader-circle" size=18 class="spin lx-show__busy"/>
                                    <span class="btn__label">{foot}</span>
                                </button>
                            </div>
                        }.into_any())
                    >
                        <form class="lx-form lx-form--sheet" method="get" action=base_href(&q) data-filters="">
                            {fields(&page, "d", lang, num)}
                        </form>
                    </Drawer>
                }
            })}
        </div>
    }
}

/// The listing without query string (`/c/<slug>` or `/search`): the filter forms' `action`.
fn base_href(q: &ListingQuery) -> String {
    ListingQuery {
        category: q.category.clone(),
        page: 1,
        ..Default::default()
    }
    .href()
}

/// A search-kind listing names itself from its query.
fn search_head(q: &ListingQuery, lang: Lang) -> impl IntoView + use<> {
    let h = search_heading(q, lang);
    view! {
        <header class="lx-head">
            {h.kicker.map(|k| view! { <p class="lx-kicker">{k}</p> })}
            <h1 class="lx-title">{h.title}</h1>
            {h.sub.map(|s| view! { <p class="lx-sub">{s}</p> })}
        </header>
    }
}

fn sort_menu(q: &ListingQuery, lang: Lang) -> impl IntoView + use<> {
    let ar = lang.is_rtl();
    view! {
        <details class="lx-sort" data-router="off">
            <summary class="lx-sort-btn">
                <Icon name="arrow-up-down" size=16/>
                <span class="lx-sort-k">{tr(lang, "Sort", "الترتيب")}</span>
                <span class="lx-sort-v">{q.sort.label().pick(ar).to_owned()}</span>
                <Icon name="chevron-down" size=16 class="lx-sort-chev"/>
            </summary>
            <ul class="lx-sort-menu" role="list">
                {Sort::ALL
                    .into_iter()
                    .map(|s| view! {
                        <li>
                            <a
                                class="lx-sort-opt"
                                href=with_sort(q, s).href()
                                data-sort=s.code()
                                aria-current=if q.sort == s { "true" } else { "false" }
                            >
                                <span>{s.label().pick(ar).to_owned()}</span>
                                <Icon name="check" size=16 class="lx-sort-tick"/>
                            </a>
                        </li>
                    })
                    .collect_view()}
            </ul>
        </details>
    }
}

/// The active filters as removable chips. `data-clear` names the field a chip resets, so the
/// browser code can apply it to the form at once and send one navigation for several taps (so
/// the router neither follows nor prefetches these links: `data-router="off"`).
fn chips(page: &ListingPage, lang: Lang, num: Numerals) -> Option<impl IntoView + use<>> {
    let q = &page.query;
    let ar = lang.is_rtl();
    // (label, href, field, value)
    let mut items: Vec<(String, String, &'static str, String)> = Vec::new();
    if q.category.is_some()
        && let Some(text) = &q.q
    {
        items.push((
            quoted(text, lang),
            without_text(q).href(),
            "q",
            String::new(),
        ));
    }
    if q.min.is_some() || q.max.is_some() {
        items.push((
            price_label(q.min, q.max, lang, num),
            with_price(q, None, None).href(),
            "price",
            String::new(),
        ));
    }
    for b in &q.brands {
        let name = page
            .facets
            .brands
            .iter()
            .find(|f| &f.id == b)
            .map(|f| f.name.pick(ar).to_owned())
            .unwrap_or_else(|| b.clone());
        items.push((name, with_brand(q, b, false).href(), "brand", b.clone()));
    }
    if q.in_stock {
        items.push((
            tr(lang, "In stock", "المتاح").to_owned(),
            with_stock(q, false).href(),
            "stock",
            String::new(),
        ));
    }
    if q.deal {
        items.push((
            tr(lang, "On sale", "عليه خصم").to_owned(),
            with_deal(q, false).href(),
            "deal",
            String::new(),
        ));
    }
    if items.is_empty() {
        return None;
    }
    let remove = tr(lang, "Remove filter", "شيل الفلتر");
    let clear_all = (items.len() > 1).then(|| cleared(q).href());
    Some(view! {
        <ul class="lx-chips" aria-label=tr(lang, "Active filters", "الفلاتر المختارة") data-router="off">
            {items
                .into_iter()
                .map(|(label, href, field, value)| {
                    let aria = format!("{remove}: {label}");
                    view! {
                        <li>
                            <a class="lx-chip" href=href aria-label=aria data-clear=field data-value=value>
                                <span>{label}</span>
                                <Icon name="x" size=14/>
                            </a>
                        </li>
                    }
                })
                .collect_view()}
            {clear_all.map(|href| view! {
                <li>
                    <a class="lx-chip-clear" href=href data-clear="all">{tr(lang, "Clear all", "امسح الكل")}</a>
                </li>
            })}
        </ul>
    })
}

/// Price, brands and availability, plus the query's text and sort (hidden). Rendered twice (the
/// sidebar form and the phone sheet); `p` keeps their ids apart.
fn fields(page: &ListingPage, p: &'static str, lang: Lang, num: Numerals) -> impl IntoView + use<> {
    let q = &page.query;
    let f = &page.facets;
    view! {
        // Hidden first: the browser code builds the URL in this order (as `ListingQuery::to_query`).
        {q.q.clone().map(|text| view! { <input type="hidden" name="q" value=text/> })}
        <fieldset class="lx-group">
            <legend class="lx-legend">{tr(lang, "Price (EGP)", "السعر (ج.م)")}</legend>
            <div class="lx-price">
                {price_input(p, "min", tr(lang, "Min", "من"), q.min, format::int(f.price_min, num))}
                <span class="lx-price-dash" aria-hidden="true">"–"</span>
                {price_input(p, "max", tr(lang, "Max", "لحد"), q.max, format::int(f.price_max, num))}
            </div>
        </fieldset>
        {brand_group(&f.brands, &q.brands, lang, num)}
        <fieldset class="lx-group">
            <legend class="lx-legend">{tr(lang, "Availability", "التوفر")}</legend>
            <ul class="lx-checks">
                {check_row("stock", tr(lang, "In stock only", "المتاح بس"), f.in_stock_count, q.in_stock, num)}
                {check_row("deal", tr(lang, "On sale", "عليه خصم"), f.deal_count, q.deal, num)}
            </ul>
        </fieldset>
        // The sort sits in the form so a filter change keeps it (no-JS Apply, and the browser code).
        <input type="hidden" name="sort" value=q.sort.code()/>
    }
}

fn price_input(
    p: &'static str,
    name: &'static str,
    label: &'static str,
    value: Option<i64>,
    placeholder: String,
) -> impl IntoView + use<> {
    let id = format!("{p}-{name}");
    view! {
        <label class="lx-field" for=format!("{p}-{name}")>
            <span class="lx-field-label">{label}</span>
            <span class="lx-input">
                <input
                    id=id
                    name=name
                    type="text"
                    inputmode="numeric"
                    autocomplete="off"
                    enterkeyhint="done"
                    maxlength="9"
                    placeholder=placeholder
                    value=value.map(|v| v.to_string())
                />
            </span>
        </label>
    }
}

fn brand_group(
    brands: &[BrandFacet],
    selected: &[String],
    lang: Lang,
    num: Numerals,
) -> Option<impl IntoView + use<>> {
    if brands.is_empty() {
        return None;
    }
    let ar = lang.is_rtl();
    let total = brands.len() as u32;
    let tail_open = brands
        .iter()
        .skip(BRANDS_SHOWN)
        .any(|b| selected.contains(&b.id));
    let row = |b: &BrandFacet| {
        check_row_owned(
            "brand",
            b.id.clone(),
            b.name.pick(ar).to_owned(),
            b.count,
            selected.contains(&b.id),
            num,
        )
    };
    let head: Vec<_> = brands.iter().take(BRANDS_SHOWN).map(row).collect();
    let tail: Vec<_> = brands.iter().skip(BRANDS_SHOWN).map(row).collect();
    let more = match lang {
        Lang::En => format!("Show all {} brands", format::count(total, num)),
        Lang::Ar => format!("كل الماركات ({})", format::count(total, num)),
    };
    Some(view! {
        <fieldset class="lx-group">
            <legend class="lx-legend">{tr(lang, "Brand", "الماركة")}</legend>
            <ul class="lx-checks">{head}</ul>
            {(!tail.is_empty()).then(|| view! {
                <details class="lx-more" open=tail_open data-key="brands">
                    <summary class="lx-more-btn">
                        <span>{more}</span>
                        <Icon name="chevron-down" size=16/>
                    </summary>
                    <ul class="lx-checks">{tail}</ul>
                </details>
            })}
        </fieldset>
    })
}

fn check_row(
    name: &'static str,
    label: &'static str,
    count: u32,
    on: bool,
    num: Numerals,
) -> impl IntoView + use<> {
    check_row_owned(name, "1".to_owned(), label.to_owned(), count, on, num)
}

fn check_row_owned(
    name: &'static str,
    value: String,
    label: String,
    count: u32,
    on: bool,
    num: Numerals,
) -> impl IntoView + use<> {
    view! {
        <li>
            <label class="lx-check">
                <input type="checkbox" name=name value=value checked=on/>
                <span class="lx-check-label">{label}</span>
                <span class="lx-check-n">{format::count(count, num)}</span>
            </label>
        </li>
    }
}

fn results(page: &ListingPage, lang: Lang, num: Numerals) -> impl IntoView + use<> {
    // Eager images for the first row (the LCP), lazy for the rest.
    let cards = page
        .items
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, card)| view! { <ProductCard card lang numerals=num priority=i < 4/> })
        .collect_view();
    view! {
        <ProductGrid class="lx-grid">{cards}</ProductGrid>
        {(page.pages > 1).then(|| pager(&page.query, page.page, page.pages, lang, num))}
    }
}

/// Nothing matched: say why, and offer the way out.
fn empty_view(page: &ListingPage, extras: &Extras, lang: Lang) -> impl IntoView + use<> {
    let ar = lang.is_rtl();
    let q = &page.query;
    // Suggestions for this very search (a whole-shop search only).
    let suggest = extras
        .suggest
        .clone()
        .filter(|s| q.q.as_deref() == Some(s.q.as_str()) && q.category.is_none());
    let (title, body, action) = if q.has_filters() {
        (
            tr(
                lang,
                "No products match these filters",
                "مفيش منتجات بالفلاتر دي",
            )
            .to_owned(),
            tr(
                lang,
                "Try removing a filter or widening the price range.",
                "جرّب تشيل فلتر أو توسّع نطاق السعر.",
            ),
            Some((
                cleared(q).href(),
                tr(lang, "Clear filters", "امسح الفلاتر").to_owned(),
            )),
        )
    } else if let Some(text) = &q.q {
        let respelt = suggest.as_ref().and_then(|s| s.did_you_mean.clone());
        let action = match (&respelt, q.category.is_some()) {
            // "Did you mean": the respelling has results, so it is the way forward.
            (Some(r), _) => (
                ListingQuery {
                    q: Some(r.clone()),
                    page: 1,
                    ..Default::default()
                }
                .href(),
                format!("{} {}", tr(lang, "Search for", "دوّر على"), quoted(r, lang)),
            ),
            (None, true) => (
                without_text(q).href(),
                tr(lang, "Show the whole aisle", "اعرض القسم كله").to_owned(),
            ),
            (None, false) => (
                "/search".to_owned(),
                tr(lang, "Browse all products", "تصفّح كل المنتجات").to_owned(),
            ),
        };
        let body = match &respelt {
            Some(_) => tr(lang, "Did you mean this instead?", "تقصد دي؟"),
            None => tr(
                lang,
                "Check the spelling, or try a shorter or more general word.",
                "اتأكد من الكلمة، أو جرّب كلمة أقصر أو أعم.",
            ),
        };
        (
            format!(
                "{} {}",
                tr(lang, "No results for", "مفيش نتائج لـ"),
                quoted(text, lang)
            ),
            body,
            Some(action),
        )
    } else {
        (
            tr(lang, "Nothing here yet", "لسه مفيش منتجات هنا").to_owned(),
            tr(
                lang,
                "New stock lands every week — have a look around meanwhile.",
                "بضاعة جديدة بتنزل كل أسبوع — لف في باقي الأقسام لحد ما توصل.",
            ),
            None,
        )
    };
    let art = if q.has_filters() || q.q.is_some() {
        "empty-search"
    } else {
        "delivery"
    };
    let state = match action {
        Some(action) => {
            Either::Left(view! { <EmptyState art=art title=title body=body action=action/> })
        }
        None => Either::Right(view! { <EmptyState art=art title=title body=body/> }),
    };
    let pill = |href: String, icon: String, text: String| {
        view! {
            <li>
                <a class="lx-pill" href=href>
                    <Icon name=icon size=16/>
                    <span>{text}</span>
                </a>
            </li>
        }
    };
    // Popular searches (a search page only: they search the whole shop).
    let popular = suggest.map(|s| s.popular).filter(|p| !p.is_empty()).map(|popular| {
        view! {
            <div class="lx-more-ways">
                <p class="lx-more-ways__title">{tr(lang, "Popular searches", "الناس بتدوّر على")}</p>
                <ul class="lx-pills" role="list">
                    {popular
                        .into_iter()
                        .map(|label| {
                            let text = label.pick(ar).to_owned();
                            let href = ListingQuery { q: Some(text.clone()), page: 1, ..Default::default() }.href();
                            pill(href, "search".to_owned(), text)
                        })
                        .collect_view()}
                </ul>
            </div>
        }
    });
    let aisles = (!extras.aisles.is_empty()).then(|| {
        view! {
            <div class="lx-more-ways">
                <p class="lx-more-ways__title">{tr(lang, "Popular aisles", "أقسام عليها الطلب")}</p>
                <ul class="lx-pills" role="list">
                    {extras
                        .aisles
                        .iter()
                        .map(|c| pill(format!("/c/{}", c.slug), c.icon.clone(), c.name.pick(ar).to_owned()))
                        .collect_view()}
                </ul>
            </div>
        }
    });
    view! {
        <div class="lx-empty">
            {state}
            {popular}
            {aisles}
        </div>
    }
}

fn page_label(n: u32, lang: Lang, num: Numerals) -> String {
    format!("{} {}", tr(lang, "Page", "صفحة"), format::count(n, num))
}

/// Pages are links to the listing at the top of its results (`#lx-results`), with or without JS.
fn pager(
    q: &ListingQuery,
    cur: u32,
    pages: u32,
    lang: Lang,
    num: Numerals,
) -> impl IntoView + use<> {
    let href = |n: u32| format!("{}#lx-results", q.with_page(n).href());
    let step = move |to: Option<String>, rel: &'static str| {
        let (icon, en, ar) = if rel == "prev" {
            ("chevron-left", "Previous", "السابق")
        } else {
            ("chevron-right", "Next", "التالي")
        };
        let inner = view! {
            <Icon name=icon size=18 class="flip-rtl"/>
            <span class="lx-step-label">{tr(lang, en, ar)}</span>
        };
        match to {
            Some(h) => {
                Either::Left(view! { <a class="lx-page lx-page--step" href=h rel=rel>{inner}</a> })
            }
            None => Either::Right(
                view! { <span class="lx-page lx-page--step is-disabled" aria-hidden="true">{inner}</span> },
            ),
        }
    };
    view! {
        <nav class="lx-pages" aria-label=tr(lang, "Pages", "الصفحات")>
            {step((cur > 1).then(|| href(cur - 1)), "prev")}
            <ol class="lx-page-list">
                {page_slots(cur, pages)
                    .into_iter()
                    .map(|s| match s {
                        Slot::Gap => EitherOf3::A(view! { <li class="lx-gap" aria-hidden="true">"…"</li> }),
                        Slot::Page { n, far } if n == cur => EitherOf3::B(view! {
                            <li class:is-far=far>
                                <span class="lx-page is-current" aria-current="page">{format::count(n, num)}</span>
                            </li>
                        }),
                        Slot::Page { n, far } => EitherOf3::C(view! {
                            <li class:is-far=far>
                                <a class="lx-page" href=href(n) aria-label=page_label(n, lang, num)>{format::count(n, num)}</a>
                            </li>
                        }),
                    })
                    .collect_view()}
            </ol>
            {step((cur < pages).then(|| href(cur + 1)), "next")}
        </nav>
    }
}
