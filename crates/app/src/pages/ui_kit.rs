//! `/ui`: the design system's own page. Every component in every state, in the visitor's language
//! and theme, with real catalogue data; it doubles as the visual test bench for the design system.
//!
//! Server-rendered like every page. The interactive specimens use the app's real browser code:
//! core's drawer and stepper (`data-ts="drawer"`, `"stepper"`), the toast store, and small `kit-…`
//! features in `client/src/features/showcase.ts` (toasts, the line total, the skeleton replay).

mod parts;

use crate::components::badge::{Badge, BadgeKind};
use crate::components::breadcrumbs::Breadcrumbs;
use crate::components::button::{Button, ButtonKind, ButtonSize, IconButton};
use crate::components::drawer::{Drawer, DrawerSide};
use crate::components::form::{Checkbox, RadioCard, RadioGroup, Select, TextArea, TextInput};
use crate::components::icons::{Icon, NAMES};
use crate::components::image::Picture;
use crate::components::layout::{Container, Rail, SectionHeader};
use crate::components::price::{Price, PriceSize};
use crate::components::product_card::{CardData, ProductCard, ProductCardSkeleton, ProductGrid};
use crate::components::rating::Rating;
use crate::components::segmented::{SegItem, Segmented, SegmentedToggle};
use crate::components::skeleton::{
    Skeleton, SkeletonField, SkeletonLines, SkeletonMedia, SkeletonRail,
};
use crate::components::states::{ComingSoon, SectionError, StateTone, StateView};
use crate::components::status_pill::{StatusPill, Tone};
use crate::components::stepper::Stepper;
use crate::format;
use crate::media::Img;
use crate::prefs::{Lang, Numerals, tr};
use domain::Piastres;
use leptos::prelude::*;
use leptos_meta::Title;

/// Real cards from the tenant's catalogue (server only; the page never renders in the browser).
fn sample_cards(n: usize) -> Vec<CardData> {
    #[cfg(feature = "ssr")]
    {
        if use_context::<crate::server::AppState>().is_none() {
            return Vec::new();
        }
        let cat = crate::server::current_catalog();
        let mut cards = cat.rail(crate::api::types::Rail::Deals, n / 2);
        cards.extend(cat.rail(crate::api::types::Rail::Featured, n));
        let mut seen = std::collections::HashSet::new();
        cards.retain(|c| seen.insert(c.id));
        // One sold-out card, so its state is on the page too.
        if let Some(oos) = cat
            .products()
            .iter()
            .find(|p| p.total_stock() == 0)
            .and_then(|p| cat.card(p.id))
        {
            if seen.insert(oos.id) {
                cards.insert(2.min(cards.len()), oos.clone());
            }
        }
        cards.truncate(n);
        cards
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = n;
        Vec::new()
    }
}

/// A block of the kit: a heading, a line on what it is for, and the specimen surface.
#[component]
fn Specimen(
    #[prop(into)] id: String,
    #[prop(into)] title: String,
    #[prop(into)] note: String,
    /// Show the specimens on a card instead of the bare page.
    #[prop(optional)]
    card: bool,
    children: Children,
) -> impl IntoView {
    let head_id = format!("{id}-title");
    let body = if card {
        "kit__body kit__body--card"
    } else {
        "kit__body"
    };
    view! {
        <section class="kit__section" id=id aria-labelledby=head_id.clone()>
            <SectionHeader title=title subtitle=note id=head_id.clone()/>
            <div class=body>{children()}</div>
        </section>
    }
}

/// A labelled row of specimens.
#[component]
fn Row(#[prop(into)] label: String, children: Children) -> impl IntoView {
    view! {
        <div class="kit__row">
            <p class="kit__label">{label}</p>
            <div class="kit__items">{children()}</div>
        </div>
    }
}

/// A skeleton beside what replaces it (children: the loading view, then the loaded one), each
/// under its label; `stack` puts them one above the other (full-width things like rails).
#[component]
fn KitPair(
    #[prop(into)] title: String,
    #[prop(into)] loading: String,
    #[prop(into)] loaded: String,
    #[prop(optional)] stack: bool,
    children: ChildrenFragment,
) -> impl IntoView {
    let mut sides = children().nodes.into_iter();
    let first = sides.next();
    let second = sides.next();
    view! {
        <div class="kit__pair" class:kit__pair--stack=stack>
            <p class="kit__pair-title">{title}</p>
            <div class="kit__pair-side">
                <p class="kit__label">{loading}</p>
                <div class="kit__pair-body" data-side="loading">{first}</div>
            </div>
            <div class="kit__pair-side">
                <p class="kit__label">{loaded}</p>
                <div class="kit__pair-body" data-side="loaded">{second}</div>
            </div>
        </div>
    }
}

