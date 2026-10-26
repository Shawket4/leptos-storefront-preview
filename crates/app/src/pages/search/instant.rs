//! `HeaderSearch`: instant search as an ARIA combobox.
//!
//! Without JS it is a plain `GET /search?q=` form. With JS, typing asks `api::search_products`
//! (debounced 180 ms; a sequence number drops answers that arrive after a newer question) and
//! shows up to 8 products with the matched part marked, "“q” in <category>" shortcuts and a
//! "see all results" row. Keyboard: ↑/↓ move through the options (wrapping), Enter opens the
//! active one or submits, Escape closes (a second Escape clears). Focus stays in the input and the
//! active option is announced through `aria-activedescendant`.
//!
//! Options are ordinary links: choosing one *clicks* it, so the islands router navigates exactly
//! as it does for any link, and middle-click / open-in-new-tab keep working.
//!
//! On phones (≤ 600 px) the header shows a search button; it opens the same form as a full-screen
//! overlay. Without JS that button is a link to `/search`, which has the form on the page.

use crate::api::{
    self,
    types::{ListingQuery, NavCategory, SearchHit, SearchResults, Seg},
};
use crate::components::{
    icons::Icon,
    price::{Price, PriceSize},
};
use crate::format;
use crate::pages::category::listing::quoted;
use crate::prefs::{Lang, tr, use_ui};
use leptos::either::{Either, EitherOf3};
use leptos::{ev, html, prelude::*};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use wasm_bindgen::JsCast;

const DEBOUNCE: Duration = Duration::from_millis(180);
const HITS: u32 = 8;

/// Where the box sits: the site header (with the phone overlay) or the top of `/search`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum SearchBox {
    #[default]
    Header,
    Page,
}

/// One row of the listbox, in keyboard order.
#[derive(Clone, Debug, PartialEq)]
enum Opt {
    Category { href: String, cat: NavCategory },
    Product { href: String, hit: SearchHit },
    All { href: String, total: u32 },
}

/// The `q` of the current URL, when it is a search page (so the box shows what was searched).
fn url_query() -> Option<String> {
    #[cfg(feature = "ssr")]
    let pq = crate::request::req().path_and_query;
    #[cfg(not(feature = "ssr"))]
    let pq = {
        let l = window().location();
        format!(
            "{}{}",
            l.pathname().unwrap_or_default(),
            l.search().unwrap_or_default()
        )
    };
    let (path, qs) = pq.split_once('?').unwrap_or((&pq, ""));
    (path == "/search")
        .then(|| ListingQuery::from_query(None, qs).q)
        .flatten()
}

/// Worth asking about: two letters or digits, in any script.
fn askable(q: &str) -> bool {
    q.chars().filter(|c| c.is_alphanumeric()).count() >= 2
}

