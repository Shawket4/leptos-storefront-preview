//! The showcase's islands: one small, focused island per live demo. Everything around them is
//! server-rendered. Every string an island renders follows the shared `Ui` store (the language
//! toggle swaps the server HTML around an island, never inside it).

use crate::api::{
    self,
    types::{CacheInfo, Rail},
};
use crate::components::button::{Button, ButtonKind, ButtonSize};
use crate::components::icons::Icon;
use crate::format;
use crate::media::Img;
use crate::pages::errors::SectionError;
use crate::prefs::{Lang, Numerals, tr, use_ui};
use domain::Piastres;
use leptos::form::ActionForm;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "hydrate")]
use super::js;
#[cfg(feature = "hydrate")]
use wasm_bindgen::{JsCast, closure::Closure};

use super::notes::{self, Board};

// ─── Small formatting helpers (shared by islands and the page) ──────────────────────────────────

/// `350 ms` / `٣٥٠ ms`.
pub fn ms(v: f64, n: Numerals) -> String {
    format!("{} ms", format::int(v.round() as i64, n))
}

/// `36.4 KB` / `1.3 MB` in the visitor's numerals.
pub fn size(bytes: f64, n: Numerals) -> String {
    let kb = bytes / 1024.0;
    if kb >= 1024.0 {
        format!("{} MB", format::dec1((kb / 1024.0) as f32, n))
    } else if kb >= 100.0 {
        format!("{} KB", format::int(kb.round() as i64, n))
    } else {
        format!("{} KB", format::dec1(kb as f32, n))
    }
}

fn money(p: i64, lang: Lang, n: Numerals) -> String {
    format::money(Piastres(p), lang, n)
}

// ─── Table of contents: category filter (+ same-page anchor links) ──────────────────────────────

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Chip {
    pub code: String,
    pub en: String,
    pub ar: String,
    pub count: u32,
}

/// The category chips. Without JS each chip is a link (`?cat=…`, filtered on the server); with JS
/// a click sets `data-filter` on the page root and CSS hides the other categories — no request.
///
/// It also keeps same-page `#anchor` links native: the islands router would otherwise intercept
/// them as navigations (re-fetch the page, no scroll). See "Sharp edges".
#[island]
pub fn TocFilter(chips: Vec<Chip>, active: String) -> impl IntoView {
    let ui = use_ui();
    let current = RwSignal::new(active);
    #[cfg(feature = "hydrate")]
    keep_anchor_links_native();
    let items = chips
        .into_iter()
        .map(|c| {
            let code = c.code.clone();
            let href = if code == "all" {
                "/showcase#features".to_owned()
            } else {
                format!("/showcase?cat={code}#features")
            };
            let on = {
                let code = code.clone();
                move || current.get() == code
            };
            let pick = move |ev: leptos::ev::MouseEvent| {
                ev.prevent_default();
                current.set(code.clone());
                #[cfg(feature = "hydrate")]
                {
                    let doc = document();
                    if let Some(root) = doc.get_element_by_id("sc") {
                        let _ = root.set_attribute("data-filter", &code);
                    }
                    // If the visitor is already below the list's start, bring its top into view.
                    if let Some(list) = doc.get_element_by_id("features") {
                        if list.get_bounding_client_rect().top() < 0.0 {
                            list.scroll_into_view();
                        }
                    }
                }
            };
            let (en, ar) = (c.en, c.ar);
            let n = c.count;
            view! {
                <a
                    class="sc-chip"
                    href=href
                    on:click=pick
                    aria-current=move || if on() { "true" } else { "false" }
                >
                    <span class="sc-chip__label">{move || if ui.lang.get().is_rtl() { ar.clone() } else { en.clone() }}</span>
                    <span class="sc-chip__n mono">{move || format::count(n, ui.numerals.get())}</span>
                </a>
            }
        })
        .collect_view();
    view! {
        <nav class="sc-filter" aria-label=move || tr(ui.lang.get(), "Filter by category", "فلترة حسب الفئة")>
            {items}
        </nav>
    }
}

/// Same-page anchors (`href="#…"`) are handled here, not by the islands router: it would treat
/// the click as a navigation (re-fetch the page, no scroll), and even a native jump fires
/// `popstate`, which the router also answers with a full re-fetch. So: scroll to the target,
/// move focus there, and `replaceState` the hash (no new history entry, no popstate). Without
/// JavaScript the browser does the jump itself.
#[cfg(feature = "hydrate")]
fn keep_anchor_links_native() {
    use std::sync::atomic::{AtomicBool, Ordering};
    static DONE: AtomicBool = AtomicBool::new(false);
    if DONE.swap(true, Ordering::Relaxed) {
        return;
    }
    let cb = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(|ev: web_sys::MouseEvent| {
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
        let Ok(Some(link)) = el.closest("a[href^='#']") else {
            return;
        };
        let id = link
            .get_attribute("href")
            .unwrap_or_default()
            .trim_start_matches('#')
            .to_owned();
        let Some(target) = (!id.is_empty())
            .then(|| document().get_element_by_id(&id))
            .flatten()
        else {
            return;
        };
        ev.prevent_default();
        ev.stop_propagation();
        // A jump from the phone contents menu also closes the menu.
        if let Ok(Some(menu)) = el.closest("details[open]") {
            let _ = menu.remove_attribute("open");
        }
        let opts = web_sys::ScrollIntoViewOptions::new();
        opts.set_block(web_sys::ScrollLogicalPosition::Start);
        opts.set_behavior(if js::matches_media("(prefers-reduced-motion: reduce)") {
            web_sys::ScrollBehavior::Instant
        } else {
            web_sys::ScrollBehavior::Smooth
        });
        target.scroll_into_view_with_scroll_into_view_options(&opts);
        if let Ok(h) = window().history() {
            let _ =
                h.replace_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&format!("#{id}")));
        }
        if !target.has_attribute("tabindex") {
            let _ = target.set_attribute("tabindex", "-1");
        }
        let _ = js::call(
            &target.into(),
            "focus",
            &[js::object(&[(
                "preventScroll",
                wasm_bindgen::JsValue::TRUE,
            )])],
        );
    });
    let _ = document().add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
    cb.forget();
}

// ─── Islands: a counter (+ a navigation probe for the islands-router demo) ─────────────────────

#[derive(Clone, Debug, Default, PartialEq)]
struct NavStat {
    ms: f64,
    wire: f64,
    html: f64,
}

/// A counter whose state lives in the browser. With `probe`, it also shows how long this
/// document has existed (only a real page load resets it) and what the last client-side
/// navigation cost.
#[island]
pub fn IslandCounter(probe: bool) -> impl IntoView {
    let ui = use_ui();
    let count = RwSignal::new(0i64);
    let age = RwSignal::new(None::<f64>);
    let navs = RwSignal::new(0u32);
    let last = RwSignal::new(None::<NavStat>);
    // Browser-only values are set after hydration (an Effect), so the first client render
    // matches the server's HTML.
    #[cfg(feature = "hydrate")]
    if probe {
        Effect::new(move |_| {
            js::grow_resource_buffer();
            let origin = js::time_origin();
            let tick = move || age.set(Some(((js_sys::Date::now() - origin) / 1000.0).max(0.0)));
            tick();
            if let Ok(h) = set_interval_with_handle(tick, std::time::Duration::from_secs(1)) {
                on_cleanup(move || h.clear());
            }
            // The server stamps each render of the tab panel; the router's diff rewrites that text.
            watch_text("router-stamp", move || {
                navs.update(|n| *n += 1);
                set_timeout(
                    move || {
                        let t = js::resources()
                            .into_iter()
                            .filter(|t| t.initiator == "fetch" && t.name.contains("/showcase"))
                            .last();
                        if let Some(t) = t {
                            last.set(Some(NavStat {
                                ms: t.duration,
                                wire: t.encoded,
                                html: t.decoded,
                            }));
                        }
                    },
                    std::time::Duration::from_millis(50),
                );
            });
        });
    }
    let probe_view = probe.then(|| {
        view! {
            <dl class="sc-probe">
                <div>
                    <dt>{move || tr(ui.lang.get(), "This document is", "عمر الصفحة دي")}</dt>
                    <dd class="mono">
                        {move || match age.get() {
                            Some(s) => {
                                let n = ui.numerals.get();
                                let s = s.floor() as i64;
                                match ui.lang.get() {
                                    Lang::En => format!("{} s old", format::int(s, n)),
                                    Lang::Ar => format!("{} ث", format::int(s, n)),
                                }
                            }
                            None => "—".to_owned(),
                        }}
                    </dd>
                </div>
                <div>
                    <dt>{move || tr(ui.lang.get(), "Client navigations", "تنقلات من غير ريلود")}</dt>
                    <dd class="mono">{move || format::count(navs.get(), ui.numerals.get())}</dd>
                </div>
                <div>
                    <dt>{move || tr(ui.lang.get(), "Last one", "آخر واحد")}</dt>
                    <dd class="mono">
                        {move || match last.get() {
                            Some(s) => {
                                let n = ui.numerals.get();
                                format!("{} · {} → {} HTML", ms(s.ms, n), size(s.wire, n), size(s.html, n))
                            }
                            None => "—".to_owned(),
                        }}
                    </dd>
                </div>
            </dl>
        }
    });
    view! {
        <div class="sc-counter">
            <button
                class="sc-ibtn"
                type="button"
                on:click=move |_| count.update(|n| *n -= 1)
                aria-label=move || tr(ui.lang.get(), "Decrease", "نقّص")
            >
                <Icon name="minus" size=18/>
            </button>
            <output class="sc-counter__value mono" aria-live="polite">
                {move || format::int(count.get(), ui.numerals.get())}
            </output>
            <button
                class="sc-ibtn"
                type="button"
                on:click=move |_| count.update(|n| *n += 1)
                aria-label=move || tr(ui.lang.get(), "Increase", "زوّد")
            >
                <Icon name="plus" size=18/>
            </button>
            <span class="sc-counter__label">{move || tr(ui.lang.get(), "state lives in this island", "الحالة جوه الـ island دي")}</span>
        </div>
        {probe_view}
    }
}

/// Call `f` whenever the text inside element `#id` changes (the islands router rewrites it).
#[cfg(feature = "hydrate")]
fn watch_text(id: &'static str, f: impl Fn() + 'static) {
    let Some(el) = document().get_element_by_id(id) else {
        return;
    };
    let cb = Closure::<dyn FnMut(js_sys::Array, web_sys::MutationObserver)>::new(
        move |_: js_sys::Array, _: web_sys::MutationObserver| f(),
    );
    if let Ok(obs) = web_sys::MutationObserver::new(cb.as_ref().unchecked_ref()) {
        let opts = web_sys::MutationObserverInit::new();
        opts.set_character_data(true);
        opts.set_child_list(true);
        opts.set_subtree(true);
        let _ = obs.observe_with_options(&el, &opts);
    }
    cb.forget();
}

