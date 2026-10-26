//! The /ui specimens for the design system's second round: notices, chips, menus and pages,
//! choices, the form additions, lines and totals, steps and progress. Each is the real component
//! in its states; the few that browser code drives (a notice revealed later, an error line filled,
//! the timeline, the live status) are moved by `kit-…` features in `client/src/features/showcase.ts`
//! with core's own helpers.

use super::{Row, Specimen};
use crate::components::button::{Button, ButtonKind, ButtonSize, IconButton};
use crate::components::choice_chips::{ChoiceChip, ChoiceChips, ChoiceKind, ChoiceState};
use crate::components::choice_list::{ChoiceList, ChoiceRow};
use crate::components::empty_state::EmptyState;
use crate::components::facet_group::FacetGroup;
use crate::components::filter_chips::{FilterChip, FilterChips};
use crate::components::form::{Checkbox, Select, TextInput};
use crate::components::item_row::{ItemList, ItemRow, ItemSize};
use crate::components::kv::{KeyValueList, KvRow};
use crate::components::link_chips::{LinkChip, LinkChips};
use crate::components::live_status::{LiveState, LiveStatus};
use crate::components::menu::{Menu, MenuAlign, MenuItem};
use crate::components::notice::{Notice, Tone};
use crate::components::pager::Pager;
use crate::components::stepper::Stepper;
use crate::components::steps::StepIndicator;
use crate::components::timeline::{StepState, Timeline, TimelineStep};
use crate::components::totals::{TotalRow, TotalTone, Totals};
use crate::format;
use crate::prefs::{Lang, Numerals, tr};
use domain::Piastres;
use leptos::prelude::*;

/// A product photo the line specimens can show (any packshot of the catalogue).
const IMG: &str = "/media/nile/p1009-1-240.webp";

#[component]
pub fn Notices(lang: Lang, num: Numerals) -> impl IntoView {
    let t = move |en: &'static str, ar: &'static str| tr(lang, en, ar);
    let d = move |en: &'static str, ar: &'static str| format::digits(tr(lang, en, ar), num);
    let tones: [(Tone, (&str, &str), (&str, &str)); 6] = [
        (Tone::Info, ("Prices include 14% VAT", "الأسعار شاملة ضريبة 14%"), ("Delivery is added at checkout.", "التوصيل بيتضاف وانت بتكمّل الطلب.")),
        (Tone::Success, ("Added to your cart", "اتضاف للسلة"), ("Galaxy S25 Ultra, 256 GB.", "جالاكسي S25 ألترا، 256 جيجا.")),
        (Tone::Warning, ("Only 3 left", "فاضل 3 بس"), ("Order soon to get it today.", "اطلب بسرعة عشان يوصلك النهارده.")),
        (Tone::Danger, ("We couldn't add that", "مقدرناش نضيف ده"), ("Your cart already has the most you can order.", "السلة فيها أقصى عدد تقدر تطلبه.")),
        (Tone::Neutral, ("Delivery times are estimates", "مواعيد التوصيل تقريبية"), ("The courier calls before arriving.", "المندوب بيكلمك قبل ما يوصل.")),
        (Tone::Brand, ("New: express delivery in Giza", "جديد: توصيل سريع في الجيزة"), ("Within two hours, every day.", "خلال ساعتين، كل يوم.")),
    ];
    view! {
        <Specimen id="notices" title=t("Notices", "التنبيهات في الصفحة") note=t("An inline message in a tone: icon, title, a line, actions. Danger and warning interrupt screen readers; the rest wait their turn.", "رسالة جوه الصفحة بلون: أيقونة وعنوان وسطر وأزرار. الخطر والتحذير بيقاطعوا قارئ الشاشة، والباقي بيستنى دوره.") card=true>
            <div class="kit__stack">
                {tones.into_iter().map(|(tone, title, body)| view! {
                    <Notice tone title=d(title.0, title.1) body=d(body.0, body.1)/>
                }).collect_view()}
                <Notice tone=Tone::Warning title=t("Today's deals didn't load", "عروض النهارده ما اتحمّلتش") body=t("Everything else on the page still works.", "باقي الصفحة شغال عادي.")>
                    <Button kind=ButtonKind::Secondary size=ButtonSize::Sm href="/ui#notices" icon="rotate-ccw">{t("Try again", "حاول تاني")}</Button>
                    <Button kind=ButtonKind::Ghost size=ButtonSize::Sm href="/">{t("Home page", "الرئيسية")}</Button>
                </Notice>
                <Notice tone=Tone::Info title=t("Title only, no line", "عنوان بس، من غير سطر")/>
            </div>
            <Row label=t("Revealed by the browser (slots)", "بيظهر من المتصفح (slots)")>
                <div class="kit__stack" data-ts="kit-notice">
                    <Button kind=ButtonKind::Secondary size=ButtonSize::Sm icon="eye" attr:data-act="reveal" attr:data-needs-js="" attr:hidden="">
                        {t("Show it", "اظهره")}
                    </Button>
                    <Notice tone=Tone::Success title="" body="" slots=true attr:hidden=true attr:data-state="added"/>
                </div>
            </Row>
        </Specimen>
    }
}

