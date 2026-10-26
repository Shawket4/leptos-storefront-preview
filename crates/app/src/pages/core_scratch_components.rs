//! `/core-scratch` (default case): the shared components built from the cart's patterns, in their
//! states, server-rendered and — for the ones browser code fills — cloned from `<template>`s by a
//! dev-only feature (`client/src/core/scratch-components.ts`), which also drives the timeline and
//! the live status with the real helpers (`setTimeline`, `setLiveStatus`, `retryIn`).

use crate::components::button::{Button, ButtonKind, ButtonSize, IconButton};
use crate::components::item_row::{ItemList, ItemRow, ItemSize};
use crate::components::kv::{KeyValueList, KvRow};
use crate::components::live_status::{LiveState, LiveStatus};
use crate::components::stepper::Stepper;
use crate::components::steps::StepIndicator;
use crate::components::timeline::{StepState, Timeline, TimelineStep};
use crate::components::totals::{TotalRow, TotalTone, Totals};
use crate::format;
use crate::prefs::{tr, use_ui};
use domain::Piastres;
use leptos::prelude::*;

const IMG: &str = "/media/nile/p1009-1-240.webp";

#[component]
pub fn Specimens() -> impl IntoView {
    let ui = use_ui();
    let (lang, num) = (ui.lang.get_untracked(), ui.numerals.get_untracked());
    let m = move |p: i64| format::money(Piastres(p), lang, num);
    let t = move |en: &'static str, ar: &'static str| tr(lang, en, ar).to_owned();

    let sums = vec![
        TotalRow::new(t("Subtotal", "المجموع"), m(1_239_700)).slot("subtotal"),
        TotalRow::new(t("Savings", "التوفير"), format!("−{}", m(150_000)))
            .tone(TotalTone::Saving)
            .slot("savings"),
        TotalRow::new(t("Delivery", "التوصيل"), m(4_500))
            .slot("delivery")
            .hidden(true),
        TotalRow::new(t("Delivery", "التوصيل"), t("Free", "مجاني"))
            .tone(TotalTone::Free)
            .slot("free"),
        TotalRow::new(t("Total", "الإجمالي"), m(1_089_700))
            .tone(TotalTone::Total)
            .slot("total"),
    ];
    // The template version: every value empty, savings and free hidden (the client fills them).
    let sums_tpl = vec![
        TotalRow::new(t("Subtotal", "المجموع"), "").slot("subtotal"),
        TotalRow::new(t("Savings", "التوفير"), "")
            .tone(TotalTone::Saving)
            .slot("savings")
            .hidden(true),
        TotalRow::new(t("Delivery", "التوصيل"), "").slot("delivery"),
        TotalRow::new(t("Delivery", "التوصيل"), t("Free", "مجاني"))
            .tone(TotalTone::Free)
            .slot("free")
            .hidden(true),
        TotalRow::new(t("Total", "الإجمالي"), "")
            .tone(TotalTone::Total)
            .slot("total"),
    ];
    let long_name = t(
        "Samsung Galaxy S25 Ultra 5G, 12 GB RAM, 512 GB, Titanium Silverblue",
        "سامسونج جالاكسي S25 ألترا 5G، رام 12 جيجا، 512 جيجا، أزرق تيتانيوم",
    );
    let (long_a, long_b) = (long_name.clone(), long_name);
    let steps = vec![
        (
            t("Address", "العنوان"),
            Some("/core-scratch?step=1".to_owned()),
        ),
        (
            t("Delivery", "التوصيل"),
            Some("/core-scratch?step=2".to_owned()),
        ),
        (
            t("Payment", "الدفع"),
            Some("/core-scratch?step=3".to_owned()),
        ),
        (t("Review", "المراجعة"), None),
    ];
    let timeline = vec![
        TimelineStep::new(t("Placed", "اتعمل الطلب"), StepState::Done)
            .time(Some("14:02".to_owned())),
        TimelineStep::new(t("Confirmed", "اتأكد"), StepState::Done).time(Some("14:05".to_owned())),
        TimelineStep::new(t("Preparing", "بيتجهز"), StepState::Current)
            .time(Some(t("Now", "دلوقتي"))),
        TimelineStep::new(t("On the way", "في الطريق"), StepState::Todo),
        TimelineStep::new(t("Delivered", "اتسلم"), StepState::Todo),
    ];

    // What the dev feature writes into the templates and the timeline: formatted here, never in TS.
    let fill = serde_json::json!({
        "sums": { "subtotal": m(9_900), "savings": null, "delivery": m(4_500), "free": null, "total": m(14_400) },
        "row": {
            "name": t("Cotton tote bag — Natural", "شنطة قماش قطن — طبيعي"), "detail": t("Natural · Large", "طبيعي · كبير"),
            "meta": null, "price": m(9_900), "href": "/p/tote", "img": IMG,
        },
        "advance": { "times": ["14:02", "14:05", "14:31", t("Now", "دلوقتي"), null] },
    })
    .to_string()
    .replace('<', "\\u003c");
    view! {
        <div class="core-scratch__grid" data-ts="scratch-components">
            <script type="application/json" id="scratch-fill" inner_html=fill></script>
            <section class="core-scratch__card" id="sc-totals">
                <h2>"Totals"</h2>
                <Totals rows=sums note=t("VAT included", "شامل الضريبة")/>
                <h3>"From a template (client-filled)"</h3>
                <template id="tpl-scratch-sums">
                    <Totals rows=sums_tpl note=t("VAT included", "شامل الضريبة")/>
                </template>
                <div id="sums-target"></div>
            </section>

            <section class="core-scratch__card" id="sc-rows">
                <h2>"ItemRow — Regular"</h2>
                <ItemList>
                    <ItemRow img=IMG name=t("OPPO A57 (Renewed) — Gold", "أوبو A57 (مجدد) — ذهبي") href="/p/oppo-a57-renewed"
                        detail=t("Gold · 64 GB", "ذهبي · 64 جيجا") meta=t("Only 3 left", "فاضل 3 بس") price=m(399_800)>
                        <Stepper name="qty" value=2 min=1 max=9 removable=true label=t("Quantity", "الكمية")/>
                    </ItemRow>
                    <ItemRow img=IMG name=long_a href="/p/x" price=m(5_499_900)>
                        <Stepper name="qty" value=1 min=1 max=9 removable=true label=t("Quantity", "الكمية")/>
                        <IconButton icon="trash-2" label=t("Remove", "شيل") kind=ButtonKind::Ghost/>
                    </ItemRow>
                    <ItemRow name=t("No picture, no link", "من غير صورة ولا لينك") price=m(10_000)/>
                </ItemList>
                <h2>"ItemRow — Compact"</h2>
                <ItemList size=ItemSize::Compact>
                    <ItemRow size=ItemSize::Compact img=IMG name=t("OPPO A57 (Renewed)", "أوبو A57 (مجدد)") href="/p/a"
                        meta=format!("{} × {}", format::count(2, num), m(199_900)) price=m(399_800)/>
                    <ItemRow size=ItemSize::Compact img=IMG name=long_b detail=t("Titanium · 512 GB", "تيتانيوم · 512 جيجا") price=m(5_499_900)/>
                </ItemList>
                <h3>"From a template (client-filled)"</h3>
                <template id="tpl-scratch-row">
                    <ItemRow name="" slots=true attr:data-key="">
                        <Stepper name="qty" value=1 min=1 max=9 removable=true label=t("Quantity", "الكمية")/>
                    </ItemRow>
                </template>
                <ItemList attr:id="rows-target"/>
            </section>

            <section class="core-scratch__card" id="sc-kv">
                <h2>"KeyValueList"</h2>
                <KeyValueList rows=vec![
                    KvRow::new("map-pin", t("Deliver to", "التوصيل على"), vec![
                        t("Home — Shawket Ibrahim", "البيت — شوكت إبراهيم"),
                        t("12 El-Nil St, Apt 4, Zamalek, Cairo", "12 شارع النيل، شقة 4، الزمالك، القاهرة"),
                        "010 1234 5678".to_owned(),
                    ]).action(t("Edit", "تعديل"), "/core-scratch?edit=address"),
                    KvRow::new("clock", t("Delivery time", "ميعاد التوصيل"), vec![t("Today, 6–8 pm", "النهارده، 6–8 مساءً")])
                        .action(t("Change", "تغيير"), "/core-scratch?edit=slot"),
                    KvRow::new("credit-card", t("Payment", "الدفع"), vec![t("Cash on delivery", "كاش عند الاستلام")]),
                ]/>
            </section>

            <section class="core-scratch__card" id="sc-steps">
                <h2>"StepIndicator"</h2>
                <StepIndicator label=t("Checkout steps", "خطوات إتمام الطلب") steps=steps.clone() current=2 done=vec![true, true, false, false]/>
                <StepIndicator label=t("Checkout steps", "خطوات إتمام الطلب") steps=steps current=0/>
            </section>

            <section class="core-scratch__card" id="sc-timeline">
                <h2>"Timeline"</h2>
                <Timeline steps=timeline.clone() progress=0.5 slots=true label=t("Order progress", "مراحل الطلب")/>
                <div class="core-scratch__row">
                    <Button kind=ButtonKind::Secondary size=ButtonSize::Sm attr:data-scratch-action="advance">"Advance"</Button>
                    <Button kind=ButtonKind::Secondary size=ButtonSize::Sm attr:data-scratch-action="stop">"Cancelled"</Button>
                </div>
            </section>

            <section class="core-scratch__card" id="sc-live">
                <h2>"LiveStatus"</h2>
                <ul class="core-scratch__list" role="list">
                    <li><LiveStatus/></li>
                    <li><LiveStatus state=LiveState::Retrying/></li>
                    <li><LiveStatus state=LiveState::Offline/></li>
                    <li><LiveStatus state=LiveState::Idle message=t("This order is complete.", "الطلب ده خلص.")/></li>
                </ul>
                <h3>"Driven by core/live-status.ts"</h3>
                <div id="live-driven"><LiveStatus/></div>
                <div class="core-scratch__row">
                    <Button kind=ButtonKind::Secondary size=ButtonSize::Sm attr:data-scratch-action="retry">"Fail: retry in 5 s"</Button>
                    <Button kind=ButtonKind::Secondary size=ButtonSize::Sm attr:data-scratch-action="offline">"Offline"</Button>
                    <Button kind=ButtonKind::Secondary size=ButtonSize::Sm attr:data-scratch-action="live">"Live"</Button>
                </div>
            </section>
        </div>
    }
}