// ─── Bundle probe: what this page costs in the browser ─────────────────────────────────────────

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskFile {
    pub name: String,
    pub bytes: u64,
}

#[derive(Clone, Debug, Default, PartialEq)]
struct PageCost {
    /// (file name, bytes over the wire, bytes decoded)
    wire: Vec<(String, f64, f64)>,
    islands: u32,
    kinds: u32,
    elements: u32,
    comments: u32,
    markers: u32,
}

/// Measures, in the visitor's browser: the WASM/JS actually transferred (Resource Timing), how
/// many islands this page hydrates, and how big the DOM is. The server passes the on-disk sizes.
#[island]
pub fn BundleProbe(files: Vec<DiskFile>, debug_build: bool) -> impl IntoView {
    let ui = use_ui();
    let cost = RwSignal::new(None::<PageCost>);
    #[cfg(feature = "hydrate")]
    {
        let names: Vec<String> = files.iter().map(|f| f.name.clone()).collect();
        let measure = move || {
            let entries = js::resources();
            let wire = names
                .iter()
                .filter_map(|n| {
                    entries
                        .iter()
                        .rev()
                        .find(|e| e.name.ends_with(&format!("/{n}")))
                        .map(|e| (n.clone(), e.encoded, e.decoded))
                })
                .collect();
            let doc = document();
            let (mut islands, mut kinds) = (0u32, std::collections::BTreeSet::new());
            if let Ok(list) = doc.query_selector_all("leptos-island") {
                islands = list.length();
                for i in 0..list.length() {
                    if let Some(el) = list
                        .get(i)
                        .and_then(|n| n.dyn_into::<web_sys::Element>().ok())
                    {
                        let c = el.get_attribute("data-component").unwrap_or_default();
                        kinds.insert(c.rsplit_once('_').map(|(k, _)| k.to_owned()).unwrap_or(c));
                    }
                }
            }
            cost.set(Some(PageCost {
                wire,
                islands,
                kinds: kinds.len() as u32,
                elements: doc.get_elements_by_tag_name("*").length(),
                comments: js::xpath_count("count(//comment())") as u32,
                markers: js::xpath_count(
                    "count(//comment()[starts-with(., 'bo-') or starts-with(., 'bc-')])",
                ) as u32,
            }));
        };
        Effect::new(move |_| set_timeout(measure.clone(), std::time::Duration::from_millis(400)));
    }
    let rows = files
        .into_iter()
        .map(|f| {
            let name = f.name.clone();
            let disk = f.bytes as f64;
            view! {
                <div class="sc-kv">
                    <dt class="mono" dir="ltr">{f.name}</dt>
                    <dd>
                        <span class="mono">{move || size(disk, ui.numerals.get())}</span>
                        <span class="sc-kv__sub">{move || tr(ui.lang.get(), " on disk", " على الديسك")}</span>
                        <span class="sc-kv__sep">" · "</span>
                        <span class="mono">
                            {
                                let name = name.clone();
                                move || {
                                    let n = ui.numerals.get();
                                    cost.get()
                                        .and_then(|c| c.wire.iter().find(|w| w.0 == name).cloned())
                                        .map(|w| size(w.1, n))
                                        .unwrap_or_else(|| "—".into())
                                }
                            }
                        </span>
                        <span class="sc-kv__sub">{move || tr(ui.lang.get(), " over the wire here", " اتنقلوا هنا فعلًا")}</span>
                    </dd>
                </div>
            }
        })
        .collect_view();
    view! {
        <dl class="sc-kvs">
            {rows}
            <div class="sc-kv">
                <dt>{move || tr(ui.lang.get(), "Islands on this page", "الـ islands في الصفحة دي")}</dt>
                <dd class="mono">
                    {move || cost.get().map(|c| {
                        let n = ui.numerals.get();
                        match ui.lang.get() {
                            Lang::En => format!("{} ({} kinds)", format::count(c.islands, n), format::count(c.kinds, n)),
                            Lang::Ar => format!("{} ({} أنواع)", format::count(c.islands, n), format::count(c.kinds, n)),
                        }
                    }).unwrap_or_else(|| "—".into())}
                </dd>
            </div>
            <div class="sc-kv">
                <dt>{move || tr(ui.lang.get(), "DOM", "الـ DOM")}</dt>
                <dd class="mono">
                    {move || cost.get().map(|c| {
                        let n = ui.numerals.get();
                        match ui.lang.get() {
                            Lang::En => format!("{} elements · {} comments ({} branch markers)", format::int(c.elements as i64, n), format::int(c.comments as i64, n), format::int(c.markers as i64, n)),
                            Lang::Ar => format!("{} عنصر · {} تعليق ({} branch markers)", format::int(c.elements as i64, n), format::int(c.comments as i64, n), format::int(c.markers as i64, n)),
                        }
                    }).unwrap_or_else(|| "—".into())}
                </dd>
            </div>
        </dl>
        <p class="sc-note">
            {move || match (ui.lang.get(), debug_build) {
                (Lang::En, true) => "Debug build (unoptimised, type-erased WASM). The release build — opt-level \"z\", LTO, strip ([profile.wasm-release]) — measured 1,131 KB raw, 440 KB gzip, 326 KB brotli for every island in the app (tools/wasm-size.mjs, 26 Oct).",
                (Lang::Ar, true) => "دي نسخة debug (الـ WASM من غير تحسين ومن غير أنواع). نسخة الـ release — ‏opt-level \"z\" و LTO و strip ‏([profile.wasm-release]) — اتقاست ١١٣١ KB خام، و ٤٤٠ KB بـ gzip، و ٣٢٦ KB بـ brotli لكل الـ islands في الأبلكيشن (‏tools/wasm-size.mjs، ٢٦ أكتوبر).",
                (Lang::En, false) => "Release build: opt-level \"z\", LTO and strip ([profile.wasm-release]).",
                (Lang::Ar, false) => "نسخة release: ‏opt-level \"z\" و LTO و strip ‏([profile.wasm-release]).",
            }}
        </p>
    }
}

// ─── Fine-grained reactivity: a price calculator ───────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Unit {
    pub en: String,
    pub ar: String,
    /// Piastres.
    pub price: i64,
}

/// Restart the "updated" flash on a text node's wrapper.
fn flash(node: NodeRef<leptos::html::Span>) {
    #[cfg(feature = "hydrate")]
    if let Some(el) = node.get_untracked() {
        let cl = el.class_list();
        let _ = cl.remove_1("is-flash");
        let _ = el.offset_width(); // reflow, so the animation starts again
        let _ = cl.add_1("is-flash");
    }
    #[cfg(not(feature = "hydrate"))]
    let _ = node;
}

/// Count and flash every change of `value` (an Effect: runs after render, in the browser only).
fn watch<T>(
    value: impl Fn() -> T + 'static,
    node: NodeRef<leptos::html::Span>,
    updates: RwSignal<u32>,
) where
    T: PartialEq + Clone + 'static,
{
    Effect::new(move |prev: Option<T>| {
        let v = value();
        if prev.is_some_and(|p| p != v) {
            updates.update(|n| *n += 1);
            flash(node);
        }
        v
    });
}

/// qty × unit (derived signal) → coupon discount (Memo) → delivery fee (Memo, only changes when
/// the subtotal crosses the free-delivery threshold) → total (Memo). Each output counts and flashes
/// its own updates; nothing else is touched.
#[island]
pub fn PriceCalc(units: Vec<Unit>, fee: i64, free_over: i64) -> impl IntoView {
    let ui = use_ui();
    let qty = RwSignal::new(2u32);
    let pick = RwSignal::new(0usize);
    let coupon = RwSignal::new(false);
    let units = StoredValue::new(units);

    // A derived signal: a plain closure, recomputed whenever it is read.
    let subtotal = Signal::derive(move || {
        units.with_value(|u| u.get(pick.get()).map(|u| u.price).unwrap_or(0)) * qty.get() as i64
    });
    // Memos: cached, and they only notify readers when the value actually changes.
    let discount = Memo::new(move |_| if coupon.get() { subtotal.get() / 10 } else { 0 });
    let delivery = Memo::new(move |_| {
        if subtotal.get() - discount.get() >= free_over {
            0
        } else {
            fee
        }
    });
    let total = Memo::new(move |_| subtotal.get() - discount.get() + delivery.get());

    let refs = [NodeRef::<leptos::html::Span>::new(); 4];
    let counts = [RwSignal::new(0u32); 4];
    watch(move || subtotal.get(), refs[0], counts[0]);
    watch(move || discount.get(), refs[1], counts[1]);
    watch(move || delivery.get(), refs[2], counts[2]);
    watch(move || total.get(), refs[3], counts[3]);

    let options = units.with_value(|u| {
        u.iter()
            .enumerate()
            .map(|(i, u)| {
                let (en, ar, p) = (u.en.clone(), u.ar.clone(), u.price);
                view! {
                    <option value=i selected=i == 0>
                        {move || {
                            let l = ui.lang.get();
                            format!("{} — {}", if l.is_rtl() { &ar } else { &en }, money(p, l, ui.numerals.get()))
                        }}
                    </option>
                }
            })
            .collect_view()
    });

    let row = move |i: usize,
                    label: (&'static str, &'static str),
                    kind: (&'static str, &'static str),
                    value: Signal<i64>,
                    total: bool| {
        view! {
            <div class="sc-calc__row" class:sc-calc__row--total=total>
                <dt>
                    <span>{move || tr(ui.lang.get(), label.0, label.1)}</span>
                    <span class="sc-tag mono">{move || tr(ui.lang.get(), kind.0, kind.1)}</span>
                </dt>
                <dd>
                    <span class="sc-flash mono" node_ref=refs[i]>
                        {move || money(value.get(), ui.lang.get(), ui.numerals.get())}
                    </span>
                    <span class="sc-calc__n mono" title=move || tr(ui.lang.get(), "times this text node was rewritten", "عدد مرات تحديث النص ده")>
                        {move || format!("×{}", format::count(counts[i].get(), ui.numerals.get()))}
                    </span>
                </dd>
            </div>
        }
    };

    view! {
        <div class="sc-calc">
            <div class="sc-calc__inputs">
                <label class="sc-field">
                    <span class="sc-field__label">{move || tr(ui.lang.get(), "Product", "المنتج")}</span>
                    <select class="sc-select" on:change=move |ev| pick.set(event_target_value(&ev).parse().unwrap_or(0))>
                        {options}
                    </select>
                </label>
                <div class="sc-field">
                    <span class="sc-field__label" id="sc-qty-label">{move || tr(ui.lang.get(), "Quantity", "الكمية")}</span>
                    <div class="sc-step" role="group" aria-labelledby="sc-qty-label">
                        <button class="sc-ibtn" type="button" on:click=move |_| qty.update(|q| *q = q.saturating_sub(1).max(1))
                            aria-label=move || tr(ui.lang.get(), "One less", "واحد أقل")>
                            <Icon name="minus" size=18/>
                        </button>
                        <output class="sc-step__value mono" aria-live="polite">{move || format::count(qty.get(), ui.numerals.get())}</output>
                        <button class="sc-ibtn" type="button" on:click=move |_| qty.update(|q| *q = (*q + 1).min(20))
                            aria-label=move || tr(ui.lang.get(), "One more", "واحد كمان")>
                            <Icon name="plus" size=18/>
                        </button>
                    </div>
                </div>
                <label class="sc-check">
                    <input type="checkbox" on:change=move |ev| coupon.set(event_target_checked(&ev))/>
                    <span>{move || tr(ui.lang.get(), "Coupon NILE10 (−10%)", "كوبون NILE10 (خصم ١٠٪)")}</span>
                </label>
            </div>
            <dl class="sc-calc__out">
                {row(0, ("Subtotal", "المجموع"), ("derived", "derived"), subtotal, false)}
                {row(1, ("Discount", "الخصم"), ("Memo", "Memo"), discount.into(), false)}
                {row(2, ("Delivery", "التوصيل"), ("Memo", "Memo"), delivery.into(), false)}
                {row(3, ("Total", "الإجمالي"), ("Memo", "Memo"), total.into(), true)}
            </dl>
            <p class="sc-note">
                {move || {
                    let l = ui.lang.get();
                    let over = money(free_over, l, ui.numerals.get());
                    match l {
                        Lang::En => format!("Delivery is free over {over}: its Memo — and its text — only changes when the subtotal crosses that line."),
                        Lang::Ar => format!("التوصيل مجاني فوق {over}: الـ Memo بتاعه — والنص بتاعه — مش بيتغيروا إلا لما المجموع يعدّي الحد ده."),
                    }
                }}
            </p>
        </div>
    }
}

