//! The listing (a category, or `/search`), rendered on the server from its `ListingQuery`: the
//! URL is the state. Bar (Filters, Sort), result count, active-filter chips, the filter form, the
//! grid and pagination.
//!
//! **Without JS** the filters are one GET form with an Apply button (on phones the Filters link
//! opens it in place via `:target`); sort, chips and pages are plain links.
//!
//! **With JS** (`features/listing.ts`): `data-ts="listing"` and `data-filters=""`. A filter or sort
//! change navigates (`replace`, scroll kept) after a short quiet, and the server renders the new
//! results; on phones the same fields sit in the one bottom sheet (`components::drawer`), which
//! the router keeps open across that navigation. Nothing here is rendered by the client.

use super::listing::*;
use crate::api::types::{ListingPage, ListingQuery, Sort};
use crate::components::{
    button::{Button, ButtonKind},
    drawer::{Drawer, DrawerSide},
    empty_state::EmptyState,
    facet_group::FacetGroup,
    filter_chips::{FilterChip, FilterChips},
    form::Checkbox,
    icons::Icon,
    link_chips::{LinkChip, LinkChips},
    menu::{Menu, MenuAlign, MenuItem},
    pager::Pager,
    product_card::{ProductCard, ProductGrid},
};
use crate::format;
use crate::pages::search::suggest::{CatLink, Suggestions};
use crate::prefs::{Lang, Numerals, tr};
use crate::strings;
use leptos::either::Either;
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
                                <Button kind=ButtonKind::Ghost attr:data-clear="all">{clear}</Button>
                                // The count is the last page's while the next loads: a spinner
                                // says so (`.is-pending`, features/listing.ts).
                                <Button kind=ButtonKind::Primary class="lx-show" attr:data-drawer-close="">
                                    <Icon name="loader-circle" size=18 class="spin lx-show__busy"/>
                                    {foot}
                                </Button>
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
    let items = Sort::ALL
        .into_iter()
        .map(|s| {
            MenuItem::new(s.label().pick(ar), with_sort(q, s).href(), q.sort == s)
                .attr("data-sort", s.code())
        })
        .collect::<Vec<_>>();
    // `data-router="off"`: a pick is applied by listing.ts (merged with any pending filter taps).
    view! {
        <Menu
            label=tr(lang, "Sort", "الترتيب")
            value=q.sort.label().pick(ar).to_owned()
            items=items
            icon="arrow-up-down"
            align=MenuAlign::End
            compact=true
            class="lx-sort"
            attr:data-router="off"
        />
    }
}

/// The active filters as removable chips. `data-clear` names the field a chip resets, so the
/// browser code can apply it to the form at once and send one navigation for several taps (so
/// the router neither follows nor prefetches these links: `data-router="off"`).
fn chips(page: &ListingPage, lang: Lang, num: Numerals) -> Option<impl IntoView + use<>> {
    let q = &page.query;
    let ar = lang.is_rtl();
    let mut items: Vec<FilterChip> = Vec::new();
    if q.category.is_some()
        && let Some(text) = &q.q
    {
        items.push(
            FilterChip::new(quoted(text, lang), without_text(q).href()).attr("data-clear", "q"),
        );
    }
    if q.min.is_some() || q.max.is_some() {
        items.push(
            FilterChip::new(
                price_label(q.min, q.max, lang, num),
                with_price(q, None, None).href(),
            )
            .attr("data-clear", "price"),
        );
    }
    for b in &q.brands {
        let name = page
            .facets
            .brands
            .iter()
            .find(|f| &f.id == b)
            .map(|f| f.name.pick(ar).to_owned())
            .unwrap_or_else(|| b.clone());
        items.push(
            FilterChip::new(name, with_brand(q, b, false).href())
                .attr("data-clear", "brand")
                .attr("data-value", b.clone()),
        );
    }
    if q.in_stock {
        items.push(
            FilterChip::new(tr(lang, "In stock", "المتاح"), with_stock(q, false).href())
                .attr("data-clear", "stock"),
        );
    }
    if q.deal {
        items.push(
            FilterChip::new(tr(lang, "On sale", "عليه خصم"), with_deal(q, false).href())
                .attr("data-clear", "deal"),
        );
    }
    if items.is_empty() {
        return None;
    }
    let clear_all = (items.len() > 1)
        .then(|| FilterChip::clear_all(cleared(q).href()).attr("data-clear", "all"));
    Some(match clear_all {
        Some(all) => Either::Left(view! {
            <FilterChips items=items clear_all=all scroll=true class="lx-chips" attr:data-router="off"/>
        }),
        None => Either::Right(view! {
            <FilterChips items=items scroll=true class="lx-chips" attr:data-router="off"/>
        }),
    })
}

