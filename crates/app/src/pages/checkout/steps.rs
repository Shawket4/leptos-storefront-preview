//! The server-rendered parts of checkout: the stepper, the delivery-slot, payment and review steps
//! (plain forms the islands router posts for us), the order summary, and the tiny island that
//! moves focus to a step's heading when the router swaps steps in.

use super::{CheckoutPayment, CheckoutPlace, CheckoutSlot, Ctx, Step, fee_label};
use crate::api::types::{CartView, DeliverySlot};
use crate::components::button::{Button, ButtonKind, ButtonSize};
use crate::components::icons::Icon;
use crate::components::image::Picture;
use crate::components::price::{Price, PriceSize};
use crate::format;
use crate::pages::cart::{ArtImage, Totals, art};
use crate::prefs::{Lang, Numerals, tr};
use domain::{L10n, PaymentMethod, Piastres};
use leptos::either::Either;
use leptos::prelude::*;
use leptos::server_fn::ServerFn;

/// Focus the step's heading (`#co-step-title`) when this step arrived by an islands-router
/// navigation, so keyboard and screen-reader users land on the new step, not on <body>. On a
/// full page load it does nothing (the browser's own focus order applies).
#[island]
pub fn StepFocus() -> impl IntoView {
    Effect::new(move |_| {
        if crate::pages::cart::store::arrived_by_navigation() {
            crate::pages::cart::store::focus_soon("#co-step-title");
        }
    });
    view! { <span class="sr-only"></span> }
}

#[component]
pub fn Stepper(step: Step, done: [bool; 4], lang: Lang, num: Numerals) -> impl IntoView {
    let progress = step.index() as f32 / (Step::ALL.len() - 1) as f32;
    view! {
        <nav class="co-steps" aria-label=tr(lang, "Checkout steps", "خطوات إتمام الطلب")>
            <ol class="co-steps__list" style=format!("--p:{progress:.4}")>
                {Step::ALL.into_iter().map(|s| {
                    let current = s == step;
                    let state = if current { "is-current" } else if done[s.index()] { "is-done" } else { "is-next" };
                    let dot = if done[s.index()] && !current {
                        Either::Left(view! { <Icon name="check" size=14/> })
                    } else {
                        Either::Right(view! { <span class="mono">{format::count(s.index() as u32 + 1, num)}</span> })
                    };
                    let inner = view! {
                        <span class="co-step__dot" aria-hidden="true">{dot}</span>
                        <span class="co-step__label">{s.label(lang)}</span>
                    };
                    // Finished steps (and the current one's predecessors) are links back.
                    let link = (done[s.index()] || s < step) && !current;
                    view! {
                        <li class=format!("co-step {state}") aria-current={if current { "step" } else { "false" }}>
                            {if link {
                                Either::Left(view! { <a class="co-step__link" href=s.href()>{inner}</a> })
                            } else {
                                Either::Right(view! { <span class="co-step__link">{inner}</span> })
                            }}
                        </li>
                    }
                }).collect_view()}
            </ol>
        </nav>
    }
}

/// An error message the server sent back for this step, focused after a router round-trip.
#[component]
fn StepError(message: Option<String>) -> impl IntoView {
    message.map(|m| {
        view! {
            <p class="notice notice--error" role="alert" tabindex="-1" data-autofocus="">
                <Icon name="circle-alert" size=20/>
                <span>{m}</span>
            </p>
        }
    })
}

#[component]
fn StepHeading(text: &'static str, #[prop(optional)] sub: Option<&'static str>) -> impl IntoView {
    view! {
        <div class="co-step-head">
            <h2 id="co-step-title" class="co-step-title" tabindex="-1">{text}</h2>
            {sub.map(|s| view! { <p class="co-step-sub">{s}</p> })}
            <StepFocus/>
        </div>
    }
}

/// Group slots by their day label (Express / Today / Tomorrow), keeping order.
fn groups(slots: &[DeliverySlot]) -> Vec<(L10n, bool, Vec<DeliverySlot>)> {
    let mut out: Vec<(L10n, bool, Vec<DeliverySlot>)> = Vec::new();
    for s in slots {
        match out.last_mut() {
            Some((day, _, list)) if *day == s.day => list.push(s.clone()),
            _ => out.push((s.day.clone(), s.express, vec![s.clone()])),
        }
    }
    out
}