// ─── Shared state across islands: a wishlist store ─────────────────────────────────────────────

/// A tiny global store for the demo, shared by separate islands (like `CartStore`).
#[derive(Clone, Copy, Debug)]
pub struct Saved {
    pub ids: RwSignal<Vec<u32>>,
    /// The last change (product id, saved?) — for the live announcement.
    pub last: RwSignal<Option<(u32, bool)>>,
}

impl Saved {
    fn new() -> Self {
        Saved {
            ids: RwSignal::new(vec![]),
            last: RwSignal::new(None),
        }
    }
}

/// Provide the demo store under the root owner. Meant for `stores::provide_all()` (server per
/// request, client once in `client::init`), like the cart's store.
pub fn provide_store() {
    provide_context(Saved::new());
}

/// The store from context. Until `stores::provide_all()` calls [`provide_store`], the first island
/// that asks provides it on the *root* owner (the one `hydrate_islands()` installs), so every
/// island still finds the same instance — the same mechanism `client::init()` relies on.
fn use_saved() -> Saved {
    if let Some(s) = use_context::<Saved>() {
        return s;
    }
    let s = Saved::new();
    let mut owner = Owner::current();
    while let Some(parent) = owner.as_ref().and_then(|o| o.parent()) {
        owner = Some(parent);
    }
    match owner {
        Some(root) => root.with(|| provide_context(s)),
        None => provide_context(s),
    }
    s
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SaveItem {
    pub id: u32,
    pub en: String,
    pub ar: String,
}

/// One "save" toggle. Each instance is a separate island.
#[island]
pub fn SaveToggle(item: SaveItem) -> impl IntoView {
    let ui = use_ui();
    let s = use_saved();
    let id = item.id;
    let on = Memo::new(move |_| s.ids.with(|v| v.contains(&id)));
    let toggle = move |_| {
        let now_saved = !on.get_untracked();
        s.ids.update(|v| {
            if now_saved {
                v.push(id)
            } else {
                v.retain(|x| *x != id)
            }
        });
        s.last.set(Some((id, now_saved)));
    };
    let (en, ar) = (item.en, item.ar);
    view! {
        <button
            class="sc-save"
            type="button"
            aria-pressed=move || on.get().to_string()
            aria-label=move || match ui.lang.get() {
                Lang::En => format!("Save {en}"),
                Lang::Ar => format!("احفظ {ar}"),
            }
            on:click=toggle
        >
            <Icon name="heart" size=18/>
            <span>{move || tr(ui.lang.get(), "Save", "احفظ")}</span>
        </button>
    }
}

/// The saved list, a separate island elsewhere on the page — bound to the same store.
#[island]
pub fn SavedTray(items: Vec<SaveItem>) -> impl IntoView {
    let ui = use_ui();
    let s = use_saved();
    let items = StoredValue::new(items);
    let name = move |id: u32, lang: Lang| {
        items.with_value(|v| {
            v.iter()
                .find(|i| i.id == id)
                .map(|i| {
                    if lang.is_rtl() {
                        i.ar.clone()
                    } else {
                        i.en.clone()
                    }
                })
                .unwrap_or_default()
        })
    };
    view! {
        <div class="sc-tray">
            <div class="sc-tray__head">
                <Icon name="heart" size=18/>
                <strong>{move || tr(ui.lang.get(), "Saved", "المحفوظ")}</strong>
                <span class="sc-badge mono">{move || format::count(s.ids.with(|v| v.len() as u32), ui.numerals.get())}</span>
                <button
                    class="sc-link"
                    type="button"
                    on:click=move |_| s.ids.set(vec![])
                    disabled=move || s.ids.with(|v| v.is_empty())
                >
                    {move || tr(ui.lang.get(), "Clear", "امسح")}
                </button>
            </div>
            <ul class="sc-tray__list">
                {move || {
                    let lang = ui.lang.get();
                    let ids = s.ids.get();
                    if ids.is_empty() {
                        view! { <li class="sc-tray__empty">{tr(lang, "Nothing saved yet — tap a heart below.", "لسه مفيش حاجة محفوظة — دوس على قلب من تحت.")}</li> }.into_any()
                    } else {
                        ids.into_iter().map(|id| view! { <li>{name(id, lang)}</li> }).collect_view().into_any()
                    }
                }}
            </ul>
            <p class="sr-only" role="status" aria-live="polite">
                {move || s.last.get().map(|(id, saved)| {
                    let lang = ui.lang.get();
                    let n = name(id, lang);
                    match (lang, saved) {
                        (Lang::En, true) => format!("Saved {n}"),
                        (Lang::En, false) => format!("Removed {n}"),
                        (Lang::Ar, true) => format!("اتحفظ {n}"),
                        (Lang::Ar, false) => format!("اتشال {n}"),
                    }
                })}
            </p>
        </div>
    }
}

// ─── Server functions: the read cache, live ────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq)]
enum LabNote {
    Read { ms: f64, missed: bool, ok: bool },
    Bumped(u64),
    Failed,
}

/// Calls three server functions from the browser: `cache_stats()`, `cache_bump()` and a catalogue
/// read (`home_rail`) to watch a miss turn into a hit.
#[island]
pub fn CacheLab(initial: CacheInfo, tenant: String) -> impl IntoView {
    let ui = use_ui();
    let info = RwSignal::new(initial);
    let note = RwSignal::new(None::<LabNote>);
    let refs = [NodeRef::<leptos::html::Span>::new(); 4];
    let counts = [RwSignal::new(0u32); 4];
    watch(move || info.get().entries, refs[0], counts[0]);
    watch(move || info.get().hits, refs[1], counts[1]);
    watch(move || info.get().misses, refs[2], counts[2]);
    watch(move || info.get().generation, refs[3], counts[3]);

    let refresh = ServerAction::<api::CacheStats>::new();
    let bump = ServerAction::<api::CacheBump>::new();
    // The server's numbers were taken before this page's own reads; fetch fresh ones once hydrated.
    Effect::new(move |_| {
        refresh.dispatch(api::CacheStats {});
    });
    Effect::new(move |_| match refresh.value().get() {
        Some(Ok(v)) => info.set(v),
        Some(Err(_)) => note.set(Some(LabNote::Failed)),
        None => {}
    });
    Effect::new(move |_| match bump.value().get() {
        Some(Ok(v)) => {
            note.set(Some(LabNote::Bumped(v.generation)));
            info.set(v);
        }
        Some(Err(_)) => note.set(Some(LabNote::Failed)),
        None => {}
    });
    // stats → read → stats: the misses counter tells whether the read was served from the cache.
    // A plain (Send) action: an island body also runs during SSR, and `new_local`/`new_unsync`
    // would put SendWrappers in the request's arena, which may be dropped on another worker.
    let read = Action::new(move |_: &()| async move {
        let before = api::cache_stats().await.ok();
        #[cfg(feature = "hydrate")]
        let t0 = js::now();
        let ok = api::home_rail(Rail::Featured, 4).await.is_ok();
        #[cfg(feature = "hydrate")]
        let dt = js::now() - t0;
        #[cfg(not(feature = "hydrate"))]
        let dt = 0.0;
        let after = api::cache_stats().await.ok();
        (ok, dt, before, after)
    });
    Effect::new(move |_| {
        if let Some((ok, dt, before, after)) = read.value().get() {
            match (before, after) {
                (Some(b), Some(a)) => {
                    note.set(Some(LabNote::Read {
                        ms: dt,
                        missed: a.misses > b.misses,
                        ok,
                    }));
                    info.set(a);
                }
                _ => note.set(Some(LabNote::Failed)),
            }
        }
    });
    let busy = Signal::derive(move || {
        refresh.pending().get() || bump.pending().get() || read.pending().get()
    });

    let tile = move |i: usize, label: (&'static str, &'static str), value: Signal<u64>| {
        view! {
            <div class="sc-stat">
                <span class="sc-stat__label">{move || tr(ui.lang.get(), label.0, label.1)}</span>
                <span class="sc-stat__value sc-flash mono" node_ref=refs[i]>
                    {move || format::int(value.get() as i64, ui.numerals.get())}
                </span>
            </div>
        }
    };
    let tag = format!("t{tenant}");
    view! {
        <div class="sc-lab">
            <div class="sc-stats">
                {tile(0, ("Entries", "عناصر"), Signal::derive(move || info.get().entries))}
                {tile(1, ("Hits", "إصابات"), Signal::derive(move || info.get().hits))}
                {tile(2, ("Misses", "إخفاقات"), Signal::derive(move || info.get().misses))}
                {tile(3, ("Generation", "الجيل"), Signal::derive(move || info.get().generation))}
            </div>
            <div class="sc-actions">
                <Button kind=ButtonKind::Primary size=ButtonSize::Sm icon="zap" loading=Signal::derive(move || read.pending().get())
                    disabled=busy on:click=move |_| { read.dispatch(()); }>
                    {move || tr(ui.lang.get(), "Read the Featured rail", "اقرا قسم «اخترنالك»")}
                </Button>
                <Button kind=ButtonKind::Secondary size=ButtonSize::Sm icon="rotate-ccw" loading=Signal::derive(move || bump.pending().get())
                    disabled=busy on:click=move |_| { bump.dispatch(api::CacheBump {}); }>
                    {move || tr(ui.lang.get(), "Bump generation", "زوّد الجيل")}
                </Button>
                <Button kind=ButtonKind::Ghost size=ButtonSize::Sm icon="info" disabled=busy
                    on:click=move |_| { refresh.dispatch(api::CacheStats {}); }>
                    {move || tr(ui.lang.get(), "Refresh", "حدّث")}
                </Button>
            </div>
            <p class="sc-lab__note" role="status" aria-live="polite">
                {move || {
                    let l = ui.lang.get();
                    let n = ui.numerals.get();
                    match note.get() {
                        None => tr(l, "Bump, then read twice: the first read after a bump is a miss (computed, then cached), the second a hit.", "زوّد الجيل، واقرا مرتين: أول قراءة بعد الزيادة بتبقى miss (بتتحسب وتتخزن)، والتانية hit.").to_owned(),
                        Some(LabNote::Read { ms: t, missed, ok: true }) => match (l, missed) {
                            (Lang::En, true) => format!("Miss — computed and cached in {}.", ms(t, n)),
                            (Lang::En, false) => format!("Hit — served from the cache in {}.", ms(t, n)),
                            (Lang::Ar, true) => format!("Miss — اتحسبت واتخزنت في {}.", ms(t, n)),
                            (Lang::Ar, false) => format!("Hit — جت من الكاش في {}.", ms(t, n)),
                        },
                        Some(LabNote::Read { ok: false, .. }) => tr(l, "The read failed (a simulated fault is on — see Simulated latency and faults).", "القراءة فشلت (فيه عطل تجريبي شغال — شوف «بطء وأعطال تجريبية»).").to_owned(),
                        Some(LabNote::Bumped(g)) => match l {
                            Lang::En => format!("Generation {} — every cached read of this shop is now unreachable. In production this is also a CDN purge of Cache-Tag {tag}.", format::int(g as i64, n)),
                            Lang::Ar => format!("الجيل {} — كل القراءات المتخزنة للمتجر ده بقت مش بتتلقي. وفي الإنتاج ده بيبقى معاه مسح للـ CDN بالتاج {tag}.", format::int(g as i64, n)),
                        },
                        Some(LabNote::Failed) => tr(l, "That request failed.", "الطلب ده فشل.").to_owned(),
                    }
                }}
            </p>
        </div>
    }
}

// ─── <Transition>: paging that keeps the old page on screen ────────────────────────────────────

/// Pages through every product, 24 at a time, with an `AsyncDerived` + `<Transition>`: the current
/// page stays (dimmed) until the next one has arrived.
///
/// Page 1 comes from the server as props and seeds the derived value, so the first poll is ready
/// on both sides: the server renders the results and hydration finds exactly that — no fallback,
/// no fetch, no hydration data. (A `Resource` or `LocalResource` here would leave the Transition
/// pending, and its hydration would depend on inline scripts and the page-wide hydration-id
/// sequence, which the islands router doesn't carry over a client navigation — see "Sharp edges".)
#[island]
pub fn ListingPager(
    pages: u32,
    first: Vec<crate::components::product_card::CardData>,
) -> impl IntoView {
    let page = RwSignal::new(1u32);
    let pending = RwSignal::new(false);
    // Pages seen this visit; page 1 from the server.
    let seen = StoredValue::new(vec![(1u32, first)]);
    let data = AsyncDerived::new(move || {
        let p = page.get();
        let known = seen.with_value(|s| s.iter().find(|(n, _)| *n == p).map(|(_, v)| v.clone()));
        async move {
            match known {
                Some(items) => Ok(items),
                None => {
                    let query = crate::api::types::ListingQuery {
                        page: p,
                        sort: crate::api::types::Sort::Newest,
                        ..Default::default()
                    };
                    let items = api::list_products(query).await.map(|lp| lp.items);
                    if let Ok(v) = &items {
                        seen.update_value(|s| s.push((p, v.clone())));
                    }
                    items
                }
            }
        }
    });
    view! {
        <div class="sc-pager" class:is-pending=move || pending.get() aria-busy=move || pending.get().to_string()>
            <PagerBar page pages/>
            // After the first page, a load keeps the old page on screen; `set_pending` dims it.
            <Transition fallback=|| view! { <ThumbsSkeleton/> } set_pending=pending.write_only()>
                <ErrorBoundary fallback=|errors| view! { <SectionError errors compact=true/> }>
                    {move || Suspend::new(async move {
                        data.await.map(|items| view! { <Thumbs items/> })
                    })}
                </ErrorBoundary>
            </Transition>
        </div>
    }
}

/// Previous / "Page n of m" / next, and the "loading" note the pending state reveals.
#[component]
fn PagerBar(page: RwSignal<u32>, pages: u32) -> impl IntoView {
    let ui = use_ui();
    view! {
        <div class="sc-pager__bar">
            <button class="sc-ibtn" type="button" disabled=move || page.get() <= 1
                on:click=move |_| page.update(|p| *p = p.saturating_sub(1).max(1))
                aria-label=move || tr(ui.lang.get(), "Previous page", "الصفحة اللي قبل")>
                <Icon name="chevron-left" size=18 class="flip-rtl"/>
            </button>
            <span class="sc-pager__label mono" aria-live="polite">
                {move || {
                    let n = ui.numerals.get();
                    match ui.lang.get() {
                        Lang::En => format!("Page {} of {}", format::count(page.get(), n), format::count(pages, n)),
                        Lang::Ar => format!("صفحة {} من {}", format::count(page.get(), n), format::count(pages, n)),
                    }
                }}
            </span>
            <button class="sc-ibtn" type="button" disabled=move || { page.get() >= pages }
                on:click=move |_| page.update(|p| *p += 1)
                aria-label=move || tr(ui.lang.get(), "Next page", "الصفحة اللي بعد")>
                <Icon name="chevron-right" size=18 class="flip-rtl"/>
            </button>
            <span class="sc-pager__state">
                <Icon name="loader-circle" size=16 class="spin"/>
                <span>{move || tr(ui.lang.get(), "loading — old page stays", "بيحمّل — القديمة فاضلة")}</span>
            </span>
        </div>
    }
}

/// 24 packshots, each a link to its product.
#[component]
fn Thumbs(items: Vec<crate::components::product_card::CardData>) -> impl IntoView {
    let ui = use_ui();
    view! {
        <ul class="sc-thumbs">
            {items.into_iter().map(|c| {
                let name = StoredValue::new(c.name.clone());
                let label = move || name.with_value(|n| n.pick(ui.lang.get().is_rtl()).to_owned());
                let img = c.img.clone();
                view! {
                    <li>
                        <a class="sc-thumb" href=format!("/p/{}", c.slug) title=label>
                            {(!img.is_missing()).then(|| view! {
                                <img src=img.src_near(240) srcset=img.srcset() sizes="80px" alt=label
                                    width="240" height="240" loading="lazy" decoding="async"/>
                            })}
                        </a>
                    </li>
                }
            }).collect_view()}
        </ul>
    }
}

#[component]
fn ThumbsSkeleton() -> impl IntoView {
    view! {
        <ul class="sc-thumbs" aria-hidden="true">
            {(0..24).map(|_| view! { <li><span class="sc-thumb skeleton"></span></li> }).collect_view()}
        </ul>
    }
}

// ─── ActionForm + ServerAction: a note board ───────────────────────────────────────────────────

fn note_error(code: &str, lang: Lang) -> &'static str {
    match code {
        notes::ERR_RATING => tr(
            lang,
            "Pick a rating from 1 to 5 first.",
            "اختر تقييمًا من ١ إلى ٥ أولًا.",
        ),
        _ => tr(
            lang,
            "That didn't go through. Please try again.",
            "لم يتم الإرسال. يُرجى المحاولة مرة أخرى.",
        ),
    }
}

