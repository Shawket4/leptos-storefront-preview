//! What the showcase says: categories, and one entry per Leptos feature the storefront uses —
//! plain explanation, why it matters here, where it is used, where the code lives.
//!
//! "Used in" lists only usages that exist in this repository's code. Where a feature is not used
//! by a page yet, the demo on /showcase is its only usage and the entry says so.

use crate::prefs::{Lang, tr};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cat {
    Rendering,
    Reactivity,
    Data,
    Forms,
    Routing,
    Platform,
    I18n,
    Media,
    A11y,
}

impl Cat {
    pub const ALL: [Cat; 9] = [
        Cat::Rendering,
        Cat::Reactivity,
        Cat::Data,
        Cat::Forms,
        Cat::Routing,
        Cat::Platform,
        Cat::I18n,
        Cat::Media,
        Cat::A11y,
    ];

    /// URL / `data-cat` value.
    pub fn code(self) -> &'static str {
        match self {
            Cat::Rendering => "rendering",
            Cat::Reactivity => "reactivity",
            Cat::Data => "data",
            Cat::Forms => "forms",
            Cat::Routing => "routing",
            Cat::Platform => "platform",
            Cat::I18n => "i18n",
            Cat::Media => "media",
            Cat::A11y => "a11y",
        }
    }

    pub fn parse(s: &str) -> Option<Cat> {
        Cat::ALL.into_iter().find(|c| c.code() == s)
    }

    /// Short label (filter chips).
    pub fn short(self) -> (&'static str, &'static str) {
        match self {
            Cat::Rendering => ("Rendering", "العرض"),
            Cat::Reactivity => ("Reactivity", "التفاعلية"),
            Cat::Data => ("Data & server", "البيانات والسيرفر"),
            Cat::Forms => ("Forms", "الفورمز"),
            Cat::Routing => ("Routing & navigation", "التوجيه والتنقل"),
            Cat::Platform => ("Platform & caching", "المنصة والكاش"),
            Cat::I18n => ("i18n / RTL", "اللغات وRTL"),
            Cat::Media => ("Media & motion", "الصور والحركة"),
            Cat::A11y => ("Accessibility", "الإتاحة"),
        }
    }

    pub fn label(self, lang: Lang) -> &'static str {
        let (en, ar) = self.short();
        tr(lang, en, ar)
    }

    /// One line under the category heading.
    pub fn blurb(self, lang: Lang) -> &'static str {
        match self {
            Cat::Rendering => tr(
                lang,
                "What runs where: HTML from the server, WebAssembly only where it's needed, streamed as data arrives.",
                "إيه اللي بيشتغل فين: HTML من السيرفر، وWebAssembly بس في الأماكن اللي محتاجاه، وبيتبعت أول بأول مع وصول البيانات.",
            ),
            Cat::Reactivity => tr(
                lang,
                "How interactive parts update: a graph of signals instead of re-rendering components.",
                "إزاي الأجزاء التفاعلية بتتحدث: شبكة signals بدل ما الكومبوننت يترسم من الأول.",
            ),
            Cat::Data => tr(
                lang,
                "How data gets from the catalogue to the screen, and what the UI does while it waits.",
                "إزاي البيانات بتوصل من الكتالوج للشاشة، والواجهة بتعمل إيه وهي مستنياها.",
            ),
            Cat::Forms => tr(
                lang,
                "Mutations that work with and without JavaScript.",
                "عمليات تعديل بتشتغل بالـ JavaScript ومن غيره.",
            ),
            Cat::Routing => tr(
                lang,
                "URLs as the source of truth, and page changes that feel like an app.",
                "الـ URL هو مصدر الحقيقة، والتنقل بين الصفحات إحساسه زي الأبلكيشن.",
            ),
            Cat::Platform => tr(
                lang,
                "Caching at the edge, and one deployment serving many shops.",
                "الكاش على الـ CDN، ونسخة واحدة بتخدم محلات كتير.",
            ),
            Cat::I18n => tr(
                lang,
                "Arabic and English, two numeral systems, right-to-left — switched without a reload.",
                "عربي وإنجليزي، ونظامين أرقام، ومن اليمين للشمال — والتبديل من غير ريلود.",
            ),
            Cat::Media => tr(
                lang,
                "Images sized for the device, and motion that explains where things went.",
                "صور على مقاس الجهاز، وحركة بتوضح الحاجة راحت فين.",
            ),
            Cat::A11y => tr(
                lang,
                "Keyboard, screen readers and focus — built into the components.",
                "الكيبورد وقارئ الشاشة والفوكس — جزء من الكومبوننتس نفسها.",
            ),
        }
    }
}

/// A link target that may depend on the tenant's catalogue.
#[derive(Clone, Copy, Debug)]
pub enum Href {
    Path(&'static str),
    /// `/c/<first category>`
    Category,
    /// `/c/<first category>?sort=price-asc&stock=1`
    CategoryQuery,
    /// `/p/<a product with several variants>`
    Product,
    /// `/p/<that product>?v=<its second variant>`
    ProductVariant,
    /// `/p/<that product>?<query>`
    ProductQuery(&'static str),
}

#[derive(Clone, Copy, Debug)]
pub struct Usage {
    pub href: Href,
    pub en: &'static str,
    pub ar: &'static str,
}

const fn used(href: Href, en: &'static str, ar: &'static str) -> Usage {
    Usage { href, en, ar }
}

pub struct Feature {
    /// Anchor id (`/showcase#<id>`).
    pub id: &'static str,
    pub cat: Cat,
    pub title: (&'static str, &'static str),
    pub what: (&'static str, &'static str),
    pub why: (&'static str, &'static str),
    pub used_in: &'static [Usage],
    /// Shown when `used_in` is empty or partial: why the demo is the usage (EN, AR).
    pub note: Option<(&'static str, &'static str)>,
    pub sources: &'static [&'static str],
}

impl Feature {
    pub fn title(&self, lang: Lang) -> &'static str {
        tr(lang, self.title.0, self.title.1)
    }
}