/// Price, brands and availability, plus the query's text and sort (hidden). Rendered twice (the
/// sidebar form and the phone sheet); `p` keeps the price boxes' ids apart.
fn fields(page: &ListingPage, p: &'static str, lang: Lang, num: Numerals) -> impl IntoView + use<> {
    let q = &page.query;
    let f = &page.facets;
    let ar = lang.is_rtl();
    let brands = (!f.brands.is_empty()).then(|| {
        // A checked brand among the hidden ones keeps "Show all" open.
        let expanded = f.brands.iter().skip(BRANDS_SHOWN).any(|b| q.brands.contains(&b.id));
        let rows = f
            .brands
            .iter()
            .map(|b| {
                view! {
                    <Checkbox
                        name="brand"
                        value=b.id.clone()
                        label=b.name.pick(ar).to_owned()
                        meta=format::count(b.count, num)
                        checked=q.brands.contains(&b.id)
                    />
                }
            })
            .collect_view();
        view! {
            <FacetGroup legend=tr(lang, "Brand", "الماركة") show_first=BRANDS_SHOWN expanded=expanded>
                {rows}
            </FacetGroup>
        }
    });
    // Owned values: the facet groups' children are built lazily.
    let min = price_input(
        p,
        "min",
        tr(lang, "Min", "من"),
        q.min,
        format::int(f.price_min, num),
    );
    let max = price_input(
        p,
        "max",
        tr(lang, "Max", "لحد"),
        q.max,
        format::int(f.price_max, num),
    );
    let (in_stock, deal) = (q.in_stock, q.deal);
    let (stock_n, deal_n) = (
        format::count(f.in_stock_count, num),
        format::count(f.deal_count, num),
    );
    view! {
        // Hidden first: the browser code builds the URL in this order (as `ListingQuery::to_query`).
        {q.q.clone().map(|text| view! { <input type="hidden" name="q" value=text/> })}
        <FacetGroup legend=tr(lang, "Price (EGP)", "السعر (ج.م)")>
            <div class="lx-price">
                {min}
                <span class="lx-price-dash" aria-hidden="true">"–"</span>
                {max}
            </div>
        </FacetGroup>
        {brands}
        <FacetGroup legend=tr(lang, "Availability", "التوفر")>
            <Checkbox name="stock" value="1" label=tr(lang, "In stock only", "المتاح بس") meta=stock_n checked=in_stock/>
            <Checkbox name="deal" value="1" label=tr(lang, "On sale", "عليه خصم") meta=deal_n checked=deal/>
        </FacetGroup>
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

fn results(page: &ListingPage, lang: Lang, num: Numerals) -> impl IntoView + use<> {
    let pq = page.query.clone();
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
        // Pages are links to the listing at the top of its results, with or without JS.
        <Pager page=page.page pages=page.pages href=move |n: u32| format!("{}#lx-results", pq.with_page(n).href())/>
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
    // "Did you mean": the respelling has results, so it is the way forward.
    let respelt = suggest
        .as_ref()
        .and_then(|s| s.did_you_mean.clone())
        .filter(|_| !q.has_filters());
    let suggestion = respelt.map(|r| {
        let href = ListingQuery {
            q: Some(r.clone()),
            page: 1,
            ..Default::default()
        }
        .href();
        (href, r)
    });
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
        let action = match (&suggestion, q.category.is_some()) {
            (Some(_), _) => None,
            (None, true) => Some((
                without_text(q).href(),
                tr(lang, "Show the whole aisle", "اعرض القسم كله").to_owned(),
            )),
            (None, false) => Some((
                "/search".to_owned(),
                tr(lang, "Browse all products", "تصفّح كل المنتجات").to_owned(),
            )),
        };
        let body = tr(
            lang,
            "Check the spelling, or try a shorter or more general word.",
            "اتأكد من الكلمة، أو جرّب كلمة أقصر أو أعم.",
        );
        (
            format!(
                "{} {}",
                tr(lang, "No results for", "مفيش نتائج لـ"),
                quoted(text, lang)
            ),
            body,
            action,
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
    let state = match (action, suggestion) {
        (Some(action), Some(suggestion)) => {
            view! { <EmptyState art=art title=title body=body action=action suggestion=suggestion/> }.into_any()
        }
        (Some(action), None) => view! { <EmptyState art=art title=title body=body action=action/> }.into_any(),
        (None, Some(suggestion)) => {
            view! { <EmptyState art=art title=title body=body suggestion=suggestion/> }.into_any()
        }
        (None, None) => view! { <EmptyState art=art title=title body=body/> }.into_any(),
    };
    // Popular searches (a search page only: they search the whole shop).
    let popular = suggest.map(|s| s.popular).filter(|p| !p.is_empty()).map(|popular| {
        let items = popular
            .into_iter()
            .map(|label| {
                let text = label.pick(ar).to_owned();
                let href = ListingQuery { q: Some(text.clone()), page: 1, ..Default::default() }.href();
                LinkChip::new(text, href).icon("search")
            })
            .collect::<Vec<_>>();
        view! { <LinkChips items=items label=tr(lang, "Popular searches", "الناس بتدوّر على") center=true/> }
    });
    let aisles = (!extras.aisles.is_empty()).then(|| {
        let items = extras
            .aisles
            .iter()
            .map(|c| LinkChip::new(c.name.pick(ar), format!("/c/{}", c.slug)).icon(&c.icon))
            .collect::<Vec<_>>();
        view! { <LinkChips items=items label=tr(lang, "Popular aisles", "أقسام عليها الطلب") center=true/> }
    });
    view! {
        <div class="lx-empty">
            {state}
            {popular}
            {aisles}
        </div>
    }
}