fn error_code(e: &ServerFnError) -> String {
    match e {
        ServerFnError::ServerError(m) => m.clone(),
        other => other.to_string(),
    }
}

/// `<ActionForm>` around a real `<form method=post action=/api/leave_note…>`. With JS the
/// island dispatches the `ServerAction` and swaps in the returned board; without JS the browser
/// posts it and the server redirects back (an error comes back as `?__err=`, passed in `error`).
#[island]
pub fn NoteBoard(board: Board, error: Option<String>) -> impl IntoView {
    let ui = use_ui();
    let action = ServerAction::<notes::LeaveNote>::new();
    let board = RwSignal::new(board);
    let err = RwSignal::new(error);
    let form = NodeRef::<leptos::html::Form>::new();
    Effect::new(move |_| match action.value().get() {
        Some(Ok(b)) => {
            board.set(b);
            err.set(None);
            if let Some(f) = form.get_untracked() {
                f.reset();
            }
        }
        Some(Err(e)) => err.set(Some(error_code(&e))),
        None => {}
    });
    let pending = action.pending();
    view! {
        <div class="sc-notes">
            <ActionForm action node_ref=form>
                <fieldset class="sc-stars">
                    <legend class="sc-field__label">{move || tr(ui.lang.get(), "Rate this showcase", "قيّم الصفحة دي")}</legend>
                    <div class="sc-stars__row">
                        {(1..=5u8).map(|n| {
                            let id = format!("sc-rate-{n}");
                            view! {
                                <input class="sc-stars__input sr-only" type="radio" name="rating" value=n id=id.clone()/>
                                <label class="sc-stars__star" for=id>
                                    <Icon name="star" size=22/>
                                    <span class="sr-only">{move || match ui.lang.get() {
                                        Lang::En => format!("{n} of 5"),
                                        Lang::Ar => format!("{} من {}", format::count(n as u32, ui.numerals.get()), format::count(5, ui.numerals.get())),
                                    }}</span>
                                </label>
                            }
                        }).collect_view()}
                    </div>
                </fieldset>
                <label class="sc-field" for="sc-note">
                    <span class="sc-field__label">{move || tr(ui.lang.get(), "A note (optional)", "ملاحظة (اختياري)")}</span>
                </label>
                <div class="sc-notes__send">
                    <input id="sc-note" class="sc-input" name="note" maxlength=notes::MAX_NOTE autocomplete="off" dir="auto"
                        placeholder=move || tr(ui.lang.get(), "What would you build with it?", "هتبني بيه إيه؟")/>
                    <Button kind=ButtonKind::Primary size=ButtonSize::Md button_type="submit" loading=pending>
                        {move || tr(ui.lang.get(), "Send", "ابعت")}
                    </Button>
                </div>
            </ActionForm>
            {move || err.get().map(|c| view! {
                <p class="sc-form-err" role="alert"><Icon name="circle-alert" size=16/>{note_error(&c, ui.lang.get())}</p>
            })}
            <div class="sc-board" aria-live="polite">
                <p class="sc-board__sum">
                    {move || {
                        let b = board.get();
                        let n = ui.numerals.get();
                        let l = ui.lang.get();
                        if b.count == 0 {
                            tr(l, "No ratings yet on this server — be the first.", "لسه مفيش تقييمات على السيرفر ده — خليك أول واحد.").to_owned()
                        } else {
                            let avg = format::dec1(b.avg_x10 as f32 / 10.0, n);
                            match l {
                                Lang::En => format!("★ {avg} from {} {}", format::count(b.count, n), format::plural_en(b.count, "rating", "ratings")),
                                Lang::Ar => format!("★ {avg} من {} تقييم", format::count(b.count, n)),
                            }
                        }
                    }}
                </p>
                <ul class="sc-board__list">
                    {move || board.get().recent.into_iter().map(|note| {
                        let stars = "★".repeat(note.rating as usize);
                        let text = note.text.clone();
                        view! {
                            <li>
                                <span class="sc-board__stars" aria-hidden="true">{stars}</span>
                                <span class="sr-only">{move || format::count(note.rating as u32, ui.numerals.get())}" / 5"</span>
                                {(!text.is_empty()).then(|| view! { <q dir="auto">{text}</q> })}
                                <time class="sc-board__time mono">{move || format::time_cairo(note.at, ui.lang.get(), ui.numerals.get())}</time>
                            </li>
                        }
                    }).collect_view()}
                </ul>
            </div>
        </div>
    }
}

