//! The purchase panel (an island): price, stock, SKU, variant picker, Madar-style add-on groups,
//! quantity, add to cart, and the phone's sticky add-to-cart bar.
//!
//! Fine-grained: the selected variant is one signal; price, compare-at, stock, SKU and image are
//! separate memos of it, so switching between two variants with the same price touches only the
//! stock line, and so on. Add-ons and quantity feed a live line total.
//!
//! Works without JS: variant values are links (`?v=<id>`, the server honours it), the add-ons are
//! named inputs, and the form posts to the server function, which redirects back here with
//! `?added=` (or the error, which server_fn encodes into the URL). With JS the same `<ActionForm>`
//! posts in the background: optimistic badge bump, button feedback, then the server's cart view
//! (or a rollback and the error inline).

use super::model::{
    self, Avail, MAX_QTY, Notice, PanelData, Stock, addons_total, bounds, group_hint, group_ok,
    group_problem, input_name, stock, stock_text, surcharge, text, variant_href,
};
use super::{Picked, use_product_store};
use crate::api::types::CartView;
use crate::components::button::{Button, ButtonKind, ButtonSize};
use crate::components::icons::Icon;
use crate::components::price::{PriceLive, PriceSize};
use crate::components::stepper::Stepper;
use crate::components::toast::{self, ToastKind};
use crate::format;
use crate::prefs::{Lang, tr, use_ui};
use crate::stores::use_cart;
use domain::{OptionKind, SelectionType};
use leptos::html;
use leptos::prelude::*;

/// Add a configured product to the cart. A thin wrapper over `api::cart::cart_add` that a plain
/// HTML form can post to: `addons` arrives as `addons[0]=…&addons[3]=…` (serde_qs sequence
/// syntax; `cart_add`'s comma-joined `addons` string can't be produced by a form with more than
/// one add-on input, because serde_qs rejects repeated keys). On a no-JS success it redirects back
/// with `?added=<qty>` so the page can confirm; errors come back encoded by server_fn (`?__err=`).
#[server(endpoint = "product_add_to_cart")]
pub async fn add_to_cart(
    product: u32,
    variant: u32,
    qty: Option<u32>,
    addons: Option<Vec<String>>,
) -> Result<CartView, ServerFnError> {
    let addons: Vec<String> = addons
        .unwrap_or_default()
        .into_iter()
        .map(|a| a.trim().to_owned())
        .filter(|a| !a.is_empty())
        .collect();
    let joined = (!addons.is_empty()).then(|| addons.join(","));
    let view = crate::api::cart::cart_add(product, variant, qty, joined).await?;
    if let Some(parts) = use_context::<http::request::Parts>() {
        let header = |n: http::header::HeaderName| {
            parts
                .headers
                .get(n)
                .and_then(|v| v.to_str().ok())
                .map(str::to_owned)
        };
        let plain_form = header(http::header::ACCEPT).is_some_and(|a| a.contains("text/html"));
        if plain_form && let Some(back) = header(http::header::REFERER) {
            leptos_axum::redirect(&with_param(
                &back,
                "added",
                &qty.unwrap_or(1).max(1).to_string(),
            ));
        }
    }
    Ok(view)
}

/// `url` with `key=value` set (and any previous error/confirmation parameters dropped).
#[cfg(feature = "ssr")]
fn with_param(url: &str, key: &str, value: &str) -> String {
    let (base, query) = url.split_once('?').unwrap_or((url, ""));
    let mut kept: Vec<&str> = query
        .split('&')
        .filter(|kv| !kv.is_empty())
        .filter(|kv| {
            let k = kv.split('=').next().unwrap_or("");
            k != key && k != "__err" && k != "__path"
        })
        .collect();
    let pair = format!("{key}={value}");
    kept.push(&pair);
    format!("{base}?{}", kept.join("&"))
}