#[component]
pub fn Chips(lang: Lang, num: Numerals) -> impl IntoView {
    let t = move |en: &'static str, ar: &'static str| tr(lang, en, ar);
    let aisles = vec![
        LinkChip::new(t("Phones", "موبايلات"), "/c/phones").icon("smartphone"),
        LinkChip::new(t("Laptops", "لابتوبات"), "/c/laptops").icon("laptop"),
        LinkChip::new(t("Audio", "صوتيات"), "/c/audio").icon("headphones"),
        LinkChip::new(t("Watches", "ساعات"), "/c/watches").icon("watch"),
    ];
    let searches = vec![
        LinkChip::new("iPhone", "/search?q=iphone").icon("search"),
        LinkChip::new(t("Air fryer", "قلاية هوائية"), "/search?q=air+fryer").icon("search"),
        LinkChip::new(t("Running shoes", "جزمة جري"), "/search?q=running+shoes").icon("search"),
    ];
    let price = match lang {
        Lang::En => format!("Under {}", format::money(Piastres(500_000), lang, num)),
        Lang::Ar => format!("أقل من {}", format::money(Piastres(500_000), lang, num)),
    };
    let active = move || {
        vec![
            FilterChip::new(t("In stock", "متوفر"), "/ui#chips"),
            FilterChip::new("Samsung", "/ui#chips"),
            FilterChip::new(t("On sale", "عليه خصم"), "/ui#chips"),
        ]
    };
    let mut many = active();
    many.extend([
        FilterChip::new("Apple", "/ui#chips"),
        FilterChip::new(t("Express", "سريع"), "/ui#chips"),
        FilterChip::new(price, "/ui#chips"),
        FilterChip::new(t("4 stars and up", "4 نجوم وأكتر"), "/ui#chips"),
    ]);
    view! {
        <Specimen id="chips" title=t("Chips", "الشرائح") note=t("Link chips are ways forward (aisles, searches); filter chips are active filters, each a link that removes it.", "شرائح اللينكات طرق تكمّل بيها (أقسام، عمليات بحث)؛ وشرائح الفلاتر هي الفلاتر الشغالة، وكل واحدة لينك بيشيلها.") card=true>
            <Row label=t("Link chips, with icons", "شرائح لينكات بأيقونات")>
                <LinkChips items=aisles label=t("Popular aisles", "أقسام مشهورة")/>
            </Row>
            <Row label=t("Centred (an empty state, a 404)", "في النص (حالة فاضية، 404)")>
                <LinkChips items=searches label=t("Popular searches", "عمليات بحث مشهورة") center=true/>
            </Row>
            <Row label=t("Filter chips, with Clear all", "شرائح فلاتر، ومعاها امسح الكل")>
                <FilterChips items=active() clear_all=FilterChip::clear_all("/ui#chips") label=t("Active filters", "الفلاتر الشغالة")/>
            </Row>
            <Row label=t("Scrolling on phones", "بتتسحب على الموبايل")>
                <div class="kit__wide"><FilterChips items=many clear_all=FilterChip::clear_all("/ui#chips") label=t("Active filters", "الفلاتر الشغالة") scroll=true/></div>
            </Row>
        </Specimen>
    }
}

