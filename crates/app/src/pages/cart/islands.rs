//! The cart's islands: the header button, the drawer, and the quick-add "+" on product cards.
//! Each control is a real `<form>` posting to a cart server function; with JS the submit goes
//! through the op queue (`store`) for an instant, sequenced, roll-back-able update.

use super::math::{self, Op};
use super::shim::drawer::Drawer;
use super::shim::toast::{ToastKind, Toaster};
use super::store::{CartOps, OpResult, use_ops};
use crate::api::cart::{CartAdd, CartRemove, CartSetQty};
use crate::api::types::CartLineView;
use crate::components::icons::Icon;
use crate::components::image::Picture;
use crate::components::price::{PriceLive, PriceSize};
use crate::components::skeleton::Skeleton;
use crate::format;
use crate::prefs::{Lang, tr, use_ui};
use crate::stores::use_cart;
use domain::{Piastres, ProductId, VariantId};
use leptos::either::{Either, EitherOf4};
use leptos::form::FromFormData;
use leptos::prelude::*;
use leptos::server_fn::ServerFn;
use serde::de::DeserializeOwned;

/// What `<ActionForm>` renders — `<form method="post" action="/api/…">`, inputs named after the
/// server function's arguments — with the parsed submit handed to `on_input` (the op queue)
/// instead of an `Action`. Without JS it is a plain POST and server_fn redirects back.
#[component]
pub fn CartForm<F>(
    #[prop(optional, into)] class: String,
    on_input: Callback<F>,
    children: Children,
) -> impl IntoView
where
    F: ServerFn + DeserializeOwned + Send + Sync + 'static,
{
    view! {
        <form class=class action=F::url() method="post" on:submit=move |ev: leptos::ev::SubmitEvent| {
            ev.prevent_default();
            match F::from_event(&ev) {
                Ok(input) => on_input.run(input),
                Err(err) => leptos::logging::error!("cart form: {err:?}"),
            }
        }>
            {children()}
        </form>
    }
}

/// Is the visitor on a page that *is* the cart (or checks it out)? There the header button is a
/// plain link instead of opening the drawer over the same content.
fn on_cart_pages() -> bool {
    let path = window().location().pathname().unwrap_or_default();
    path.starts_with("/cart") || path.starts_with("/checkout")
}

// ── Header button ─────────────────────────────────────────────────────────────────────────────

/// The bag in the header. One signal drives it (`CartStore::count`); it pops when the count goes
/// up. The server renders it empty — public pages never contain a visitor's cart — and the client
/// fills it from the `cart_n` cookie right after hydration. Without JS it is a link to `/cart`.
#[island]
pub fn CartButton() -> impl IntoView {
    let cart = use_cart();
    let ui = use_ui();
    // Render exactly what the server rendered during hydration, then switch to the real count.
    let mounted = RwSignal::new(false);
    Effect::new(move |_| mounted.set(true));
    let count = move || if mounted.get() { cart.count.get() } else { 0 };
    let label = move || {
        let (lang, n) = (ui.lang.get(), count());
        if n == 0 {
            tr(lang, "Cart, empty", "السلة، فاضية").to_owned()
        } else {
            format!(
                "{}, {}",
                tr(lang, "Cart", "السلة"),
                format::items(n, lang, ui.numerals.get())
            )
        }
    };
    // Two identical keyframe animations, alternated, so every bump restarts the pop.
    let badge_class = move || match cart.pulse.get() {
        0 => "icon-btn__badge cart-btn__badge",
        n if n % 2 == 1 => "icon-btn__badge cart-btn__badge is-pop-a",
        _ => "icon-btn__badge cart-btn__badge is-pop-b",
    };
    view! {
        <a href="/cart" class="icon-btn icon-btn--outline cart-btn" aria-label=label
            aria-haspopup="dialog" aria-controls="cart-drawer" data-count=move || { count().to_string() }
            on:click=move |ev| {
                if !on_cart_pages() {
                    ev.prevent_default();
                    cart.open.set(true);
                }
            }>
            <Icon name="shopping-bag" size=20/>
            <span class=badge_class aria-hidden="true">{move || format::count(count(), ui.numerals.get())}</span>
        </a>
    }
}

