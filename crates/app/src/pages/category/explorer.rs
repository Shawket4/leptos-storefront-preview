//! `CatalogExplorer`: a listing's filters and results, as one island.
//!
//! **Without JS** the server renders it complete from the URL: the filters are one GET form with
//! an Apply button; sort, pagination and the active-filter chips are plain links.
//!
//! **With JS** the island owns the URL state. Inputs and links update `query`; an `AsyncDerived`
//! loads `api::list_products(query)`; a `<Transition>` keeps the previous results on screen
//! (dimmed, a progress bar, `aria-busy`) until the next page arrives, so nothing jumps. The URL
//! follows with `history.pushState`.
//!
//! - Seeded: the first value comes from the `initial` prop (already rendered by the server), so
//!   hydration doesn't fetch. `AsyncDerived` rather than `Resource` on purpose: a `Resource` made
//!   inside an island serialises its value into the page *again* (the prop already carries it) and
//!   ties the island to the page-wide hydration-id sequence. Pages seen this visit are kept, so
//!   Back/Forward and undoing a filter are instant.
//! - Links the island handles carry `data-lx` and get `preventDefault()`, so the islands router
//!   leaves them alone; everything else (product cards, other categories) is the router's.
//! - The islands router never diffs inside an island. When it navigates to the same route (another
//!   category from the header, a new search), the island notices through the `data-listing-href`
//!   attribute the router *does* diff onto the server-rendered wrapper, and reloads itself.

use super::listing::*;
use super::shim::{Drawer, DrawerSide, EmptyState};
use crate::api::types::{BrandFacet, ListingPage, ListingQuery, Sort};
use crate::components::{
    button::{Button, ButtonKind},
    icons::Icon,
    product_card::{CardData, ProductCard, ProductGrid, ProductGridSkeleton},
};
use crate::format;
use crate::prefs::{Lang, Numerals, Ui, tr, use_ui};
use domain::ProductId;
use leptos::either::{Either, EitherOf3};
use leptos::ev;
use leptos::html;
use leptos::prelude::*;
use std::time::Duration;
use wasm_bindgen::JsCast;

/// The wrapper attribute (rendered by the page) that mirrors the listing's URL.
pub const HREF_ATTR: &str = "data-listing-href";
/// Brands listed before "Show all".
const BRANDS_SHOWN: usize = 8;
const PRICE_DEBOUNCE: Duration = Duration::from_millis(350);

/// Pages this island has loaded, newest last.
struct Seen(Vec<(ListingQuery, ListingPage)>);

impl Seen {
    fn get(&self, q: &ListingQuery) -> Option<ListingPage> {
        self.0.iter().find(|(k, _)| k == q).map(|(_, p)| p.clone())
    }
    fn put(&mut self, q: ListingQuery, p: ListingPage) {
        self.0.retain(|(k, _)| *k != q);
        self.0.push((q, p));
        if self.0.len() > 12 {
            self.0.remove(0);
        }
    }
}

/// The island's state, `Copy` so every handler can take it.
#[derive(Clone, Copy)]
struct Cx {
    ui: Ui,
    /// What the URL says: the source of truth.
    query: RwSignal<ListingQuery>,
    /// The last page that loaded: results, facets and totals stay put while the next one is in
    /// flight, and when a load fails.
    shown: Memo<ListingPage>,
    /// The latest load failed (the previous results stay, under a notice).
    failed: Memo<bool>,
    brands: Memo<Vec<BrandFacet>>,
    pending: RwSignal<bool>,
    retry: RwSignal<u32>,
    drawer: RwSignal<bool>,
    hydrated: RwSignal<bool>,
    min_text: RwSignal<String>,
    max_text: RwSignal<String>,
    timer: StoredValue<Option<TimeoutHandle>>,
    /// The island's root element (set after hydration; never on the server).
    root: NodeRef<html::Div>,
    results: NodeRef<html::Section>,
}

impl Cx {
    fn lang(self) -> Lang {
        self.ui.lang.get()
    }
    fn num(self) -> Numerals {
        self.ui.numerals.get()
    }