#[component]
pub fn MenusAndPages(lang: Lang) -> impl IntoView {
    let t = move |en: &'static str, ar: &'static str| tr(lang, en, ar);
    let sorts = move || {
        vec![
            MenuItem::new(t("Popular", "الأشهر"), "/ui?sort=popular#menus", true),
            MenuItem::new(t("Price: low to high", "السعر: من الأقل"), "/ui?sort=price-asc#menus", false),
            MenuItem::new(t("Price: high to low", "السعر: من الأعلى"), "/ui?sort=price-desc#menus", false),
            MenuItem::new(t("Newest", "الأحدث"), "/ui?sort=new#menus", false),
        ]
    };
    let href = |n: u32| format!("/ui?page={n}#menus");
    view! {
        <Specimen id="menus" title=t("Menus & pages", "القوائم والصفحات") note=t("A menu is links in a popover, its current value on the trigger. The pager fits its own width: on a narrow column it keeps the first, the current and the last page.", "القائمة لينكات في نافذة صغيرة، والقيمة الحالية على الزرار. والصفحات بتظبط نفسها على عرضها: لو العمود ضيق بتسيب الأولى والحالية والأخيرة.") card=true>
            <Row label=t("Open, with the current value", "مفتوحة، ومعاها القيمة الحالية")>
                <div class="kit__menu-room">
                    <Menu label=t("Sort", "الترتيب") value=t("Popular", "الأشهر") icon="arrow-up-down" items=sorts() attr:open=true/>
                </div>
            </Row>
            <Row label=t("End-aligned, compact", "على الطرف، صغيرة")>
                <Menu label=t("Sort", "الترتيب") value=t("Newest", "الأحدث") items=sorts() align=MenuAlign::End compact=true/>
            </Row>
            <Row label=t("First page", "الصفحة الأولى")>
                <div class="kit__wide"><Pager page=1 pages=12 href=href label=t("Pages", "الصفحات")/></div>
            </Row>
            <Row label=t("A middle page", "صفحة في النص")>
                <div class="kit__wide"><Pager page=6 pages=12 href=href label=t("Pages", "الصفحات")/></div>
            </Row>
            <Row label=t("Last page", "الصفحة الأخيرة")>
                <div class="kit__wide"><Pager page=12 pages=12 href=href label=t("Pages", "الصفحات")/></div>
            </Row>
        </Specimen>
    }
}