// ── Quick add ─────────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq)]
enum Qa {
    Idle,
    Busy,
    Added,
}

/// The round "+" on product cards (single variant, nothing required). The badge bumps and pops
/// the moment it's pressed; the toast names the product once the server confirms; a failure
/// rolls the count back and says why. Without JS: a POST to `cart_add`, redirected back.
#[island]
pub fn QuickAdd(product: ProductId, variant: VariantId) -> impl IntoView {
    let ops = use_ops();
    let ui = use_ui();
    let state = RwSignal::new(Qa::Idle);
    let on_input = move |input: CartAdd| {
        state.set(Qa::Busy);
        let (p, v) = (input.product, input.variant);
        let done = Callback::new(move |r: OpResult| match r {
            Ok(view) => {
                state.set(Qa::Added);
                set_timeout(
                    move || state.set(Qa::Idle),
                    std::time::Duration::from_millis(1600),
                );
                let lang = ops.ui.lang.get_untracked();
                let name = view
                    .lines
                    .iter()
                    .find(|l| l.product.0 == p && l.variant.0 == v)
                    .map(|l| l.name.pick(lang.is_rtl()).to_owned())
                    .unwrap_or_default();
                let text = match lang {
                    Lang::En => format!("{name} is in your cart"),
                    Lang::Ar => format!("{name} اتضاف للسلة"),
                };
                ops.toasts.show_with_action(
                    ToastKind::Success,
                    text,
                    tr(lang, "View cart", "شوف السلة"),
                    move || ops.cart.open.set(true),
                );
            }
            Err(code) => {
                state.set(Qa::Idle);
                ops.toasts.show(
                    ToastKind::Error,
                    super::copy::message(&code, ops.ui.lang.get_untracked()),
                );
            }
        });
        ops.submit(
            Op::Add {
                product: p,
                variant: v,
                qty: input.qty.unwrap_or(1).max(1),
                addons: vec![],
                line: None,
            },
            Some(done),
        );
    };
    let label = move || match state.get() {
        Qa::Added => tr(ui.lang.get(), "Added to cart", "اتضاف للسلة"),
        _ => tr(ui.lang.get(), "Add to cart", "أضف للسلة"),
    };
    view! {
        <CartForm class="quick-add" on_input=Callback::new(on_input)>
            <input type="hidden" name="product" value=product.0/>
            <input type="hidden" name="variant" value=variant.0/>
            <button type="submit" class="quick-add__btn" data-state=move || match state.get() {
                Qa::Idle => "idle",
                Qa::Busy => "busy",
                Qa::Added => "added",
            } aria-label=label>
                {move || match state.get() {
                    Qa::Idle => Either::Left(view! { <Icon name="plus" size=20/> }),
                    Qa::Busy => Either::Right(Either::Left(view! { <Icon name="loader-circle" size=20 class="spin"/> })),
                    Qa::Added => Either::Right(Either::Right(view! { <Icon name="check" size=20/> })),
                }}
            </button>
        </CartForm>
    }
}

// ── Drawer ────────────────────────────────────────────────────────────────────────────────────