    /// Move to `next`: push it to the history and the wrapper, then load it.
    fn go(self, next: ListingQuery) {
        if self.query.with_untracked(|q| *q == next) {
            return;
        }
        let href = next.href();
        if let Ok(history) = window().history() {
            let _ = history.push_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&href));
        }
        self.mirror(&href);
        self.query.set(next);
    }

    /// The page's wrapper that carries `data-listing-href`, if any.
    fn wrapper(self) -> Option<web_sys::Element> {
        let root: web_sys::Element = self.root.get_untracked()?.into();
        root.closest(&format!("[{HREF_ATTR}]")).ok().flatten()
    }

    /// Keep the wrapper's attribute equal to the URL, so the next router diff compares like with like.
    fn mirror(self, href: &str) {
        if let Some(w) = self.wrapper() {
            let _ = w.set_attribute(HREF_ATTR, href);
        }
    }

    /// Adopt a URL the island didn't push (Back/Forward, a router navigation).
    fn sync(self, next: ListingQuery) {
        if self.query.with_untracked(|q| *q != next) {
            self.cancel_timer();
            self.drawer.set(false);
            self.query.set(next);
        }
    }

    fn apply_price(self) {
        let mut lo = parse_amount(&self.min_text.get_untracked()).filter(|v| *v > 0);
        let mut hi = parse_amount(&self.max_text.get_untracked()).filter(|v| *v > 0);
        if let (Some(a), Some(b)) = (lo, hi)
            && a > b
        {
            (lo, hi) = (Some(b), Some(a));
            self.min_text.set(b.to_string());
            self.max_text.set(a.to_string());
        }
        let q = self.query.get_untracked();
        if q.min != lo || q.max != hi {
            self.go(with_price(&q, lo, hi));
        }
    }

    fn cancel_timer(self) {
        if let Some(h) = self.timer.try_update_value(Option::take).flatten() {
            h.clear();
        }
    }

    fn schedule_price(self) {
        self.cancel_timer();
        let h = set_timeout_with_handle(
            move || {
                self.timer.set_value(None);
                self.apply_price();
            },
            PRICE_DEBOUNCE,
        );
        self.timer.set_value(h.ok());
    }

    fn flush_price(self) {
        self.cancel_timer();
        self.apply_price();
    }

    /// Bring the top of the results into view and move focus there (the pager the shopper used is
    /// about to be replaced).
    fn scroll_to_results(self) {
        let Some(el) = self.results.get_untracked() else {
            return;
        };
        let opts = web_sys::ScrollIntoViewOptions::new();
        opts.set_block(web_sys::ScrollLogicalPosition::Start);
        opts.set_behavior(if reduced_motion() {
            web_sys::ScrollBehavior::Instant
        } else {
            web_sys::ScrollBehavior::Smooth
        });
        el.scroll_into_view_with_scroll_into_view_options(&opts);
        focus_quietly(&el);
    }
}

/// Local adapter until `ListingQuery.brands` is `#[serde(default)]`: server-function arguments
/// travel URL-encoded, which drops an empty `Vec`, so the browser can't send `api::list_products`
/// a query without brands. This sends the listing's own canonical query string instead and hands
/// the parsed query to `api::list_products` (latency, cache, `fail` demo and all) on the server.
#[server]
pub async fn list_listing(
    category: Option<String>,
    qs: String,
) -> Result<ListingPage, ServerFnError> {
    crate::api::list_products(ListingQuery::from_query(category, &qs)).await
}