#[component]
pub fn Choices(lang: Lang, num: Numerals) -> impl IntoView {
    let t = move |en: &'static str, ar: &'static str| tr(lang, en, ar);
    let n = move |v: u32| format::count(v, num);
    let m = move |p: i64| format::money(Piastres(p), lang, num);
    let brands = [("Samsung", 24), ("Apple", 18), ("Xiaomi", 15), ("OPPO", 9), ("realme", 6), ("Nokia", 3)];
    let storage = vec![
        ChoiceChip::new("128 GB", "128").href("/ui?storage=128#choices").state(ChoiceState::Selected),
        ChoiceChip::new("256 GB", "256").href("/ui?storage=256#choices"),
        ChoiceChip::new("512 GB", "512").href("/ui?storage=512#choices").state(ChoiceState::Unavailable).title(t("Sold out", "خلص")),
    ];
    let colours = vec![
        ChoiceChip::new(t("Titanium Black", "تيتانيوم أسود"), "black").swatch("#2B2B2D").href("/ui?colour=black#choices").state(ChoiceState::Selected),
        ChoiceChip::new(t("Titanium Gray", "تيتانيوم رمادي"), "gray").swatch("#8E8E93").href("/ui?colour=gray#choices"),
        ChoiceChip::new(t("Titanium Silverblue", "تيتانيوم أزرق فضي"), "blue").swatch("#A9B8C9").href("/ui?colour=blue#choices"),
        ChoiceChip::new(t("Titanium Violet", "تيتانيوم بنفسجي"), "violet").swatch("#8E7CC3").href("/ui?colour=violet#choices").state(ChoiceState::Unavailable).title(t("Sold out", "خلص")),
    ];
    let cities = vec![
        ChoiceChip::new(t("Cairo", "القاهرة"), "cairo"),
        ChoiceChip::new(t("Giza", "الجيزة"), "giza"),
        ChoiceChip::new(t("Alexandria", "الإسكندرية"), "alex"),
        ChoiceChip::new(t("Aswan", "أسوان"), "aswan").state(ChoiceState::Unavailable).title(t("No delivery yet", "لسه مفيش توصيل")),
    ];
    view! {
        <Specimen id="choices" title=t("Choices", "الاختيارات") note=t("Facets filter a listing; choice chips pick one value (variant links, radios, colour swatches); choice lists are rows with a price.", "الفلاتر بتصفّي القائمة؛ وشرائح الاختيار بتختار قيمة واحدة (لينكات للأنواع، أو راديو، أو ألوان)؛ وقوائم الاختيار صفوف ومعاها سعر.") card=true>
            <div class="kit__cols">
                <FacetGroup legend=t("Brand · collapsed", "الماركة · مقفولة") show_first=3>
                    {brands.into_iter().map(|(b, c)| view! { <Checkbox name="brand" value=b.to_lowercase() label=b meta=n(c) checked={b == "Apple"}/> }).collect_view()}
                </FacetGroup>
                <FacetGroup legend=t("Brand · expanded", "الماركة · مفتوحة") show_first=3 expanded=true>
                    {brands.into_iter().map(|(b, c)| view! { <Checkbox name="brand-x" value=b.to_lowercase() label=b meta=n(c)/> }).collect_view()}
                </FacetGroup>
            </div>
            <Row label=t("Chips (variant links)", "شرائح (لينكات للأنواع)")>
                <ChoiceChips label=t("Storage", "المساحة") items=storage show_value=true note=t("512 GB is sold out in this colour.", "الـ 512 جيجا خلصت في اللون ده.")/>
            </Row>
            <Row label=t("Swatches", "ألوان")>
                <ChoiceChips label=t("Colour", "اللون") items=colours kind=ChoiceKind::Swatch show_value=true/>
            </Row>
            <Row label=t("Radios, required, with an error", "راديو، مطلوب، ومعاه خطأ")>
                <ChoiceChips label=t("City", "المدينة") items=cities name="kit-city" id="kit-city" required=true error=t("Choose a city.", "اختار مدينة.").to_owned()/>
            </Row>
            <div class="kit__cols">
                <ChoiceList legend=t("Delivery · one", "التوصيل · واحد بس") name="kit-delivery" single=true items=vec![
                    ChoiceRow::new("std", t("Standard", "عادي")).description(format::digits(t("In 2–3 days", "خلال 2–3 أيام"), num)).meta(t("Free", "مجاني")).checked(true),
                    ChoiceRow::new("exp", t("Express", "سريع")).description(t("Within two hours", "خلال ساعتين")).meta(m(4_900)),
                    ChoiceRow::new("same", t("Same day", "نفس اليوم")).meta(m(9_900)).disabled(true),
                ]/>
                <ChoiceList legend=t("Add-ons · several, invalid", "إضافات · أكتر من واحد، فيها خطأ") name="kit-addons" id="kit-addons" error=t("Pick at most two add-ons.", "اختار إضافتين بالكتير.").to_owned() items=vec![
                    ChoiceRow::new("case", t("Clear case", "جراب شفاف")).meta(m(29_900)).checked(true),
                    ChoiceRow::new("glass", t("Screen glass", "سكرينة")).meta(m(19_900)).checked(true),
                    ChoiceRow::new("care", t("2-year care", "ضمان سنتين")).meta(m(149_900)).checked(true),
                ]/>
            </div>
        </Specimen>
    }
}