#[island]
pub fn PurchasePanel(data: PanelData) -> impl IntoView {
    view! { <PanelBody data=data/> }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Phase {
    Idle,
    Added,
}

/// Replace `?v=` in the address bar (no history entry), dropping one-off parameters.
fn replace_url(v: Option<u32>) {
    let Ok(loc) = window().location().pathname() else {
        return;
    };
    let search = window().location().search().unwrap_or_default();
    let Ok(params) = web_sys::UrlSearchParams::new_with_str(&search) else {
        return;
    };
    for k in ["__err", "__path", "added"] {
        params.delete(k);
    }
    if let Some(v) = v {
        params.set("v", &v.to_string());
    }
    let qs = String::from(params.to_string());
    let url = if qs.is_empty() {
        loc
    } else {
        format!("{loc}?{qs}")
    };
    if let Ok(h) = window().history() {
        let _ = h.replace_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&url));
    }
}

#[component]
pub fn PanelBody(data: PanelData) -> impl IntoView {
    let ui = use_ui();
    let cart = use_cart();
    let store = use_product_store();
    let pid = data.product;
    let form_id = format!("pdp-buy-{pid}");
    let d = StoredValue::new(data.clone());

    // ── Selection and its memos ──
    let sel = RwSignal::new(data.selected);
    let variant =
        Memo::new(move |_| d.with_value(|d| model::variant(&d.variants, sel.get()).clone()));
    let price = Memo::new(move |_| variant.with(|v| v.price));
    let compare = Memo::new(move |_| variant.with(|v| v.compare_at.filter(|c| *c > v.price)));
    let stock_n = Memo::new(move |_| variant.with(|v| v.stock));
    let sku = Memo::new(move |_| variant.with(|v| v.sku.clone()));
    let image = Memo::new(move |_| variant.with(|v| v.image));
    let max_qty = Memo::new(move |_| stock_n.get().clamp(1, MAX_QTY));

    let picks: Vec<RwSignal<Vec<String>>> = data
        .groups
        .iter()
        .map(|_| RwSignal::new(Vec::new()))
        .collect();
    let picks = StoredValue::new(picks);
    let pick = move |g: usize| picks.with_value(|p| p[g]);
    let addons = Memo::new(move |_| {
        let chosen: Vec<Vec<String>> = picks.with_value(|p| p.iter().map(|s| s.get()).collect());
        d.with_value(|d| addons_total(&d.groups, &chosen))
    });
    let qty = RwSignal::new(1u32);
    let unit = Memo::new(move |_| price.get() + addons.get());
    let total = Memo::new(move |_| unit.get() * qty.get());

    // ── State of the add ──
    let js = RwSignal::new(false);
    let tried = RwSignal::new(false);
    let phase = RwSignal::new(Phase::Idle);
    let bar = RwSignal::new(false);
    let (initial_error, initial_added) = match &data.notice {
        Some(Notice::Error(c)) => (Some(c.clone()), None),
        Some(Notice::Added(n)) => (None, Some(*n)),
        None => (None, None),
    };
    let has_notice = initial_error.is_some() || initial_added.is_some();
    let error = RwSignal::new(initial_error);
    let added_notice = RwSignal::new(initial_added);
    let action = ServerAction::<AddToCart>::new();
    let pending = action.pending();
    let bumped = StoredValue::new(0u32);
    let buy_row = NodeRef::<html::Div>::new();
    let groups_ref = NodeRef::<html::Div>::new();

    let group_bad = move |g: usize| d.with_value(|d| !group_ok(&d.groups[g], &pick(g).get()));
    let any_bad = move || (0..d.with_value(|d| d.groups.len())).any(group_bad);

    // ── Effects (client only) ──
    Effect::new(move |_| {
        js.set(true);
        // One-off parameters (no-JS confirmation, errors) shouldn't survive a refresh.
        replace_url(None);
    });
    Effect::new(move |_| {
        store.picked.set(Some(Picked {
            product: pid,
            variant: sel.get(),
            image: image.get(),
        }));
    });
    Effect::watch(
        move || sel.get(),
        move |v, _, _| replace_url(Some(*v)),
        false,
    );
    Effect::watch(
        move || max_qty.get(),
        move |m, _, _| {
            if qty.get_untracked() > *m {
                qty.set(*m);
            }
        },
        false,
    );
    // Optimistic: the badge counts the units the moment the request leaves.
    Effect::watch(
        move || action.input().get().map(|i| i.qty.unwrap_or(1).max(1)),
        move |q, _, _| {
            if let Some(q) = *q {
                bumped.set_value(q);
                cart.count.update(|c| *c += q);
            }
        },
        false,
    );
    // The server's answer: adopt its cart, or roll back and say why.
    Effect::watch(
        move || action.version().get(),
        move |_, _, _| match action.value().get_untracked() {
            Some(Ok(view)) => {
                cart.apply(view);
                cart.pulse.update(|p| *p += 1);
                cart.open.set(true);
                error.set(None);
                phase.set(Phase::Added);
                let name = d.with_value(|d| text(&d.name, ui.lang.get_untracked()));
                toast::show(
                    match ui.lang.get_untracked() {
                        Lang::En => format!("Added to your cart · {name}"),
                        Lang::Ar => format!("اتضاف لسلتك · {name}"),
                    },
                    ToastKind::Success,
                );
                set_timeout(
                    move || phase.set(Phase::Idle),
                    std::time::Duration::from_millis(1800),
                );
            }
            Some(Err(e)) => {
                let q = bumped.get_value();
                cart.count.update(|c| *c = c.saturating_sub(q));
                error.set(Some(model::error_code(&e)));
            }
            None => {}
        },
        false,
    );
    // The phone's bar appears once the main button has scrolled away above.
    Effect::new(move |_| {
        use wasm_bindgen::{JsCast, closure::Closure};
        let Some(el) = buy_row.get() else { return };
        let cb = Closure::<dyn FnMut(js_sys::Array, web_sys::IntersectionObserver)>::new(
            move |entries: js_sys::Array, _obs: web_sys::IntersectionObserver| {
                if let Ok(e) = entries
                    .get(entries.length().saturating_sub(1))
                    .dyn_into::<web_sys::IntersectionObserverEntry>()
                {
                    bar.set(!e.is_intersecting() && e.bounding_client_rect().top() < 0.0);
                }
            },
        );
        if let Ok(io) = web_sys::IntersectionObserver::new(cb.as_ref().unchecked_ref()) {
            io.observe(&el);
        }
        cb.forget();
    });

    // Retry = press the real button again: same validation, offline check and optimistic bump.
    let retry = move || {
        use wasm_bindgen::JsCast;
        error.set(None);
        if let Some(row) = buy_row.get_untracked()
            && let Ok(Some(btn)) = row.query_selector(".pp-add")
            && let Ok(btn) = btn.dyn_into::<web_sys::HtmlElement>()
        {
            btn.click();
        }
    };
    // Back online: an "offline" error is stale; let the visitor try again.
    let online = window_event_listener(leptos::ev::online, move |_| {
        if error.get_untracked().as_deref() == Some("offline") {
            error.set(Some("back_online".into()));
        }
    });
    on_cleanup(move || online.remove());

    let select = move |v: u32| {
        if v != sel.get_untracked() {
            error.set(None);
            added_notice.set(None);
            sel.set(v);
        }
    };

    // Validate before the form submits (a click on a submit button that is prevented never fires
    // `submit`, so `<ActionForm>` doesn't dispatch).
    let on_add = move |ev: web_sys::MouseEvent| {
        if stock_n.get_untracked() == 0 || pending.get_untracked() {
            ev.prevent_default();
            return;
        }
        added_notice.set(None);
        // Offline: say so now instead of sending a request that can only fail.
        if !crate::components::net::is_online() {
            ev.prevent_default();
            error.set(Some("offline".into()));
            return;
        }
        if any_bad() {
            ev.prevent_default();
            tried.set(true);
            error.set(None);
            if let Some(root) = groups_ref.get_untracked()
                && let Ok(Some(first)) = root.query_selector(".pp-group.is-bad input")
            {
                use wasm_bindgen::JsCast;
                if let Ok(input) = first.dyn_into::<web_sys::HtmlElement>() {
                    let _ = input.focus();
                }
                let opts = web_sys::ScrollIntoViewOptions::new();
                opts.set_block(web_sys::ScrollLogicalPosition::Center);
                opts.set_behavior(web_sys::ScrollBehavior::Smooth);
                if let Ok(Some(g)) = root.query_selector(".pp-group.is-bad") {
                    g.scroll_into_view_with_scroll_into_view_options(&opts);
                }
            }
            return;
        }
        error.set(None);
    };

    // ── Variant picker ──
    let axes = data
        .axes
        .iter()
        .cloned()
        .enumerate()
        .map(|(ai, axis)| {
            let swatch = axis.kind == OptionKind::Swatch;
            let head_id = format!("pp-ax-{pid}-{ai}");
            let head_ref = head_id.clone();
            let note_id = format!("pp-axn-{pid}-{ai}");
            let axis_name = axis.name.clone();
            let axis_for_value = axis.clone();
            let current_label = move || {
                let l = ui.lang.get();
                variant.with(|v| {
                    v.opts
                        .get(ai)
                        .and_then(|o| axis_for_value.values.iter().find(|x| &x.id == o))
                        .map(|x| text(&x.label, l))
                        .unwrap_or_default()
                })
            };
            let ids: Vec<String> = axis.values.iter().map(|v| v.id.clone()).collect();
            let ids = StoredValue::new(ids);
            let note = move || {
                let l = ui.lang.get();
                let cur = variant.get();
                d.with_value(|d| {
                    ids.with_value(|ids| {
                        ids.iter()
                            .filter(|id| !model::avail(&d.variants, &cur, ai, id).enabled())
                            .map(|id| model::reason(&d.axes, &d.variants, &cur, ai, id, l))
                            .collect::<Vec<_>>()
                            .join(" · ")
                    })
                })
            };
            // Arrow keys move the choice to the next value that can be picked (radio semantics).
            let keys = move |ev: web_sys::KeyboardEvent| {
                let rtl = ui.lang.get_untracked().is_rtl();
                let fwd = match ev.key().as_str() {
                    "ArrowDown" => true,
                    "ArrowUp" => false,
                    "ArrowRight" => !rtl,
                    "ArrowLeft" => rtl,
                    _ => return,
                };
                ev.prevent_default();
                let cur = variant.get_untracked();
                let next = d.with_value(|d| {
                    ids.with_value(|ids| {
                        let at = cur.opts.get(ai).and_then(|o| ids.iter().position(|x| x == o)).unwrap_or(0);
                        let order: Vec<usize> = if fwd {
                            (at + 1..ids.len()).chain(0..at).collect()
                        } else {
                            (0..at).rev().chain((at + 1..ids.len()).rev()).collect()
                        };
                        order.into_iter().find_map(|i| match model::avail(&d.variants, &cur, ai, &ids[i]) {
                            Avail::Open(v) => Some(v),
                            _ => None,
                        })
                    })
                });
                if let Some(v) = next {
                    select(v);
                    // Focus follows the choice once the DOM reflects it.
                    let head = format!("pp-ax-{pid}-{ai}");
                    request_animation_frame(move || {
                        if let Ok(Some(el)) =
                            document().query_selector(&format!("[aria-labelledby=\"{head}\"] [aria-checked=\"true\"]"))
                        {
                            use wasm_bindgen::JsCast;
                            if let Ok(el) = el.dyn_into::<web_sys::HtmlElement>() {
                                let _ = el.focus();
                            }
                        }
                    });
                }
            };
            let values = axis
                .values
                .iter()
                .cloned()
                .map(|val| {
                    let vid = val.id.clone();
                    let rid = StoredValue::new(val.id.clone());
                    let st = Memo::new(move |_| d.with_value(|d| model::avail(&d.variants, &variant.get(), ai, &vid)));
                    let label = val.label.clone();
                    let label2 = val.label.clone();
                    let label3 = val.label.clone();
                    let note_id = note_id.clone();
                    let slug = d.with_value(|d| d.slug.clone());
                    let href = move || match st.get() {
                        Avail::Open(v) => Some(variant_href(&slug, v)),
                        Avail::Selected => Some(variant_href(&slug, sel.get())),
                        _ => None,
                    };
                    let dot = val
                        .swatch
                        .as_deref()
                        .filter(|c| c.len() == 7 && c.starts_with('#') && c[1..].chars().all(|x| x.is_ascii_hexdigit()))
                        .map(|c| format!("--sw:{c}"));
                    let inner = if swatch {
                        view! { <span class="pp-swatch__dot" style=dot aria-hidden="true"></span> }.into_any()
                    } else {
                        view! { <span class="pp-chip__label trim">{move || text(&label, ui.lang.get())}</span> }.into_any()
                    };
                    view! {
                        <a
                            class=if swatch { "pp-swatch" } else { "pp-chip" }
                            role="radio"
                            href=href
                            aria-checked=move || if st.get() == Avail::Selected { "true" } else { "false" }
                            aria-disabled=move || (!st.get().enabled()).then_some("true")
                            aria-describedby=move || (!st.get().enabled()).then(|| note_id.clone())
                            aria-label=move || swatch.then(|| text(&label2, ui.lang.get()))
                            // A value that can't be picked says why on hover too (the note below says it
                            // for everyone; the swatch's colour name otherwise).
                            title=move || {
                                let l = ui.lang.get();
                                if st.get().enabled() {
                                    swatch.then(|| text(&label3, l))
                                } else {
                                    Some(rid.with_value(|id| {
                                        d.with_value(|d| model::reason(&d.axes, &d.variants, &variant.get(), ai, id, l))
                                    }))
                                }
                            }
                            tabindex=move || js.get().then(|| if st.get() == Avail::Selected { "0" } else { "-1" })
                            class:is-on=move || st.get() == Avail::Selected
                            class:is-off=move || !st.get().enabled()
                            data-press=""
                            on:click=move |ev| {
                                ev.prevent_default();
                                if let Avail::Open(v) = st.get_untracked() {
                                    select(v);
                                }
                            }
                        >
                            {inner}
                        </a>
                    }
                })
                .collect_view();
            view! {
                <div class="pp-axis" role="radiogroup" aria-labelledby=head_ref on:keydown=keys>
                    <p class="pp-axis__head" id=head_id>
                        <span class="pp-axis__name">{move || text(&axis_name, ui.lang.get())}</span>
                        <span class="pp-axis__value">{current_label}</span>
                    </p>
                    <div class=if swatch { "pp-swatches" } else { "pp-chips" }>{values}</div>
                    <p class="pp-axis__note" id=note_id class:is-empty=move || note().is_empty()>{note}</p>
                </div>
            }
        })
        .collect_view();

    // ── Add-on groups ──
    let groups = data
        .groups
        .iter()
        .cloned()
        .enumerate()
        .map(|(gi, g)| {
            let single = g.selection == SelectionType::Single;
            let (min, max) = bounds(&g);
            let optional_single = single && min == 0;
            let err_id = format!("pp-ge-{pid}-{gi}");
            let err_ref = err_id.clone();
            let gname = g.name.clone();
            let g2 = g.clone();
            let g3 = g.clone();
            let bad = move || tried.get() && group_bad(gi);
            let none_name = d.with_value(|d| input_name(&d.groups, gi, 0));
            let none = optional_single.then(|| {
                view! {
                    <label class="pp-opt">
                        <input
                            class="pp-opt__input"
                            type="radio"
                            name=none_name
                            value=""
                            checked=true
                            prop:checked=move || pick(gi).with(|p| p.is_empty())
                            on:change=move |_| pick(gi).set(Vec::new())
                        />
                        <span class="pp-opt__box" aria-hidden="true"></span>
                        <span class="pp-opt__name">{move || tr(ui.lang.get(), "No, thanks", "لا، شكرًا")}</span>
                        <span class="pp-opt__price"></span>
                    </label>
                }
            });
            let options = g
                .options
                .iter()
                .cloned()
                .enumerate()
                .map(|(oi, o)| {
                    let name = d.with_value(|d| input_name(&d.groups, gi, oi));
                    let id = StoredValue::new(o.id.clone());
                    let id3 = o.id.clone();
                    let oname = o.name.clone();
                    let checked = move || id.with_value(|id| pick(gi).with(|p| p.contains(id)));
                    let full = move || !single && id.with_value(|id| pick(gi).with(|p| p.len() as u32 >= max && !p.contains(id)));
                    let toggle = move |_| {
                        let s = pick(gi);
                        if single {
                            s.set(vec![id3.clone()]);
                        } else {
                            s.update(|p| {
                                if let Some(i) = p.iter().position(|x| x == &id3) {
                                    p.remove(i);
                                } else {
                                    p.push(id3.clone());
                                }
                            });
                        }
                        if !group_bad(gi) && !any_bad() {
                            tried.set(false);
                        }
                    };
                    let p = o.price;
                    view! {
                        <label class="pp-opt" class:is-on=checked class:is-full=full>
                            <input
                                class="pp-opt__input"
                                type=if single { "radio" } else { "checkbox" }
                                name=name
                                value=o.id.clone()
                                prop:checked=checked
                                prop:disabled=full
                                on:change=toggle
                            />
                            <span class="pp-opt__box" aria-hidden="true">
                                {(!single).then(|| view! { <Icon name="check" size=14/> })}
                            </span>
                            <span class="pp-opt__name">{move || text(&oname, ui.lang.get())}</span>
                            <span class="pp-opt__price mono">{move || surcharge(p, ui.lang.get(), ui.numerals.get())}</span>
                        </label>
                    }
                })
                .collect_view();
            view! {
                <fieldset
                    class="pp-group"
                    class:is-bad=bad
                    aria-describedby=err_ref
                    aria-invalid=move || bad().then_some("true")
                >
                    <legend class="pp-group__head">
                        <span class="pp-group__name">{move || text(&gname, ui.lang.get())}</span>
                        <span class="pp-group__hint" class:is-req={min > 0}>
                            {move || group_hint(&g2, ui.lang.get(), ui.numerals.get())}
                        </span>
                    </legend>
                    <div class="pp-group__opts">
                        {none}
                        {options}
                    </div>
                    <p class="pp-group__err" id=err_id role="alert">
                        {move || bad().then(|| view! {
                            <Icon name="circle-alert" size=16/>
                            <span>{group_problem(&g3, ui.lang.get(), ui.numerals.get())}</span>
                        })}
                    </p>
                </fieldset>
            }
        })
        .collect_view();
    let has_axes = !data.axes.is_empty();
    let has_groups = !data.groups.is_empty();

    // ── Live text ──
    let stock_state = Memo::new(move |_| stock(stock_n.get()));
    let stock_line = move || {
        let s = stock_state.get();
        let icon = match s {
            Stock::In => "circle-check",
            Stock::Low(_) => "timer",
            Stock::Out => "circle-alert",
        };
        view! {
            <Icon name=icon size=16/>
            <span>{stock_text(s, ui.lang.get(), ui.numerals.get())}</span>
        }
    };
    let saving = move || {
        compare.get().map(|was| {
            let off = format::money(was - price.get(), ui.lang.get(), ui.numerals.get());
            match ui.lang.get() {
                Lang::En => format!("You save {off}"),
                Lang::Ar => format!("هتوفّر {off}"),
            }
        })
    };
    let button_label = move || {
        let l = ui.lang.get();
        if stock_n.get() == 0 {
            tr(l, "Out of stock", "غير متوفر")
        } else if phase.get() == Phase::Added {
            tr(l, "Added", "اتضاف")
        } else if pending.get() {
            tr(l, "Adding…", "بنضيفه…")
        } else {
            tr(l, "Add to cart", "أضف للسلة")
        }
    };
    let total_text = move || {
        (stock_n.get() > 0).then(|| format::money(total.get(), ui.lang.get(), ui.numerals.get()))
    };
    let qty_label = move || tr(ui.lang.get(), "Quantity", "الكمية");
    let name = data.name.clone();

    view! {
        <div class="pp" class:is-js=move || js.get()>
            <div class="pp-card">
                <div class="pp-price">
                    <PriceLive amount=price compare_at=compare size=PriceSize::Xl/>
                    <p class="pp-price__save" class:is-empty=move || compare.get().is_none()>{saving}</p>
                    <p class="pp-price__tax">{move || tr(ui.lang.get(), "VAT included", "شامل ضريبة القيمة المضافة")}</p>
                </div>
                <div class="pp-stock">
                    <p
                        class="pp-stock__state"
                        class:is-low=move || matches!(stock_state.get(), Stock::Low(_))
                        class:is-out=move || stock_state.get() == Stock::Out
                        role="status"
                    >
                        {stock_line}
                    </p>
                    <p class="pp-sku">
                        <span class="pp-sku__k">{move || tr(ui.lang.get(), "SKU", "رقم المنتج")}</span>
                        " "
                        <span class="mono" dir="ltr">{move || sku.get()}</span>
                    </p>
                </div>
                <ActionForm action=action attr:id=form_id.clone() attr:class="pp-form">
                    <input type="hidden" name="product" value=pid/>
                    <input type="hidden" name="variant" value=move || sel.get().to_string() prop:value=move || sel.get().to_string()/>
                    {has_axes.then(|| view! { <div class="pp-axes">{axes}</div> })}
                    {has_groups.then(|| view! { <div class="pp-groups" node_ref=groups_ref>{groups}</div> })}
                    <div class="pp-buy" node_ref=buy_row>
                        // The design-system stepper drives the signal; the hidden field carries it in the
                        // form (without JS the quantity is 1 — it can be changed in the cart).
                        <input type="hidden" name="qty" value="1" prop:value=move || qty.get().to_string()/>
                        // `max` follows the variant's stock (the stepper takes it as a plain number).
                        {move || {
                            let max = max_qty.get();
                            view! {
                                <Stepper
                                    value=qty
                                    on_change=Callback::new(move |v: u32| qty.set(v.clamp(1, max)))
                                    min=1
                                    max=max
                                    label=Signal::derive(move || qty_label().to_owned())
                                    disabled=Signal::derive(move || stock_n.get() == 0)
                                    class="pp-qty"
                                />
                            }
                        }}
                        <Button
                            kind=ButtonKind::Primary
                            size=ButtonSize::Lg
                            button_type="submit"
                            full_width=true
                            class="pp-add"
                            loading=Signal::derive(move || pending.get())
                            disabled=Signal::derive(move || stock_n.get() == 0)
                            on:click=on_add
                        >
                            <span class="pp-add__main">
                                <span class="pp-add__state" aria-hidden="true">
                                    {move || (phase.get() == Phase::Added).then(|| view! { <Icon name="check" size=18/> })}
                                </span>
                                <span class="pp-add__label trim">{button_label}</span>
                            </span>
                            <span class="pp-add__total mono" class:is-empty=move || stock_n.get() == 0>{total_text}</span>
                        </Button>
                    </div>
                    // A notice the server rendered (a no-JS add came back here) takes focus on load, so the
                    // browser scrolls to it — no script needed.
                    <div
                        class="pp-msgs"
                        aria-live="polite"
                        tabindex=has_notice.then_some("-1")
                        autofocus=has_notice
                    >
                        {move || error.get().map(|code| view! {
                            <p
                                class="pp-msg"
                                class:pp-msg--error={code != "back_online"}
                                class:pp-msg--info={code == "back_online"}
                                role="alert"
                            >
                                {if code == "back_online" {
                                    view! { <Icon name="info" size=16/> }.into_any()
                                } else {
                                    view! { <Icon name="circle-alert" size=16/> }.into_any()
                                }}
                                <span class="pp-msg__text">{model::error_text(&code, ui.lang.get())}</span>
                                {model::retryable(&code).then(|| view! {
                                    <button type="button" class="pp-msg__action" on:click=move |_| retry()>
                                        {move || tr(ui.lang.get(), "Try again", "حاول مرة أخرى")}
                                    </button>
                                })}
                            </p>
                        })}
                        {move || added_notice.get().map(|n| {
                            let l = ui.lang.get();
                            let what = format::items(n, l, ui.numerals.get());
                            view! {
                                <p class="pp-msg pp-msg--ok" role="status">
                                    <Icon name="circle-check" size=16/>
                                    <span>{match l { Lang::En => format!("Added {what} to your cart."), Lang::Ar => format!("اتضاف {what} لسلتك.") }}</span>
                                    <a class="pp-msg__link" href="/cart">{tr(l, "View cart", "اعرض السلة")}</a>
                                </p>
                            }
                        })}
                    </div>
                </ActionForm>
            </div>
            <div
                class="pp-bar"
                class:is-shown=move || bar.get()
                inert=move || (!bar.get()).then_some("")
                aria-hidden=move || (!bar.get()).then_some("true")
            >
                <div class="pp-bar__info">
                    <p class="pp-bar__name">{move || text(&name, ui.lang.get())}</p>
                    <p class="pp-bar__price mono">{move || format::money(total.get(), ui.lang.get(), ui.numerals.get())}</p>
                </div>
                <Button
                    kind=ButtonKind::Primary
                    size=ButtonSize::Md
                    button_type="submit"
                    class="pp-bar__add"
                    loading=Signal::derive(move || pending.get())
                    disabled=Signal::derive(move || stock_n.get() == 0)
                    attr:form=form_id
                    on:click=on_add
                >
                    {button_label}
                </Button>
            </div>
        </div>
    }
}