// ─── leptos_meta: read the <head> back ─────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HeadTag {
    pub key: String,
    pub value: String,
}

/// Lists what is actually in `document.head` (title, description, canonical, robots,
/// OpenGraph/Twitter tags, JSON-LD). Before JS runs it shows what the server says it set.
#[island]
pub fn HeadReadout(server: Vec<HeadTag>) -> impl IntoView {
    let ui = use_ui();
    let read = RwSignal::new(None::<Vec<HeadTag>>);
    #[cfg(feature = "hydrate")]
    Effect::new(move |_| {
        let doc = document();
        let mut tags = vec![HeadTag {
            key: "<title>".into(),
            value: doc.title(),
        }];
        let sel = "meta[name='description'], meta[name='robots'], link[rel='canonical'], meta[property^='og:'], meta[name^='twitter:'], script[type='application/ld+json']";
        if let Ok(list) = doc.query_selector_all(sel) {
            for i in 0..list.length() {
                let Some(el) = list
                    .get(i)
                    .and_then(|n| n.dyn_into::<web_sys::Element>().ok())
                else {
                    continue;
                };
                let a = |n: &str| el.get_attribute(n).unwrap_or_default();
                let (key, value) = match el.tag_name().to_ascii_lowercase().as_str() {
                    "link" => (format!("link rel={}", a("rel")), a("href")),
                    "script" => {
                        let text = el.text_content().unwrap_or_default();
                        let ty = text
                            .split("\"@type\":\"")
                            .nth(1)
                            .and_then(|s| s.split('"').next())
                            .unwrap_or("?")
                            .to_owned();
                        (
                            "script ld+json".to_owned(),
                            format!("@type {ty} · {} B", text.len()),
                        )
                    }
                    _ => {
                        let k = if el.has_attribute("property") {
                            a("property")
                        } else {
                            a("name")
                        };
                        (format!("meta {k}"), a("content"))
                    }
                };
                tags.push(HeadTag { key, value });
            }
        }
        read.set(Some(tags));
    });
    view! {
        <p class="sc-note">
            {move || match (read.with(|r| r.is_some()), ui.lang.get()) {
                (true, Lang::En) => "Read back from document.head in your browser:",
                (true, Lang::Ar) => "مقروءة من document.head في المتصفح عندك:",
                (false, Lang::En) => "What the server set (JavaScript reads the live <head> back here):",
                (false, Lang::Ar) => "اللي السيرفر حطه (الـ JavaScript بيقرا الـ <head> الحقيقي هنا):",
            }}
        </p>
        <dl class="sc-head">
            {move || read.get().unwrap_or_else(|| server.clone()).into_iter().map(|t| view! {
                <div class="sc-head__row">
                    <dt class="mono" dir="ltr">{t.key}</dt>
                    <dd dir="auto">{t.value}</dd>
                </div>
            }).collect_view()}
        </dl>
    }
}

// ─── Caching hooks: response headers, live ─────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Probe {
    pub en: String,
    pub ar: String,
    pub url: String,
    pub method: String,
}

#[cfg(feature = "hydrate")]
const PROBE_HEADERS: [&str; 5] = [
    "cache-control",
    "cdn-cache-control",
    "vary",
    "cache-tag",
    "x-read-cache",
];

/// `fetch(url, {method: 'HEAD'})` a few routes (and POST one server function) and show the
/// caching headers each one actually sends.
#[island]
pub fn HeaderProbe(targets: Vec<Probe>) -> impl IntoView {
    let ui = use_ui();
    let n = targets.len();
    let results = RwSignal::new(vec![
        None::<Result<(u16, Vec<Option<String>>, f64), String>>;
        n
    ]);
    let running = RwSignal::new(false);
    let targets = StoredValue::new(targets);
    let run = move || {
        if running.get_untracked() {
            return;
        }
        running.set(true);
        results.set(vec![None; n]);
        #[cfg(feature = "hydrate")]
        leptos::task::spawn_local(async move {
            let list = targets.get_value();
            for (i, t) in list.iter().enumerate() {
                let r = js::head(&t.url, &t.method, &PROBE_HEADERS)
                    .await
                    .map(|h| (h.status, h.values, h.ms));
                results.update(|v| v[i] = Some(r));
            }
            running.set(false);
        });
    };
    #[cfg(feature = "hydrate")]
    Effect::new(move |_| set_timeout(run, std::time::Duration::from_millis(1200)));
    view! {
        <div class="sc-probes">
            {move || {
                let lang = ui.lang.get();
                let num = ui.numerals.get();
                let res = results.get();
                targets.get_value().into_iter().enumerate().map(|(i, t)| {
                    let r = res.get(i).cloned().flatten();
                    let label = if lang.is_rtl() { t.ar.clone() } else { t.en.clone() };
                    let (status, vals, t_ms) = match &r {
                        Some(Ok((s, v, m))) => (format::count(*s as u32, num), v.clone(), Some(*m)),
                        Some(Err(_)) => (tr(lang, "failed", "فشل").to_owned(), vec![None; 5], None),
                        None => ("…".to_owned(), vec![None; 5], None),
                    };
                    let val = |i: usize| vals.get(i).cloned().flatten().unwrap_or_else(|| "—".into());
                    let public = vals.get(1).cloned().flatten().is_some_and(|v| v.starts_with("public"));
                    view! {
                        <div class="sc-probe-row" class:is-public=public>
                            <div class="sc-probe-row__head">
                                <strong>{label}</strong>
                                <code class="mono" dir="ltr">{format!("{} {}", t.method, t.url)}</code>
                                <span class="sc-probe-row__status mono">{status}{t_ms.map(|m| format!(" · {}", ms(m, num)))}</span>
                            </div>
                            <dl class="sc-probe-row__kv" dir="ltr">
                                <div><dt>"Cache-Control"</dt><dd class="mono">{val(0)}</dd></div>
                                <div><dt>"CDN-Cache-Control"</dt><dd class="mono">{val(1)}</dd></div>
                                <div><dt>"Vary"</dt><dd class="mono">{val(2)}</dd></div>
                                <div><dt>"Cache-Tag"</dt><dd class="mono">{val(3)}</dd></div>
                                <div><dt>"x-read-cache"</dt><dd class="mono">{val(4)}</dd></div>
                            </dl>
                        </div>
                    }
                }).collect_view()
            }}
            <div class="sc-actions">
                <Button kind=ButtonKind::Secondary size=ButtonSize::Sm icon="rotate-ccw" loading=Signal::derive(move || running.get())
                    on:click=move |_| run()>
                    {move || tr(ui.lang.get(), "Run again", "شغّل تاني")}
                </Button>
            </div>
        </div>
    }
}

// ─── SSR modes: a stream inspector ─────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StreamTarget {
    pub en: String,
    pub ar: String,
    pub url: String,
    /// Send `Islands-Router: true`, like a client-side navigation.
    pub router: bool,
}

#[cfg(feature = "hydrate")]
type StreamResult = Result<js::Stream, String>;

#[derive(Clone, Debug, Default, PartialEq)]
struct Shot {
    status: u16,
    /// (ms, bytes, templates)
    chunks: Vec<(f64, f64, u32)>,
    total_ms: f64,
    bytes: f64,
    markers: u32,
    marker_bytes: f64,
    templates: u32,
    error: Option<String>,
}

#[cfg(feature = "hydrate")]
fn shot(r: StreamResult) -> Shot {
    match r {
        Ok(s) => Shot {
            status: s.status,
            chunks: s
                .chunks
                .iter()
                .map(|c| (c.at, c.bytes, c.templates))
                .collect(),
            total_ms: s.total_ms,
            bytes: s.bytes,
            markers: s.markers,
            marker_bytes: s.marker_bytes,
            templates: s.templates,
            error: None,
        },
        Err(e) => Shot {
            error: Some(e),
            ..Default::default()
        },
    }
}