/// The form additions, appended to the Forms section.
#[component]
pub fn FormGaps(lang: Lang, num: Numerals) -> impl IntoView {
    let t = move |en: &'static str, ar: &'static str| tr(lang, en, ar);
    let o = move |v: &str, en: &'static str, ar: &'static str| (v.to_owned(), t(en, ar).to_owned());
    view! {
        <div class="kit__form">
            <TextInput id="kit-code" name="code" label=format::digits(t("Promo code (4–12 letters)", "كود الخصم (4–12 حرف)"), num) minlength=4 maxlength=12 dir="ltr"/>
            <Select
                id="kit-area"
                name="area"
                label=t("Area", "المنطقة")
                required=true
                groups=vec![
                    (t("Cairo", "القاهرة").to_owned(), vec![o("zamalek", "Zamalek", "الزمالك"), o("maadi", "Maadi", "المعادي"), o("nasr", "Nasr City", "مدينة نصر")]),
                    (t("Giza", "الجيزة").to_owned(), vec![o("dokki", "Dokki", "الدقي"), o("october", "6th of October", "6 أكتوبر")]),
                ]
            />
            // The error line is always there (empty), so filling it moves nothing: "Check" fills it.
            <div data-ts="kit-check" class="kit__stack">
                <TextInput id="kit-street" name="street" label=t("Street and building", "الشارع والعمارة") autocomplete="address-line1"/>
                <Button kind=ButtonKind::Secondary size=ButtonSize::Sm icon="check" attr:data-act="check" attr:data-needs-js="" attr:hidden="">{t("Check", "اتأكد")}</Button>
            </div>
        </div>
    }
}

/// An empty search with "Did you mean", appended to the States section.
#[component]
pub fn EmptySuggestion(lang: Lang) -> impl IntoView {
    let t = move |en: &'static str, ar: &'static str| tr(lang, en, ar);
    view! {
        <div class="kit__state">
            <p class="kit__label">{t("Empty · did you mean", "فاضي · قصدك")}</p>
            <EmptyState
                art="empty-search"
                title=t("No results for “galaxi”", "مفيش نتايج لـ «galaxi»")
                body=t("Check the spelling, or try fewer words.", "اتأكد من الكتابة، أو جرّب كلمات أقل.")
                suggestion=("/search?q=galaxy".to_owned(), "galaxy".to_owned())
                compact=true
            />
        </div>
    }
}