#[island]
pub fn CatalogExplorer(initial: ListingPage, site_name: String) -> impl IntoView {
    let ui = use_ui();
    let is_search = initial.query.category.is_none();
    let query = RwSignal::new(initial.query.clone());
    let retry = RwSignal::new(0u32);

    let seen = StoredValue::new(Seen(vec![(initial.query.clone(), initial.clone())]));
    let page = AsyncDerived::new(move || {
        let q = query.get();
        retry.track();
        let known = seen.with_value(|s| s.get(&q));
        async move {
            match known {
                // Ready on the first poll: no fetch, and no fallback during hydration.
                Some(p) => Ok(p),
                None => {
                    let res = list_listing(q.category.clone(), q.to_query()).await;
                    if let Ok(p) = &res {
                        seen.update_value(|s| s.put(q, p.clone()));
                    }
                    res
                }
            }
        }
    });
    // While a load is in flight the derived still holds the previous value, so these follow the
    // last *finished* load.
    let failed = Memo::new(move |_| matches!(page.get(), Some(Err(_))));
    let shown = Memo::new({
        let seed = initial.clone();
        move |prev: Option<&ListingPage>| match page.get() {
            Some(Ok(p)) => p,
            _ => prev.cloned().unwrap_or_else(|| seed.clone()),
        }
    });
    // Eager images for the first row the server rendered (the LCP), lazy for everything after.
    let eager = StoredValue::new(
        initial
            .items
            .iter()
            .take(4)
            .map(|c| c.id)
            .collect::<Vec<ProductId>>(),
    );

    let cx = Cx {
        ui,
        query,
        shown,
        failed,
        brands: Memo::new(move |_| shown.with(|s| s.facets.brands.clone())),
        pending: RwSignal::new(false),
        retry,
        drawer: RwSignal::new(false),
        hydrated: RwSignal::new(false),
        min_text: RwSignal::new(initial.query.min.map(|v| v.to_string()).unwrap_or_default()),
        max_text: RwSignal::new(initial.query.max.map(|v| v.to_string()).unwrap_or_default()),
        timer: StoredValue::new(None),
        root: NodeRef::new(),
        results: NodeRef::new(),
    };

    // The price boxes follow the URL (a chip removed, Back) unless they already say the same.
    Effect::new(move |_| {
        let (min, max) = cx.query.with(|q| (q.min, q.max));
        for (text, value) in [(cx.min_text, min), (cx.max_text, max)] {
            if parse_amount(&text.get_untracked()).filter(|v| *v > 0) != value {
                text.set(value.map(|v| v.to_string()).unwrap_or_default());
            }
        }
    });
    // The tab title follows client-side changes (page number, a search's heading).
    Effect::new(move |first: Option<()>| {
        let title = cx.shown.with(|s| {
            doc_title(
                &cx.query.get(),
                s.category.as_ref().map(|c| &c.name),
                &site_name,
                cx.lang(),
                cx.num(),
            )
        });
        if first.is_some() {
            document().set_title(&title);
        }
    });
    // Live from here on: the no-JS affordances (Apply, the sheet's close link) step aside.
    Effect::new(move |_| cx.hydrated.set(true));

    let sort = NodeRef::<html::Details>::new();
    Effect::new(move |done: Option<bool>| {
        if done == Some(true) {
            return true;
        }
        let Some(el) = cx.root.get() else {
            return false;
        };
        connect(cx, el.into(), sort);
        true
    });

    // One delegated handler for every listing link inside the island.
    let on_click = move |ev: ev::MouseEvent| {
        if ev.default_prevented()
            || ev.button() != 0
            || ev.meta_key()
            || ev.ctrl_key()
            || ev.shift_key()
            || ev.alt_key()
        {
            return;
        }
        let Some(el) = ev
            .target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
        else {
            return;
        };
        let Ok(Some(a)) = el.closest("a[data-lx]") else {
            return;
        };
        let Some(next) = a.get_attribute("href").and_then(|h| query_from_href(&h)) else {
            return;
        };
        // Another category is another page: the router's job (it re-renders the header too).
        if cx.query.with_untracked(|q| q.category != next.category) {
            return;
        }
        ev.prevent_default();
        if let Ok(Some(menu)) = a.closest("details") {
            let _ = menu.remove_attribute("open");
        }
        cx.cancel_timer();
        cx.go(next);
        if a.has_attribute("data-lx-top") {
            cx.scroll_to_results();
        }
    };

    // Cards keyed by product (and display prefs): a card that survives a filter or sort change
    // keeps its DOM, so its image doesn't reload or fade in again.
    let cards = move || {
        let (lang, num) = (cx.lang(), cx.num());
        cx.shown.with(|s| {
            s.items
                .iter()
                .map(|c| (c.clone(), lang, num))
                .collect::<Vec<_>>()
        })
    };

    view! {
        <div class="lx" class:lx--search=is_search node_ref=cx.root on:click=on_click>
            {is_search.then(|| search_head(cx))}
            <div class="lx-bar" id="lx-bar">
                <Button
                    kind=ButtonKind::Secondary
                    href="#lx-filters"
                    icon="sliders-horizontal"
                    class="lx-filter-btn"
                    attr:aria-haspopup="dialog"
                    on:click=move |ev: ev::MouseEvent| {
                        if !(ev.meta_key() || ev.ctrl_key() || ev.shift_key()) {
                            ev.prevent_default();
                            cx.drawer.set(true);
                        }
                    }
                >
                    {move || tr(cx.lang(), "Filters", "الفلاتر")}
                    {move || {
                        let n = cx.query.with(panel_filter_count);
                        (n > 0).then(|| view! { <span class="lx-badge">{format::count(n, cx.num())}</span> })
                    }}
                </Button>
                <p class="lx-total" role="status">
                    {move || cx.shown.with(|s| range_label(s.page, s.total, cx.lang(), cx.num()))}
                </p>
                {sort_menu(cx, sort)}
            </div>
            {move || chips(cx)}
            <div class="lx-body">
                <aside class="lx-filters" id="lx-filters" aria-labelledby="lx-filters-title">
                    <form
                        class="lx-form"
                        method="get"
                        action=move || cx.query.with(|q| ListingQuery { category: q.category.clone(), ..Default::default() }.href())
                        on:submit=move |ev: ev::SubmitEvent| {
                            ev.prevent_default();
                            cx.flush_price();
                        }
                    >
                        <div class="lx-form-head">
                            <h2 class="lx-form-title" id="lx-filters-title">
                                {move || tr(cx.lang(), "Filters", "الفلاتر")}
                            </h2>
                            <a class="lx-sheet-close" href="#lx-bar" hidden=move || cx.hydrated.get()>
                                {move || tr(cx.lang(), "Close", "إغلاق")}
                            </a>
                        </div>
                        {filter_fields(cx, "f")}
                        {move || {
                            let q = cx.query.get();
                            view! {
                                {(q.sort != Sort::Relevance).then(|| view! { <input type="hidden" name="sort" value=q.sort.code()/> })}
                                {q.q.map(|text| view! { <input type="hidden" name="q" value=text/> })}
                            }
                        }}
                        <div class="lx-form-foot" hidden=move || cx.hydrated.get()>
                            <Button kind=ButtonKind::Secondary href=cx.query.with_untracked(cleared).href()>
                                {move || tr(cx.lang(), "Clear", "امسح")}
                            </Button>
                            <Button kind=ButtonKind::Primary button_type="submit">
                                {move || tr(cx.lang(), "Apply", "طبّق")}
                            </Button>
                        </div>
                    </form>
                </aside>
                <section
                    class="lx-results"
                    node_ref=cx.results
                    tabindex="-1"
                    aria-busy=move || if cx.pending.get() { "true" } else { "false" }
                    aria-label=move || tr(cx.lang(), "Products", "المنتجات")
                >
                    <div class="lx-progress" class:is-on=move || cx.pending.get() aria-hidden="true">
                        <span></span>
                    </div>
                    <Transition
                        fallback=|| view! { <ProductGridSkeleton count=8 class="lx-grid"/> }
                        set_pending=cx.pending.write_only()
                    >
                        // What the Transition waits on. It draws nothing itself; the results below
                        // read the finished page, so they stay on screen until the next one is in.
                        {move || Suspend::new(async move {
                            let _ = page.await;
                        })}
                        {move || cx.failed.get().then(|| failed_notice(cx))}
                        <div class="lx-results-body" class:is-pending=move || cx.pending.get()>
                            {move || cx.shown.with(|s| s.items.is_empty()).then(|| empty_view(cx))}
                            <ProductGrid class="lx-grid">
                                <For
                                    each=cards
                                    key=|(c, lang, num): &(CardData, Lang, Numerals)| (c.id, *lang, *num)
                                    children=move |(card, lang, num): (CardData, Lang, Numerals)| {
                                        let priority = eager.with_value(|e| e.contains(&card.id));
                                        view! { <ProductCard card lang numerals=num priority/> }
                                    }
                                />
                            </ProductGrid>
                            {move || {
                                let (lang, num) = (cx.lang(), cx.num());
                                cx.shown.with(|s| (s.pages > 1).then(|| pager(&s.query, s.page, s.pages, lang, num)))
                            }}
                        </div>
                    </Transition>
                </section>
            </div>
            <Drawer
                open=cx.drawer
                side=DrawerSide::Bottom
                title=Signal::derive(move || tr(cx.lang(), "Filters", "الفلاتر").to_owned())
            >
                <form
                    class="lx-form lx-form--sheet"
                    on:submit=move |ev: ev::SubmitEvent| {
                        ev.prevent_default();
                        cx.flush_price();
                    }
                >
                    {filter_fields(cx, "d")}
                </form>
                <div class="lx-sheet-foot">
                    <Button
                        kind=ButtonKind::Ghost
                        on:click=move |_| {
                            cx.cancel_timer();
                            cx.go(cx.query.with_untracked(cleared));
                        }
                    >
                        {move || tr(cx.lang(), "Clear", "امسح")}
                    </Button>
                    <Button
                        kind=ButtonKind::Primary
                        class="lx-show"
                        on:click=move |_| {
                            cx.flush_price();
                            cx.drawer.set(false);
                            cx.scroll_to_results();
                        }
                    >
                        {move || cx.pending.get().then(|| view! { <Icon name="loader-circle" size=18 class="spin"/> })}
                        {move || {
                            let n = cx.shown.with(|s| format::items(s.total, cx.lang(), cx.num()));
                            format!("{} {n}", tr(cx.lang(), "Show", "اعرض"))
                        }}
                    </Button>
                </div>
            </Drawer>
        </div>
    }
}