/// The local day a slot id (`d{day}-{hour}`) belongs to, as a date label.
fn slot_date(id: &str, now_ms: i64, lang: Lang, num: Numerals) -> Option<String> {
    let day: i64 = id.strip_prefix('d')?.split('-').next()?.parse().ok()?;
    let offset = format::cairo_offset_hours(now_ms) * 3_600_000;
    // Noon local time of that day, as a UTC instant.
    Some(format::date_cairo(
        day * 86_400_000 + 12 * 3_600_000 - offset,
        lang,
        num,
    ))
}

#[component]
pub fn SlotStep(
    slots: Vec<DeliverySlot>,
    chosen: Option<String>,
    error: Option<String>,
    now_ms: i64,
    lang: Lang,
    num: Numerals,
) -> impl IntoView {
    let ar = lang.is_rtl();
    let checked = chosen.or_else(|| slots.iter().find(|s| s.available).map(|s| s.id.clone()));
    let groups = groups(&slots);
    view! {
        <form class="co-form" method="post" action=CheckoutSlot::url() data-pending-ui="" aria-busy="false">
            <StepHeading text=tr(lang, "When should we deliver?", "نوصّل لك إمتى؟")
                sub=tr(lang, "Pick a window that suits you — we'll text you when the courier sets off.", "اختار الميعاد اللي يناسبك، وهنبعتلك رسالة لما المندوب يتحرك.")/>
            <StepError message=error/>
            <fieldset class="slots">
                <legend class="sr-only">{tr(lang, "Delivery time", "ميعاد التوصيل")}</legend>
                <For each=move || groups.clone() key=|(day, _, list)| format!("{}:{}", day.en, list.first().map(|s| s.id.clone()).unwrap_or_default()) let:group>
                    {
                        let (day, express, list) = group;
                        let date = (!express).then(|| list.first().and_then(|s| slot_date(&s.id, now_ms, lang, num))).flatten();
                        let checked = checked.clone();
                        view! {
                            <div class="slot-group">
                                <h3 class="slot-group__title">
                                    {express.then(|| view! { <Icon name="zap" size=16/> })}
                                    <span>{day.pick(ar).to_owned()}</span>
                                    {date.map(|d| view! { <span class="slot-group__date">{d}</span> })}
                                </h3>
                                <div class="slot-grid">
                                    <For each=move || list.clone() key=|s| s.id.clone() let:item>
                                        <SlotCard is_checked={checked.as_deref() == Some(item.id.as_str())} item lang num/>
                                    </For>
                                </div>
                            </div>
                        }
                    }
                </For>
            </fieldset>
            <div class="co-actions">
                <Button button_type="submit" kind=ButtonKind::Primary size=ButtonSize::Lg icon_end="arrow-right" attr:data-focus-id="co-next">
                    {tr(lang, "Continue to payment", "كمّل للدفع")}
                </Button>
            </div>
        </form>
    }
}

#[component]
fn SlotCard(item: DeliverySlot, is_checked: bool, lang: Lang, num: Numerals) -> impl IntoView {
    let slot = item;
    let ar = lang.is_rtl();
    let reason_id = format!("slot-{}-why", slot.id);
    let reason = (!slot.available).then(|| {
        if slot.express {
            tr(
                lang,
                "Express runs 9 AM – 11 PM",
                "التوصيل السريع من ٩ الصبح لـ ١١ بالليل",
            )
        } else {
            tr(lang, "Too late for today", "الميعاد ده فات النهارده")
        }
    });
    view! {
        <label class="slot" data-available=slot.available.to_string()>
            <input class="slot__input" type="radio" name="slot" value=slot.id.clone() checked=is_checked
                disabled={!slot.available} aria-describedby=reason_id.clone()/>
            <span class="slot__body">
                <span class="slot__window">{slot.window.pick(ar).to_owned()}</span>
                <span class={if slot.fee.0 == 0 { "slot__fee slot__fee--free" } else { "slot__fee" }}>{fee_label(slot.fee, lang, num)}</span>
                {reason.map(|r| view! { <span class="slot__why" id=reason_id>{r}</span> })}
            </span>
            <span class="slot__tick" aria-hidden="true"><Icon name="check" size=14/></span>
        </label>
    }
}