#[component]
pub fn Lines(lang: Lang, num: Numerals) -> impl IntoView {
    let t = move |en: &'static str, ar: &'static str| tr(lang, en, ar).to_owned();
    let m = move |p: i64| format::money(Piastres(p), lang, num);
    let sums = move || {
        vec![
            TotalRow::new(t("Subtotal", "المجموع"), m(1_239_700)),
            TotalRow::new(t("Savings", "التوفير"), format!("−{}", m(150_000))).tone(TotalTone::Saving),
            TotalRow::new(t("Delivery", "التوصيل"), m(4_500)).hidden(true),
            TotalRow::new(t("Delivery", "التوصيل"), t("Free", "مجاني")).tone(TotalTone::Free),
            TotalRow::new(t("Total", "الإجمالي"), m(1_089_700)).tone(TotalTone::Total),
        ]
    };
    let long_name = t(
        "Samsung Galaxy S25 Ultra 5G, 12 GB RAM, 512 GB, Titanium Silverblue, with the S Pen",
        "سامسونج جالاكسي S25 ألترا 5G، رام 12 جيجا، 512 جيجا، أزرق تيتانيوم فضي، ومعاه القلم",
    );
    let phone = format::digits("010 1234 5678", num);
    let (long_a, long_b) = (long_name.clone(), long_name);
    view! {
        <Specimen id="lines" title=t("Lines & totals", "السطور والإجماليات") note=t("A product line (picture, name, detail, price, controls), label–amount totals, and key–value blocks with an Edit action.", "سطر المنتج (صورة واسم وتفاصيل وسعر وأزرار)، وإجماليات اسم وقيمة، وبلوكات معلومة ومعاها تعديل.") card=true>
            <div class="kit__cols">
                <div class="kit__stack">
                    <p class="kit__label">{t("Totals: saving, free, total; a hidden row", "إجماليات: توفير ومجاني وإجمالي؛ وسطر مخفي")}</p>
                    <Totals rows=sums() note=t("VAT included", "شامل الضريبة")/>
                </div>
                <div class="kit__stack">
                    <p class="kit__label">{t("Updating (aria-busy dims it)", "بيتحدّث (aria-busy بيبهّته)")}</p>
                    <Totals rows=sums() note=t("VAT included", "شامل الضريبة") attr:aria-busy="true"/>
                </div>
            </div>
            <p class="kit__label">{t("Item rows · regular: controls on their own row", "سطور منتجات · عادي: الأزرار في سطر لوحدها")}</p>
            <ItemList>
                <ItemRow img=IMG name=t("OPPO A57 (Renewed) — Gold", "أوبو A57 (مجدد) — ذهبي") href="/ui#lines" detail=format::digits(&t("Gold · 64 GB", "ذهبي · 64 جيجا"), num) meta=format::digits(&t("Only 3 left", "فاضل 3 بس"), num) price=m(399_800)>
                    <Stepper name="kit-line-a" value=2u32 min=1 max=9 removable=true/>
                </ItemRow>
                <ItemRow img=IMG name=long_a href="/ui#lines" price=m(129_999_900)>
                    <Stepper name="kit-line-b" value=1u32 min=1 max=9 removable=true/>
                    <IconButton icon="trash-2" label=t("Remove", "شيل") kind=ButtonKind::Ghost/>
                </ItemRow>
                <ItemRow name=t("No picture, no link", "من غير صورة ولا لينك") price=m(10_000)/>
            </ItemList>
            <p class="kit__label">{t("Item rows · compact: one row (checkout, order)", "سطور منتجات · صغيرة: سطر واحد (الدفع، الطلب)")}</p>
            <ItemList size=ItemSize::Compact>
                <ItemRow size=ItemSize::Compact img=IMG name=t("OPPO A57 (Renewed)", "أوبو A57 (مجدد)") href="/ui#lines" meta=format!("{} × {}", format::count(2, num), m(199_900)) price=m(399_800)/>
                <ItemRow size=ItemSize::Compact img=IMG name=long_b detail=t("Titanium · 512 GB", "تيتانيوم · 512 جيجا") price=m(129_999_900)/>
            </ItemList>
            <p class="kit__label">{t("Key–value list, with Edit", "قايمة معلومات، ومعاها تعديل")}</p>
            <KeyValueList rows=vec![
                KvRow::new("map-pin", t("Deliver to", "التوصيل على"), vec![
                    t("Home — Mona Adel", "البيت — منى عادل"),
                    t("12 El-Nil St, Apt 4, Zamalek, Cairo", "12 شارع النيل، شقة 4، الزمالك، القاهرة"),
                    phone,
                ]).action(t("Edit", "تعديل"), "/ui#lines"),
                KvRow::new("clock", t("Delivery time", "ميعاد التوصيل"), vec![format::digits(&t("Today, 6–8 pm", "النهارده، 6–8 بالليل"), num)]).action(t("Change", "تغيير"), "/ui#lines"),
                KvRow::new("credit-card", t("Payment", "الدفع"), vec![t("Cash on delivery", "كاش عند الاستلام")]),
            ]/>
        </Specimen>
    }
}