/// Fetches the same page in each streaming mode and draws when every piece of HTML arrived.
#[island]
pub fn StreamInspector(targets: Vec<StreamTarget>) -> impl IntoView {
    let ui = use_ui();
    let n = targets.len();
    let shots = RwSignal::new(vec![None::<Shot>; n]);
    let running = RwSignal::new(false);
    let targets = StoredValue::new(targets);
    let run = move |_| {
        if running.get_untracked() {
            return;
        }
        running.set(true);
        shots.set(vec![None; n]);
        #[cfg(feature = "hydrate")]
        leptos::task::spawn_local(async move {
            for (i, t) in targets.get_value().iter().enumerate() {
                let headers: Vec<(&str, &str)> = if t.router {
                    vec![("Islands-Router", "true")]
                } else {
                    vec![]
                };
                let r = js::stream(&t.url, &headers).await;
                shots.update(|v| v[i] = Some(shot(r)));
            }
            running.set(false);
        });
    };
    let max_ms = move || shots.with(|v| v.iter().flatten().map(|s| s.total_ms).fold(1.0, f64::max));
    view! {
        <div class="sc-streams">
            <div class="sc-actions">
                <Button kind=ButtonKind::Primary size=ButtonSize::Sm icon="timer" loading=Signal::derive(move || running.get()) on:click=run>
                    {move || tr(ui.lang.get(), "Fetch all three and time every chunk", "هات التلاتة واحسب وقت كل chunk")}
                </Button>
            </div>
            {move || {
                let lang = ui.lang.get();
                let num = ui.numerals.get();
                let all = shots.get();
                let scale = max_ms();
                targets.get_value().into_iter().enumerate().map(|(i, t)| {
                    let label = if lang.is_rtl() { t.ar.clone() } else { t.en.clone() };
                    let s = all.get(i).cloned().flatten();
                    let ticks = s.as_ref().map(|s| s.chunks.iter().map(|(at, _, tpl)| {
                        let x = (at / scale * 100.0).clamp(0.0, 100.0);
                        view! { <i class="sc-tl__tick" class:is-ooo={*tpl > 0} style=format!("--x:{x:.2}%")></i> }
                    }).collect_view());
                    let end = s.as_ref().map(|s| (s.total_ms / scale * 100.0).clamp(0.0, 100.0)).unwrap_or(0.0);
                    let stats = s.as_ref().map(|s| match &s.error {
                        Some(e) => e.clone(),
                        None => {
                            let first = s.chunks.first().map(|c| c.0).unwrap_or(0.0);
                            match lang {
                                Lang::En => format!(
                                    "first bytes {} · done {} · {} reads · {} out-of-order chunks · {} HTML, {} of it in {} branch markers",
                                    ms(first, num), ms(s.total_ms, num), format::count(s.chunks.len() as u32, num),
                                    format::count(s.templates, num), size(s.bytes, num), size(s.marker_bytes, num), format::int(s.markers as i64, num)
                                ),
                                Lang::Ar => format!(
                                    "أول بايتات {} · خلص {} · {} قراءة · {} chunk بأي ترتيب · {} HTML، منها {} في {} branch markers",
                                    ms(first, num), ms(s.total_ms, num), format::count(s.chunks.len() as u32, num),
                                    format::count(s.templates, num), size(s.bytes, num), size(s.marker_bytes, num), format::int(s.markers as i64, num)
                                ),
                            }
                        }
                    });
                    view! {
                        <div class="sc-stream">
                            <div class="sc-stream__head">
                                <strong>{label}</strong>
                                <code class="mono" dir="ltr">{format!("GET {}{}", t.url, if t.router { "  (Islands-Router: true)" } else { "" })}</code>
                            </div>
                            <div class="sc-tl" dir="ltr" aria-hidden="true">
                                <span class="sc-tl__bar" style=format!("--w:{end:.2}%")></span>
                                {ticks}
                            </div>
                            <p class="sc-stream__stats mono">{stats.unwrap_or_else(|| "—".into())}</p>
                        </div>
                    }
                }).collect_view()
            }}
        </div>
    }
}

// ─── i18n: an island that follows the Ui store ─────────────────────────────────────────────────

/// Its text is rendered by the island itself, so only the `Ui` store (fed by the `<html>`
/// MutationObserver in `client::init`) can change its language — and it does, live.
#[island]
pub fn LiveLang(amount: i64, items: u32, at_ms: i64) -> impl IntoView {
    let ui = use_ui();
    let renders = RwSignal::new(0u32);
    Effect::new(move |prev: Option<()>| {
        ui.lang.track();
        ui.numerals.track();
        if prev.is_some() {
            renders.update(|n| *n += 1);
        }
    });
    view! {
        <div class="sc-receipt">
            <p class="sc-receipt__title">{move || tr(ui.lang.get(), "Your order is on its way", "طلبك في الطريق")}</p>
            <dl class="sc-receipt__rows">
                <div>
                    <dt>{move || tr(ui.lang.get(), "Items", "المنتجات")}</dt>
                    <dd>{move || format::items(items, ui.lang.get(), ui.numerals.get())}</dd>
                </div>
                <div>
                    <dt>{move || tr(ui.lang.get(), "Total", "الإجمالي")}</dt>
                    <dd class="mono">{move || money(amount, ui.lang.get(), ui.numerals.get())}</dd>
                </div>
                <div>
                    <dt>{move || tr(ui.lang.get(), "Arrives", "يوصل")}</dt>
                    <dd>{move || format!("{} · {}", format::date_cairo(at_ms, ui.lang.get(), ui.numerals.get()), format::time_cairo(at_ms, ui.lang.get(), ui.numerals.get()))}</dd>
                </div>
            </dl>
            <p class="sc-receipt__store mono" dir="ltr">
                {move || format!("Ui {{ lang: {}, numerals: {}, theme: {} }}", ui.lang.get().code(), ui.numerals.get().code(), ui.theme.get().code())}
            </p>
            <p class="sc-note">
                {move || {
                    let n = format::count(renders.get(), ui.numerals.get());
                    match ui.lang.get() {
                        Lang::En => format!("This island re-rendered its text {n}× — no reload, no request of its own."),
                        Lang::Ar => format!("الـ island دي أعادت رسم نصها {n} مرة — من غير ريلود ومن غير ما تطلب حاجة."),
                    }
                }}
            </p>
        </div>
    }
}

// ─── View transitions: support check ───────────────────────────────────────────────────────────

#[island]
pub fn VtSupport() -> impl IntoView {
    let ui = use_ui();
    let state = RwSignal::new(None::<(bool, bool)>);
    #[cfg(feature = "hydrate")]
    Effect::new(move |_| {
        state.set(Some((
            js::has_view_transitions(),
            js::matches_media("(prefers-reduced-motion: reduce)"),
        )))
    });
    view! {
        <p class="sc-support" class:is-yes=move || state.get().is_some_and(|s| s.0)>
            <Icon name="eye" size=16/>
            <span>
                {move || {
                    let l = ui.lang.get();
                    match state.get() {
                        None => tr(l, "Your browser's support is checked when JavaScript runs.", "دعم المتصفح بيتشيّك لما الـ JavaScript يشتغل.").to_owned(),
                        Some((true, false)) => tr(l, "This browser supports view transitions.", "المتصفح ده بيدعم الـ view transitions.").to_owned(),
                        Some((true, true)) => tr(l, "Supported — but you asked for reduced motion, so the swap is instant.", "مدعوم — بس انت طالب حركة أقل، فالتبديل بيحصل على طول.").to_owned(),
                        Some((false, _)) => tr(l, "Not supported here: the router simply swaps the page without animation.", "مش مدعوم هنا: الراوتر بيبدّل الصفحة من غير حركة وخلاص.").to_owned(),
                    }
                }}
            </span>
        </p>
    }
}

// ─── Responsive images: which file did the browser pick? ───────────────────────────────────────

#[derive(Clone, Debug, Default, PartialEq)]
struct Picked {
    src: String,
    box_w: f64,
    dpr: f64,
    natural: u32,
}

/// Renders one image with `srcset` + `sizes` and reports `currentSrc`: the rung the browser chose
/// for this box width and screen density. Resize the window to watch it change (browsers keep a
/// bigger file they already have rather than download a smaller one).
#[island]
pub fn CurrentSrc(img: Img, sizes: String) -> impl IntoView {
    let ui = use_ui();
    let node = NodeRef::<leptos::html::Img>::new();
    let picked = RwSignal::new(None::<Picked>);
    let measure = move || {
        #[cfg(feature = "hydrate")]
        if let Some(el) = node.get_untracked() {
            if el.complete() && el.natural_width() > 0 {
                picked.set(Some(Picked {
                    src: el.current_src(),
                    box_w: el.client_width() as f64,
                    dpr: window().device_pixel_ratio(),
                    natural: el.natural_width(),
                }));
            }
        }
    };
    #[cfg(feature = "hydrate")]
    Effect::new(move |_| {
        set_timeout(measure, std::time::Duration::from_millis(100));
        let handle = window_event_listener(leptos::ev::resize, move |_| measure());
        on_cleanup(move || handle.remove());
    });
    let alt_img = img.alt.clone();
    let height = (640.0 / img.aspect.max(0.1)).round() as u32;
    let style = format!(
        "background-color:{};background-image:url({})",
        img.color, img.lqip
    );
    view! {
        <figure class="sc-img">
            <div class="sc-img__frame" style=style>
                <img
                    node_ref=node
                    src=img.src_near(640)
                    srcset=img.srcset()
                    sizes=sizes
                    alt=move || alt_img.pick(ui.lang.get().is_rtl()).to_owned()
                    width="640"
                    height=height
                    loading="lazy"
                    decoding="async"
                    on:load=move |_| measure()
                />
            </div>
            <figcaption class="sc-img__picked" aria-live="polite">
                {move || {
                    let l = ui.lang.get();
                    let n = ui.numerals.get();
                    match picked.get() {
                        None => tr(l, "currentSrc appears here once the image has loaded (needs JavaScript).", "الـ currentSrc بيظهر هنا لما الصورة تحمّل (محتاج JavaScript).").to_owned(),
                        Some(p) => {
                            let file = p.src.rsplit('/').next().unwrap_or_default().to_owned();
                            let need = (p.box_w * p.dpr).round() as i64;
                            match l {
                                Lang::En => format!("currentSrc: {file} ({}w) — the box is {} px wide at {}× density, so it needs ≥ {} px.",
                                    format::int(p.natural as i64, n), format::int(p.box_w.round() as i64, n), format::dec1(p.dpr as f32, n), format::int(need, n)),
                                Lang::Ar => format!("الـ currentSrc: ‏{file} ({}w) — المساحة {} بكسل وكثافة الشاشة {}×، فمحتاجة ≥ {} بكسل.",
                                    format::int(p.natural as i64, n), format::int(p.box_w.round() as i64, n), format::dec1(p.dpr as f32, n), format::int(need, n)),
                            }
                        }
                    }
                }}
            </figcaption>
        </figure>
    }
}

// ─── Accessibility: a modal sheet with real focus management ───────────────────────────────────