#[island]
pub fn HeaderSearch(#[prop(optional)] kind: Option<SearchBox>) -> impl IntoView {
    let ui = use_ui();
    let kind = kind.unwrap_or_default();
    let p: &'static str = match kind {
        SearchBox::Header => "hs",
        SearchBox::Page => "ps",
    };
    let initial = url_query().unwrap_or_default();
    let text = RwSignal::new(initial.clone());
    let open = RwSignal::new(false);
    let overlay = RwSignal::new(false);
    let active = RwSignal::new(None::<usize>);
    // The last answer and the question it answers.
    let answer = RwSignal::new(None::<(String, Result<SearchResults, ()>)>);
    let loading = RwSignal::new(false);
    let seq = StoredValue::new(0u64);
    let timer = StoredValue::new(None::<TimeoutHandle>);
    let root = NodeRef::<html::Div>::new();
    let form = NodeRef::<html::Form>::new();
    let input = NodeRef::<html::Input>::new();
    let trigger = NodeRef::<html::A>::new();

    let options = Memo::new(move |_| {
        answer.with(|a| {
            let Some((asked, Ok(r))) = a else {
                return Vec::new();
            };
            let mut out = Vec::new();
            for c in &r.categories {
                let href = ListingQuery {
                    category: Some(c.slug.clone()),
                    q: Some(asked.clone()),
                    page: 1,
                    ..Default::default()
                }
                .href();
                out.push(Opt::Category {
                    href,
                    cat: c.clone(),
                });
            }
            for h in &r.hits {
                out.push(Opt::Product {
                    href: format!("/p/{}", h.card.slug),
                    hit: h.clone(),
                });
            }
            if r.total > 0 {
                let href = ListingQuery {
                    q: Some(asked.clone()),
                    page: 1,
                    ..Default::default()
                }
                .href();
                out.push(Opt::All {
                    href,
                    total: r.total,
                });
            }
            out
        })
    });
    // The panel shows when the box is engaged and there is something to say.
    let showing = move || {
        open.get() && askable(&text.get()) && (answer.with(Option::is_some) || loading.get())
    };

    let ask = move |q: String| {
        let me = seq.try_update_value(|s| {
            *s += 1;
            *s
        });
        loading.set(true);
        leptos::task::spawn_local(async move {
            let res = api::search_products(q.clone(), HITS).await;
            // A newer question is already under way: this answer is stale.
            if seq.get_value() != me.unwrap_or_default() {
                return;
            }
            loading.set(false);
            active.set(None);
            answer.set(Some((q, res.map_err(|_| ()))));
        });
    };
    let cancel_timer = move || {
        if let Some(h) = timer.try_update_value(Option::take).flatten() {
            h.clear();
        }
    };
    let schedule = move |v: &str| {
        cancel_timer();
        let q = v.trim().to_owned();
        if !askable(&q) {
            seq.update_value(|s| *s += 1);
            loading.set(false);
            answer.set(None);
            return;
        }
        if answer.with_untracked(|a| a.as_ref().is_some_and(|(asked, _)| *asked == q)) {
            // Back to the question already answered: whatever is in flight is now stale.
            seq.update_value(|s| *s += 1);
            loading.set(false);
            return;
        }
        let h = set_timeout_with_handle(
            move || {
                timer.set_value(None);
                ask(q);
            },
            DEBOUNCE,
        );
        timer.set_value(h.ok());
    };

    let close_list = move || {
        open.set(false);
        active.set(None);
    };
    let lock = |on: bool| {
        if let Some(html) = document().document_element() {
            let _ = html.class_list().toggle_with_force("hs-lock", on);
        }
    };
    let close_overlay = move |restore_focus: bool| {
        if let Some(f) = form.get_untracked() {
            let _ = f.hide_popover();
            let _ = f.set_popover(None);
        }
        overlay.set(false);
        close_list();
        lock(false);
        if restore_focus && let Some(t) = trigger.get_untracked() {
            let _ = t.focus();
        }
    };
    let open_overlay = move |ev: ev::MouseEvent| {
        if ev.button() != 0 || ev.meta_key() || ev.ctrl_key() || ev.shift_key() || ev.alt_key() {
            return;
        }
        ev.prevent_default();
        // The same form becomes a full-screen sheet in the top layer: above every stacking
        // context, and positioned against the viewport even though the header's material
        // (`backdrop-filter`) would otherwise contain a `position: fixed` child. It is shown and
        // focused inside this tap: iOS only raises the keyboard for a focus() made while handling
        // the gesture (the reactive class below follows a tick later).
        if let Some(f) = form.get_untracked() {
            let _ = f.class_list().add_1("is-overlay");
            let _ = f.set_popover(Some("manual"));
            let _ = f.show_popover();
        }
        overlay.set(true);
        lock(true);
        if let Some(i) = input.get_untracked() {
            let _ = i.focus();
        }
        if answer.with_untracked(Option::is_some) {
            open.set(true);
        }
    };

    let opt_id = move |i: usize| format!("{p}-o-{i}");
    let click_option = move |i: usize| {
        if let Some(el) = document()
            .get_element_by_id(&opt_id(i))
            .and_then(|e| e.dyn_into::<web_sys::HtmlElement>().ok())
        {
            el.click();
        }
    };
    let reveal = move |i: usize| {
        if let Some(el) = document().get_element_by_id(&opt_id(i)) {
            let opts = web_sys::ScrollIntoViewOptions::new();
            opts.set_block(web_sys::ScrollLogicalPosition::Nearest);
            el.scroll_into_view_with_scroll_into_view_options(&opts);
        }
    };

    let on_keydown = move |ev: ev::KeyboardEvent| {
        let n = options.with_untracked(Vec::len);
        match ev.key().as_str() {
            key @ ("ArrowDown" | "ArrowUp") => {
                ev.prevent_default();
                open.set(true);
                if n == 0 {
                    return;
                }
                let next = match (active.get_untracked(), key == "ArrowDown") {
                    (None, true) => 0,
                    (None, false) => n - 1,
                    (Some(i), true) => (i + 1) % n,
                    (Some(i), false) => (i + n - 1) % n,
                };
                active.set(Some(next));
                reveal(next);
            }
            "Enter" => {
                if let Some(i) = active
                    .get_untracked()
                    .filter(|i| *i < n && open.get_untracked())
                {
                    ev.prevent_default();
                    click_option(i);
                }
                // Otherwise the form submits: to the islands router with JS, the server without.
            }
            "Escape" => {
                if overlay.get_untracked() {
                    ev.prevent_default();
                    close_overlay(true);
                } else if open.get_untracked() && (n > 0 || loading.get_untracked()) {
                    ev.prevent_default();
                    close_list();
                } else if !text.get_untracked().is_empty() {
                    ev.prevent_default();
                    cancel_timer();
                    text.set(String::new());
                    answer.set(None);
                }
            }
            "Tab" => close_list(),
            _ => {}
        }
    };
    let on_input = move |ev: ev::Event| {
        let v = event_target_value(&ev);
        text.set(v.clone());
        active.set(None);
        open.set(true);
        schedule(&v);
    };
    let on_submit = move |ev: ev::SubmitEvent| {
        cancel_timer();
        if text.get_untracked().trim().is_empty() {
            ev.prevent_default();
            if let Some(i) = input.get_untracked() {
                let _ = i.focus();
            }
            return;
        }
        close_list();
        if overlay.get_untracked() {
            close_overlay(false);
        }
        // Drop the phone keyboard; the submission itself is left to the router / the browser.
        if let Some(i) = input.get_untracked() {
            let _ = i.blur();
        }
    };
    let on_focusout = move |ev: ev::FocusEvent| {
        if overlay.get_untracked() {
            return;
        }
        let inside = match (
            ev.related_target()
                .and_then(|t| t.dyn_into::<web_sys::Node>().ok()),
            root.get_untracked(),
        ) {
            (Some(n), Some(r)) => r.contains(Some(&n)),
            _ => false,
        };
        if !inside {
            close_list();
        }
    };
    let clear = move |_| {
        cancel_timer();
        seq.update_value(|s| *s += 1);
        text.set(String::new());
        answer.set(None);
        loading.set(false);
        if let Some(i) = input.get_untracked() {
            let _ = i.focus();
        }
    };

    let placeholder = move || {
        tr(
            ui.lang.get(),
            "Search products and brands",
            "دوّر على منتج أو ماركة",
        )
    };
    let status = move || {
        let lang = ui.lang.get();
        let n = options.with(|o| {
            o.iter()
                .filter(|o| matches!(o, Opt::Product { .. }))
                .count()
        }) as u32;
        if !showing() || loading.get() {
            String::new()
        } else {
            match lang {
                Lang::En => format!(
                    "{} {}",
                    format::count(n, ui.numerals.get()),
                    format::plural_en(n, "suggestion", "suggestions")
                ),
                Lang::Ar => format!(
                    "{} {}",
                    format::count(n, ui.numerals.get()),
                    format::plural_ar(n, "اقتراح", "اقتراحان", "اقتراحات", "اقتراحًا")
                ),
            }
        }
    };

    let panel = move || {
        let lang = ui.lang.get();
        let num = ui.numerals.get();
        answer.with(|a| match a {
            None => skeleton_rows().into_any(),
            Some((_, Err(()))) => view! {
                <div class="hs-note" role="alert">
                    <p>{tr(lang, "Search isn't available right now. Try again in a moment.", "البحث غير متاح الآن. حاول مرة أخرى بعد قليل.")}</p>
                </div>
            }
            .into_any(),
            Some((asked, Ok(r))) if r.hits.is_empty() && r.categories.is_empty() => view! {
                <div class="hs-note">
                    <p class="hs-note__title">{format!("{} {}", tr(lang, "No matches for", "مفيش نتائج لـ"), quoted(asked, lang))}</p>
                    <p>{tr(lang, "Check the spelling or try a different word.", "اتأكد من الكلمة أو جرّب كلمة تانية.")}</p>
                </div>
            }
            .into_any(),
            Some((asked, Ok(_))) => {
                let asked = asked.clone();
                options.with(|opts| listbox(p, opts, &asked, lang, num, active, opt_id))
            }
        })
    };

    let header = kind == SearchBox::Header;
    view! {
        <div
            class="hs"
            class:hs--page=!header
            class:is-loading=move || loading.get()
            node_ref=root
            on:focusout=on_focusout
        >
            {header.then(|| view! {
                <a class="hs-trigger" href="/search" node_ref=trigger on:click=open_overlay aria-haspopup="dialog">
                    <Icon name="search" size=20/>
                    <span class="hs-trigger__text" class:is-filled=move || !text.with(String::is_empty)>
                        {move || if text.with(String::is_empty) { placeholder().to_owned() } else { text.get() }}
                    </span>
                </a>
            })}
            <form
                class="hs-form"
                class:is-overlay=move || overlay.get()
                action="/search"
                method="get"
                role="search"
                node_ref=form
                on:submit=on_submit
            >
                <div class="hs-field">
                    {header.then(|| view! {
                        <button
                            type="button"
                            class="hs-back"
                            aria-label=move || tr(ui.lang.get(), "Close search", "اقفل البحث")
                            on:click=move |_| close_overlay(true)
                        >
                            <Icon name="arrow-left" size=20 class="flip-rtl"/>
                        </button>
                    })}
                    <span class="hs-lead" aria-hidden="true">
                        <Icon name="search" size=18 class="hs-glass"/>
                        <Icon name="loader-circle" size=18 class="hs-busy spin"/>
                    </span>
                    <input
                        class="hs-input"
                        id=format!("{p}-input")
                        type="search"
                        name="q"
                        role="combobox"
                        aria-autocomplete="list"
                        aria-controls=format!("{p}-lb")
                        aria-expanded=move || if showing() { "true" } else { "false" }
                        aria-activedescendant=move || active.get().filter(|_| showing()).map(opt_id)
                        aria-label=move || tr(ui.lang.get(), "Search products", "دوّر في المنتجات")
                        autocomplete="off"
                        autocapitalize="off"
                        spellcheck="false"
                        enterkeyhint="search"
                        maxlength="80"
                        placeholder=placeholder
                        value=initial
                        prop:value=move || text.get()
                        node_ref=input
                        on:input=on_input
                        on:keydown=on_keydown
                        on:focus=move |_| {
                            if answer.with_untracked(Option::is_some) {
                                open.set(true);
                            }
                        }
                    />
                    <button
                        type="button"
                        class="hs-clear"
                        hidden=move || text.with(String::is_empty)
                        aria-label=move || tr(ui.lang.get(), "Clear", "امسح")
                        on:click=clear
                    >
                        <Icon name="x" size=18/>
                    </button>
                    {(!header).then(|| view! {
                        // The page's box names its action; the header's submits with Enter (the
                        // phone keyboard shows "Search") and stays one quiet field.
                        <button type="submit" class="hs-submit">
                            <span class="hs-submit__label">{move || tr(ui.lang.get(), "Search", "دوّر")}</span>
                        </button>
                    })}
                </div>
                <div
                    class="hs-panel"
                    hidden=move || !showing()
                    on:mousedown=|ev: ev::MouseEvent| ev.prevent_default()
                    on:click=move |ev: ev::MouseEvent| {
                        let picked = ev
                            .target()
                            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                            .and_then(|el| el.closest("[role=option]").ok().flatten())
                            .is_some();
                        if picked {
                            close_list();
                            if overlay.get_untracked() {
                                close_overlay(false);
                            }
                        }
                    }
                >
                    {panel}
                </div>
                <p class="sr-only" role="status">{status}</p>
            </form>
        </div>
    }
}