#[component]
pub fn Progress(lang: Lang, num: Numerals) -> impl IntoView {
    let t = move |en: &'static str, ar: &'static str| tr(lang, en, ar).to_owned();
    let steps = move || {
        vec![
            (t("Address", "العنوان"), Some("/ui?step=1#progress".to_owned())),
            (t("Delivery", "التوصيل"), Some("/ui?step=2#progress".to_owned())),
            (t("Payment", "الدفع"), Some("/ui?step=3#progress".to_owned())),
            (t("Review", "المراجعة"), None),
        ]
    };
    let clock = |h: u32, m: u32| format::digits(&format!("{h:02}:{m:02}"), num);
    let stages = vec![
        TimelineStep::new(t("Placed", "اتعمل الطلب"), StepState::Done).time(Some(clock(14, 2))),
        TimelineStep::new(t("Confirmed", "اتأكد"), StepState::Done).time(Some(clock(14, 5))),
        TimelineStep::new(t("Preparing", "بيتجهز"), StepState::Current).time(Some(t("Now", "دلوقتي"))),
        TimelineStep::new(t("On the way", "في الطريق"), StepState::Todo),
        TimelineStep::new(t("Delivered", "اتسلم"), StepState::Todo),
    ];
    // What the timeline demo writes (formatted here, never in TS): each stage's time as it lands.
    let fill = serde_json::json!({
        "times": [clock(14, 2), clock(14, 5), clock(14, 31), clock(15, 2), clock(15, 40)],
        "now": t("Now", "دلوقتي"),
    })
    .to_string()
    .replace('<', "\\u003c");
    view! {
        <Specimen id="progress" title=t("Steps & progress", "الخطوات والتقدّم") note=t("Numbered steps on a filling track (finished steps link back), the order timeline, and the state of a region that polls.", "خطوات مترقّمة على خط بيتملى (الخطوات اللي خلصت بترجعلها)، ومراحل الطلب، وحالة الجزء اللي بيتحدّث لوحده.") card=true>
            <p class="kit__label">{t("First step", "أول خطوة")}</p>
            <StepIndicator label=t("Checkout steps", "خطوات إتمام الطلب") steps=steps() current=0/>
            <p class="kit__label">{t("A middle step", "خطوة في النص")}</p>
            <StepIndicator label=t("Checkout steps", "خطوات إتمام الطلب") steps=steps() current=2 done=vec![true, true, false, false]/>
            <p class="kit__label">{t("Last step", "آخر خطوة")}</p>
            <StepIndicator label=t("Checkout steps", "خطوات إتمام الطلب") steps=steps() current=3 done=vec![true, true, true, false]/>
            <div class="kit__cols">
                <div class="kit__stack" data-ts="kit-timeline">
                    <script type="application/json" data-ts-json="" inner_html=fill></script>
                    <p class="kit__label">{t("Timeline, live (core/timeline.ts)", "مراحل الطلب، حيّة (core/timeline.ts)")}</p>
                    <Timeline steps=stages progress=0.5 slots=true label=t("Order progress", "مراحل الطلب")/>
                    <p class="kit__items" data-needs-js="" hidden>
                        <Button kind=ButtonKind::Secondary size=ButtonSize::Sm icon="arrow-right" attr:data-act="advance">{t("Advance", "قدّم")}</Button>
                        <Button kind=ButtonKind::Outline size=ButtonSize::Sm icon="x" attr:data-act="stop">{t("Cancelled", "اتلغى")}</Button>
                        <Button kind=ButtonKind::Ghost size=ButtonSize::Sm icon="rotate-ccw" attr:data-act="reset">{t("Reset", "من الأول")}</Button>
                    </p>
                </div>
                <div class="kit__stack">
                    <p class="kit__label">{t("Live status: live, retrying, offline, idle", "حالة التحديث: شغال، بيحاول، من غير نت، واقف")}</p>
                    <LiveStatus/>
                    <LiveStatus state=LiveState::Retrying/>
                    <LiveStatus state=LiveState::Offline/>
                    <LiveStatus state=LiveState::Idle message=t("This order is complete.", "الطلب ده خلص.")/>
                    <div class="kit__stack" data-ts="kit-live">
                        <p class="kit__label">{t("Driven by core/live-status.ts", "بيتحرك من core/live-status.ts")}</p>
                        <LiveStatus/>
                        <p class="kit__items" data-needs-js="" hidden>
                            <Button kind=ButtonKind::Secondary size=ButtonSize::Sm icon="timer" attr:data-act="retry">{format::digits(&t("Fail: retry in 5 s", "فشل: هيحاول بعد 5 ثواني"), num)}</Button>
                            <Button kind=ButtonKind::Outline size=ButtonSize::Sm icon="wifi-off" attr:data-act="offline">{t("Offline", "من غير نت")}</Button>
                            <Button kind=ButtonKind::Ghost size=ButtonSize::Sm icon="wifi" attr:data-act="live">{t("Live", "شغال")}</Button>
                        </p>
                    </div>
                </div>
            </div>
        </Specimen>
    }
}