#[component]
pub fn UiKitPage() -> impl IntoView {
    let rq = crate::request::req();
    let lang = rq.prefs.lang;
    let num = rq.prefs.numerals;
    let t = move |en: &'static str, ar: &'static str| tr(lang, en, ar);
    let cards = sample_cards(10);
    let rail_cards = cards.clone();
    let pair_card = cards.first().cloned();
    let pair_rail: Vec<CardData> = cards.iter().take(6).cloned().collect();
    let first_img = cards
        .first()
        .map(|c| c.img.clone())
        .unwrap_or_else(|| Img::missing("x"));
    let pair_img = first_img.clone();
    let photo = crate::components::assets::media_img("brand/og-nile-en")
        .unwrap_or_else(|| Img::missing("og"));

    let swatches: [(&str, &str); 24] = [
        ("--page", "page"),
        ("--surface", "surface"),
        ("--sunk", "sunk"),
        ("--well", "well"),
        ("--text", "text"),
        ("--text-2", "text-2"),
        ("--text-3", "text-3"),
        ("--rule", "rule"),
        ("--brand", "brand"),
        ("--on-brand", "on-brand"),
        ("--ink", "ink"),
        ("--on-ink", "on-ink"),
        ("--success", "success"),
        ("--success-wash", "success-wash"),
        ("--warning", "warning"),
        ("--warning-wash", "warning-wash"),
        ("--danger", "danger"),
        ("--danger-wash", "danger-wash"),
        ("--info", "info"),
        ("--info-wash", "info-wash"),
        ("--line", "line"),
        ("--line-strong", "line-strong"),
        ("--star", "star"),
        ("--focus", "focus"),
    ];
    let radii: [(&str, &str); 6] = [
        ("--r-xs", "6"),
        ("--r-sm", "8"),
        ("--r-md", "12"),
        ("--r-lg", "16"),
        ("--r-xl", "20"),
        ("--r-2xl", "26"),
    ];
    let springs: [(&str, &str); 4] = [
        ("default", "damping 1 · response .35s · 510ms"),
        ("snappy", "damping 1 · response .25s · 370ms"),
        ("gentle", "damping 1 · response .5s · 730ms"),
        ("bouncy", "damping .8 · response .3s · 400ms"),
    ];
    let next = "/ui";
    let lang_items = vec![
        SegItem::new(
            "English",
            crate::components::prefs_controls::prefs_href("lang", "en", next),
            lang == Lang::En,
        )
        .lang("en"),
        SegItem::new(
            "العربية",
            crate::components::prefs_controls::prefs_href("lang", "ar", next),
            lang == Lang::Ar,
        )
        .lang("ar"),
    ];
    let num_items = vec![
        SegItem::new(
            "123",
            crate::components::prefs_controls::prefs_href("num", "latn", next),
            num == Numerals::Latn,
        ),
        SegItem::new(
            "١٢٣",
            crate::components::prefs_controls::prefs_href("num", "arab", next),
            num == Numerals::Arab,
        ),
    ];
    let theme = rq.prefs.theme;
    let theme_items = vec![
        SegItem::new(
            t("Light", "فاتح"),
            crate::components::prefs_controls::prefs_href("theme", "light", next),
            theme == crate::prefs::Theme::Light,
        )
        .icon("sun"),
        SegItem::new(
            t("Dark", "داكن"),
            crate::components::prefs_controls::prefs_href("theme", "dark", next),
            theme == crate::prefs::Theme::Dark,
        )
        .icon("moon"),
        SegItem::new(
            t("Auto", "تلقائي"),
            crate::components::prefs_controls::prefs_href("theme", "system", next),
            theme == crate::prefs::Theme::System,
        )
        .icon("monitor"),
    ];
    let sort_items = vec![
        SegItem::new(t("Popular", "الأشهر"), "/ui?sort=popular#segmented", true),
        SegItem::new(t("Price", "السعر"), "/ui?sort=price#segmented", false),
        SegItem::new(t("Newest", "الأحدث"), "/ui?sort=new#segmented", false),
    ];
    let view_items = vec![
        SegItem::new(t("Grid", "شبكة"), "/ui?view=grid#segmented", true).icon("layout-grid"),
        SegItem::new(t("List", "قائمة"), "/ui?view=list#segmented", false).icon("list"),
    ];

    view! {
        <Title text=t("Design system", "نظام التصميم")/>
        <Container class="kit">
            <header class="kit__intro">
                <p class="t-kicker">{t("Storefront design system", "نظام تصميم المتجر")}</p>
                <h1 class="t-display kit__title">{t("Porcelain & graphite", "بورسلين وجرافيت")}</h1>
                <p class="t-lead kit__lead">
                    {t(
                        "One palette for every shop. Continuous corners, springs that start from where things are, text set in Cairo and figures in Plex Mono. Switch language, numerals and theme here — every specimen below follows.",
                        "باليتة واحدة لكل المحلات. أركان ناعمة، حركة طبيعية بتبدأ من مكان العنصر، الخط كايرو والأرقام بلكس مونو. غيّر اللغة والأرقام والمظهر من هنا، وكل حاجة تحت هتتغير معاك.",
                    )}
                </p>
                <div class="kit__switches">
                    <Segmented items=lang_items label=t("Language", "اللغة")/>
                    <Segmented items=num_items label=t("Numbers", "الأرقام")/>
                    <Segmented items=theme_items label=t("Appearance", "المظهر")/>
                </div>
                <nav class="kit__toc" aria-label=t("On this page", "في الصفحة دي")>
                    {[
                        ("colour", t("Colour", "الألوان")), ("type", t("Type", "الخط")), ("shape", t("Shape & motion", "الشكل والحركة")),
                        ("buttons", t("Buttons", "الأزرار")), ("segmented", t("Segmented", "مفاتيح")), ("labels", t("Badges & status", "شارات وحالات")),
                        ("price", t("Price & rating", "السعر والتقييم")), ("picture", t("Pictures", "الصور")), ("cards", t("Product cards", "كروت المنتجات")),
                        ("overlays", t("Drawers & toasts", "الأدراج والتنبيهات")), ("forms", t("Forms", "النماذج")), ("states", t("States", "الحالات")),
                        ("network", t("Network", "الشبكة")), ("skeletons", t("Skeletons", "هياكل التحميل")), ("icons", t("Icons", "الأيقونات")),
                        ("notices", t("Notices", "التنبيهات في الصفحة")), ("chips", t("Chips", "الشرائح")), ("menus", t("Menus & pages", "القوائم والصفحات")),
                        ("choices", t("Choices", "الاختيارات")), ("lines", t("Lines & totals", "السطور والإجماليات")), ("progress", t("Steps & progress", "الخطوات والتقدّم")),
                    ]
                        .into_iter()
                        .map(|(id, label)| view! { <a class="kit__toc-link" href=format!("#{id}") target="_self">{label}</a> })
                        .collect_view()}
                </nav>
            </header>

            <Specimen id="colour" title=t("Colour", "الألوان") note=t("Porcelain neutrals and a graphite accent, the same for every shop. Colour comes from product photos and status.", "ألوان محايدة وأسود جرافيت، نفس الباليتة لكل المحلات. الألوان الحقيقية بتيجي من صور المنتجات والحالات.")>
                <ul class="kit__swatches" role="list">
                    {swatches
                        .into_iter()
                        .map(|(var, name)| view! {
                            <li class="kit__swatch">
                                <span class="kit__chip" style=format!("background:var({var})")></span>
                                <span class="kit__swatch-name mono">{name}</span>
                            </li>
                        })
                        .collect_view()}
                </ul>
            </Specimen>

            <Specimen id="type" title=t("Type", "الخط") note=t("Cairo for words in both scripts — tracking tightens as text grows, never in Arabic. Plex Mono for figures.", "كايرو للكلام باللغتين، والمسافات بين الحروف بتضيق مع الكبر (مش في العربي). بلكس مونو للأرقام.") card=true>
                <div class="kit__type">
                    <p class="t-display">{t("Everything, delivered", "كل حاجة توصلك")}</p>
                    <p class="t-h1">{t("Today's deals", "عروض النهارده")}</p>
                    <p class="t-h2">{t("Picked for you", "اخترنالك")}</p>
                    <p class="t-h3">{t("Delivery and returns", "التوصيل والاسترجاع")}</p>
                    <p class="t-lead">{t("Genuine products, delivered across Egypt — pay cash on delivery.", "منتجات أصلية، توصيل لكل مصر، والدفع عند الاستلام.")}</p>
                    <p>{t("Body text sits at 16px with open leading; Arabic gets a little more room between lines than Latin.", "النص الأساسي ١٦ بكسل بمسافة سطور مريحة، والعربي بياخد مسافة أكبر شوية من الإنجليزي.")}</p>
                    <p class="t-small t-muted">{t("Secondary text for details and hints.", "نص ثانوي للتفاصيل والملاحظات.")}</p>
                    <p class="t-kicker">{t("Kicker label", "عنوان صغير")}</p>
                    <p class="kit__figures mono">"1,234,567.89 · NIL-48213 · ٠١٢٣٤٥٦٧٨٩"</p>
                </div>
            </Specimen>

            <Specimen id="shape" title=t("Shape & motion", "الشكل والحركة") note=t("Continuous corners (squircles), no pills except true circles. Springs, not fixed curves: press any tile.", "أركان متصلة (سكويركل)، ومفيش كبسولات غير الدواير. الحركة سبرينج مش منحنى ثابت — اضغط على أي مربع.")>
                <div class="kit__shapes">
                    {radii
                        .into_iter()
                        .map(|(var, px)| view! {
                            <div class="kit__shape" style=format!("border-radius:var({var})") data-press="">
                                <span class="mono">{px}</span>
                            </div>
                        })
                        .collect_view()}
                    <div class="kit__shape kit__shape--round is-round" data-press="">
                        <span class="mono">"◯"</span>
                    </div>
                </div>
                <ul class="kit__springs" role="list">
                    {springs
                        .into_iter()
                        .map(|(name, spec)| view! {
                            <li class="kit__spring" style=format!("--s:var(--spring-{name});--d:var(--spring-{name}-dur)")>
                                <span class="kit__spring-track"><span class="kit__spring-dot"></span></span>
                                <span class="kit__spring-name mono">{format!("--spring-{name}")}</span>
                                <span class="kit__spring-spec">{spec}</span>
                            </li>
                        })
                        .collect_view()}
                </ul>
            </Specimen>

            <Specimen id="buttons" title=t("Buttons", "الأزرار") note=t("Graphite is the one primary per screen. Labels sit optically centred; press responds on touch-down.", "الأسود الجرافيت هو الزرار الأساسي الوحيد في الشاشة. الكلام في نص الزرار بالظبط، والضغط بيستجيب من أول لمسة.") card=true>
                <Row label=t("Kinds", "الأنواع")>
                    <Button kind=ButtonKind::Primary>{t("Add to cart", "أضف للسلة")}</Button>
                    <Button kind=ButtonKind::Secondary>{t("Save for later", "احفظه لبعدين")}</Button>
                    <Button kind=ButtonKind::Outline>{t("Compare", "قارن")}</Button>
                    <Button kind=ButtonKind::Ghost>{t("Cancel", "إلغاء")}</Button>
                    <Button kind=ButtonKind::Danger>{t("Remove", "شيل")}</Button>
                </Row>
                <Row label=t("Sizes", "المقاسات")>
                    <Button size=ButtonSize::Sm>{t("Small", "صغير")}</Button>
                    <Button size=ButtonSize::Md>{t("Medium", "متوسط")}</Button>
                    <Button size=ButtonSize::Lg>{t("Checkout", "إتمام الطلب")}</Button>
                </Row>
                <Row label=t("With icons", "بأيقونات")>
                    <Button icon="shopping-bag">{t("Add to cart", "أضف للسلة")}</Button>
                    <Button kind=ButtonKind::Outline icon_end="arrow-right" href="/ui#buttons">{t("Continue", "كمّل")}</Button>
                    <Button kind=ButtonKind::Secondary icon="heart">{t("Wishlist", "المفضلة")}</Button>
                </Row>
                <Row label=t("States", "الحالات")>
                    <Button loading=true>{t("Placing order", "جاري تأكيد الطلب")}</Button>
                    <Button disabled=true>{t("Sold out", "خلصت")}</Button>
                    <Button kind=ButtonKind::Outline disabled=true>{t("Unavailable", "مش متاح")}</Button>
                </Row>
                <Row label=t("Full width", "بعرض كامل")>
                    <Button size=ButtonSize::Lg full_width=true icon="credit-card">{t("Pay EGP 1,299.00", "ادفع 1,299.00 ج.م")}</Button>
                </Row>
                <Row label=t("Icon buttons (true circles)", "أزرار أيقونات (دواير)")>
                    <IconButton icon="search" label=t("Search", "بحث")/>
                    <IconButton icon="heart" label=t("Save", "حفظ") kind=ButtonKind::Ghost/>
                    <IconButton icon="plus" label=t("Add", "إضافة") kind=ButtonKind::Primary/>
                    <IconButton icon="trash-2" label=t("Remove", "حذف") kind=ButtonKind::Danger/>
                    <IconButton icon="shopping-bag" label=t("Cart, 3 items", "السلة، ٣ منتجات")>
                        <span class="icon-btn__badge mono">{crate::format::count(3, num)}</span>
                    </IconButton>
                    <IconButton icon="chevron-right" label=t("Next", "التالي")/>
                    <IconButton icon="x" label=t("Close", "إغلاق") disabled=true/>
                </Row>
            </Specimen>

            <Specimen id="segmented" title=t("Segmented & chips", "المفاتيح والشرائح") note=t("Links, not toggles: each option is a page, the current one raised. 32px segments, 36px chips — both 44px to a finger.", "روابط مش مفاتيح: كل اختيار صفحة، والحالي مرفوع. الشرائح ٣٢ بكسل والتصنيفات ٣٦ — وكلها ٤٤ للمس.") card=true>
                <Row label=t("Sort", "الترتيب")>
                    <Segmented items=sort_items label=t("Sort by", "رتّب حسب")/>
                </Row>
                <Row label=t("View", "العرض")>
                    <Segmented items=view_items label=t("View", "العرض")/>
                </Row>
                <Row label=t("Toggles (state, not pages)", "مفاتيح (حالة مش صفحات)")>
                    <SegmentedToggle
                        items=vec![
                            SegItem::toggle(t("Grid", "شبكة"), "grid", true).icon("layout-grid"),
                            SegItem::toggle(t("List", "قائمة"), "list", false).icon("list"),
                        ]
                        label=t("Show", "اعرض")
                    />
                    <SegmentedToggle
                        items=vec![
                            SegItem::toggle(t("In stock", "متوفر"), "stock", true),
                            SegItem::toggle(t("On sale", "عليه خصم"), "deal", false),
                            SegItem::toggle(t("Express", "سريع"), "express", true),
                        ]
                        label=t("Filters", "الفلاتر")
                        multiple=true
                    />
                </Row>
                <Row label=t("A link that needs a full load", "لينك محتاج تحميل كامل")>
                    <Segmented
                        items=vec![
                            SegItem::new(t("Tour", "الجولة"), "/showcase", false),
                            SegItem::new(t("Streaming demo", "ديمو البث"), "/showcase/stream", false).attr("data-router", "off"),
                        ]
                        label=t("Showcase", "العرض")
                    />
                </Row>
                <Row label=t("Category chips", "شرائح الأقسام")>
                    <a class="cat-nav__link" href="/ui#segmented" aria-current="false" data-press=""><Icon name="smartphone" size=16/><span class="cat-nav__label">{t("Phones", "موبايلات")}</span></a>
                    <a class="cat-nav__link" href="/ui#segmented" aria-current="page" data-press=""><Icon name="headphones" size=16/><span class="cat-nav__label">{t("Audio", "صوتيات")}</span></a>
                    <a class="cat-nav__link" href="/ui#segmented" aria-current="false" data-press=""><Icon name="watch" size=16/><span class="cat-nav__label">{t("Watches", "ساعات")}</span></a>
                </Row>
            </Specimen>

            <parts::Chips lang num/>
            <parts::MenusAndPages lang/>

            <Specimen id="labels" title=t("Badges & status", "الشارات والحالات") note=t("Badges sit on photos, so they are solid. Status labels: icon + word on the tone's wash, 6px corners.", "الشارات فوق الصور، فلازم تكون مصمتة. الحالات: أيقونة وكلمة على لون فاتح من نفس الدرجة.") card=true>
                <Row label=t("Merchandising", "تسويقية")>
                    <Badge kind=BadgeKind::Deal(20) lang=lang numerals=num/>
                    <Badge kind=BadgeKind::Express lang=lang numerals=num/>
                    <Badge kind=BadgeKind::New lang=lang numerals=num/>
                    <Badge kind=BadgeKind::Bestseller lang=lang numerals=num/>
                    <Badge kind=BadgeKind::LowStock(3) lang=lang numerals=num/>
                    <Badge kind=BadgeKind::OutOfStock lang=lang numerals=num/>
                </Row>
                <Row label=t("Order status", "حالة الطلب")>
                    <StatusPill tone=Tone::Neutral label=t("Received", "تم الاستلام")/>
                    <StatusPill tone=Tone::Info label=t("Preparing", "بنجهزه") icon="package"/>
                    <StatusPill tone=Tone::Brand label=t("On the way", "في الطريق") icon="truck"/>
                    <StatusPill tone=Tone::Success label=t("Delivered", "اتسلم")/>
                    <StatusPill tone=Tone::Warning label=t("Delayed", "متأخر")/>
                    <StatusPill tone=Tone::Danger label=t("Cancelled", "اتلغى")/>
                </Row>
            </Specimen>

            <Specimen id="price" title=t("Price & rating", "السعر والتقييم") note=t("Money in ink; a sale is red with what it was beside it. Stars fill to the tenth, from the reading start.", "الفلوس باللون الأسود؛ الخصم بالأحمر وجنبه السعر القديم. النجوم بتتملي بالعُشر من بداية القراية.") card=true>
                <Row label=t("Sizes", "المقاسات")>
                    <Price amount=Piastres(129_900) size=PriceSize::Sm lang=lang numerals=num/>
                    <Price amount=Piastres(129_900) size=PriceSize::Md lang=lang numerals=num/>
                    <Price amount=Piastres(129_900) size=PriceSize::Lg lang=lang numerals=num/>
                    <Price amount=Piastres(6_499_900) size=PriceSize::Xl lang=lang numerals=num/>
                </Row>
                <Row label=t("Sale, from, refund", "خصم، يبدأ من، استرداد")>
                    <Price amount=Piastres(99_900) compare_at=Piastres(139_900) lang=lang numerals=num/>
                    <Price amount=Piastres(6_499_900) compare_at=Piastres(7_199_900) size=PriceSize::Xl lang=lang numerals=num/>
                    <Price amount=Piastres(74_99) from=true lang=lang numerals=num/>
                    <Price amount=Piastres(-5_000) size=PriceSize::Lg lang=lang numerals=num/>
                </Row>
                <Row label=t("Ratings", "التقييمات")>
                    <Rating value=4.6 count=312 lang=lang numerals=num/>
                    <Rating value=3.5 count=18 lang=lang numerals=num/>
                    <Rating value=5.0 count=1240 lang=lang numerals=num/>
                    <Rating value=4.2 count=176 lang=lang numerals=num compact=true/>
                    <Rating value=0.0 count=0 lang=lang numerals=num/>
                </Row>
            </Specimen>

            <Specimen id="picture" title=t("Pictures", "الصور") note=t("The ratio is reserved before a byte arrives; a blurred placeholder holds the place. Packshots rest on the well — with a rim light in dark mode.", "مساحة الصورة محجوزة قبل ما توصل، وفيه صورة مبهمة مكانها لحد ما تحمّل. صور المنتجات على خلفية هادية، وفي الوضع الداكن عليها إضاءة خفيفة.")>
                <div class="kit__pics">
                    <figure class="kit__pic">
                        <Picture img=first_img.clone() sizes="(min-width: 960px) 280px, 45vw" lang=lang class="kit__pic-box"/>
                        <figcaption class="t-caption">{t("Transparent packshot on the well", "منتج بخلفية شفافة على الخلفية الهادية")}</figcaption>
                    </figure>
                    <figure class="kit__pic">
                        <Picture img=photo sizes="(min-width: 960px) 280px, 45vw" lang=lang aspect=1.0 class="kit__pic-box"/>
                        <figcaption class="t-caption">{t("Photo, cropped to a square", "صورة مقصوصة مربع")}</figcaption>
                    </figure>
                    <figure class="kit__pic">
                        <Picture img=Img::missing("nothing") sizes="280px" lang=lang class="kit__pic-box"/>
                        <figcaption class="t-caption">{t("Missing image", "صورة مش موجودة")}</figcaption>
                    </figure>
                </div>
            </Specimen>

            <Specimen id="cards" title=t("Product cards", "كروت المنتجات") note=t("Real catalogue items. Fixed rows keep a grid aligned and let the skeleton match exactly; a sold-out card is veiled.", "منتجات حقيقية من الكتالوج. الصفوف الثابتة بتخلي الكروت متساوية والهيكل بنفس المقاس؛ والمنتج اللي خلص عليه غطا خفيف.")>
                <ProductGrid label=t("Products", "المنتجات")>
                    {cards
                        .iter()
                        .cloned()
                        .enumerate()
                        .map(|(i, c)| view! { <ProductCard card=c lang=lang numerals=num priority=i < 2/> })
                        .collect_view()}
                    <ProductCardSkeleton/>
                </ProductGrid>
                <div class="kit__rail">
                    <p class="kit__label">{t("Rail (scroll-snap, edges fade, arrows on desktop)", "شريط أفقي (بيقف على كل كارت، والأطراف بتختفي، وأسهم على الكمبيوتر)")}</p>
                    <Rail label=t("More products", "منتجات تانية")>
                        {rail_cards
                            .into_iter()
                            .rev()
                            .map(|c| view! { <ProductCard card=c lang=lang numerals=num/> })
                            .collect_view()}
                    </Rail>
                </div>
            </Specimen>

            <Specimen id="overlays" title=t("Drawers & toasts", "الأدراج والتنبيهات") note=t("Springs you can grab: drawers follow the finger, velocity decides, and they leave the way they came. Toasts are the page turned over.", "حركة تقدر تمسكها: الدرج بيمشي مع صباعك والسرعة هي اللي بتقرر، وبيخرج من نفس السكة اللي دخل منها. التنبيهات بألوان الصفحة مقلوبة.") card=true>
                <Row label=t("Drawer & sheet", "درج وشيت")>
                    <KitDrawers lang num/>
                </Row>
                <Row label=t("Toasts", "تنبيهات")>
                    <KitToasts lang/>
                </Row>
                <Row label=t("Quantity", "الكمية")>
                    <KitStepper lang num/>
                </Row>
            </Specimen>

            <parts::Notices lang num/>

            <Specimen id="forms" title=t("Forms", "النماذج") note=t("52px fields with 16px text, labels above, hints and errors below — and radio cards for choices that need a line of context.", "خانات ٥٢ بكسل بخط ١٦، العنوان فوق والملاحظة والخطأ تحت — وكروت اختيار للحاجات اللي محتاجة سطر شرح.") card=true>
                <div class="kit__form">
                    <TextInput id="kit-name" name="name" label=t("Full name", "الاسم بالكامل") autocomplete="name" value="Mona Adel"/>
                    <TextInput id="kit-phone" name="phone" label=t("Mobile number", "رقم الموبايل") input_type="tel" inputmode="tel" autocomplete="tel" dir="ltr" icon="phone" hint=t("We'll text you when the courier is on the way", "هنبعتلك رسالة لما المندوب يتحرك") placeholder="01x xxxx xxxx"/>
                    <TextInput id="kit-email" name="email" label=t("Email", "الإيميل") input_type="email" optional=true dir="ltr" value="mona@" error=t("Enter a full email address, like name@example.com", "اكتب إيميل كامل، زي name@example.com")/>
                    <Select
                        id="kit-city"
                        name="city"
                        label=t("City", "المدينة")
                        options=vec![
                            ("cairo".to_owned(), t("Cairo", "القاهرة").to_owned()),
                            ("giza".to_owned(), t("Giza", "الجيزة").to_owned()),
                            ("alex".to_owned(), t("Alexandria", "الإسكندرية").to_owned()),
                        ]
                        value="giza"
                    />
                    <TextArea id="kit-notes" name="notes" label=t("Notes for the courier", "ملاحظات للمندوب") optional=true placeholder=t("Ring the bell twice", "اضرب الجرس مرتين")/>
                    <RadioGroup legend=t("Delivery time", "ميعاد التوصيل") id="kit-slot" hint=t("Express is the only paid slot.", "السريع هو الميعاد الوحيد اللي بفلوس.")>
                        <RadioCard required=true name="slot" value="express" label=t("Express", "سريع") description=t("Within 2 hours", "خلال ساعتين") icon="zap" trailing=crate::format::money(Piastres(4_900), lang, num) checked=true/>
                        <RadioCard required=true name="slot" value="today" label=t("Today, 6–9 PM", "النهارده، ٦–٩ بالليل") description=t("Pick a window at checkout", "اختار الميعاد وانت بتكمّل الطلب") icon="calendar-clock" trailing=t("Free", "مجاني")/>
                        <RadioCard required=true name="slot" value="tomorrow" label=t("Tomorrow", "بكرة") icon="truck" trailing=t("Free", "مجاني") disabled=true/>
                    </RadioGroup>
                    <Checkbox name="save" label=t("Save this address", "احفظ العنوان ده") checked=true/>
                </div>
                <parts::FormGaps lang num/>
            </Specimen>

            <Specimen id="states" title=t("States", "الحالات") note=t("Every state says what happened and offers a way on: empty, failed, offline, not found, not built yet. Errors are worded in plain Modern Standard Arabic.", "كل حالة بتقول اللي حصل وبتديك خطوة جاية: فاضي، فشل، من غير نت، مش موجود، لسه ما اتعملش. رسايل الأخطاء بالفصحى البسيطة.") card=true>
                <Breadcrumbs items=vec![
                    (t("Home", "الرئيسية").to_owned(), Some("/".to_owned())),
                    (t("Phones", "موبايلات").to_owned(), Some("/c/phones".to_owned())),
                    ("Galaxy S25 Ultra".to_owned(), None),
                ]/>
                <div class="kit__states">
                    <parts::EmptySuggestion lang/>
                    <div class="kit__state">
                        <p class="kit__label">{t("Empty · cart", "فاضي · السلة")}</p>
                        <StateView compact=true art="empty-cart" title=t("Your cart is empty", "سلتك فاضية") body=t("Today's deals are a good place to start.", "عروض النهارده بداية حلوة.")>
                            <Button href="/search?deal=1&sort=discount" icon="tag">{t("See today's deals", "شوف عروض النهارده")}</Button>
                        </StateView>
                    </div>
                    <div class="kit__state">
                        <p class="kit__label">{t("Empty · no results", "فاضي · مفيش نتائج")}</p>
                        <StateView compact=true tone=StateTone::Empty title=t("No results for “galaxy s99”", "مفيش نتائج لـ «galaxy s99»") body=t("Check the spelling, or try fewer words.", "اتأكد من الكتابة، أو جرّب كلمات أقل.")>
                            <Button kind=ButtonKind::Outline href="/search" icon="x">{t("Clear the search", "امسح البحث")}</Button>
                        </StateView>
                    </div>
                    <div class="kit__state">
                        <p class="kit__label">{t("Error", "خطأ")}</p>
                        <StateView compact=true tone=StateTone::Error title=t("Your orders didn't load", "تعذّر تحميل طلباتك") body=t("It's not you. Please try again in a moment.", "المشكلة ليست عندك. يُرجى المحاولة مرة أخرى بعد قليل.") footnote=t("Reference 7F3K-21", "رقم المرجع 7F3K-21")>
                            <Button href="/ui#states" icon="rotate-ccw">{t("Try again", "إعادة المحاولة")}</Button>
                        </StateView>
                    </div>
                    <div class="kit__state">
                        <p class="kit__label">{t("Offline", "من غير نت")}</p>
                        <StateView compact=true tone=StateTone::Offline title=t("You're offline", "لا يوجد اتصال بالإنترنت") body=t("Check your connection. We'll reconnect by ourselves.", "تحقّق من اتصالك، وسنعيد الاتصال تلقائيًا.")>
                            <Button href="/ui#states" icon="rotate-ccw">{t("Try again", "إعادة المحاولة")}</Button>
                        </StateView>
                    </div>
                    <div class="kit__state">
                        <p class="kit__label">{t("Not found", "مش موجود")}</p>
                        <StateView compact=true tone=StateTone::NotFound kicker=t("Error 404", "خطأ 404") title=t("We couldn't find that page", "لم نعثر على هذه الصفحة") body=t("The link may be old or mistyped.", "ربما يكون الرابط قديمًا أو به خطأ.")>
                            <Button kind=ButtonKind::Outline href="/" icon="house">{t("Home page", "الصفحة الرئيسية")}</Button>
                        </StateView>
                    </div>
                    <div class="kit__state">
                        <p class="kit__label">{t("Coming soon · payment", "قريبًا · الدفع")}</p>
                        <ComingSoon
                            compact=true
                            feature=t("Card and wallet payments", "الدفع بالكارت والمحفظة")
                            description=t("Pay by card, Apple Pay or a mobile wallet. In this preview every order is cash on delivery.", "هتقدر تدفع بالكارت أو Apple Pay أو محفظة الموبايل. في المعاينة دي كل الطلبات كاش عند الاستلام.")
                            back_href="/ui#states"
                        />
                    </div>
                    <div class="kit__state">
                        <p class="kit__label">{t("Coming soon · account", "قريبًا · الحساب")}</p>
                        <ComingSoon
                            compact=true
                            feature=t("Your account", "حسابك")
                            description=t("Saved addresses, past orders and one-tap reorder, on every device.", "عناوينك المحفوظة وطلباتك القديمة وإعادة الطلب بلمسة، على كل أجهزتك.")
                            back_href="/ui#states"
                            back_label=t("Keep shopping", "كمّل تسوق")
                        />
                    </div>
                </div>
                <Row label=t("A section fails", "قسم فشل")>
                    <div class="kit__section-errors">
                        <SectionError what=t("Today's deals", "عروض اليوم") retry_href="/ui#states"/>
                        <SectionError compact=true retry_href="/ui#states"/>
                    </div>
                </Row>
                <Row label=t("A whole page fails", "الصفحة كلها فشلت")>
                    <Button kind=ButtonKind::Outline href="/ui/no-such-page" icon="search">{t("Open the 404 page", "افتح صفحة 404")}</Button>
                    <Button kind=ButtonKind::Outline href="/showcase?crash=1" icon="triangle-alert">{t("Open the 500 page", "افتح صفحة 500")}</Button>
                </Row>
            </Specimen>

            <parts::Choices lang num/>
            <parts::Lines lang num/>
            <parts::Progress lang num/>

            <Specimen id="network" title=t("Network", "الشبكة") note=t("What a visitor sees when the connection drops, a page is slow or a navigation fails. The banner, the alert and the progress bar are page-wide pieces of the shell, driven by the router.", "اللي الزائر بيشوفه لما النت يقطع، أو صفحة تتأخر، أو التنقل يفشل. الشريط والتنبيه وشريط التحميل أجزاء ثابتة في إطار الصفحة، والراوتر هو اللي بيحركها.") card=true>
                <ul class="kit__netlist" role="list">
                    <li><Icon name="wifi-off" size=18/><span>{t("Offline: a calm banner at the top with Try again; back online, the page refreshes in place.", "من غير نت: شريط هادي فوق ومعاه «حاول تاني»؛ ولما النت يرجع الصفحة بتتحدّث مكانها.")}</span></li>
                    <li><Icon name="triangle-alert" size=18/><span>{t("A navigation fails (offline, too slow, server error): an alert says why and offers Retry; the page you were on stays.", "التنقل فشل (من غير نت، اتأخر، خطأ في السيرفر): تنبيه بيقول السبب وبيديك «إعادة المحاولة»؛ والصفحة اللي كنت فيها بتفضل.")}</span></li>
                    <li><Icon name="loader-circle" size=18/><span>{t("A slow navigation: a thin progress bar, shown only past 150 ms.", "تنقل بطيء: شريط تحميل رفيع، مابيظهرش غير بعد 150 مللي ثانية.")}</span></li>
                </ul>
                // Stages the real banner, alert and progress bar through core's `previewNet` (nothing
                // is really disconnected). Hidden until the browser code says the hook is there.
                <div class="kit__netbtns" data-ts="kit-net">
                    <Row label=t("Offline banner", "شريط عدم الاتصال")>
                        <Button kind=ButtonKind::Secondary icon="wifi-off" attr:data-net="offline" attr:data-needs-js="" attr:hidden="">{t("Go offline", "اقطع النت")}</Button>
                        <Button kind=ButtonKind::Secondary icon="wifi" attr:data-net="online" attr:data-needs-js="" attr:hidden="">{t("Come back online", "رجّع النت")}</Button>
                    </Row>
                    <Row label=t("A navigation fails", "التنقل فشل")>
                        <Button kind=ButtonKind::Outline attr:data-net="fail:offline" attr:data-needs-js="" attr:hidden="">{t("Offline", "من غير نت")}</Button>
                        <Button kind=ButtonKind::Outline attr:data-net="fail:timeout" attr:data-needs-js="" attr:hidden="">{t("Too slow", "اتأخر")}</Button>
                        <Button kind=ButtonKind::Outline attr:data-net="fail:server" attr:data-needs-js="" attr:hidden="">{t("Server error", "خطأ في السيرفر")}</Button>
                    </Row>
                    <Row label=t("A slow navigation", "تنقل بطيء")>
                        <Button kind=ButtonKind::Outline attr:data-net="progress:1800" attr:data-needs-js="" attr:hidden="">{t("Show the progress bar", "اعرض شريط التحميل")}</Button>
                    </Row>
                </div>
                <p class="kit__note t-caption">
                    {t(
                        "Every read goes through core/net.ts: one 10 s timeout, one retry after a network failure or a 5xx (never after a 4xx), and no try is spent while offline.",
                        "كل قراية بتعدّي على core/net.ts: مهلة واحدة 10 ثواني، ومحاولة تانية واحدة بعد عطل في الشبكة أو 5xx (عمرها ما بتحصل بعد 4xx)، ومن غير نت مفيش محاولة بتضيع.",
                    )}
                </p>
            </Specimen>

            <Specimen id="skeletons" title=t("Skeletons", "هياكل التحميل") note=t("Solid blocks that pulse, built from the boxes of what replaces them. Each pair is loading beside loaded: nothing moves when one becomes the other.", "بلوكات مصمتة بتنبض، معمولة بنفس مقاسات اللي هيظهر مكانها. كل اتنين جنب بعض تحميل وجاهز: مفيش حاجة بتتحرك لما واحد يبقى التاني.") card=true>
                <div class="kit__pairs" data-ts="kit-skeletons">
                    <p class="kit__replay" data-needs-js="" hidden>
                        <Button kind=ButtonKind::Secondary size=ButtonSize::Sm icon="rotate-ccw" attr:data-act="replay">{t("Watch them load", "اتفرج عليهم وهما بيحمّلوا")}</Button>
                        <span class="t-caption mono" data-slot="cls"></span>
                    </p>
                    <KitPair title=t("Heading and text", "عنوان وكلام") loading=t("Loading", "بيحمّل") loaded=t("Loaded", "جاهز")>
                        <div class="kit__copy">
                            <SkeletonLines lines=1 class="t-h3" last_width="70%"/>
                            <SkeletonLines lines=3 class="kit__para"/>
                        </div>
                        <div class="kit__copy">
                            <h3 class="t-h3">{t("Galaxy S25", "جالاكسي S25")}</h3>
                            <p class="kit__para kit__clamp">
                                {t(
                                    "A 6.9-inch screen that stays readable in the midday sun, a titanium frame, the S Pen tucked inside, and a 200 MP camera that keeps detail at night. Seven years of updates, and delivery in Cairo within hours.",
                                    "شاشة ٦٫٩ بوصة بتتقري في شمس الضهر، وإطار تيتانيوم، والقلم جوه الموبايل، وكاميرا ٢٠٠ ميجابكسل بتحافظ على التفاصيل بالليل. سبع سنين تحديثات، وتوصيل في القاهرة خلال ساعات.",
                                )}
                            </p>
                        </div>
                    </KitPair>
                    <KitPair title=t("Product card", "كارت منتج") loading=t("Loading", "بيحمّل") loaded=t("Loaded", "جاهز")>
                        <ProductCardSkeleton/>
                        {pair_card.map(|c| view! { <ProductCard card=c lang=lang numerals=num/> })}
                    </KitPair>
                    <KitPair title=t("Picture", "صورة") loading=t("Loading", "بيحمّل") loaded=t("Loaded", "جاهز")>
                        <SkeletonMedia aspect=1.0 class="kit__pic-box"/>
                        <Picture img=pair_img sizes="(min-width: 768px) 280px, 45vw" lang=lang aspect=1.0 class="kit__pic-box"/>
                    </KitPair>
                    <KitPair title=t("Fields", "خانات") loading=t("Loading", "بيحمّل") loaded=t("Loaded", "جاهز")>
                        <div class="kit__fields">
                            <SkeletonField hint=true/>
                            <SkeletonField multiline=true/>
                        </div>
                        <div class="kit__fields">
                            <TextInput id="kit-skel-name" name="skel-name" label=t("Full name", "الاسم") hint=t("For the courier", "للمندوب") value="Mona Adel"/>
                            <TextArea id="kit-skel-notes" name="skel-notes" label=t("Notes", "ملاحظات") placeholder=t("Ring the bell twice", "اضرب الجرس مرتين")/>
                        </div>
                    </KitPair>
                    <KitPair title=t("Rail", "شريط منتجات") loading=t("Loading", "بيحمّل") loaded=t("Loaded", "جاهز") stack=true>
                        <SkeletonRail/>
                        <Rail label=t("Picks", "مختارات")>
                            {pair_rail
                                .into_iter()
                                .map(|c| view! { <ProductCard card=c lang=lang numerals=num/> })
                                .collect_view()}
                        </Rail>
                    </KitPair>
                    <div class="kit__skeletons">
                        <Skeleton width="3.5rem" height="3.5rem" circle=true/>
                        <SkeletonLines lines=2 class="t-h3" last_width="40%"/>
                        <Skeleton width="100%" height="2.75rem"/>
                    </div>
                </div>
            </Specimen>

            <Specimen id="icons" title=t("Icons", "الأيقونات") note=t("Lucide geometry, one path per glyph so a patched page swaps an attribute, not a tree. Arrows and chevrons mirror in Arabic.", "أيقونات لوسيد، كل أيقونة مسار واحد. الأسهم بتنعكس في العربي.")>
                <ul class="kit__icons" role="list">
                    {NAMES
                        .iter()
                        .map(|n| {
                            let c = if crate::components::button::directional(n) { "flip-rtl" } else { "" };
                            view! {
                                <li class="kit__icon" title=*n>
                                    <Icon name=*n size=24 class=c/>
                                    <span class="kit__icon-name mono">{*n}</span>
                                </li>
                            }
                        })
                        .collect_view()}
                </ul>
            </Specimen>
        </Container>
    }
}