#[component]
pub fn PaymentStep(
    chosen: Option<PaymentMethod>,
    error: Option<String>,
    lang: Lang,
) -> impl IntoView {
    let chosen = chosen.unwrap_or(PaymentMethod::CashOnDelivery);
    let options: [(
        PaymentMethod,
        &str,
        &'static str,
        &'static str,
        &'static str,
    ); 3] = [
        (
            PaymentMethod::CashOnDelivery,
            "cod",
            "banknote",
            tr(lang, "Cash on delivery", "كاش عند الاستلام"),
            tr(
                lang,
                "Pay the courier when your order arrives.",
                "ادفع للمندوب لما الطلب يوصلك.",
            ),
        ),
        (
            PaymentMethod::Card,
            "card",
            "credit-card",
            tr(lang, "Debit or credit card", "كارت بنكي"),
            tr(
                lang,
                "Online card payments aren't switched on in this preview — you can still choose it to see the flow. Nothing is charged.",
                "الدفع بالكارت أونلاين مش متفعّل في النسخة التجريبية دي — تقدر تختاره وتشوف الخطوات، ومفيش أي مبلغ هيتخصم.",
            ),
        ),
        (
            PaymentMethod::Wallet,
            "wallet",
            "smartphone",
            tr(
                lang,
                "Mobile wallet or InstaPay",
                "محفظة موبايل أو InstaPay",
            ),
            tr(
                lang,
                "You'd get a payment request after ordering. In this preview nothing is sent.",
                "هيوصلك طلب دفع بعد ما تأكد الطلب. في النسخة التجريبية مفيش حاجة هتتبعت.",
            ),
        ),
    ];
    view! {
        <form class="co-form" method="post" action=CheckoutPayment::url() data-pending-ui="" aria-busy="false">
            <StepHeading text=tr(lang, "How would you like to pay?", "هتدفع إزاي؟")/>
            <StepError message=error/>
            <fieldset class="pay">
                <legend class="sr-only">{tr(lang, "Payment method", "طريقة الدفع")}</legend>
                {options.into_iter().map(|(method, code, icon, title, note)| {
                    let note_id = format!("pay-{code}-note");
                    view! {
                        <label class="pay-opt">
                            <input class="pay-opt__input" type="radio" name="payment" value=code checked={method == chosen}
                                aria-describedby=note_id.clone()/>
                            <span class="pay-opt__icon" aria-hidden="true"><Icon name=icon size=22/></span>
                            <span class="pay-opt__text">
                                <span class="pay-opt__title">{title}</span>
                                <span class="pay-opt__note" id=note_id>{note}</span>
                            </span>
                            <span class="pay-opt__tick" aria-hidden="true"><Icon name="check" size=14/></span>
                        </label>
                    }
                }).collect_view()}
            </fieldset>
            <div class="co-actions">
                <Button button_type="submit" kind=ButtonKind::Primary size=ButtonSize::Lg icon_end="arrow-right" attr:data-focus-id="co-next">
                    {tr(lang, "Review your order", "راجع طلبك")}
                </Button>
            </div>
        </form>
    }
}

pub fn payment_label(p: PaymentMethod, lang: Lang) -> &'static str {
    match p {
        PaymentMethod::CashOnDelivery => tr(lang, "Cash on delivery", "كاش عند الاستلام"),
        PaymentMethod::Card => tr(
            lang,
            "Card (preview — nothing is charged)",
            "كارت بنكي (تجريبي — مفيش خصم)",
        ),
        PaymentMethod::Wallet => tr(
            lang,
            "Mobile wallet / InstaPay (preview)",
            "محفظة موبايل / InstaPay (تجريبي)",
        ),
    }
}

