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
        ],
        note: None,
        sources: &[
            "crates/frontend/src/lib.rs",
            "crates/app/src/pages/home.rs",
            "crates/app/src/lib.rs",
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
            "Leptos streams a route in one of four modes. OutOfOrder (default) sends each <Suspense> as soon as it resolves, but slots it in with a few bytes of inline JavaScript. InOrder waits at each <Suspense> in document order and needs no script. PartiallyBlocked holds the first byte until blocking resources resolve, so <head> and key HTML can use data. Async waits for everything.",
            "‏Leptos بيبعت الـ route بواحدة من أربع طرق. OutOfOrder (الافتراضي) بيبعت كل <Suspense> أول ما يخلص، بس بيحطه مكانه بشوية JavaScript جوه الصفحة. InOrder بيستنى عند كل <Suspense> بترتيب الصفحة ومش محتاج أي سكريبت. PartiallyBlocked بيأخر أول بايت لحد ما الـ blocking resources تخلص، فالـ <head> والـ HTML المهم يقدروا يستخدموا البيانات. وAsync بيستنى كله.",
        ),
        why: (
            "Out-of-order is the fastest first paint but shows skeletons forever without JavaScript, and its <head> can't wait for data — so each route picks its mode: the product page blocks for its own data (title, OpenGraph, JSON-LD), listings and checkout stream in order, home streams out of order with an in-order twin for no-JS visitors.",
            "الـ OutOfOrder أسرع أول ظهور، بس من غير JavaScript الـ skeletons بتفضل للأبد، والـ <head> بتاعه مايقدرش يستنى البيانات — عشان كده كل route بيختار طريقته: صفحة المنتج بتستنى بياناتها (العنوان وOpenGraph وJSON-LD)، والقوايم والدفع بيتبعتوا بالترتيب، والرئيسية بأي ترتيب ومعاها نسخة بالترتيب للي من غير JavaScript.",
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
                "Listings, search, cart, checkout, orders: InOrder",
                "القوايم والبحث والسلة والدفع والطلبات: InOrder",
            ),
            used(
                Href::Product,
                "Product page: PartiallyBlocked",
                "صفحة المنتج: PartiallyBlocked",
            ),
        ],
        note: None,
        sources: &["crates/app/src/lib.rs"],
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
                Href::Path("/no-such-page"),
                "404: popular categories (blocking)",
                "صفحة 404: الأقسام (blocking)",
            ),
        ],
        note: None,
        sources: &[
            "crates/app/src/pages/home.rs",
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
                Href::Path("/no-such-page"),
                "404 page (SectionError)",
                "صفحة 404 (‏SectionError)",
            ),
        ],
        note: None,
        sources: &[
            "crates/app/src/pages/errors.rs",
            "crates/app/src/pages/home.rs",
            "crates/app/src/server/sim.rs",
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
        used_in: &[used(
            Href::Path("/"),
            "Home: title, OpenGraph, WebSite JSON-LD",
            "الرئيسية: العنوان وOpenGraph و WebSite JSON-LD",
        )],
        note: None,
        sources: &["crates/app/src/pages/home.rs", "crates/app/src/lib.rs"],
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
        used_in: &[used(
            Href::Path("/#rail-deals"),
            "Home: the countdown is a signal ticking once a second",
            "الرئيسية: العد التنازلي signal بيتحدث كل ثانية",
        )],
        note: Some((
            "Memo and Effect aren't used by a page yet — the calculator below is their usage.",
            "الـ Memo والـ Effect لسه مش مستخدمين في صفحة — الحاسبة اللي تحت هي استخدامهم.",
        )),
        sources: &[
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
        used_in: &[used(
            Href::Path("/#rail-deals"),
            "Home: the countdown reads the shared Ui store",
            "الرئيسية: العد التنازلي بيقرا store الـ Ui المشترك",
        )],
        note: Some((
            // The cart islands are on agent/commerce (2548a2f); the shell mounts the badge and drawer.
            "CartStore (stores.rs) is the one signal behind the header badge; the commerce branch's badge, drawer and quick-add islands all read and write it, mounted by the app shell.",
            "‏CartStore (‏stores.rs) هو الـ signal الوحيد ورا عدّاد الهيدر؛ وجزر العدّاد والدرج والإضافة السريعة في فرع الشراء كلها بتقرا وتكتب فيه، والـ shell هو اللي بيركّبها.",
        )),
        sources: &[
            "crates/app/src/client.rs",
            "crates/app/src/stores.rs",
            "crates/app/src/prefs.rs",
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
        used_in: &[used(
            Href::Path("/"),
            "Home: nav_categories + home_rail during SSR",
            "الرئيسية: nav_categories و home_rail وقت الـ SSR",
        )],
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
        used_in: &[],
        note: Some((
            "Not used by a page yet: listings page through plain links (server-rendered). This island is its usage.",
            "لسه مش مستخدم في صفحة: صفحات القوايم بتتقلب بلينكات عادية (بتترسم على السيرفر). الـ island دي هي استخدامه.",
        )),
        sources: &[
            "crates/app/src/pages/showcase/islands.rs",
            "crates/app/src/api/mod.rs",
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
        // `/cart` lands with agent/commerce (2548a2f): page.rs renders every control as a stock ActionForm.
        used_in: &[used(
            Href::Path("/cart"),
            "Cart: every quantity, remove and add control is a stock ActionForm",
            "السلة: كل تحكم في الكمية والحذف والإضافة ActionForm عادي",
        )],
        note: Some((
            "On /cart the forms live in a server component, so with JavaScript the islands router posts them and diffs the page the server sends back — no island at all. This demo shows the other style: an island that dispatches the ServerAction itself.",
            "في ‎/cart‎ الفورمز جوه كومبوننت سيرفر، فمع JavaScript الـ islands router هو اللي بيبعتها ويعمل diff للصفحة اللي راجعة — من غير أي island. والديمو ده بيوري الطريقة التانية: island بتشغّل الـ ServerAction بنفسها.",
        )),
        sources: &[
            "crates/app/src/pages/showcase/notes.rs",
            "crates/app/src/api/cart.rs",
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
        // `/cart` lands with agent/commerce (2548a2f): keyed <For> lines, forms the router posts and diffs.
        used_in: &[
            used(
                Href::Path("/"),
                "Every link and form on every page",
                "كل لينك وكل فورم في كل صفحة",
            ),
            used(
                Href::Path("/cart"),
                "Cart: posts diffed in place, keyed <For> lines",
                "السلة: الطلبات بتتعمل diff مكانها، وسطور <For> بمفاتيح",
            ),
        ],
        note: None,
        sources: &[
            "crates/app/src/lib.rs",
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
        used_in: &[used(
            Href::Path("/search?deal=1&sort=discount"),
            "Home's “See all” links",
            "لينكات «عرض الكل» في الرئيسية",
        )],
        note: Some((
            "The listing and product pages that read these params land with the browse and product pages; this demo reads its own query the same way.",
            "صفحات القوايم والمنتج اللي بتقرا الـ params دي هتيجي مع صفحات التصفح والمنتج؛ والديمو ده بيقرا الـ query بتاعته بنفس الطريقة.",
        )),
        sources: &["crates/app/src/lib.rs", "crates/app/src/api/types.rs"],
    },
    // ── Platform & caching ─────────────────────────────────────────────────────────────────────
    Feature {
        id: "caching",
        cat: Cat::Platform,
        title: (
            "HTTP caching hooks: Cache-Control + Cache-Tag",
            "الكاش: Cache-Control و Cache-Tag",
        ),
        what: (
            "Each page decides its caching synchronously, before streaming starts: catalogue pages are public with a short edge TTL and Cache-Tags; anything personal is private, no-store. Reads behind them go through a moka cache keyed by tenant + generation.",
            "كل صفحة بتقرر الكاش بتاعها بشكل متزامن قبل ما البث يبدأ: صفحات الكتالوج public بـ TTL قصير على الـ CDN ومعاها Cache-Tags؛ وأي حاجة شخصية private, no-store. والقراءات وراها بتعدّي على كاش moka بمفتاح المتجر + الـ generation.",
        ),
        why: (
            "Cloudflare can serve the catalogue from the edge to every visitor, and a price change purges exactly that product's pages by tag.",
            "‏Cloudflare يقدر يقدّم الكتالوج من أقرب سيرفر لكل زائر، وتغيير سعر بيمسح صفحات المنتج ده بس بالتاج.",
        ),
        used_in: &[used(
            Href::Path("/"),
            "Home: public + Cache-Tag: t{tenant}",
            "الرئيسية: ‏public مع Cache-Tag: t{tenant}",
        )],
        note: None,
        sources: &[
            "crates/app/src/server/cache.rs",
            "crates/app/src/api/mod.rs",
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
        used_in: &[used(
            Href::Path("/#rail-deals"),
            "Home: the countdown follows the language and numerals",
            "الرئيسية: العد التنازلي بيتبع اللغة والأرقام",
        )],
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
        used_in: &[used(
            Href::Path("/"),
            "Every navigation (cross-fade by default)",
            "كل تنقل (cross-fade افتراضيًا)",
        )],
        note: Some((
            "The card → product photo morph needs a shared view-transition-name on both pages; it lands with the design system's Picture (vt_name) and the product page. The layout switch below is a working morph.",
            "حركة الصورة من الكارت لصفحة المنتج محتاجة view-transition-name مشترك في الصفحتين؛ وهتيجي مع Picture بتاع الـ design system (‏vt_name) وصفحة المنتج. وتبديل الشكل اللي تحت حركة شغالة فعلًا.",
        )),
        sources: &[
            "leptos-0.8.22/leptos/src/hydration/islands_routing.js",
            "style/pages/showcase.css",
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
        ],
        note: None,
        sources: &[
            "crates/app/src/media.rs",
            "tools/media/process.py",
            "crates/app/src/pages/home.rs",
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
        used_in: &[used(
            Href::Path("/"),
            "Home: labelled sections, list semantics, role=status fallbacks",
            "الرئيسية: أقسام ليها عناوين، وقوايم، وrole=status للأعطال",
        )],
        note: Some((
            "The instant-search combobox, the cart drawer's focus handling and the toast live region land with the browse, commerce and design-system branches; the demos below use the same patterns.",
            "البحث الفوري (combobox) والفوكس في درج السلة ومنطقة الإشعارات هييجوا مع فروع التصفح والشراء والـ design system؛ والديموهات اللي تحت بتستخدم نفس الأنماط.",
        )),
        sources: &[
            "crates/app/src/pages/showcase/islands.rs",
            "crates/app/src/pages/errors.rs",
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