/// Drawer demo: core's drawer from the inline end and the bottom sheet, opened by plain triggers.
#[component]
fn KitDrawers(lang: Lang, num: Numerals) -> impl IntoView {
    let t = move |en: &'static str, ar: &'static str| tr(lang, en, ar);
    let show = match lang {
        Lang::En => format!("Show {} products", format::count(128, num)),
        Lang::Ar => format!("اعرض {} منتج", format::count(128, num)),
    };
    view! {
        <div class="kit__items">
            <Button icon="shopping-bag" attr:data-drawer-open="kit-cart">{t("Open the cart drawer", "افتح درج السلة")}</Button>
            <Button kind=ButtonKind::Outline icon="sliders-horizontal" attr:data-drawer-open="kit-sheet">{t("Open the filters sheet", "افتح شيت الفلاتر")}</Button>
        </div>
        <Drawer
            id="kit-cart"
            title=t("Your cart", "سلتك").to_owned()
            footer=Box::new(move || view! {
                <Button size=ButtonSize::Lg full_width=true attr:data-drawer-close="">{t("Checkout", "إتمام الطلب")}</Button>
            }.into_any())
        >
            <div class="kit__drawer-body">
                <p class="t-muted">{t("Drag me toward the edge: let go fast to close, slowly to snap back.", "اسحبني ناحية الطرف: لو سبت بسرعة هقفل، ولو براحة هرجع مكاني.")}</p>
                <div class="kit__line">
                    <span class="kit__line-name">{t("Galaxy S25 Ultra", "جالاكسي S25 ألترا")}</span>
                    <Stepper name="kit-drawer-qty" value=2u32 min=1 max=5/>
                </div>
            </div>
        </Drawer>
        <Drawer
            id="kit-sheet"
            side=DrawerSide::Bottom
            title=t("Filters", "الفلاتر").to_owned()
            footer=Box::new(move || view! {
                <Button size=ButtonSize::Lg full_width=true attr:data-drawer-close="">{show}</Button>
            }.into_any())
        >
            <div class="kit__drawer-body">
                <p class="t-muted">{t("Pull the grabber down to dismiss.", "اسحب المقبض لتحت عشان تقفل.")}</p>
                <Checkbox name="stock" label=t("In stock only", "المتوفر بس")/>
                <Checkbox name="deal" label=t("On sale", "عليه خصم") checked=true/>
                <Checkbox name="express" label=t("Express delivery", "توصيل سريع") description=t("Within hours in Cairo and Giza", "خلال ساعات في القاهرة والجيزة")/>
            </div>
        </Drawer>
    }
}