/// The address as a few short lines, in the visitor's language (city and area by id).
pub fn address_lines(a: &super::rules::AddressInput, lang: Lang, num: Numerals) -> Vec<String> {
    let mut l1 = vec![a.street.clone()];
    let mut bits = vec![format!("{} {}", tr(lang, "Building", "عمارة"), a.building)];
    if !a.floor.is_empty() {
        bits.push(format!("{} {}", tr(lang, "floor", "الدور"), a.floor));
    }
    if !a.apartment.is_empty() {
        bits.push(format!("{} {}", tr(lang, "apt", "شقة"), a.apartment));
    }
    l1.push(bits.join(if lang.is_rtl() { "، " } else { ", " }));
    let city = super::rules::city(&a.city)
        .map(|c| c.name(lang))
        .unwrap_or_default();
    let area = super::rules::area_name(&a.area, lang).unwrap_or_default();
    let mut lines = vec![
        format!(
            "{} · {}",
            a.name,
            format::ltr(&format::digits(&super::rules::pretty_phone(&a.phone), num))
        ),
        l1.join(if lang.is_rtl() { "، " } else { ", " }),
        format!("{area}{}{city}", if lang.is_rtl() { "، " } else { ", " }),
    ];
    if !a.landmark.is_empty() {
        lines.push(format!(
            "{}: {}",
            tr(lang, "Landmark", "علامة مميزة"),
            a.landmark
        ));
    }
    if !a.notes.is_empty() {
        lines.push(format!("{}: {}", tr(lang, "Notes", "ملاحظات"), a.notes));
    }
    lines
}

#[component]
pub fn ReviewStep(
    ctx: Ctx,
    extra_fee: Piastres,
    error: Option<String>,
    lang: Lang,
    num: Numerals,
) -> impl IntoView {
    let ar = lang.is_rtl();
    let address = ctx.address.clone().unwrap_or_default();
    let slot = ctx
        .slot
        .as_ref()
        .and_then(|id| ctx.slots.iter().find(|s| &s.id == id))
        .cloned();
    let slot_text = slot
        .map(|s| {
            let date = (!s.express)
                .then(|| slot_date(&s.id, ctx.now_ms, lang, num))
                .flatten();
            let mut t = format!("{}, {}", s.day.pick(ar), s.window.pick(ar));
            if let Some(d) = date {
                t.push_str(&format!(" · {d}"));
            }
            t
        })
        .unwrap_or_default();
    let payment = ctx
        .payment
        .map(|p| payment_label(p, lang))
        .unwrap_or_default();
    let total = ctx.cart.total + extra_fee;
    let place_label = format!(
        "{} · {}",
        tr(lang, "Place order", "أكّد الطلب"),
        format::money(total, lang, num)
    );
    let lines = ctx.cart.lines.clone();
    view! {
        <div class="co-review">
            <StepHeading text=tr(lang, "Check and place your order", "راجع طلبك وأكّده")/>
            <StepError message=error/>
            <dl class="review">
                <div class="review__row">
                    <dt><Icon name="map-pin" size=18/><span>{tr(lang, "Deliver to", "التوصيل على")}</span></dt>
                    <dd>{address_lines(&address, lang, num).into_iter().map(|l| view! { <span class="review__line">{l}</span> }).collect_view()}</dd>
                    <a class="review__edit" href=Step::Address.href()>{tr(lang, "Edit", "تعديل")}</a>
                </div>
                <div class="review__row">
                    <dt><Icon name="clock" size=18/><span>{tr(lang, "Delivery time", "ميعاد التوصيل")}</span></dt>
                    <dd><span class="review__line">{slot_text}</span></dd>
                    <a class="review__edit" href=Step::Slot.href()>{tr(lang, "Change", "تغيير")}</a>
                </div>
                <div class="review__row">
                    <dt><Icon name="credit-card" size=18/><span>{tr(lang, "Payment", "الدفع")}</span></dt>
                    <dd><span class="review__line">{payment}</span></dd>
                    <a class="review__edit" href=Step::Payment.href()>{tr(lang, "Change", "تغيير")}</a>
                </div>
            </dl>
            <h3 class="co-sub">{format!("{} · {}", tr(lang, "Items", "المنتجات"), format::items(ctx.cart.count, lang, num))}</h3>
            <ul class="mini-lines" role="list">
                <For each=move || lines.clone() key=|l| l.key.clone() let:l>
                    <MiniLine l lang num/>
                </For>
            </ul>
            <div class="co-review__totals">
                <Totals v=ctx.cart.clone() extra_fee lang num/>
            </div>
            <form class="co-place" method="post" action=CheckoutPlace::url() data-pending-ui="" aria-busy="false">
                <input type="hidden" name="total" value=total.0.to_string()/>
                <Button button_type="submit" kind=ButtonKind::Primary size=ButtonSize::Lg full_width=true attr:data-focus-id="co-place">
                    {place_label}
                </Button>
                <p class="co-legal">
                    {tr(lang, "By placing your order you agree to the shop's terms of sale.", "بإتمام الطلب فإنك توافق على شروط البيع الخاصة بالمتجر.")}
                </p>
            </form>
        </div>
    }
}