/// The options, grouped: categories, products, then "see all".
fn listbox(
    p: &'static str,
    opts: &[Opt],
    asked: &str,
    lang: Lang,
    num: crate::prefs::Numerals,
    active: RwSignal<Option<usize>>,
    opt_id: impl Fn(usize) -> String + Copy + 'static,
) -> AnyView {
    let ar = lang.is_rtl();
    let row = move |i: usize, o: &Opt| {
        let selected = move || active.get() == Some(i);
        let hover = move |_| {
            if active.get_untracked() != Some(i) {
                active.set(Some(i));
            }
        };
        match o {
            Opt::Category { href, cat } => EitherOf3::A(view! {
                <a
                    role="option"
                    id=opt_id(i)
                    class="hs-opt hs-opt--cat"
                    href=href.clone()
                    tabindex="-1"
                    aria-selected=move || if selected() { "true" } else { "false" }
                    on:mousemove=hover
                >
                    <span class="hs-opt__ic" aria-hidden="true"><Icon name=cat.icon.clone() size=18/></span>
                    <span class="hs-opt__main">
                        {quoted(asked, lang)}
                        " "
                        {tr(lang, "in", "في")}
                        " "
                        <strong>{cat.name.pick(ar).to_owned()}</strong>
                    </span>
                </a>
            }),
            Opt::Product { href, hit } => {
                let segs = if ar {
                    hit.name_ar.clone()
                } else {
                    hit.name_en.clone()
                };
                let img = hit.card.img.clone();
                EitherOf3::B(view! {
                    <a
                        role="option"
                        id=opt_id(i)
                        class="hs-opt hs-opt--hit"
                        href=href.clone()
                        tabindex="-1"
                        aria-selected=move || if selected() { "true" } else { "false" }
                        on:mousemove=hover
                    >
                        <span class="hs-thumb" style=format!("--hs-tint:{}", img.color)>
                            {(!img.is_missing()).then(|| view! {
                                <img src=img.src_near(96) srcset=img.srcset() sizes="44px" alt="" width="44" height="44" loading="lazy" decoding="async"/>
                            })}
                        </span>
                        <span class="hs-opt__main">
                            <span class="hs-name">{marked(segs)}</span>
                            <span class="hs-meta">{hit.category.pick(ar).to_owned()}</span>
                        </span>
                        <span class="hs-price">
                            <Price amount=hit.card.price compare_at=hit.card.compare_at size=PriceSize::Sm no_badge=true lang=lang numerals=num/>
                        </span>
                    </a>
                })
            }
            Opt::All { href, total } => EitherOf3::C(view! {
                <a
                    role="option"
                    id=opt_id(i)
                    class="hs-opt hs-opt--all"
                    href=href.clone()
                    tabindex="-1"
                    aria-selected=move || if selected() { "true" } else { "false" }
                    on:mousemove=hover
                >
                    <span class="hs-opt__ic" aria-hidden="true"><Icon name="search" size=18/></span>
                    <span class="hs-opt__main">
                        {match lang {
                            Lang::En => format!("See all {} results for {}", format::int(*total as i64, num), quoted(asked, lang)),
                            Lang::Ar => format!("شوف كل النتائج لـ{} ({})", quoted(asked, lang), format::int(*total as i64, num)),
                        }}
                    </span>
                    <Icon name="arrow-right" size=18 class="flip-rtl hs-opt__go"/>
                </a>
            }),
        }
    };
    let cats: Vec<_> = opts
        .iter()
        .enumerate()
        .filter(|(_, o)| matches!(o, Opt::Category { .. }))
        .map(|(i, o)| row(i, o))
        .collect();
    let hits: Vec<_> = opts
        .iter()
        .enumerate()
        .filter(|(_, o)| matches!(o, Opt::Product { .. }))
        .map(|(i, o)| row(i, o))
        .collect();
    let all: Vec<_> = opts
        .iter()
        .enumerate()
        .filter(|(_, o)| matches!(o, Opt::All { .. }))
        .map(|(i, o)| row(i, o))
        .collect();
    let group = move |label: &'static str, rows: Vec<_>| {
        (!rows.is_empty()).then(|| {
            view! {
                <div class="hs-group" role="group" aria-label=label data-label=label>
                    {rows}
                </div>
            }
        })
    };
    view! {
        <div class="hs-list" role="listbox" id=format!("{p}-lb") aria-label=tr(lang, "Suggestions", "اقتراحات")>
            {group(tr(lang, "Categories", "الأقسام"), cats)}
            {group(tr(lang, "Products", "المنتجات"), hits)}
            {all}
        </div>
    }
    .into_any()
}

/// Matched runs in `<mark>`.
fn marked(segs: Vec<Seg>) -> impl IntoView {
    segs.into_iter()
        .map(|s| {
            if s.hit {
                Either::Left(view! { <mark class="hs-mark">{s.text}</mark> })
            } else {
                Either::Right(s.text)
            }
        })
        .collect_view()
}

/// While the first answer is on its way: rows the size of real ones.
fn skeleton_rows() -> impl IntoView {
    view! {
        <div class="hs-list" aria-hidden="true">
            {(0..3).map(|_| view! {
                <div class="hs-opt hs-opt--hit hs-opt--skel">
                    <span class="hs-thumb skeleton"></span>
                    <span class="hs-opt__main">
                        <span class="skeleton hs-skel-line"></span>
                        <span class="skeleton hs-skel-line hs-skel-line--short"></span>
                    </span>
                </div>
            }).collect_view()}
        </div>
    }
}