/// Toast demo: each button calls `toast()` from the one toast store (features/showcase.ts).
#[component]
fn KitToasts(lang: Lang) -> impl IntoView {
    let t = move |en: &'static str, ar: &'static str| tr(lang, en, ar);
    view! {
        <div class="kit__items" data-ts="kit-toasts">
            <Button kind=ButtonKind::Secondary icon="circle-check" attr:data-toast="success" attr:data-needs-js="" attr:hidden="">{t("Success", "تمام")}</Button>
            <Button kind=ButtonKind::Secondary icon="info" attr:data-toast="info" attr:data-needs-js="" attr:hidden="">{t("Info", "معلومة")}</Button>
            <Button kind=ButtonKind::Secondary icon="circle-x" attr:data-toast="error" attr:data-needs-js="" attr:hidden="">{t("Error", "خطأ")}</Button>
            <Button kind=ButtonKind::Secondary icon="rotate-ccw" attr:data-toast="action" attr:data-needs-js="" attr:hidden="">{t("With Undo", "ومعاه تراجع")}</Button>
            <noscript><span class="t-caption">{t("Toasts need JavaScript.", "التنبيهات محتاجة JavaScript.")}</span></noscript>
        </div>
    }
}

/// Steppers: core's stepper. The line total for each quantity is formatted by the server (the
/// client picks one, it never computes money); the removable one becomes a bin at its minimum.
#[component]
fn KitStepper(lang: Lang, num: Numerals) -> impl IntoView {
    let t = move |en: &'static str, ar: &'static str| tr(lang, en, ar);
    let unit = Piastres(129_900);
    let totals: std::collections::BTreeMap<String, String> =
        (1..=5u32).map(|q| (q.to_string(), format::money(unit * q, lang, num))).collect();
    let json = serde_json::to_string(&totals).unwrap_or_else(|_| "{}".into());
    view! {
        <div class="kit__items" data-ts="kit-total">
            <script type="application/json" data-ts-json="" inner_html=json></script>
            <Stepper name="kit-a" value=1u32 min=1 max=5 label=t("Quantity of Galaxy S25", "كمية جالاكسي S25")/>
            <output class="kit__total mono" data-slot="total">{format::money(unit, lang, num)}</output>
        </div>
        <div class="kit__items" data-ts="kit-remove">
            <Stepper name="kit-b" value=1u32 min=1 max=9 removable=true/>
            <Stepper name="kit-c" value=3u32 min=1 max=9 disabled=true/>
        </div>
    }
}