#[component]
fn MiniLine(l: crate::api::types::CartLineView, lang: Lang, num: Numerals) -> impl IntoView {
    let ar = lang.is_rtl();
    let mut detail = Vec::new();
    if !l.variant_label.pick(ar).is_empty() {
        detail.push(l.variant_label.pick(ar).to_owned());
    }
    detail.extend(l.addons.iter().map(|a| format!("+ {}", a.pick(ar))));
    view! {
        <li class="mini-line">
            <span class="mini-line__media"><Picture img=l.img.clone() sizes="48px" lang/></span>
            <span class="mini-line__main">
                <span class="mini-line__name">{l.name.pick(ar).to_owned()}</span>
                <span class="mini-line__meta">
                    <span class="mono">{format!("{}×", format::count(l.qty, num))}</span>
                    {(!detail.is_empty()).then(|| view! { <span>{detail.join(" · ")}</span> })}
                </span>
            </span>
            <span class="mini-line__total"><Price amount=l.line_total size=PriceSize::Sm no_badge=true lang numerals=num/></span>
        </li>
    }
}

/// The order summary beside the steps (a disclosure on phones — works without JS).
#[component]
pub fn Summary(cart: CartView, extra_fee: Piastres, lang: Lang, num: Numerals) -> impl IntoView {
    let total = cart.total + extra_fee;
    let lines = cart.lines.clone();
    let lines2 = cart.lines.clone();
    view! {
        <details class="co-summary-sm card">
            <summary>
                <span class="co-summary-sm__label"><Icon name="receipt" size=18/>{tr(lang, "Order summary", "ملخص الطلب")}</span>
                <span class="co-summary-sm__total"><Price amount=total size=PriceSize::Sm no_badge=true lang numerals=num/></span>
                <Icon name="chevron-down" size=18 class="co-summary-sm__chev"/>
            </summary>
            <div class="co-summary-sm__body">
                <ul class="mini-lines" role="list">
                    <For each=move || lines.clone() key=|l| l.key.clone() let:l>
                        <MiniLine l lang num/>
                    </For>
                </ul>
                <Totals v=cart.clone() extra_fee lang num/>
            </div>
        </details>
        <aside class="co-summary card" aria-labelledby="co-summary-title">
            <h2 class="co-summary__title" id="co-summary-title">{tr(lang, "Order summary", "ملخص الطلب")}</h2>
            <ul class="mini-lines" role="list">
                <For each=move || lines2.clone() key=|l| l.key.clone() let:l>
                    <MiniLine l lang num/>
                </For>
            </ul>
            <Totals v=cart extra_fee lang num/>
        </aside>
    }
}

#[component]
pub fn CheckoutEmpty(lang: Lang) -> impl IntoView {
    view! {
        <div class="cart-empty cart-empty--page card co-empty">
            <ArtImage art=art("empty-cart") lang/>
            <h1 class="cart-empty__title">{tr(lang, "Nothing to check out yet", "مفيش حاجة لسه نكمّل بيها")}</h1>
            <p class="cart-empty__body">{tr(lang, "Your cart is empty. Add something you like, then come back here.", "سلتك فاضية. ضيف اللي يعجبك وارجع هنا.")}</p>
            <Button href="/" kind=ButtonKind::Primary size=ButtonSize::Lg>{tr(lang, "Start shopping", "ابدأ التسوق")}</Button>
        </div>
    }
}