/// In display order (grouped by category, in [`Cat::ALL`] order).
pub static FEATURES: &[Feature] = &[
    // ── Rendering ──────────────────────────────────────────────────────────────────────────────
    Feature {
        id: "islands",
        cat: Cat::Rendering,
        title: ("Islands architecture", "معمارية الجزر (Islands)"),
        what: (
            "Every #[component] renders on the server only and ships no JavaScript. Only #[island] components are compiled to WebAssembly and hydrate in the browser; the rest of the page stays plain HTML.",
            "كل ‎#[component]‎ بيترسم على السيرفر بس ومن غير أي JavaScript. الـ ‎#[island]‎ بس هي اللي بتتحول WebAssembly وتصحى في المتصفح، وباقي الصفحة HTML عادي.",
        ),
        why: (
            "A storefront is mostly content — grids, descriptions, prices. Paying for JavaScript only where it's interactive (add to cart, steppers, countdowns) keeps budget phones fast and the bundle flat as the catalogue grows.",
            "المتجر أغلبه محتوى — شبكات منتجات ووصف وأسعار. لما تدفع تمن الـ JavaScript في الأجزاء التفاعلية بس (أضف للسلة، العدّاد، العد التنازلي) الموبايلات الاقتصادية بتفضل سريعة، وحجم الـ bundle مش بيكبر مع الكتالوج.",
        ),
        used_in: &[
            used(
                Href::Path("/#rail-deals"),
                "Home: the deals countdown",
                "الرئيسية: العد التنازلي للعروض",
            ),
            used(
                Href::Path("/"),
                "Home: every rail, tile and card is server-only",
                "الرئيسية: كل الأقسام والصور والكروت سيرفر بس",
            ),
            used(
                Href::Product,
                "Product page: two islands (gallery, purchase panel); specs, description and related products are plain HTML",
                "صفحة المنتج: جزيرتين بس (الصور ولوحة الشراء)؛ والمواصفات والوصف والمنتجات المشابهة HTML عادي",
            ),
            used(
                Href::Path("/cart"),
                "Every page: the header's cart button and search box, the cart drawer, quick-add, the offline banner",
                "كل صفحة: زرار السلة والبحث في الهيدر، ودرج السلة، والإضافة السريعة، وتنبيه انقطاع النت",
            ),
        ],
        note: None,
        sources: &[
            "crates/frontend/src/lib.rs",
            "crates/app/src/pages/home.rs",
            "crates/app/src/pages/product/parts.rs",
        ],
    },
    Feature {
        id: "ssr-modes",
        cat: Cat::Rendering,
        title: (
            "SSR modes per route: the honest trade-off",
            "طرق الـ SSR لكل route: المقايضة بصراحة",
        ),
        what: (
            "Leptos streams a route in one of four modes. OutOfOrder (default) sends each <Suspense> as soon as it resolves, but slots it in with a few bytes of inline JavaScript. InOrder waits at each <Suspense> in document order and needs no script. PartiallyBlocked holds the first byte until its blocking resources resolve — in 0.8.22 that is all it does: leptos_axum doesn't splice them in yet, so their <Suspense> still streams as a template + script. Async waits for everything.",
            "‏Leptos بيبعت الـ route بواحدة من أربع طرق. OutOfOrder (الافتراضي) بيبعت كل <Suspense> أول ما يخلص، بس بيحطه مكانه بشوية JavaScript جوه الصفحة. InOrder بيستنى عند كل <Suspense> بترتيب الصفحة ومش محتاج أي سكريبت. PartiallyBlocked بيأخر أول بايت لحد ما الـ blocking resources بتاعته تخلص — وفي 0.8.22 ده كل اللي بيعمله: ‏leptos_axum لسه مابيحطهاش جوه الصفحة، فالـ <Suspense> بتاعها بيتبعت برضه كـ template و script. وAsync بيستنى كله.",
        ),
        why: (
            "Out-of-order is the fastest first paint but shows skeletons forever without JavaScript, and no mode's <head> waits for streamed data — so each route picks its mode: the product page holds its first byte for its data and builds its <head> (title, OpenGraph, JSON-LD) synchronously from the in-memory catalogue; listings, cart, checkout and orders stream in order; home streams out of order with an in-order twin for no-JS visitors.",
            "الـ OutOfOrder أسرع أول ظهور، بس من غير JavaScript الـ skeletons بتفضل للأبد، ومفيش طريقة الـ <head> بتاعها بيستنى البيانات المتبعتة — عشان كده كل route بيختار طريقته: صفحة المنتج بتأخر أول بايت لحد ما بياناتها تيجي وبتبني الـ <head> (العنوان وOpenGraph وJSON-LD) على طول من الكتالوج اللي في الذاكرة؛ والقوايم والسلة والدفع والطلبات بالترتيب؛ والرئيسية بأي ترتيب ومعاها نسخة بالترتيب للي من غير JavaScript.",
        ),
        used_in: &[
            used(Href::Path("/"), "Home: OutOfOrder", "الرئيسية: OutOfOrder"),
            used(
                Href::Path("/nojs"),
                "/nojs: the same home page, InOrder",
                "‏/nojs: نفس الرئيسية، InOrder",
            ),
            used(
                Href::Category,
                "Listings and search: InOrder",
                "القوايم والبحث: InOrder",
            ),
            used(
                Href::Path("/cart"),
                "Cart, checkout, orders: InOrder — complete without JavaScript",
                "السلة والدفع والطلبات: InOrder — كاملة من غير JavaScript",
            ),
            used(
                Href::Product,
                "Product page: PartiallyBlocked",
                "صفحة المنتج: PartiallyBlocked",
            ),
        ],
        note: None,
        sources: &["crates/app/src/lib.rs", "crates/app/src/pages/product.rs"],
    },
    Feature {
        id: "streaming",
        cat: Cat::Rendering,
        title: (
            "Streaming SSR: Resource + Suspense, out of order",
            "بث الـ SSR: ‏Resource و Suspense بأي ترتيب",
        ),
        what: (
            "The shell and <head> go out first; each <Suspense> streams in the moment its Resource resolves, in whatever order the data arrives. A blocking resource is the opt-out: it renders inline, in the first chunk.",
            "الهيكل والـ <head> بيوصلوا الأول؛ وكل <Suspense> بيوصل أول ما الـ Resource بتاعه يخلص، بالترتيب اللي البيانات بتيجي بيه. والـ blocking resource هو الاستثناء: بيترسم جوه أول chunk.",
        ),
        why: (
            "The first byte doesn't wait for the slowest backend call: the hero and prices paint while recommendations are still loading.",
            "أول بايت مش بيستنى أبطأ نداء للباك إند: الواجهة والأسعار بتظهر والترشيحات لسه بتحمّل.",
        ),
        used_in: &[
            used(
                Href::Path("/"),
                "Home: five rails, each its own Suspense",
                "الرئيسية: خمس أقسام، كل واحد Suspense لوحده",
            ),
            used(
                Href::Product,
                "Product page: the product is a blocking resource; “You may also like” streams on its own",
                "صفحة المنتج: المنتج نفسه blocking resource؛ و«ممكن يعجبك» بيتبعت لوحده",
            ),
            used(
                Href::Path("/no-such-page"),
                "404: popular categories (blocking)",
                "صفحة 404: الأقسام (blocking)",
            ),
        ],
        note: None,
        sources: &[
            "crates/app/src/pages/home.rs",
            "crates/app/src/pages/product.rs",
            "crates/app/src/pages/errors.rs",
        ],
    },
    Feature {
        id: "errors",
        cat: Cat::Rendering,
        title: ("Per-section <ErrorBoundary>", "‏<ErrorBoundary> لكل قسم"),
        what: (
            "Rendering an Err bubbles up to the nearest boundary, which swaps in a fallback for that section only. Each rail and widget has its own, so one failing backend call can't blank the page.",
            "لما تعرض Err بيطلع لأقرب boundary، وده بيحط بديل للقسم ده بس. كل قسم وكل widget ليه boundary خاص بيه، فنداء واحد فاشل للباك إند ميقدرش يفضّي الصفحة.",
        ),
        why: (
            "Recommendations failing must never cost a sale: the product, the price and add-to-cart keep working.",
            "لو الترشيحات وقعت، ده مينفعش يضيّع بيعة: المنتج والسعر وزرار الإضافة للسلة يفضلوا شغالين.",
        ),
        used_in: &[
            used(
                Href::Path("/?fail=deals"),
                "Home with the deals rail failing",
                "الرئيسية وقسم العروض واقع",
            ),
            used(
                Href::ProductQuery("fail=related"),
                "Product page with “You may also like” failing",
                "صفحة المنتج و«ممكن يعجبك» واقع",
            ),
            used(
                Href::ProductQuery("fail=product"),
                "Product page with the product itself failing",
                "صفحة المنتج والمنتج نفسه واقع",
            ),
        ],
        note: None,
        sources: &[
            "crates/app/src/pages/errors.rs",
            "crates/app/src/pages/product/parts.rs",
            "crates/app/src/server/sim.rs",
        ],
    },
    Feature {
        id: "states",
        cat: Cat::Rendering,
        title: (
            "Every state, designed: loading, empty, failed, offline, not found, not built",
            "كل الحالات متصممة: تحميل، فاضي، عطل، من غير نت، مش موجود، لسه ما اتعملش",
        ),
        what: (
            "Skeletons the exact size of what replaces them, empty states that point somewhere, a failed section that says so and offers a retry, reads that retry by themselves, an offline banner, a designed 404, a 500/503 page and a “not in this preview yet” view — real components the pages use, shown live here.",
            "‏Skeletons بنفس مقاس اللي هيجي مكانها، وحالات فاضية بتوجهك لحاجة، وقسم واقع بيقول كده ويعرض إعادة المحاولة، وقرايات بتعيد المحاولة لوحدها، وتنبيه لما النت يقطع، وصفحة 404 وصفحة 500/503 متصممين، وصفحة «مش موجودة في المعاينة دي لسه» — كومبوننتس حقيقية الصفحات بتستخدمها، ومعروضة هنا على الهوا.",
        ),
        why: (
            "A shop is judged in its bad moments — slow 3G, a backend hiccup, a dead link. None of them should look broken or lose the cart.",
            "المتجر بيتحكم عليه في لحظاته الوحشة — نت بطيء، أو عطل في الباك إند، أو لينك مش شغال. ولا واحدة منهم المفروض تبان بايظة أو تضيّع السلة.",
        ),
        used_in: &[
            used(
                Href::Path("/no-such-page"),
                "The router's 404 (NotFound)",
                "صفحة 404 بتاعة الراوتر (‏NotFound)",
            ),
            used(
                Href::Path("/?fail=deals"),
                "Home: a failing rail",
                "الرئيسية: قسم واقع",
            ),
            used(
                Href::Path("/search?q=zzqx"),
                "Search with no results (StateView)",
                "بحث من غير نتايج (‏StateView)",
            ),
            used(
                Href::Path("/checkout"),
                "Checkout with an empty cart (StateView)",
                "الدفع والسلة فاضية (‏StateView)",
            ),
            used(
                Href::Path("/"),
                "Every page: the offline banner (mounted once by the shell)",
                "كل صفحة: تنبيه انقطاع النت (الـ shell بيركّبه مرة واحدة)",
            ),
        ],
        note: Some((
            "ComingSoon and net::retry aren't used by a page yet — the tiles above are their usage (and /ui).",
            "‏ComingSoon و net::retry لسه مش مستخدمين في صفحة — المربعات اللي فوق هي استخدامهم (ومعاهم ‎/ui‎).",
        )),
        sources: &[
            "crates/app/src/components/states.rs",
            "crates/app/src/components/offline.rs",
            "crates/app/src/components/net.rs",
            "crates/app/src/pages/errors.rs",
        ],
    },
    Feature {
        id: "meta",
        cat: Cat::Rendering,
        title: (
            "leptos_meta: the <head> from components",
            "‏leptos_meta: الـ <head> من جوه الكومبوننتس",
        ),
        what: (
            "<Title>, <Meta>, <Link> and <Script> placed anywhere in a page are collected during the render and injected into <head> before the first byte — per tenant and per language.",
            "‏<Title> و<Meta> و<Link> و<Script> في أي حتة في الصفحة بيتجمعوا وقت الرسم ويتحطوا في الـ <head> قبل أول بايت — لكل متجر ولكل لغة.",
        ),
        why: (
            "Titles, canonical URLs, OpenGraph cards and JSON-LD are what get a shop found on Google and shared on WhatsApp.",
            "العناوين والـ canonical وكروت OpenGraph والـ JSON-LD هي اللي بتخلي المتجر يبان على جوجل ويتشير على واتساب.",
        ),
        used_in: &[
            used(
                Href::Path("/"),
                "Home: title, OpenGraph, WebSite JSON-LD",
                "الرئيسية: العنوان وOpenGraph و WebSite JSON-LD",
            ),
            used(
                Href::Product,
                "Product page: canonical, OpenGraph with product:price, Twitter card, LCP preload, Product + BreadcrumbList JSON-LD",
                "صفحة المنتج: الـ canonical و OpenGraph بـ product:price وكارت تويتر و preload للصورة و JSON-LD لـ Product و BreadcrumbList",
            ),
            used(
                Href::Category,
                "Category: canonical and BreadcrumbList JSON-LD",
                "القسم: الـ canonical و JSON-LD لـ BreadcrumbList",
            ),
        ],
        note: Some((
            "Every head tag is registered before the first chunk: in 0.8.22 a tag inside a <Suspense> never reaches <head>, even over a blocking resource (see Sharp edges).",
            "كل تاجات الـ head بتتسجل قبل أول chunk: في 0.8.22 أي تاج جوه <Suspense> عمره ما بيوصل للـ <head>، حتى لو فوق blocking resource (شوف «حاجات جرحتنا»).",
        )),
        sources: &[
            "crates/app/src/pages/home.rs",
            "crates/app/src/pages/product/head.rs",
            "crates/app/src/lib.rs",
        ],
    },
    // ── Reactivity ─────────────────────────────────────────────────────────────────────────────
    Feature {
        id: "reactivity",
        cat: Cat::Reactivity,
        title: ("Fine-grained reactivity", "تفاعلية دقيقة (Fine-grained)"),
        what: (
            "Signals, derived signals, Memo and Effect form a dependency graph. The component function runs once; a change re-runs only the closures that read it and rewrites only their text nodes. There is no virtual DOM.",
            "الـ Signals والـ derived signals والـ Memo والـ Effect بيكوّنوا شبكة اعتماديات. دالة الكومبوننت بتشتغل مرة واحدة؛ وأي تغيير بيعيد تشغيل الـ closures اللي بتقراه بس، وبيعدّل الـ text nodes بتاعتها بس. مفيش virtual DOM.",
        ),
        why: (
            "Cart totals, free-delivery thresholds and variant prices update instantly on low-end phones, because nothing that didn't change is touched.",
            "إجمالي السلة وحد التوصيل المجاني وأسعار الاختيارات بتتحدث فورًا حتى على الموبايلات الضعيفة، لأن مفيش حاجة بتتلمس إلا اللي اتغيرت فعلًا.",
        ),
        used_in: &[
            used(
                Href::ProductVariant,
                "Product page: the variant is one signal; price, stock, SKU and photo are memos of it",
                "صفحة المنتج: الاختيار signal واحد؛ والسعر والمخزون والـ SKU والصورة memos منه",
            ),
            used(
                Href::Path("/search"),
                "Header search: text, open and the active option are signals; the option list is a memo of the latest answer",
                "البحث في الهيدر: النص والفتح والاختيار الحالي signals؛ وقايمة الاختيارات memo من آخر رد",
            ),
            used(
                Href::Path("/#rail-deals"),
                "Home: the countdown is a signal ticking once a second",
                "الرئيسية: العد التنازلي signal بيتحدث كل ثانية",
            ),
        ],
        note: None,
        sources: &[
            "crates/app/src/pages/product/panel.rs",
            "crates/app/src/pages/showcase/islands.rs",
            "crates/app/src/pages/home.rs",
        ],
    },
    Feature {
        id: "shared-state",
        cat: Cat::Reactivity,
        title: ("Shared state across islands", "حالة مشتركة بين الجزر"),
        what: (
            "Islands are separate roots, but hydrate_islands() leaves one root owner current. Stores provided on it in client::init() are visible to every island through use_context, so islands far apart share the same signals.",
            "كل island ليها جذر منفصل، بس hydrate_islands()‎ بيسيب owner رئيسي واحد شغال. أي store بيتوفر عليه في client::init()‎ بيبان لكل الـ islands عن طريق use_context، فالجزر البعيدة عن بعض بتشارك نفس الـ signals.",
        ),
        why: (
            "The header's cart badge, the cart drawer and every add-to-cart button are separate islands in different places; they all read and write one CartStore.",
            "عدّاد السلة في الهيدر، ودرج السلة، وكل زرار «أضف للسلة» islands منفصلة في أماكن مختلفة؛ وكلهم بيقروا ويكتبوا في CartStore واحد.",
        ),
        used_in: &[
            used(
                Href::Path("/cart"),
                "Every page: the header badge, the cart drawer and every quick-add share one CartStore",
                "كل صفحة: عدّاد الهيدر ودرج السلة وكل زرار إضافة سريعة بيشاركوا CartStore واحد",
            ),
            used(
                Href::ProductVariant,
                "Product page: the purchase panel picks a colour, the gallery shows its photo",
                "صفحة المنتج: لوحة الشراء بتختار اللون، والصور بتعرض صورته",
            ),
            used(
                Href::Path("/#rail-deals"),
                "Home: the countdown reads the shared Ui store",
                "الرئيسية: العد التنازلي بيقرا store الـ Ui المشترك",
            ),
        ],
        note: None,
        sources: &[
            "crates/app/src/client.rs",
            "crates/app/src/stores.rs",
            "crates/app/src/pages/cart/store.rs",
        ],
    },
    // ── Data & server ──────────────────────────────────────────────────────────────────────────
    Feature {
        id: "server-fns",
        cat: Cat::Data,
        title: ("Server functions", "دوال السيرفر (Server functions)"),
        what: (
            "A #[server] async fn runs on the server and becomes a typed POST endpoint that the browser calls like a normal function. During SSR the same function is called directly, without an HTTP round-trip.",
            "الـ ‎#[server]‎ دالة async بتشتغل على السيرفر وبتتحول لـ endpoint من نوع POST بأنواع واضحة، والمتصفح بينادي عليها كأنها دالة عادية. ووقت الـ SSR نفس الدالة بتتنادى مباشرة من غير رحلة HTTP.",
        ),
        why: (
            "One Rust type crosses the wire (CartView, ListingPage), so the cart and the listings can't drift from the server's rules — prices come from madar-catalog.",
            "نفس الـ type بتاع Rust بيعدّي بين الطرفين (CartView وListingPage)، فالسلة والقوايم مستحيل يختلفوا عن قواعد السيرفر — والأسعار جاية من madar-catalog.",
        ),
        used_in: &[
            used(
                Href::Path("/"),
                "Home: nav_categories + home_rail during SSR",
                "الرئيسية: nav_categories و home_rail وقت الـ SSR",
            ),
            used(
                Href::Path("/cart"),
                "Cart: cart_set_qty, cart_remove, cart_add — the same functions behind the forms and the islands",
                "السلة: cart_set_qty و cart_remove و cart_add — نفس الدوال ورا الفورمز والـ islands",
            ),
            used(
                Href::Path("/search"),
                "Header search: search_products as you type",
                "البحث في الهيدر: search_products وانت بتكتب",
            ),
            used(
                Href::Category,
                "Listings: list_products for each filter, sort and page",
                "القوايم: list_products لكل فلتر وترتيب وصفحة",
            ),
        ],
        note: None,
        sources: &[
            "crates/app/src/api/mod.rs",
            "crates/app/src/api/cart.rs",
            "crates/app/src/server/cache.rs",
        ],
    },
    Feature {
        id: "transition",
        cat: Cat::Data,
        title: (
            "<Transition>: keep the old content while loading",
            "‏<Transition>: المحتوى القديم يفضل لحد ما الجديد يوصل",
        ),
        what: (
            "Like <Suspense>, but after the first load it keeps showing the previous content while its resource refetches, and reports pending so the UI can dim it.",
            "زي <Suspense>، بس بعد أول تحميل بيفضل عارض المحتوى القديم وهو بيجيب الجديد، وبيبلّغ إنه pending عشان الواجهة تعتّمه.",
        ),
        why: (
            "Paging or filtering a listing never flashes empty skeletons: the grid stays put and swaps once the next page is ready.",
            "التنقل بين الصفحات أو الفلترة عمرها ما بتفضّي الشاشة لـ skeletons: الشبكة بتفضل مكانها وبتتبدل لما الصفحة الجاية تجهز.",
        ),
        used_in: &[used(
            Href::Category,
            "Listings: filters, sort and paging keep the old results on screen",
            "القوايم: الفلاتر والترتيب والصفحات بتسيب النتايج القديمة على الشاشة",
        )],
        note: Some((
            "The listing and the pager below both seed their first page from server props through an AsyncDerived that is ready on its first poll — a pending Resource inside an island breaks hydration after a client navigation (see Sharp edges).",
            "القوايم والـ pager اللي تحت الاتنين بياخدوا أول صفحة من props السيرفر عن طريق AsyncDerived جاهز من أول poll — لإن Resource مستني جوه island بيكسر الـ hydration بعد التنقل (شوف «حاجات جرحتنا»).",
        )),
        sources: &[
            "crates/app/src/pages/category/explorer.rs",
            "crates/app/src/pages/showcase/islands.rs",
        ],
    },
    Feature {
        id: "sim",
        cat: Cat::Data,
        title: ("Simulated latency and faults", "بطء وأعطال تجريبية"),
        what: (
            "Every catalogue read waits its widget's pretend backend latency (a base × a per-widget factor) and can be made to fail. A cookie sets both per visitor, so streaming and error boundaries can be felt, not imagined.",
            "كل قراءة من الكتالوج بتستنى زمن تجريبي للباك إند (أساس × معامل لكل widget) وممكن تخليها تفشل. كوكي بتظبط الاتنين لكل زائر، عشان تحس بالبث والـ error boundaries بنفسك.",
        ),
        why: (
            "Real backends are slow and sometimes down. The UI has to be designed for that, not for localhost.",
            "الباك إند الحقيقي بيبطأ وساعات بيقع. والواجهة لازم تتصمم على كده، مش على localhost.",
        ),
        used_in: &[used(
            Href::Path("/"),
            "Every read on every page",
            "كل قراءة في كل صفحة",
        )],
        note: None,
        sources: &["crates/app/src/server/sim.rs", "crates/server/src/main.rs"],
    },
    // ── Forms ──────────────────────────────────────────────────────────────────────────────────
    Feature {
        id: "action-form",
        cat: Cat::Forms,
        title: (
            "ActionForm + ServerAction: progressive forms",
            "‏ActionForm و ServerAction: فورمز شغالة في كل الأحوال",
        ),
        what: (
            "<ActionForm> renders a real <form method=post> aimed at the server function. Without JavaScript the browser posts it and the server redirects back; with JavaScript it dispatches the ServerAction and updates in place, with pending and the result as signals.",
            "‏<ActionForm> بيطلع <form method=post> حقيقي رايح لدالة السيرفر. من غير JavaScript المتصفح بيبعته والسيرفر يرجّعك لنفس الصفحة؛ ومع JavaScript بيشغّل الـ ServerAction ويحدّث مكانه، والـ pending والنتيجة متاحين كـ signals.",
        ),
        why: (
            "Add to cart and checkout must work on every phone, good network or not — one server function serves both paths.",
            "«أضف للسلة» والدفع لازم يشتغلوا على أي موبايل، النت كويس أو لأ — ودالة سيرفر واحدة بتخدم الطريقتين.",
        ),
        used_in: &[
            used(
                Href::Path("/cart"),
                "Cart: every quantity and remove control is a stock ActionForm in a server component",
                "السلة: كل تحكم في الكمية والحذف ActionForm عادي جوه كومبوننت سيرفر",
            ),
            used(
                Href::Product,
                "Product page: add to cart — an ActionForm in the purchase panel (no JS: it posts and comes back with ?added=)",
                "صفحة المنتج: «أضف للسلة» — ActionForm في لوحة الشراء (من غير JS: بيبعت ويرجع بـ ‎?added=‎)",
            ),
            used(
                Href::Path("/checkout"),
                "Checkout: the address step, checked by the same Rust rules in the browser and on the server",
                "الدفع: خطوة العنوان، بتتراجع بنفس قواعد Rust في المتصفح وعلى السيرفر",
            ),
        ],
        note: Some((
            "Two styles side by side: the cart page's forms live in a server component, so with JavaScript the islands router posts them and diffs the page the server sends back — no island at all. The product panel, checkout and this demo dispatch the ServerAction from an island.",
            "طريقتين جنب بعض: فورمز صفحة السلة جوه كومبوننت سيرفر، فمع JavaScript الـ islands router هو اللي بيبعتها ويعمل diff للصفحة اللي راجعة — من غير أي island. ولوحة الشراء والدفع والديمو ده بيشغّلوا الـ ServerAction من جوه island.",
        )),
        sources: &[
            "crates/app/src/pages/cart/page.rs",
            "crates/app/src/pages/product/panel.rs",
            "crates/app/src/pages/showcase/notes.rs",
        ],
    },
    // ── Routing & navigation ───────────────────────────────────────────────────────────────────
    Feature {
        id: "islands-router",
        cat: Cat::Routing,
        title: (
            "islands-router: app-like navigation, no SPA",
            "‏islands-router: تنقل زي الأبلكيشن من غير SPA",
        ),
        what: (
            "Same-origin links and forms are intercepted; the next page's HTML is fetched and diffed into the current DOM inside a View Transition. Islands are skipped by the diff, so their state survives the navigation.",
            "اللينكات والفورمز اللي على نفس الموقع بيتم اعتراضها: الـ HTML بتاع الصفحة الجاية بيتجاب ويتعمله diff جوه الـ DOM الحالي جوه View Transition. والـ islands مش بيتعملها diff، فحالتها بتفضل زي ما هي.",
        ),
        why: (
            "Page changes without a white flash and without losing the open cart, while every page is still server-rendered, cacheable HTML that works without JavaScript.",
            "تنقل من غير وميض أبيض ومن غير ما السلة المفتوحة تتقفل، وفي نفس الوقت كل صفحة HTML متجهز على السيرفر، بيتعمله كاش، وشغال من غير JavaScript.",
        ),
        used_in: &[
            used(
                Href::Path("/"),
                "Every link and form on every page",
                "كل لينك وكل فورم في كل صفحة",
            ),
            used(
                Href::Path("/cart"),
                "Cart: each post is diffed in place; the lines are a keyed <For>",
                "السلة: كل طلب بيتعمله diff مكانه؛ والسطور <For> بمفاتيح",
            ),
            used(
                Href::Path("/checkout"),
                "Checkout: each step is a form the router posts; the server redirects to the next",
                "الدفع: كل خطوة فورم الراوتر بيبعته؛ والسيرفر بيحوّل للخطوة اللي بعدها",
            ),
            used(
                Href::Product,
                "Product → product replaces the islands (a keyed <For>); a language switch keeps them",
                "من منتج لمنتج الـ islands بتتغير (‏<For> بمفتاح)؛ وتبديل اللغة بيسيبها زي ما هي",
            ),
        ],
        note: None,
        sources: &[
            "crates/app/src/lib.rs",
            "crates/app/src/pages/cart/page.rs",
            "leptos-0.8.22/leptos/src/hydration/islands_routing.js",
        ],
    },
    Feature {
        id: "router",
        cat: Cat::Routing,
        title: (
            "Router: params and query drive the server render",
            "الراوتر: الـ params والـ query بيحددوا اللي يترسم",
        ),
        what: (
            "/c/:slug and /p/:slug are typed routes. use_params_map and use_query_map read the path and the query in server components, and the page is rendered for that exact URL — filters, sort, page and variant included.",
            "‏/c/:slug و /p/:slug ‏routes بأنواع واضحة. و use_params_map و use_query_map بيقروا المسار والـ query في كومبوننتس السيرفر، والصفحة بتترسم للـ URL ده بالظبط — بالفلاتر والترتيب والصفحة والاختيار.",
        ),
        why: (
            "Every filter, sort, page and variant is a URL you can share, cache at the edge and open without JavaScript: the URL is the state.",
            "كل فلتر وترتيب وصفحة واختيار ليه URL تقدر تبعته، ويتعمله كاش على الـ CDN، ويفتح من غير JavaScript: الـ URL هو الحالة.",
        ),
        used_in: &[
            used(
                Href::CategoryQuery,
                "Category: use_params_map for :slug, the query for filters/sort/page",
                "القسم: ‏use_params_map للـ ‎:slug‎، والـ query للفلاتر والترتيب والصفحة",
            ),
            used(
                Href::ProductVariant,
                "Product page: ?v= picks the variant (variant values are links without JavaScript)",
                "صفحة المنتج: ‎?v=‎ بيختار الاختيار (والاختيارات لينكات من غير JavaScript)",
            ),
            used(
                Href::Path("/search?deal=1&sort=discount"),
                "Search and the home page's “See all” links",
                "البحث ولينكات «عرض الكل» في الرئيسية",
            ),
            used(
                Href::Path("/orders"),
                "Order lookup: a GET form; a known reference redirects to /orders/:id",
                "البحث عن طلب: فورم GET؛ والرقم الصح بيحوّلك لـ ‎/orders/:id‎",
            ),
        ],
        note: None,
        sources: &[
            "crates/app/src/lib.rs",
            "crates/app/src/pages/category.rs",
            "crates/app/src/pages/product.rs",
            "crates/app/src/api/types.rs",
        ],
    },
    Feature {
        id: "router-patches",
        cat: Cat::Routing,
        title: (
            "islands-router: what we had to patch",
            "الـ islands-router: اللي اضطرينا نرقّعه",
        ),
        what: (
            "The router itself is a small script (leptos/src/hydration/islands_routing.js) that diffs fetched HTML into the page. The app wraps it with one more script (components/shell.js, inlined once, no WASM): realign <head> before each diff, keep <noscript> inert, manage scroll, always revalidate, send the pressed submit button, warm pages on intent, show progress and failed navigations, and name only the clicked card for the photo morph. The server helps too: router responses carry no <noscript>.",
            "الراوتر نفسه سكريبت صغير (‏leptos/src/hydration/islands_routing.js) بيعمل diff للـ HTML اللي بيجيبه جوه الصفحة. والأبلكيشن بيلفّه بسكريبت تاني (‏components/shell.js، مرة واحدة في الصفحة ومن غير WASM): يظبط الـ <head> قبل كل diff، ويخلي الـ <noscript> مالوش أثر، ويتحكم في الـ scroll، ويتأكد من السيرفر كل مرة، ويبعت الزرار اللي اتداس عليه مع الفورم، ويجهّز الصفحة قبل الضغط، ويوري التقدم والتنقل اللي فشل، ويدي اسم الـ morph للكارت اللي اتداس عليه بس. والسيرفر بيساعد كمان: مفيش <noscript> refresh في ردود الراوتر.",
        ),
        why: (
            "These are the gaps between “works in the example” and “works in a shop”: without them a cross-page navigation could leave the old page on screen, a no-JS redirect could fire for JS users, every new page opened at the old scroll position, and the cart's “remove” button posted as “update”.",
            "دي الفجوات بين «شغال في المثال» و«شغال في متجر»: من غيرها التنقل بين صفحتين ممكن يسيب الصفحة القديمة على الشاشة، وتحويل الـ no-JS ممكن يشتغل للي عندهم JS، وكل صفحة جديدة تفتح في نفس مكان الـ scroll القديم، وزرار «احذف» في السلة يتبعت كأنه «حدّث».",
        ),
        used_in: &[
            used(
                Href::Path("/"),
                "Every page: the shell inlines shell.js once (21 KB, about 7 KB gzipped)",
                "كل صفحة: الـ shell بيحط shell.js مرة واحدة (‏21 KB، حوالي 7 KB مضغوط)",
            ),
            used(
                Href::Path("/cart"),
                "Cart: remove buttons reach the server with their value",
                "السلة: زرار الحذف بيوصل للسيرفر بقيمته",
            ),
        ],
        note: None,
        sources: &[
            "crates/app/src/components/shell.js",
            "crates/app/src/components/shell.rs",
            "crates/app/src/lib.rs",
        ],
    },
    // ── Platform & caching ─────────────────────────────────────────────────────────────────────
    Feature {
        id: "caching",
        cat: Cat::Platform,
        title: (
            "HTTP caching hooks: Cache-Control, CDN-Cache-Control, Cache-Tag",
            "الكاش: Cache-Control و CDN-Cache-Control و Cache-Tag",
        ),
        what: (
            "Each page decides its caching synchronously, before streaming starts. Catalogue pages tell browsers to revalidate (no-cache — language and theme live in cookies) and tell the edge to keep a short-lived copy (CDN-Cache-Control) tagged for targeted purges; anything personal is private, no-store. Reads behind them go through a moka cache keyed by tenant + generation.",
            "كل صفحة بتقرر الكاش بتاعها بشكل متزامن قبل ما البث يبدأ. صفحات الكتالوج بتقول للمتصفح «اتأكد كل مرة» (‏no-cache — لإن اللغة والثيم في الكوكيز) وبتقول للـ CDN «احتفظ بنسخة قصيرة العمر» (‏CDN-Cache-Control) عليها تاجات للمسح بالتحديد؛ وأي حاجة شخصية private, no-store. والقراءات وراها بتعدّي على كاش moka بمفتاح المتجر + الـ generation.",
        ),
        why: (
            "Cloudflare can serve the catalogue from the edge to every visitor, and a price change purges exactly that product's pages by tag.",
            "‏Cloudflare يقدر يقدّم الكتالوج من أقرب سيرفر لكل زائر، وتغيير سعر بيمسح صفحات المنتج ده بس بالتاج.",
        ),
        used_in: &[
            used(
                Href::Path("/"),
                "Home: edge copy (CDN-Cache-Control) + Cache-Tag: t{tenant}",
                "الرئيسية: نسخة على الـ CDN (‏CDN-Cache-Control) مع Cache-Tag: t{tenant}",
            ),
            used(
                Href::Product,
                "Product page: Cache-Tag t{tenant}, t{tenant}-p{id}, t{tenant}-cat{id} — a price change purges just those",
                "صفحة المنتج: ‏Cache-Tag ‏t{tenant} و t{tenant}-p{id} و t{tenant}-cat{id} — تغيير السعر بيمسح دول بس",
            ),
            used(
                Href::Path("/cart"),
                "Cart, checkout, orders: private, no-store",
                "السلة والدفع والطلبات: private, no-store",
            ),
        ],
        note: None,
        sources: &[
            "crates/app/src/server/cache.rs",
            "crates/app/src/api/mod.rs",
            "crates/app/src/pages/product.rs",
        ],
    },
    Feature {
        id: "tenancy",
        cat: Cat::Platform,
        title: ("Multi-tenancy by Host", "تعدد المتاجر بالـ Host"),
        what: (
            "The first label of the Host header picks the tenant before anything renders (nile.…, souq.…); a cookie covers hosts that have none. Name, logo, default language, catalogue, delivery rules and cache tags all follow; the look is one platform-wide palette.",
            "أول جزء في الـ Host بيحدد المتجر قبل أي رسم (‎nile.…‎ و ‎souq.…‎)؛ والكوكي بتغطي الـ hosts اللي من غيره. والاسم واللوجو واللغة الافتراضية والكتالوج وقواعد التوصيل وتاجات الكاش كلهم بيمشوا وراه؛ والشكل نفسه واحد للمنصة كلها.",
        ),
        why: (
            "One deployment serves every shop on its own subdomain — the same model as Madar's per-shop subdomains.",
            "نسخة واحدة من التطبيق بتخدم كل المحلات، كل محل على الـ subdomain بتاعه — نفس فكرة الـ subdomains في مدار.",
        ),
        used_in: &[used(
            Href::Path("/"),
            "Every request (resolved in request::init)",
            "كل request (بيتحدد في request::init)",
        )],
        note: None,
        sources: &[
            "crates/app/src/tenant.rs",
            "crates/app/src/request.rs",
            "crates/app/src/server/mod.rs",
        ],
    },
    // ── i18n / RTL ─────────────────────────────────────────────────────────────────────────────
    Feature {
        id: "i18n",
        cat: Cat::I18n,
        title: (
            "Language, direction, numerals, theme — no reload",
            "اللغة والاتجاه والأرقام والثيم — من غير ريلود",
        ),
        what: (
            "The toggles are plain links to /__prefs. The islands router follows the redirect and diffs the new HTML in, which rewrites <html lang dir data-theme>; a MutationObserver mirrors those attributes into the Ui store, so islands re-render their own text.",
            "أزرار التبديل لينكات عادية لـ ‎/__prefs‎. الـ islands router بيتبع التحويل ويعمل diff للـ HTML الجديد، فبيتغير <html lang dir data-theme>؛ وفيه MutationObserver بينقل الـ attributes دي لـ store الـ Ui، فالـ islands بتعيد رسم نصوصها.",
        ),
        why: (
            "Egyptian shoppers switch between Arabic and English — and between ٣ and 3 — mid-session. It has to be instant and keep the cart open.",
            "المشتري المصري بيبدّل بين العربي والإنجليزي — وبين ٣ و 3 — وهو بيتسوق. لازم يحصل فورًا ومن غير ما السلة تتقفل.",
        ),
        used_in: &[
            used(
                Href::Path("/#rail-deals"),
                "Home: the countdown follows the language and numerals",
                "الرئيسية: العد التنازلي بيتبع اللغة والأرقام",
            ),
            used(
                Href::CategoryQuery,
                "Listings: switching language keeps your filters (shell.js points next= at the live address)",
                "القوايم: تبديل اللغة بيحافظ على الفلاتر (‏shell.js بيوجّه next=‎ للعنوان الحالي)",
            ),
            used(
                Href::Path("/checkout"),
                "Checkout: field errors in the visitor's language, from the same Rust rules on both sides",
                "الدفع: أخطاء الخانات بلغة الزائر، من نفس قواعد Rust على الناحيتين",
            ),
        ],
        note: None,
        sources: &[
            "crates/server/src/main.rs",
            "crates/app/src/client.rs",
            "crates/app/src/prefs.rs",
            "crates/app/src/format.rs",
        ],
    },
    // ── Media & motion ─────────────────────────────────────────────────────────────────────────
    Feature {
        id: "view-transitions",
        cat: Cat::Media,
        title: ("View Transitions API", "‏View Transitions API"),
        what: (
            "The islands router wraps each DOM diff in document.startViewTransition(). Give an element on the old and the new page the same view-transition-name and the browser morphs one into the other.",
            "الـ islands router بيلف كل diff جوه document.startViewTransition()‎. لو اديت عنصر في الصفحة القديمة والجديدة نفس الـ view-transition-name، المتصفح بيحوّل واحد للتاني بحركة.",
        ),
        why: (
            "A product photo can fly from its card into the product page: app-like continuity with plain links and no animation code.",
            "صورة المنتج تقدر تطير من الكارت لصفحة المنتج: إحساس الأبلكيشن بلينكات عادية ومن غير كود أنيميشن.",
        ),
        used_in: &[
            used(
                Href::Category,
                "Any product card → its page: the photo flies into the product's main image",
                "أي كارت منتج → صفحته: الصورة بتطير لصورة المنتج الرئيسية",
            ),
            used(
                Href::Path("/"),
                "Every other navigation: a cross-fade",
                "أي تنقل تاني: cross-fade",
            ),
        ],
        note: Some((
            "The card's image box carries data-vt=\"p{id}\" and the product page names its main image p{id}. shell.js names only the card that was clicked: a product can sit on a page twice, and duplicate names abort the transition.",
            "صورة الكارت عليها data-vt=\"p{id}\" وصفحة المنتج بتدي صورتها الرئيسية الاسم p{id}. و shell.js بيدي الاسم للكارت اللي اتداس عليه بس: المنتج ممكن يتكرر في الصفحة، والأسماء المكررة بتلغي الحركة.",
        )),
        sources: &[
            "crates/app/src/components/shell.js",
            "crates/app/src/components/product_card.rs",
            "leptos-0.8.22/leptos/src/hydration/islands_routing.js",
        ],
    },
    Feature {
        id: "motion",
        cat: Cat::Media,
        title: (
            "Apple-style motion in Rust/WASM: springs you can grab",
            "حركة على طريقة Apple في Rust/WASM: سبرينج تقدر تمسكه",
        ),
        what: (
            "components::motion solves a damped spring in closed form every frame, set by Apple's two parameters (damping, response). Motion starts from the value on screen, inherits the finger's release velocity, projects momentum to pick where a flick lands, and rubber-bands past the edges — Rust compiled into the islands' WASM, driven by requestAnimationFrame.",
            "‏components::motion بيحل معادلة السبرينج بشكل مباشر في كل فريم، بالمعاملين بتوع Apple (‏damping و response). الحركة بتبدأ من مكان العنصر على الشاشة، وبتاخد سرعة صباعك لحظة ما تسيبه، وبتتوقع العنصر هيقف فين بعد الـ flick، وبتقاوم بنعومة لما تشده بعد الحافة — كل ده Rust متحوّل لـ WASM الـ islands، وشغال على requestAnimationFrame.",
        ),
        why: (
            "Drawers and bottom sheets that follow the finger, can be caught mid-flight and settle with the hand's momentum feel native on phones; CSS transitions can't be interrupted cleanly or handed a velocity.",
            "الأدراج والـ bottom sheets اللي بتمشي مع صباعك، وتقدر تمسكها وهي بتتحرك، وتقف بنفس اندفاع إيدك بتحس إنها جزء من الموبايل؛ والـ CSS transitions ماتقدرش تتقاطع بنعومة ولا تاخد سرعة.",
        ),
        used_in: &[
            used(
                Href::Product,
                "Product gallery: swipe, flick, rubber-band ends, a pinch-zoom viewer",
                "صور المنتج: سحب وقذف وحواف مطاطية وعارض بتكبير الصباعين",
            ),
            used(
                Href::Path("/cart"),
                "The cart drawer and the listing's filter sheet: drag to dismiss",
                "درج السلة وشيت الفلاتر في القوايم: اسحب عشان تقفل",
            ),
            used(
                Href::Path("/ui#overlays"),
                "UI kit: every drawer",
                "الـ UI kit: كل الأدراج",
            ),
        ],
        note: None,
        sources: &[
            "crates/app/src/components/motion.rs",
            "crates/app/src/components/drawer.rs",
            "crates/app/src/pages/product/gallery.rs",
        ],
    },
    Feature {
        id: "images",
        cat: Cat::Media,
        title: (
            "Responsive images: srcset, sizes, LQIP",
            "صور متجاوبة: srcset و sizes و LQIP",
        ),
        what: (
            "Every image is a key with a WebP width ladder, an aspect ratio, a 20 px blurred placeholder and a dominant colour (media::Img). The browser picks the rung from srcset + sizes; the box is reserved, so nothing jumps.",
            "كل صورة عبارة عن مفتاح ليه سلسلة مقاسات WebP، ونسبة أبعاد، وصورة مموهة ٢٠ بكسل، ولون غالب (media::Img). المتصفح بيختار المقاس من srcset و sizes؛ والمساحة محجوزة فمفيش حاجة بتنط.",
        ),
        why: (
            "Product photos are most of a shop's bytes. Picking 400w instead of 960w on a phone is the biggest single win.",
            "صور المنتجات هي أغلب حجم المتجر. إنك تختار 400w بدل 960w على الموبايل هو أكبر مكسب لوحده.",
        ),
        used_in: &[
            used(
                Href::Path("/#latest-title"),
                "Home: “The latest” tiles and product cards",
                "الرئيسية: «الأحدث» وكروت المنتجات",
            ),
            used(
                Href::Path("/#categories"),
                "Home: category tiles",
                "الرئيسية: صور الأقسام",
            ),
            used(
                Href::Product,
                "Product gallery: the first photo is preloaded in <head> with the same sizes, so the browser fetches one file",
                "صور المنتج: أول صورة بتتعمل لها preload في الـ <head> بنفس الـ sizes، فالمتصفح بيجيب ملف واحد",
            ),
        ],
        note: None,
        sources: &[
            "crates/app/src/media.rs",
            "crates/app/src/components/image.rs",
            "tools/media/process.py",
        ],
    },
    // ── Accessibility ──────────────────────────────────────────────────────────────────────────
    Feature {
        id: "a11y",
        cat: Cat::A11y,
        title: ("Accessibility patterns", "أنماط الإتاحة"),
        what: (
            "Semantic HTML first, ARIA where HTML runs out: a modal sheet that moves focus in and back out, aria-live regions for status messages, labelled controls, and visible focus rings.",
            "‏HTML الدلالي الأول، وARIA لما الـ HTML ميكفيش: شاشة منبثقة بتنقل الفوكس جواها وترجّعه، ومناطق aria-live لرسائل الحالة، وعناصر تحكم ليها labels، وحلقة فوكس واضحة.",
        ),
        why: (
            "Screen-reader and keyboard users buy things too, and the same structure is what search engines read.",
            "اللي بيستخدموا قارئ الشاشة والكيبورد بيشتروا برضه، ونفس الهيكل ده هو اللي محركات البحث بتقراه.",
        ),
        used_in: &[
            used(
                Href::Path("/search"),
                "Header search: an ARIA combobox (aria-activedescendant, ↑/↓, Esc)",
                "البحث في الهيدر: ‏combobox (‏aria-activedescendant، والأسهم، و Esc)",
            ),
            used(
                Href::Path("/ui#overlays"),
                "Drawers on a native <dialog>; toasts in a role=status region",
                "أدراج على <dialog> أصلي؛ والإشعارات في منطقة role=status",
            ),
            used(
                Href::Path("/checkout"),
                "Checkout: focus moves to each new step's heading; an invalid form focuses the first problem and says how many",
                "الدفع: الفوكس بيروح لعنوان كل خطوة جديدة؛ والفورم الغلط بيركّز على أول مشكلة ويقول عددهم",
            ),
            used(
                Href::Product,
                "Product gallery: arrows, Home/End, Enter to open, Escape to close",
                "صور المنتج: الأسهم و Home/End و Enter للفتح و Escape للقفل",
            ),
            used(
                Href::Path("/"),
                "Skip link, labelled sections, role=status fallbacks",
                "لينك «انتقل للمحتوى»، وأقسام ليها عناوين، و role=status للأعطال",
            ),
        ],
        note: None,
        sources: &[
            "crates/app/src/pages/search/instant.rs",
            "crates/app/src/components/drawer.rs",
            "crates/app/src/components/toast.rs",
            "crates/app/src/pages/showcase/islands.rs",
        ],
    },
];

/// Display number (01, 02, …) of a feature.
pub fn number(id: &str) -> usize {
    FEATURES
        .iter()
        .position(|f| f.id == id)
        .map(|i| i + 1)
        .unwrap_or(0)
}