/// Listen for URL changes the island didn't make.
fn connect(cx: Cx, root: web_sys::Element, sort: NodeRef<html::Details>) {
    let wrap = cx.wrapper();

    // Back/Forward between states of this same listing are the island's to restore. It answers
    // in the capture phase and stops the event there: otherwise the islands router would also
    // re-fetch the whole page and, ~0.4 s later, `replaceState` and diff in the URL it popped —
    // undoing whatever the shopper did in the meantime. (Server-rendered parts are identical
    // within one listing; the island keeps the title in step itself.) Any other entry — another
    // category, another page, or this island already gone — stays the router's.
    {
        let root = root.clone();
        let registered = StoredValue::new_local(None::<wasm_bindgen::JsValue>);
        let on_pop = wasm_bindgen::closure::Closure::<dyn Fn(web_sys::Event)>::new(
            move |ev: web_sys::Event| {
                if !root.is_connected() {
                    if let Some(f) = registered.try_update_value(Option::take).flatten() {
                        let _ = window().remove_event_listener_with_callback_and_bool(
                            "popstate",
                            f.unchecked_ref(),
                            true,
                        );
                    }
                    return;
                }
                let loc = window().location();
                let href = format!(
                    "{}{}",
                    loc.pathname().unwrap_or_default(),
                    loc.search().unwrap_or_default()
                );
                if let Some(q) = query_from_href(&href)
                    && cx.query.with_untracked(|cur| cur.category == q.category)
                {
                    ev.stop_immediate_propagation();
                    cx.sync(q);
                    cx.mirror(&href);
                }
            },
        )
        .into_js_value();
        let _ = window().add_event_listener_with_callback_and_bool(
            "popstate",
            on_pop.unchecked_ref(),
            true,
        );
        registered.set_value(Some(on_pop));
    }

    // Close the sort menu on a click elsewhere.
    while_connected(&root, ev::pointerdown, move |ev| {
        let Some(menu) = sort.get_untracked() else {
            return;
        };
        let inside = ev
            .target()
            .and_then(|t| t.dyn_into::<web_sys::Node>().ok())
            .is_some_and(|n| menu.contains(Some(&n)));
        if menu.open() && !inside {
            menu.set_open(false);
        }
    });

    // Same-route router navigations rewrite the wrapper's attribute; follow it.
    if let Some(wrap) = wrap {
        let watched = wrap.clone();
        let on_change = wasm_bindgen::closure::Closure::<
            dyn FnMut(js_sys::Array, web_sys::MutationObserver),
        >::new(
            move |_: js_sys::Array, observer: web_sys::MutationObserver| {
                if !root.is_connected() {
                    observer.disconnect();
                    return;
                }
                if let Some(q) = watched
                    .get_attribute(HREF_ATTR)
                    .and_then(|h| query_from_href(&h))
                {
                    cx.sync(q);
                }
            },
        );
        if let Ok(observer) = web_sys::MutationObserver::new(on_change.as_ref().unchecked_ref()) {
            let opts = web_sys::MutationObserverInit::new();
            opts.set_attributes(true);
            opts.set_attribute_filter(&js_sys::Array::of1(&HREF_ATTR.into()));
            let _ = observer.observe_with_options(&wrap, &opts);
        }
        on_change.forget();
    }
}