/// The slide-over cart, mounted once by the app shell. Lines load the first time it opens (and
/// are re-checked each time after); quantities change optimistically; removing a line offers undo.
#[island]
pub fn CartDrawer() -> impl IntoView {
    let cart = use_cart();
    let ops = use_ops();
    let ui = use_ui();
    Effect::new(move |was_open: Option<bool>| {
        let open = cart.open.get();
        if open && was_open != Some(true) {
            ops.revalidate();
        }
        open
    });
    let title = Signal::derive(move || tr(ui.lang.get(), "Your cart", "سلتك").to_owned());
    view! {
        <Drawer open=cart.open title id="cart-drawer" class="cart-drawer">
            <DrawerContent/>
            <Toaster in_drawer=true/>
        </Drawer>
        <Toaster/>
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Body {
    Loading,
    Failed,
    Empty,
    Lines,
}

fn close_drawer() {
    use_cart().open.set(false);
}

#[component]
fn DrawerContent() -> impl IntoView {
    let cart = use_cart();
    let ops = use_ops();
    let ui = use_ui();
    let body =
        Memo::new(
            move |_| match cart.view.with(|v| v.as_ref().map(|v| v.lines.is_empty())) {
                None if ops.load_failed.get() => Body::Failed,
                None => Body::Loading,
                // A quick add whose line the server hasn't described yet.
                Some(true) if cart.count.get() > 0 => Body::Loading,
                Some(true) => Body::Empty,
                Some(false) => Body::Lines,
            },
        );
    move || match body.get() {
        Body::Loading => EitherOf4::A(view! { <DrawerSkeleton/> }),
        Body::Failed => EitherOf4::B(view! {
            <div class="cart-note" role="alert">
                <Icon name="triangle-alert" size=24/>
                <p>{move || tr(ui.lang.get(), "We couldn't load your cart.", "تعذّر تحميل السلة.")}</p>
                <button type="button" class="btn btn--secondary btn--md" on:click=move |_| ops.revalidate()>
                    {move || tr(ui.lang.get(), "Try again", "جرّب تاني")}
                </button>
            </div>
        }),
        Body::Empty => EitherOf4::C(view! {
            <div class="cart-empty">
                <span class="cart-empty__art" aria-hidden="true"><Icon name="shopping-bag" size=36/></span>
                <h3 class="cart-empty__title">{move || tr(ui.lang.get(), "Your cart is empty", "سلتك فاضية")}</h3>
                <p class="cart-empty__body">
                    {move || tr(ui.lang.get(), "Add something you like — it'll wait for you here.", "ضيف اللي يعجبك، وهيستناك هنا.")}
                </p>
                <a class="btn btn--primary btn--md" href="/" on:click=move |_| close_drawer()>
                    {move || tr(ui.lang.get(), "Start shopping", "ابدأ التسوق")}
                </a>
            </div>
        }),
        Body::Lines => EitherOf4::D(view! {
            <div class="cart-drawer__scroll">
                <DrawerLines/>
            </div>
            <DrawerSummary/>
        }),
    }
}

#[component]
fn DrawerSkeleton() -> impl IntoView {
    view! {
        <div class="cart-drawer__scroll" aria-hidden="true">
            <ul class="clines">
                {(0..3).map(|_| view! {
                    <li class="cline cline--skeleton">
                        <span class="cline__media"><Skeleton class="cline__skel-media"/></span>
                        <span class="cline__main">
                            <Skeleton width="70%" height="1.1em"/>
                            <Skeleton width="40%" height="0.9em"/>
                            <Skeleton width="30%" height="1em"/>
                        </span>
                    </li>
                }).collect_view()}
            </ul>
        </div>
    }
}

#[component]
fn DrawerLines() -> impl IntoView {
    let cart = use_cart();
    let ui = use_ui();
    let keys = Memo::new(move |_| {
        cart.view.with(|v| {
            v.as_ref()
                .map(|v| v.lines.iter().map(|l| l.key.clone()).collect::<Vec<_>>())
                .unwrap_or_default()
        })
    });
    view! {
        <ul class="clines" aria-label=move || tr(ui.lang.get(), "Items in your cart", "المنتجات في سلتك")>
            <For each=move || keys.get() key=|k| k.clone() let:key>
                <DrawerLine key/>
            </For>
        </ul>
    }
}

/// Move focus off a line that's about to disappear: to the next line's stepper, the previous
/// one's, or the drawer's close button.
fn focus_after_removal(key: &str) {
    use wasm_bindgen::JsCast;
    let doc = document();
    let row = doc
        .query_selector(&format!("#cart-drawer [data-line-key=\"{key}\"]"))
        .ok()
        .flatten();
    let next = row
        .as_ref()
        .and_then(|r| {
            r.next_element_sibling()
                .or_else(|| r.previous_element_sibling())
        })
        .and_then(|r| r.query_selector(".qty__btn").ok().flatten());
    let target = next.or_else(|| {
        doc.query_selector("#cart-drawer .drawer__close")
            .ok()
            .flatten()
    });
    if let Some(el) = target.and_then(|e| e.dyn_into::<web_sys::HtmlElement>().ok()) {
        let _ = el.focus();
    }
}

/// Remove a line now; the toast offers to put it back (in place, with its quantity and add-ons).
fn remove_with_undo(ops: CartOps, line: CartLineView) {
    focus_after_removal(&line.key);
    ops.submit(
        Op::Remove {
            key: line.key.clone(),
        },
        None,
    );
    let lang = ops.ui.lang.get_untracked();
    let name = line.name.pick(lang.is_rtl()).to_owned();
    let text = match lang {
        Lang::En => format!("Removed {name}"),
        Lang::Ar => format!("شيلنا {name} من السلة"),
    };
    ops.toasts.show_with_action(
        ToastKind::Info,
        text,
        tr(lang, "Undo", "رجّعه"),
        move || {
            if let Some((product, variant, addons)) = math::parse_key(&line.key) {
                ops.submit(
                    Op::Add {
                        product,
                        variant,
                        qty: line.qty,
                        addons,
                        line: Some(Box::new(line.clone())),
                    },
                    None,
                );
            }
        },
    );
}

#[component]
fn DrawerLine(key: String) -> impl IntoView {
    let cart = use_cart();
    let ops = use_ops();
    let ui = use_ui();
    let line = Memo::new({
        let key = key.clone();
        move |_| {
            cart.view.with(|v| {
                v.as_ref()
                    .and_then(|v| v.lines.iter().find(|l| l.key == key).cloned())
            })
        }
    });
    let Some(first) = line.get_untracked() else {
        return None;
    };
    let qty = move || line.with(|l| l.as_ref().map(|l| l.qty).unwrap_or(0));
    let max = move || line.with(|l| l.as_ref().map(|l| l.max_qty).unwrap_or(0));
    let unit =
        Signal::derive(move || line.with(|l| l.as_ref().map(|l| l.unit_price).unwrap_or_default()));
    let was = Signal::derive(move || line.with(|l| l.as_ref().and_then(|l| l.compare_at)));
    let total = Signal::derive(move || {
        line.with(|l| l.as_ref().map(|l| l.line_total).unwrap_or(Piastres::ZERO))
    });
    let href = format!("/p/{}", first.slug);
    let name = first.name.clone();
    let detail = {
        let (variant, addons) = (first.variant_label.clone(), first.addons.clone());
        move || {
            let ar = ui.lang.get().is_rtl();
            let mut parts: Vec<String> = Vec::new();
            if !variant.pick(ar).is_empty() {
                parts.push(variant.pick(ar).to_owned());
            }
            parts.extend(addons.iter().map(|a| format!("+ {}", a.pick(ar))));
            parts.join(" · ")
        }
    };
    let img = first.img.clone();
    let on_qty = move |input: CartSetQty| {
        if input.qty == 0 {
            if let Some(l) = line.get_untracked() {
                remove_with_undo(ops, l);
            }
        } else {
            ops.submit(
                Op::SetQty {
                    key: input.key,
                    qty: input.qty,
                },
                None,
            );
        }
    };
    let on_remove = move |_: CartRemove| {
        if let Some(l) = line.get_untracked() {
            remove_with_undo(ops, l);
        }
    };
    let name_for = {
        let name = name.clone();
        move || name.pick(ui.lang.get().is_rtl()).to_owned()
    };
    let dec_label = {
        let n = name_for.clone();
        move || {
            let lang = ui.lang.get();
            if qty() <= 1 {
                format!("{}: {}", tr(lang, "Remove", "شيل"), n())
            } else {
                format!("{}: {}", tr(lang, "One less", "قلّل واحد"), n())
            }
        }
    };
    let inc_label = {
        let n = name_for.clone();
        move || format!("{}: {}", tr(ui.lang.get(), "One more", "زوّد واحد"), n())
    };
    let rm_label = {
        let n = name_for.clone();
        move || format!("{}: {}", tr(ui.lang.get(), "Remove", "شيل"), n())
    };
    let low_stock = move || {
        let (m, lang, num) = (max(), ui.lang.get(), ui.numerals.get());
        (m > 0 && m <= 5).then(|| match lang {
            Lang::En => format!("Only {} in stock", format::count(m, num)),
            Lang::Ar => format!("المتاح {} بس", format::count(m, num)),
        })
    };
    let (key_row, key_qty, key_rm) = (key.clone(), key.clone(), key);
    Some(view! {
        <li class="cline" data-line-key=key_row>
            <a class="cline__media" href=href.clone() tabindex="-1" aria-hidden="true" on:click=move |_| close_drawer()>
                {move || view! { <Picture img=img.clone() sizes="72px" lang=ui.lang.get()/> }}
            </a>
            <div class="cline__main">
                <a class="cline__name" href=href on:click=move |_| close_drawer()>{name_for.clone()}</a>
                {move || { let d = detail(); (!d.is_empty()).then(|| view! { <p class="cline__detail">{d}</p> }) }}
                <div class="cline__unit">
                    <PriceLive amount=unit compare_at=was size=PriceSize::Sm/>
                    {move || (qty() > 1).then(|| view! { <span class="cline__each">{tr(ui.lang.get(), "each", "للقطعة")}</span> })}
                </div>
                {move || low_stock().map(|t| view! { <p class="cline__stock">{t}</p> })}
                <CartForm class="cline__remove" on_input=Callback::new(on_remove)>
                    <input type="hidden" name="key" value=key_rm/>
                    <button type="submit" class="link-btn" aria-label=rm_label>{move || tr(ui.lang.get(), "Remove", "شيل")}</button>
                </CartForm>
            </div>
            <div class="cline__side">
                <CartForm class="qty" on_input=Callback::new(on_qty)>
                    <input type="hidden" name="key" value=key_qty/>
                    <button type="submit" name="qty" value=move || { qty().saturating_sub(1).to_string() } class="qty__btn"
                        aria-label=dec_label>
                        {move || if qty() <= 1 { view! { <Icon name="trash-2" size=18/> } } else { view! { <Icon name="minus" size=18/> } }}
                    </button>
                    <output class="qty__val mono" aria-live="polite" aria-atomic="true">
                        {move || format::count(qty(), ui.numerals.get())}
                    </output>
                    <button type="submit" name="qty" value=move || { (qty() + 1).to_string() } class="qty__btn"
                        disabled=move || { qty() >= max() } aria-label=inc_label>
                        <Icon name="plus" size=18/>
                    </button>
                </CartForm>
                <span class="cline__total"><PriceLive amount=total size=PriceSize::Sm/></span>
            </div>
        </li>
    })
}

#[component]
fn DrawerSummary() -> impl IntoView {
    let cart = use_cart();
    let ops = use_ops();
    let ui = use_ui();
    let pick = move |f: fn(&crate::api::types::CartView) -> Piastres| {
        Signal::derive(move || cart.view.with(|v| v.as_ref().map(f).unwrap_or_default()))
    };
    let subtotal = pick(|v| v.subtotal);
    let savings = pick(|v| v.savings);
    let fee = pick(|v| v.delivery_fee);
    let total = pick(|v| v.total);
    let vat = pick(|v| v.tax_included);
    view! {
        <div class="cart-summary" aria-busy=move || ops.busy.get().to_string()>
            <FreeDeliveryMeter/>
            <dl class="totals">
                <div class="totals__row">
                    <dt>{move || tr(ui.lang.get(), "Subtotal", "المجموع")}</dt>
                    <dd><PriceLive amount=subtotal size=PriceSize::Sm/></dd>
                </div>
                {move || (savings.get().0 > 0).then(|| view! {
                    <div class="totals__row totals__row--save">
                        <dt>{move || tr(ui.lang.get(), "You save", "وفّرت")}</dt>
                        <dd><PriceLive amount=savings size=PriceSize::Sm/></dd>
                    </div>
                })}
                <div class="totals__row">
                    <dt>{move || tr(ui.lang.get(), "Delivery", "التوصيل")}</dt>
                    <dd>{move || if fee.get().0 == 0 {
                        Either::Left(view! { <span class="totals__free">{tr(ui.lang.get(), "Free", "مجاني")}</span> })
                    } else {
                        Either::Right(view! { <PriceLive amount=fee size=PriceSize::Sm/> })
                    }}</dd>
                </div>
                <div class="totals__row totals__row--total">
                    <dt>{move || tr(ui.lang.get(), "Total", "الإجمالي")}</dt>
                    <dd><PriceLive amount=total size=PriceSize::Lg/></dd>
                </div>
            </dl>
            <p class="totals__vat">
                {move || {
                    let (lang, num) = (ui.lang.get(), ui.numerals.get());
                    let amount = format::money(vat.get(), lang, num);
                    match lang {
                        Lang::En => format!("Prices include 14% VAT ({amount})."),
                        Lang::Ar => format!("الأسعار شاملة ضريبة القيمة المضافة {}٪ ({amount}).", format::count(14, num)),
                    }
                }}
            </p>
            <div class="cart-summary__actions">
                <a class="btn btn--primary btn--lg btn--block" href="/checkout" on:click=move |_| close_drawer()>
                    <span class="btn__label">{move || tr(ui.lang.get(), "Checkout", "كمّل الطلب")}</span>
                    <Icon name="arrow-right" size=18 class="flip-rtl btn__end"/>
                </a>
                <a class="btn btn--ghost btn--md btn--block" href="/cart" on:click=move |_| close_drawer()>
                    {move || tr(ui.lang.get(), "View cart", "شوف السلة")}
                </a>
            </div>
        </div>
    }
}

/// "Add EGP 230.00 more for free delivery", with a solid progress bar that grows (transform only).
#[component]
pub fn FreeDeliveryMeter() -> impl IntoView {
    let cart = use_cart();
    let ui = use_ui();
    let left = move || {
        cart.view.with(|v| {
            v.as_ref()
                .map(|v| (v.free_delivery_over.0 - v.subtotal.0).max(0))
                .unwrap_or(0)
        })
    };
    let progress = move || {
        cart.view
            .with(|v| v.as_ref().map(math::free_delivery_progress).unwrap_or(0.0))
    };
    let text = move || {
        let (lang, num) = (ui.lang.get(), ui.numerals.get());
        let left = left();
        if left == 0 {
            tr(lang, "Delivery is on us.", "التوصيل علينا.").to_owned()
        } else {
            let m = format::money(Piastres(left), lang, num);
            match lang {
                Lang::En => format!("Add {m} more for free delivery."),
                Lang::Ar => format!("ضيف بـ {m} كمان والتوصيل يبقى مجاني."),
            }
        }
    };
    view! {
        <div class="ship" data-done=move || (left() == 0).to_string()>
            <p class="ship__text"><Icon name="truck" size=18/><span>{text.clone()}</span></p>
            <div class="ship__bar" role="progressbar" aria-valuemin="0" aria-valuemax="100"
                aria-valuenow=move || ((progress() * 100.0).round() as u32).to_string() aria-valuetext=text>
                <span class="ship__fill" style=move || format!("--p:{:.4}", progress())></span>
            </div>
        </div>
    }
}