/// A native `<dialog>` opened with `showModal()`: focus moves in, the page behind is inert (Tab
/// can't leave), Esc closes it, focus returns to the button. The open button also carries
/// `command="show-modal"` so browsers with invoker commands open it before WASM has loaded.
#[island]
pub fn SheetDemo() -> impl IntoView {
    let ui = use_ui();
    let dialog = NodeRef::<leptos::html::Dialog>::new();
    let status = RwSignal::new(String::new());
    let open = move |ev: leptos::ev::MouseEvent| {
        // With JS the island opens the sheet; the `command` attribute is the no-JS path.
        ev.prevent_default();
        #[cfg(feature = "hydrate")]
        if let Some(d) = dialog.get_untracked() {
            if !d.open() {
                let _ = d.show_modal();
            }
            status.set(tr(ui.lang.get_untracked(), "Sheet open.", "الشاشة مفتوحة.").to_owned());
        }
    };
    let closed = move |_| {
        let picked = {
            #[cfg(feature = "hydrate")]
            {
                document()
                    .query_selector("input[name='sc-slot']:checked")
                    .ok()
                    .flatten()
                    .and_then(|el| el.get_attribute("data-label"))
            }
            #[cfg(not(feature = "hydrate"))]
            {
                None::<String>
            }
        };
        let l = ui.lang.get_untracked();
        status.set(match (picked, l) {
            (Some(p), Lang::En) => format!("Delivery slot set: {p}. Focus is back on the button."),
            (Some(p), Lang::Ar) => format!("اتحدد ميعاد التوصيل: {p}. والفوكس رجع للزرار."),
            (None, _) => tr(
                l,
                "Sheet closed. Focus is back on the button.",
                "الشاشة اتقفلت. والفوكس رجع للزرار.",
            )
            .to_owned(),
        });
    };
    let slots = [
        ("Today, 6–9 pm", "النهارده، ٦–٩ م"),
        ("Tomorrow, 10 am–1 pm", "بكرة، ١٠ ص–١ م"),
        ("Tomorrow, 6–9 pm", "بكرة، ٦–٩ م"),
    ];
    view! {
        <div class="sc-sheet-demo">
            <button
                class="btn btn--secondary btn--md"
                type="button"
                aria-haspopup="dialog"
                commandfor="sc-sheet"
                command="show-modal"
                on:click=open
            >
                <Icon name="calendar-clock" size=18/>
                <span class="btn__label">{move || tr(ui.lang.get(), "Choose a delivery slot", "اختار ميعاد التوصيل")}</span>
            </button>
            <dialog id="sc-sheet" class="sc-sheet" node_ref=dialog aria-labelledby="sc-sheet-title" on:close=closed>
                <form method="dialog" class="sc-sheet__body">
                    <h3 id="sc-sheet-title" class="sc-sheet__title">{move || tr(ui.lang.get(), "Delivery slot", "ميعاد التوصيل")}</h3>
                    <p class="sc-note">{move || tr(ui.lang.get(), "Tab stays inside this sheet; Esc or Done closes it and focus goes back to the button.", "الـ Tab بيفضل جوه الشاشة دي؛ وEsc أو «تم» بيقفلوها والفوكس بيرجع للزرار.")}</p>
                    <fieldset class="sc-slots">
                        <legend class="sr-only">{move || tr(ui.lang.get(), "Delivery slot", "ميعاد التوصيل")}</legend>
                        {slots.into_iter().enumerate().map(|(i, (en, ar))| view! {
                            <label class="sc-slot">
                                <input type="radio" name="sc-slot" value=i checked=i == 0
                                    data-label=move || tr(ui.lang.get(), en, ar)/>
                                <span>{move || tr(ui.lang.get(), en, ar)}</span>
                            </label>
                        }).collect_view()}
                    </fieldset>
                    <button class="btn btn--primary btn--md btn--block" value="done">
                        <span class="btn__label">{move || tr(ui.lang.get(), "Done", "تم")}</span>
                    </button>
                </form>
            </dialog>
            <p class="sc-sheet-demo__status" role="status" aria-live="polite">{move || status.get()}</p>
        </div>
    }
}

// ─── Motion: a spring you can grab ─────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct Release {
    v: f64,
    projected: f64,
    bouncy: bool,
}

#[derive(Clone, Copy, Debug)]
struct Grab {
    pointer: i32,
    x0: f64,
    p0: f64,
}

fn stop_label(i: usize, lang: Lang) -> &'static str {
    match i {
        0 => tr(lang, "start", "البداية"),
        1 => tr(lang, "middle", "النص"),
        _ => tr(lang, "end", "النهاية"),
    }
}

/// A puck on a track with three resting places, driven by `components::motion`. Drag it (1:1,
/// from where you grabbed it), pull it past an end (it rubber-bands), let go mid-flick (the release
/// velocity carries on and the projected landing picks the resting place), or catch it mid-flight
/// (it starts from where it is on screen). Arrow keys and the buttons do the same without a pointer.
#[island]
pub fn SpringLab() -> impl IntoView {
    use crate::components::motion::{Spring, SpringValue, VelocityTracker, project, rubberband};
    let ui = use_ui();
    let rail = NodeRef::<leptos::html::Div>::new();
    let puck = NodeRef::<leptos::html::Div>::new();
    let x_now = RwSignal::new(0.0_f64);
    let snap = RwSignal::new(0usize);
    let last = RwSignal::new(None::<Release>);
    let grab = StoredValue::new(None::<Grab>);
    let tracker = StoredValue::new(VelocityTracker::default());
    // The spring draws each frame by moving the puck; the readout follows.
    let pos = SpringValue::new(0.0, move |x| {
        #[cfg(feature = "hydrate")]
        if let Some(el) = puck.get_untracked() {
            let _ = web_sys::HtmlElement::style(&el)
                .set_property("transform", &format!("translateX({x:.2}px)"));
        }
        x_now.set(x);
    });
    // How far the puck can travel, and its three resting places.
    let travel = move || -> f64 {
        #[cfg(feature = "hydrate")]
        {
            match (rail.get_untracked(), puck.get_untracked()) {
                (Some(r), Some(p)) => (r.client_width() - p.offset_width()).max(0) as f64,
                _ => 0.0,
            }
        }
        #[cfg(not(feature = "hydrate"))]
        {
            0.0
        }
    };
    let stops = move || {
        let m = travel();
        [0.0, m / 2.0, m]
    };
    let settle = move |v: f64| {
        let projected = pos.get() + project(v, 0.998);
        let s = stops();
        let i = (0..3)
            .min_by(|a, b| {
                (s[*a] - projected)
                    .abs()
                    .total_cmp(&(s[*b] - projected).abs())
            })
            .unwrap_or(0);
        // A little overshoot only when a flick carried momentum.
        let bouncy = v.abs() > 600.0;
        snap.set(i);
        last.set(Some(Release {
            v,
            projected,
            bouncy,
        }));
        pos.animate_to(
            s[i],
            if bouncy {
                Spring::BOUNCY
            } else {
                Spring::DEFAULT
            },
            Some(v),
            None,
        );
    };
    let go = move |i: usize| {
        snap.set(i);
        pos.animate_to(stops()[i], Spring::DEFAULT, None, None);
    };
    #[cfg(feature = "hydrate")]
    Effect::new(move |_| {
        // Keep the resting place when the track changes width.
        let handle = window_event_listener(leptos::ev::resize, move |_| {
            pos.set(stops()[snap.get_untracked()])
        });
        on_cleanup(move || handle.remove());
    });
    let on_down = move |ev: leptos::ev::PointerEvent| {
        if ev.button() != 0 {
            return;
        }
        ev.prevent_default();
        #[cfg(feature = "hydrate")]
        if let Some(el) = puck.get_untracked() {
            let _ = el.set_pointer_capture(ev.pointer_id());
        }
        // Catch it where it is on screen (this also stops a spring in flight).
        let p0 = pos.get();
        pos.set(p0);
        tracker.update_value(|t| {
            t.reset();
            t.push(ev.time_stamp(), p0);
        });
        grab.set_value(Some(Grab {
            pointer: ev.pointer_id(),
            x0: ev.client_x() as f64,
            p0,
        }));
    };
    let on_move = move |ev: leptos::ev::PointerEvent| {
        let Some(g) = grab.get_value().filter(|g| g.pointer == ev.pointer_id()) else {
            return;
        };
        let raw = g.p0 + (ev.client_x() as f64 - g.x0);
        tracker.update_value(|t| t.push(ev.time_stamp(), raw));
        let max = travel();
        let d = max.max(1.0);
        // Past an end, follow less and less — never a hard stop.
        let x = if raw < 0.0 {
            -rubberband(-raw, d, 0.55)
        } else if raw > max {
            max + rubberband(raw - max, d, 0.55)
        } else {
            raw
        };
        pos.set(x);
    };
    let on_up = move |ev: leptos::ev::PointerEvent| {
        if grab
            .get_value()
            .is_none_or(|g| g.pointer != ev.pointer_id())
        {
            return;
        }
        grab.set_value(None);
        settle(tracker.with_value(|t| t.velocity()));
    };
    let on_key = move |ev: leptos::ev::KeyboardEvent| {
        let i = snap.get_untracked();
        let n = match ev.key().as_str() {
            "ArrowRight" | "ArrowUp" => (i + 1).min(2),
            "ArrowLeft" | "ArrowDown" => i.saturating_sub(1),
            "Home" => 0,
            "End" => 2,
            _ => return,
        };
        ev.prevent_default();
        go(n);
    };
    view! {
        <div class="sc-spring">
            // A physical track: left is left in both languages, like a media scrubber.
            <div class="sc-spring__track" dir="ltr">
                <div class="sc-spring__rail" node_ref=rail>
                    <span class="sc-spring__stop" style="--k:0"></span>
                    <span class="sc-spring__stop" style="--k:0.5"></span>
                    <span class="sc-spring__stop" style="--k:1"></span>
                    <div
                        class="sc-spring__puck"
                        node_ref=puck
                        tabindex="0"
                        role="slider"
                        aria-label=move || tr(ui.lang.get(), "Spring puck", "كرة السبرينج")
                        aria-valuemin="0"
                        aria-valuemax="2"
                        aria-valuenow=move || snap.get().to_string()
                        aria-valuetext=move || stop_label(snap.get(), ui.lang.get())
                        on:pointerdown=on_down
                        on:pointermove=on_move
                        on:pointerup=on_up
                        on:pointercancel=on_up
                        on:keydown=on_key
                    ></div>
                </div>
            </div>
            <div class="sc-actions">
                <Button kind=ButtonKind::Secondary size=ButtonSize::Sm icon="chevron-left" on:click=move |_| settle(-2400.0)>
                    {move || tr(ui.lang.get(), "Flick left", "اقذف شمال")}
                </Button>
                <Button kind=ButtonKind::Secondary size=ButtonSize::Sm icon_end="chevron-right" on:click=move |_| settle(2400.0)>
                    {move || tr(ui.lang.get(), "Flick right", "اقذف يمين")}
                </Button>
            </div>
            <dl class="sc-probe">
                <div>
                    <dt>{move || tr(ui.lang.get(), "Position now", "المكان دلوقتي")}</dt>
                    <dd class="mono">{move || format!("{} px", format::int(x_now.get().round() as i64, ui.numerals.get()))}</dd>
                </div>
                <div>
                    <dt>{move || tr(ui.lang.get(), "Release velocity", "السرعة لحظة الترك")}</dt>
                    <dd class="mono">{move || last.get().map(|r| format!("{} px/s", format::int(r.v.round() as i64, ui.numerals.get()))).unwrap_or_else(|| "—".into())}</dd>
                </div>
                <div>
                    <dt>{move || tr(ui.lang.get(), "Projected landing → rest", "المتوقع → مكان الوقوف")}</dt>
                    <dd class="mono">{move || last.get().map(|r| {
                        let n = ui.numerals.get();
                        format!("{} px → {}", format::int(r.projected.round() as i64, n), stop_label(snap.get(), ui.lang.get()))
                    }).unwrap_or_else(|| "—".into())}</dd>
                </div>
                <div>
                    <dt>{move || tr(ui.lang.get(), "Spring", "السبرينج")}</dt>
                    <dd class="mono">{move || match last.get() {
                        Some(Release { bouncy: true, .. }) => "BOUNCY · damping 0.8 · response 0.3 s".to_owned(),
                        Some(_) => "DEFAULT · damping 1.0 · response 0.35 s".to_owned(),
                        None => "—".to_owned(),
                    }}</dd>
                </div>
            </dl>
        </div>
    }
}