/// A window listener that removes itself once `root` has left the document (the islands router
/// drops an island's DOM on navigation but never disposes its reactive owner).
fn while_connected<E>(root: &web_sys::Element, event: E, f: impl Fn(E::EventType) + 'static)
where
    E: ev::EventDescriptor + 'static,
    E::EventType: JsCast,
{
    let handle = StoredValue::new(None::<WindowListenerHandle>);
    let root = root.clone();
    let h = window_event_listener(event, move |e| {
        if root.is_connected() {
            f(e);
        } else if let Some(h) = handle.try_update_value(Option::take).flatten() {
            h.remove();
        }
    });
    handle.set_value(Some(h));
}

fn reduced_motion() -> bool {
    window()
        .match_media("(prefers-reduced-motion: reduce)")
        .ok()
        .flatten()
        .is_some_and(|m| m.matches())
}

/// `el.focus({ preventScroll: true })` (web-sys' `FocusOptions` isn't enabled in this crate).
fn focus_quietly(el: &web_sys::HtmlElement) {
    let opts = js_sys::Object::new();
    let _ = js_sys::Reflect::set(&opts, &"preventScroll".into(), &true.into());
    if let Ok(focus) =
        js_sys::Reflect::get(el, &"focus".into()).and_then(|f| f.dyn_into::<js_sys::Function>())
    {
        let _ = focus.call1(el, &opts);
    }
}

/// A search-kind listing names itself from its query (and so must re-render with it).
fn search_head(cx: Cx) -> impl IntoView {
    move || {
        let h = cx.query.with(|q| search_heading(q, cx.lang()));
        view! {
            <header class="lx-head">
                {h.kicker.map(|k| view! { <p class="lx-kicker">{k}</p> })}
                <h1 class="lx-title">{h.title}</h1>
                {h.sub.map(|s| view! { <p class="lx-sub">{s}</p> })}
            </header>
        }
    }
}

fn sort_menu(cx: Cx, node: NodeRef<html::Details>) -> impl IntoView {
    let close_on_escape = move |ev: ev::KeyboardEvent| {
        if ev.key() != "Escape" {
            return;
        }
        if let Some(menu) = node.get_untracked()
            && menu.open()
        {
            menu.set_open(false);
            if let Some(summary) = menu
                .query_selector("summary")
                .ok()
                .flatten()
                .and_then(|s| s.dyn_into::<web_sys::HtmlElement>().ok())
            {
                let _ = summary.focus();
            }
        }
    };
    view! {
        <details class="lx-sort" node_ref=node on:keydown=close_on_escape>
            <summary class="lx-sort-btn">
                <Icon name="arrow-up-down" size=16/>
                <span class="lx-sort-k">{move || tr(cx.lang(), "Sort", "الترتيب")}</span>
                <span class="lx-sort-v">{move || cx.query.with(|q| q.sort).label().pick(cx.lang().is_rtl()).to_owned()}</span>
                <Icon name="chevron-down" size=16 class="lx-sort-chev"/>
            </summary>
            <ul class="lx-sort-menu" role="list">
                {Sort::ALL
                    .into_iter()
                    .map(|s| {
                        view! {
                            <li>
                                <a
                                    class="lx-sort-opt"
                                    href=move || cx.query.with(|q| with_sort(q, s)).href()
                                    data-lx
                                    aria-current=move || (cx.query.with(|q| q.sort) == s).then_some("true")
                                >
                                    <span>{move || s.label().pick(cx.lang().is_rtl()).to_owned()}</span>
                                    <Icon name="check" size=16 class="lx-sort-tick"/>
                                </a>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </details>
    }
}

/// The active filters as removable chips.
fn chips(cx: Cx) -> impl IntoView {
    let (lang, num) = (cx.lang(), cx.num());
    let q = cx.query.get();
    let ar = lang.is_rtl();
    let mut items: Vec<(String, String)> = Vec::new();
    if q.category.is_some()
        && let Some(text) = &q.q
    {
        items.push((quoted(text, lang), without_text(&q).href()));
    }
    if q.min.is_some() || q.max.is_some() {
        items.push((
            price_label(q.min, q.max, lang, num),
            with_price(&q, None, None).href(),
        ));
    }
    let brands = cx.brands.get();
    for b in &q.brands {
        let name = brands
            .iter()
            .find(|f| &f.id == b)
            .map(|f| f.name.pick(ar).to_owned())
            .unwrap_or_else(|| b.clone());
        items.push((name, with_brand(&q, b, false).href()));
    }
    if q.in_stock {
        items.push((
            tr(lang, "In stock", "المتاح").to_owned(),
            with_stock(&q, false).href(),
        ));
    }
    if q.deal {
        items.push((
            tr(lang, "On sale", "عليه خصم").to_owned(),
            with_deal(&q, false).href(),
        ));
    }
    if items.is_empty() {
        return None;
    }
    let remove = tr(lang, "Remove filter", "شيل الفلتر");
    let clear_all = (items.len() > 1).then(|| cleared(&q).href());
    Some(view! {
        <ul class="lx-chips" aria-label=tr(lang, "Active filters", "الفلاتر المختارة")>
            {items
                .into_iter()
                .map(|(label, href)| {
                    let aria = format!("{remove}: {label}");
                    view! {
                        <li>
                            <a class="lx-chip" href=href data-lx aria-label=aria>
                                <span>{label}</span>
                                <Icon name="x" size=14/>
                            </a>
                        </li>
                    }
                })
                .collect_view()}
            {clear_all.map(|href| view! {
                <li>
                    <a class="lx-chip-clear" href=href data-lx>{tr(lang, "Clear all", "امسح الكل")}</a>
                </li>
            })}
        </ul>
    })
}

/// Price, brands and availability. Rendered twice (the sidebar form and the phone sheet), both
/// bound to the same signals; `p` keeps their ids apart.
fn filter_fields(cx: Cx, p: &'static str) -> impl IntoView {
    view! {
        <fieldset class="lx-group">
            <legend class="lx-legend">{move || tr(cx.lang(), "Price (EGP)", "السعر (ج.م)")}</legend>
            <div class="lx-price">
                {price_input(cx, p, true)}
                <span class="lx-price-dash" aria-hidden="true">"–"</span>
                {price_input(cx, p, false)}
            </div>
        </fieldset>
        {move || brand_group(cx, cx.brands.get())}
        <fieldset class="lx-group">
            <legend class="lx-legend">{move || tr(cx.lang(), "Availability", "التوفر")}</legend>
            <ul class="lx-checks">
                {toggle_row(cx, "stock", ("In stock only", "المتاح بس"), |s| s.facets.in_stock_count, |q| q.in_stock, with_stock)}
                {toggle_row(cx, "deal", ("On sale", "عليه خصم"), |s| s.facets.deal_count, |q| q.deal, with_deal)}
            </ul>
        </fieldset>
    }
}

fn price_input(cx: Cx, p: &'static str, is_min: bool) -> impl IntoView {
    let text = if is_min { cx.min_text } else { cx.max_text };
    let (name, en, ar) = if is_min {
        ("min", "Min", "من")
    } else {
        ("max", "Max", "لحد")
    };
    let id = format!("{p}-{name}");
    let label_for = id.clone();
    let placeholder = move || {
        let v = cx.shown.with(|s| {
            if is_min {
                s.facets.price_min
            } else {
                s.facets.price_max
            }
        });
        format::int(v, cx.num())
    };
    view! {
        <label class="lx-field" for=label_for>
            <span class="lx-field-label">{move || tr(cx.lang(), en, ar)}</span>
            <span class="lx-input">
                <input
                    id=id
                    name=name
                    type="text"
                    inputmode="numeric"
                    autocomplete="off"
                    enterkeyhint="done"
                    placeholder=placeholder
                    value=text.get_untracked()
                    prop:value=move || text.get()
                    on:input=move |ev| {
                        text.set(event_target_value(&ev));
                        cx.schedule_price();
                    }
                    on:change=move |_| cx.flush_price()
                />
            </span>
        </label>
    }
}

/// Re-rendered only when the brand facets change (another category or search), never on a
/// filter toggle, so a checkbox keeps focus while it is being used.
fn brand_group(cx: Cx, brands: Vec<BrandFacet>) -> Option<impl IntoView> {
    if brands.is_empty() {
        return None;
    }
    let total = brands.len() as u32;
    let selected = cx.query.with_untracked(|q| q.brands.clone());
    let tail_open = brands
        .iter()
        .skip(BRANDS_SHOWN)
        .any(|b| selected.contains(&b.id));
    let mut head = brands;
    let tail = head.split_off(head.len().min(BRANDS_SHOWN));
    let rows =
        move |list: Vec<BrandFacet>| list.into_iter().map(|b| brand_row(cx, b)).collect_view();
    Some(view! {
        <fieldset class="lx-group">
            <legend class="lx-legend">{move || tr(cx.lang(), "Brand", "الماركة")}</legend>
            <ul class="lx-checks">{rows(head)}</ul>
            {(!tail.is_empty()).then(|| view! {
                <details class="lx-more" open=tail_open>
                    <summary class="lx-more-btn">
                        <span>
                            {move || match cx.lang() {
                                Lang::En => format!("Show all {} brands", format::count(total, cx.num())),
                                Lang::Ar => format!("كل الماركات ({})", format::count(total, cx.num())),
                            }}
                        </span>
                        <Icon name="chevron-down" size=16/>
                    </summary>
                    <ul class="lx-checks">{rows(tail)}</ul>
                </details>
            })}
        </fieldset>
    })
}

fn brand_row(cx: Cx, b: BrandFacet) -> impl IntoView {
    let id = b.id.clone();
    let initially = cx.query.with_untracked(|q| q.brands.contains(&id));
    let checked = move || cx.query.with(|q| q.brands.contains(&id));
    let id = b.id.clone();
    view! {
        <li>
            <label class="lx-check">
                <input
                    type="checkbox"
                    name="brand"
                    value=b.id.clone()
                    checked=initially
                    prop:checked=checked
                    on:change=move |ev| {
                        let on = event_target_checked(&ev);
                        cx.go(cx.query.with_untracked(|q| with_brand(q, &id, on)));
                    }
                />
                <span class="lx-check-label">{move || b.name.pick(cx.lang().is_rtl()).to_owned()}</span>
                <span class="lx-check-n">{move || format::count(b.count, cx.num())}</span>
            </label>
        </li>
    }
}

fn toggle_row(
    cx: Cx,
    name: &'static str,
    label: (&'static str, &'static str),
    count: fn(&ListingPage) -> u32,
    get: fn(&ListingQuery) -> bool,
    set: fn(&ListingQuery, bool) -> ListingQuery,
) -> impl IntoView {
    let checked = move || cx.query.with(get);
    view! {
        <li>
            <label class="lx-check">
                <input
                    type="checkbox"
                    name=name
                    value="1"
                    checked=cx.query.with_untracked(get)
                    prop:checked=checked
                    on:change=move |ev| {
                        let on = event_target_checked(&ev);
                        cx.go(cx.query.with_untracked(|q| set(q, on)));
                    }
                />
                <span class="lx-check-label">{move || tr(cx.lang(), label.0, label.1)}</span>
                <span class="lx-check-n">{move || format::count(cx.shown.with(count), cx.num())}</span>
            </label>
        </li>
    }
}

/// Nothing matched: say why, and offer the way out.
fn empty_view(cx: Cx) -> impl IntoView {
    let lang = cx.lang();
    let q = cx.shown.with(|s| s.query.clone());
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
            Some((tr(lang, "Clear filters", "امسح الفلاتر"), cleared(&q).href())),
        )
    } else if let Some(text) = &q.q {
        let back = if q.category.is_some() {
            (
                tr(lang, "Show the whole aisle", "اعرض القسم كله"),
                without_text(&q).href(),
            )
        } else {
            (
                tr(lang, "Browse all products", "تصفّح كل المنتجات"),
                "/search".to_owned(),
            )
        };
        (
            format!(
                "{} {}",
                tr(lang, "No results for", "مفيش نتائج لـ"),
                quoted(text, lang)
            ),
            tr(
                lang,
                "Check the spelling, or try a shorter or more general word.",
                "اتأكد من الكلمة، أو جرّب كلمة أقصر أو أعم.",
            ),
            Some(back),
        )
    } else {
        (
            tr(lang, "Nothing here yet", "لسه مفيش منتجات هنا").to_owned(),
            tr(
                lang,
                "New stock lands every week — check back soon.",
                "بضاعة جديدة بتنزل كل أسبوع — ارجع لنا قريب.",
            ),
            None,
        )
    };
    match action {
        Some((label, href)) => Either::Left(view! {
            <EmptyState
                art="search"
                title=title
                body=body
                action=view! { <Button kind=ButtonKind::Primary href=href attr:data-lx="">{label}</Button> }.into_any()
            />
        }),
        None => Either::Right(view! { <EmptyState art="package" title=title body=body/> }),
    }
}

/// The latest load failed: keep what is on screen, say so, offer a retry.
fn failed_notice(cx: Cx) -> impl IntoView {
    let lang = cx.lang();
    view! {
        <div class="lx-notice" role="alert">
            <Icon name="triangle-alert" size=20 class="lx-notice-icon"/>
            <p class="lx-notice-text">
                <strong>{tr(lang, "These results couldn't be updated.", "تعذّر تحديث النتائج.")}</strong>
                " "
                <span>{tr(lang, "You're seeing the previous ones.", "المعروض هو النتائج السابقة.")}</span>
            </p>
            <Button kind=ButtonKind::Secondary on:click=move |_| cx.retry.update(|n| *n += 1)>
                {tr(lang, "Try again", "حاول مرة أخرى")}
            </Button>
        </div>
    }
}

fn page_label(n: u32, lang: Lang, num: Numerals) -> String {
    format!("{} {}", tr(lang, "Page", "صفحة"), format::count(n, num))
}

fn pager(
    q: &ListingQuery,
    cur: u32,
    pages: u32,
    lang: Lang,
    num: Numerals,
) -> impl IntoView + use<> {
    let href = |n: u32| q.with_page(n).href();
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
            Some(h) => Either::Left(view! {
                <a class="lx-page lx-page--step" href=h rel=rel data-lx data-lx-top>{inner}</a>
            }),
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
                                <a class="lx-page" href=href(n) data-lx data-lx-top aria-label=page_label(n, lang, num)>
                                    {format::count(n, num)}
                                </a>
                            </li>
                        }),
                    })
                    .collect_view()}
            </ol>
            {step((cur < pages).then(|| href(cur + 1)), "next")}
        </nav>
    }
}