// ─── islands-router: are the patches live on this page? ────────────────────────────────────────

/// Checks, in the visitor's browser, that the shell's router patches are in place, and asks the
/// server for the home page both ways to show the <noscript> refresh is left out of router
/// responses.
#[island]
pub fn PatchProbe() -> impl IntoView {
    let ui = use_ui();
    // [head realignment, scroll after swap, inert noscript, router response without refresh, first load with it]
    let checks = RwSignal::new([None::<bool>; 5]);
    #[cfg(feature = "hydrate")]
    Effect::new(move |_| {
        let fetch_wrapped = !js::is_native(&js::get(&js::global(), "fetch"));
        let doc: wasm_bindgen::JsValue = document().into();
        let vt = js::get(&doc, "startViewTransition");
        let manual = js::get(&js::get(&js::global(), "history"), "scrollRestoration")
            .as_string()
            .as_deref()
            == Some("manual");
        let scroll = manual && (vt.is_undefined() || !js::is_native(&vt));
        let noscript_inert = document()
            .query_selector_all("noscript")
            .map(|l| {
                (0..l.length()).all(|i| {
                    l.get(i)
                        .and_then(|n| n.text_content())
                        .unwrap_or_default()
                        .trim()
                        .is_empty()
                })
            })
            .unwrap_or(false);
        checks.update(|c| {
            c[0] = Some(fetch_wrapped);
            c[1] = Some(scroll);
            c[2] = Some(noscript_inert);
        });
        leptos::task::spawn_local(async move {
            let routed = js::text("/", &[("Islands-Router", "true")]).await.ok();
            let first = js::text("/", &[]).await.ok();
            checks.update(|c| {
                c[3] = routed.map(|h| js::count_in_html(&h, "noscript") == 0);
                c[4] = first.map(|h| js::count_in_html(&h, "noscript") > 0);
            });
        });
    });
    let rows: [(&str, &str); 5] = [
        (
            "<head> is realigned before each diff (fetch is wrapped)",
            "الـ <head> بيتظبط قبل كل diff (الـ fetch متلفوف)",
        ),
        (
            "Scroll is handled after each swap (scrollRestoration = manual)",
            "الـ scroll بيتظبط بعد كل تبديل (‏scrollRestoration = manual)",
        ),
        (
            "<noscript> in this page is inert",
            "الـ <noscript> في الصفحة دي مالوش أثر",
        ),
        (
            "Router responses for / carry no <noscript> at all",
            "ردود الراوتر لـ ‎/‎ مافيهاش <noscript> خالص",
        ),
        (
            "…while a first load of / keeps its no-JS fallback (a link to /nojs)",
            "…وأول تحميل لـ ‎/‎ لسه فيه البديل بتاع الـ no-JS (لينك لـ ‎/nojs‎)",
        ),
    ];
    view! {
        <ul class="sc-checks">
            {rows.into_iter().enumerate().map(|(i, (en, ar))| view! {
                <li class="sc-check-row" data-ok=move || match checks.get()[i] { Some(true) => "yes", Some(false) => "no", None => "wait" }>
                    <span class="sc-check-row__mark" aria-hidden="true">{move || match checks.get()[i] { Some(true) => "✓", Some(false) => "✗", None => "…" }}</span>
                    <span>{move || tr(ui.lang.get(), en, ar)}</span>
                    <span class="sr-only">{move || match checks.get()[i] {
                        Some(true) => tr(ui.lang.get(), "yes", "أيوه"),
                        Some(false) => tr(ui.lang.get(), "no", "لأ"),
                        None => tr(ui.lang.get(), "checking", "بيتشيّك"),
                    }}</span>
                </li>
            }).collect_view()}
        </ul>
    }
}

// ─── States: offline ───────────────────────────────────────────────────────────────────────────

/// The connection as the browser reports it (`navigator.onLine`), and the shell's real offline
/// banner (`components::offline::OfflineBanner`, mounted once on every page). The buttons fire the
/// same `offline`/`online` window events the browser fires, so the banner reacts exactly as it
/// would; the status line keeps reading the browser's own answer.
#[island]
pub fn OfflineDemo() -> impl IntoView {
    let ui = use_ui();
    let online = RwSignal::new(true);
    #[cfg(feature = "hydrate")]
    Effect::new(move |_| {
        let sync = move || online.set(window().navigator().on_line());
        sync();
        let up = window_event_listener(leptos::ev::online, move |_| sync());
        let down = window_event_listener(leptos::ev::offline, move |_| sync());
        on_cleanup(move || {
            up.remove();
            down.remove();
        });
    });
    let fire = move |kind: &'static str| {
        #[cfg(feature = "hydrate")]
        if let Ok(ev) = web_sys::Event::new(kind) {
            let _ = window().dispatch_event(&ev);
        }
        #[cfg(not(feature = "hydrate"))]
        let _ = kind;
    };
    view! {
        <p class="sc-online" data-online=move || if online.get() { "yes" } else { "no" } role="status">
            <span class="sc-online__dot" aria-hidden="true"></span>
            <span>{move || if online.get() {
                tr(ui.lang.get(), "Your browser says: online.", "المتصفح بيقول: متصل.")
            } else {
                tr(ui.lang.get(), "Your browser says: offline.", "المتصفح بيقول: مش متصل.")
            }}</span>
        </p>
        <div class="sc-actions">
            <Button kind=ButtonKind::Secondary size=ButtonSize::Sm icon="wifi-off" on:click=move |_| fire("offline")>
                {move || tr(ui.lang.get(), "Lose the connection", "اقطع النت")}
            </Button>
            <Button kind=ButtonKind::Ghost size=ButtonSize::Sm icon="wifi" on:click=move |_| fire("online")>
                {move || tr(ui.lang.get(), "Get it back", "رجّعه")}
            </Button>
        </div>
    }
}

// ─── States: a read that retries ───────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq)]
struct Try {
    n: u32,
    /// Since the press.
    at_ms: f64,
    ok: Option<bool>,
}

/// `components::net::retry` against a server function that fails its first two tries
/// (`notes::flaky_read`): each try's start time shows the backoff, 300 ms and then 600 ms on top
/// of the round trips. Offline, no try is spent: the helper waits for the `online` event.
#[island]
pub fn RetryDemo() -> impl IntoView {
    let ui = use_ui();
    let tries = RwSignal::new(Vec::<Try>::new());
    let running = RwSignal::new(false);
    let outcome = RwSignal::new(None::<bool>);
    let run = move |_| {
        if running.get_untracked() {
            return;
        }
        running.set(true);
        tries.set(Vec::new());
        outcome.set(None);
        #[cfg(feature = "hydrate")]
        leptos::task::spawn_local(async move {
            let t0 = js::now();
            let mut n = 0u32;
            let res = crate::components::net::retry(|| {
                n += 1;
                let k = n;
                tries.update(|v| {
                    v.push(Try {
                        n: k,
                        at_ms: js::now() - t0,
                        ok: None,
                    })
                });
                async move {
                    let r = notes::flaky_read(k, 2).await;
                    let ok = r.is_ok();
                    tries.update(|v| {
                        if let Some(t) = v.iter_mut().find(|t| t.n == k) {
                            t.ok = Some(ok);
                        }
                    });
                    r
                }
            })
            .await;
            outcome.set(Some(res.is_ok()));
            running.set(false);
        });
    };
    view! {
        <div class="sc-retry">
            <Button kind=ButtonKind::Secondary size=ButtonSize::Sm icon="rotate-ccw"
                loading=Signal::derive(move || running.get()) on:click=run>
                {move || tr(ui.lang.get(), "Read a flaky endpoint", "اقرا من endpoint مش مستقر")}
            </Button>
            <ol class="sc-retry__log" aria-live="polite">
                {move || {
                    let l = ui.lang.get();
                    let num = ui.numerals.get();
                    tries.get().into_iter().map(|t| {
                        let state = match t.ok {
                            None => "wait",
                            Some(true) => "ok",
                            Some(false) => "fail",
                        };
                        let label = match (l, t.ok) {
                            (_, None) => "…",
                            (Lang::En, Some(true)) => "200 OK",
                            (Lang::Ar, Some(true)) => "200 تمام",
                            (Lang::En, Some(false)) => "failed",
                            (Lang::Ar, Some(false)) => "فشلت",
                        };
                        view! {
                            <li class="sc-retry__try" data-state=state>
                                <span class="sc-retry__n">{match l {
                                    Lang::En => format!("Try {}", format::count(t.n, num)),
                                    Lang::Ar => format!("محاولة {}", format::count(t.n, num)),
                                }}</span>
                                <span class="sc-retry__at mono">{format!("+{}", ms(t.at_ms, num))}</span>
                                <span class="sc-retry__res">{label}</span>
                            </li>
                        }
                    }).collect_view()
                }}
            </ol>
            <p class="sc-note" role="status">{move || {
                let l = ui.lang.get();
                match outcome.get() {
                    Some(true) => tr(l, "Got it on the third try — the visitor only saw a short wait.", "جت من المحاولة التالتة — والزائر شاف انتظار قصير وبس."),
                    Some(false) => tr(l, "Four tries failed: now the section shows its error and a Retry.", "أربع محاولات فشلوا: دلوقتي القسم بيعرض الخطأ و«إعادة المحاولة»."),
                    None => tr(l, "This endpoint fails its first two tries on purpose.", "الـ endpoint ده بيفشل في أول محاولتين بالقصد."),
                }
            }}</p>
        </div>
    }
}
