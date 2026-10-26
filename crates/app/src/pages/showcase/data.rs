//! What the showcase says: two parts (Leptos on the server, the TypeScript client) plus the one
//! decision card, and one entry per piece of the architecture as built — what it is, why it matters
//! here, where the app uses it, where the code lives.
//!
//! "Used in" lists only usages that exist in this repository. Numbers in prose are written as
//! `{{379}}` so the card can show them in the visitor's numerals (`Card` renders them through
//! `crate::format::digits`); digits that are code (`2307:1:2`, `0.8.22`) stay as they are.

use crate::prefs::{Lang, tr};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cat {
    Server,
    Client,
    Decisions,
}

impl Cat {
    pub const ALL: [Cat; 3] = [Cat::Server, Cat::Client, Cat::Decisions];

    /// URL (`?cat=`) and `data-cat` value.
    pub fn code(self) -> &'static str {
        match self {
            Cat::Server => "server",
            Cat::Client => "client",
            Cat::Decisions => "decisions",
        }
    }

    pub fn parse(s: &str) -> Option<Cat> {
        Cat::ALL.into_iter().find(|c| c.code() == s)
    }

    /// Short label (filter chips, card tags).
    pub fn short(self) -> (&'static str, &'static str) {
        match self {
            Cat::Server => ("Leptos on the server", "Leptos على السيرفر"),
            Cat::Client => ("The TypeScript client", "عميل TypeScript"),
            Cat::Decisions => ("Decisions", "قرارات"),
        }
    }

    pub fn label(self, lang: Lang) -> &'static str {
        let (en, ar) = self.short();
        tr(lang, en, ar)
    }

    /// One line under the part's heading.
    pub fn blurb(self, lang: Lang) -> &'static str {
        match self {
            Cat::Server => tr(
                lang,
                "Every page is a Leptos component rendered on the server into one complete HTML document. No Rust runs in the browser.",
                "كل صفحة كومبوننت Leptos بيترسم على السيرفر في مستند HTML واحد كامل. مفيش Rust بيشتغل في المتصفح.",
            ),
            Cat::Client => tr(
                lang,
                "One small ES module, bundled by esbuild. It changes state on the spot and asks the server only for data it doesn't have.",
                "ملف ES module واحد صغير، بيجمّعه esbuild. بيغيّر الحالة في ساعتها، ومابيكلمش السيرفر غير لما يحتاج بيانات مش عنده.",
            ),
            Cat::Decisions => tr(
                lang,
                "What we built first, what it cost, and why we changed course.",
                "عملنا إيه الأول، وكلّفنا كام، وليه غيّرنا الطريق.",
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
    /// `/p/<a product with several variants>`
    Product,
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
    /// Shown under "Used in": why the demo is the only usage, or where the numbers come from.
    pub note: Option<(&'static str, &'static str)>,
    pub sources: &'static [&'static str],
}

impl Feature {
    pub fn title(&self, lang: Lang) -> &'static str {
        tr(lang, self.title.0, self.title.1)
    }
}

/// In display order (grouped by part, in [`Cat::ALL`] order).
pub static FEATURES: &[Feature] = &[
    // ── Leptos on the server ───────────────────────────────────────────────────────────────────
    Feature {
        id: "in-order",
        cat: Cat::Server,
        title: ("In-order SSR on every page", "SSR بالترتيب في كل صفحة"),
        what: (
            "Every route is rendered with SsrMode::InOrder. Leptos renders the page top to bottom and streams it in that order, waiting at each <Suspense> for its data, so every byte that leaves the server is final HTML. No script has to move anything into place.",
            "كل route بيترسم بـ SsrMode::InOrder. ‏Leptos بيرسم الصفحة من فوق لتحت وبيبعتها بنفس الترتيب، وبيستنى عند كل <Suspense> لحد ما بياناته توصل، فكل بايت طالع من السيرفر HTML نهائي. مفيش سكريبت محتاج يحط حاجة في مكانها.",
        ),
        why: (
            "Pages must work without JavaScript, and the TypeScript router swaps whole server-rendered pages: both need complete HTML. The status code and cache headers are set before the first byte, so a 404 is a real 404.",
            "الصفحات لازم تشتغل من غير JavaScript، والراوتر بتاع TypeScript بيبدّل صفحات كاملة جاية من السيرفر: الاتنين محتاجين HTML كامل. وكود الحالة وهيدرز الكاش بيتحددوا قبل أول بايت، فالـ 404 بيبقى 404 بجد.",
        ),
        used_in: &[
            used(Href::Path("/"), "Home", "الرئيسية"),
            used(Href::Category, "Category listings", "صفحات الأقسام"),
            used(Href::Product, "Product page", "صفحة المنتج"),
            used(Href::Path("/cart"), "Cart and checkout", "السلة والدفع"),
        ],
        note: None,
        sources: &["crates/app/src/lib.rs", "crates/app/src/components/shell.rs"],
    },
    Feature {
        id: "streaming",
        cat: Cat::Server,
        title: ("Out-of-order streaming, on its own route", "البث بأي ترتيب، في route لوحده"),
        what: (
            "Leptos can also send the page's frame first and then each <Suspense> the moment its data resolves. Each section arrives as a <template> plus a small inline script that moves it into place. The demo route /showcase/stream is the only page here that streams this way.",
            "‏Leptos يقدر كمان يبعت إطار الصفحة الأول، وبعدين كل <Suspense> أول ما بياناته تخلص. كل قسم بيوصل كـ <template> ومعاه سكريبت صغير بيحطه في مكانه. صفحة الديمو ‎/showcase/stream‎ هي الوحيدة هنا اللي بتتبعت بالطريقة دي.",
        ),
        why: (
            "It gives the fastest first paint, but sections only land with JavaScript on, and a router that fetches the page as HTML gets it only once the slowest section is done. That's why the demo links carry data-router=\"off\": only a full page load lets you watch the stream.",
            "بيدّي أسرع أول رسمة، بس الأقسام مابتتحطش في مكانها غير لو الـ JavaScript شغال، والراوتر اللي بيجيب الصفحة كـ HTML مش هياخدها غير لما أبطأ قسم يخلص. عشان كده لينكات الديمو عليها data-router=\"off\": التحميل الكامل للصفحة هو الطريقة الوحيدة اللي تتفرج بيها على البث.",
        ),
        used_in: &[],
        note: Some((
            "Only the demo route streams out of order; every shop page renders in order (the card above).",
            "صفحة الديمو بس هي اللي بتتبعت بأي ترتيب؛ كل صفحات المتجر بتترسم بالترتيب (الكارت اللي فوق).",
        )),
        sources: &["crates/app/src/pages/showcase/stream.rs", "crates/app/src/lib.rs"],
    },
    Feature {
        id: "server-fns",
        cat: Cat::Server,
        title: ("Server functions are the data API", "دوال السيرفر هي الـ API"),
        what: (
            "#[server(endpoint = \"cache/stats\", input = GetUrl)] turns an async Rust function into an HTTP endpoint with a fixed URL. Server components call it like any function. The TypeScript client fetches the URL and gets JSON back, with money already formatted.",
            "‏#[server(endpoint = \"cache/stats\", input = GetUrl)]‎ بيحوّل دالة Rust ‏async لـ endpoint ليه URL ثابت. كومبوننتس السيرفر بتناديها زي أي دالة. وعميل TypeScript بيعمل fetch للـ URL ويرجعله JSON، والفلوس متنسقة جاهزة.",
        ),
        why: (
            "One Rust function serves the server's own reads and the client's fetches, so there's no second REST layer to keep in step. Fixed names keep URLs the same from build to build (default names carry a hash), so the client and the CDN can rely on them.",
            "دالة Rust واحدة بتخدم قراءات السيرفر نفسه وطلبات العميل، فمفيش طبقة REST تانية لازم تفضل ماشية معاها. والأسماء الثابتة بتخلّي الـ URLs زي ما هي من build للتاني (الأسماء الافتراضية فيها hash)، فالعميل والـ CDN يقدروا يعتمدوا عليها.",
        ),
        used_in: &[used(
            Href::Path("/cart"),
            "Cart prices: GET /api/cart/quote",
            "أسعار السلة: GET /api/cart/quote",
        )],
        note: None,
        sources: &["crates/app/src/api/mod.rs", "client/src/core/net.ts"],
    },
    Feature {
        id: "errors",
        cat: Cat::Server,
        title: ("Error boundaries, one per section", "حدود للأخطاء، واحد لكل قسم"),
        what: (
            "Each section that reads data has its own <ErrorBoundary>. When a read fails, that section says what failed and offers Retry, and the rest of the page renders normally. A failure that is known before the first byte becomes a full error page with a real 500 or 503.",
            "كل قسم بيقرا بيانات ليه <ErrorBoundary> خاص بيه. لو القراءة فشلت، القسم ده بس بيقول إيه اللي فشل ويديك «إعادة المحاولة»، وباقي الصفحة بيترسم عادي. ولو الفشل معروف قبل أول بايت، بتطلع صفحة خطأ كاملة بـ 500 أو 503 حقيقي.",
        ),
        why: (
            "A slow or broken recommendations service shouldn't take the product page down with it. ?fail=<widget> breaks one read on purpose, so every error state can be checked.",
            "لو خدمة الترشيحات بطيئة أو واقعة، ماينفعش توقّع صفحة المنتج معاها. و ‎?fail=<widget>‎ بيبوّظ قراءة واحدة بالقصد، عشان نقدر نجرّب كل حالة خطأ.",
        ),
        used_in: &[
            used(
                Href::Path("/?fail=deals"),
                "Home, with the deals rail failing",
                "الرئيسية، وقسم العروض فاشل",
            ),
            used(
                Href::ProductQuery("fail=related"),
                "Product page, with related products failing",
                "صفحة المنتج، والمنتجات المشابهة فاشلة",
            ),
        ],
        note: None,
        sources: &["crates/app/src/pages/errors.rs", "crates/app/src/server/sim.rs"],
    },
    Feature {
        id: "read-cache",
        cat: Cat::Server,
        title: ("The moka read cache", "كاش القراءة (moka)"),
        what: (
            "Catalogue reads go through one moka cache keyed tenant:generation:kind:key, the shape of Madar's MenuCache. Entries expire after their TTL, and bumping a shop's generation drops all of its entries at once, without a scan.",
            "قراءات الكتالوج كلها بتعدّي على كاش moka واحد، مفتاحه tenant:generation:kind:key، بنفس شكل MenuCache بتاع مدار. العناصر بتنتهي بعد الـ TTL بتاعها، ولو زوّدنا الـ generation بتاع محل، كل عناصره بتتشال مرة واحدة من غير ما نلف عليها.",
        ),
        why: (
            "Listings, rails and search results are computed once and served from memory to every visitor who asks the same thing. A catalogue edit is one counter bump, plus a CDN purge by Cache-Tag in production.",
            "الليستات والأقسام ونتايج البحث بتتحسب مرة واحدة وتتقدّم من الميموري لكل زائر بيطلب نفس الحاجة. وتعديل الكتالوج مجرد زيادة عداد، ومعاها purge للـ CDN بالـ Cache-Tag في الإنتاج.",
        ),
        used_in: &[
            used(Href::Path("/"), "Home rails", "أقسام الرئيسية"),
            used(Href::Category, "Category listings", "صفحات الأقسام"),
            used(Href::Path("/search?q=phone"), "Search", "البحث"),
        ],
        note: None,
        sources: &["crates/app/src/server/cache.rs", "crates/app/src/api/mod.rs"],
    },
    Feature {
        id: "tenancy",
        cat: Cat::Server,
        title: ("Tenancy by Host", "المحل من الـ Host"),
        what: (
            "One binary serves both shops. The shop is the first label of the Host header (nile.…, souq.…), as with Madar's per-shop subdomains. It is resolved before rendering, so the logo, the name and the default language are in the first byte.",
            "سيرفر واحد بيخدم المحلين. المحل بيتحدد من أول جزء في الـ Host ‏(nile.…‏ و souq.…)، زي الـ subdomains بتاعة كل محل في مدار. وبيتحدد قبل الرسم، فاللوجو والاسم واللغة الافتراضية بيبقوا في أول بايت.",
        ),
        why: (
            "Adding a shop is data, not a deployment. Hosts without a shop label, like a bare IP on the phone, fall back to a tenant cookie.",
            "إضافة محل جديد بيانات، مش deployment. والـ hosts اللي من غير اسم محل، زي IP على الموبايل، بترجع لكوكي المحل.",
        ),
        used_in: &[used(
            Href::Path("/"),
            "Every page: logo, name, catalogue, delivery terms",
            "كل صفحة: اللوجو والاسم والكتالوج وشروط التوصيل",
        )],
        note: None,
        sources: &["crates/app/src/tenant.rs", "crates/app/src/request.rs"],
    },
    Feature {
        id: "meta",
        cat: Cat::Server,
        title: ("Head and meta", "الـ head والـ meta"),
        what: (
            "<Title>, <Meta> and <Link> from leptos_meta are written synchronously from in-memory data. Title, description, canonical, OpenGraph and JSON-LD are all in the <head> of the first chunk.",
            "‏<Title> و <Meta> و <Link> من leptos_meta بيتكتبوا في ساعتها من بيانات في الميموري. العنوان والوصف والـ canonical والـ OpenGraph والـ JSON-LD كلهم بيبقوا في الـ <head> من أول chunk.",
        ),
        why: (
            "Crawlers and link previews read the first response and never run scripts. With in-order SSR and no head that waits for data, they see exactly what the shopper sees.",
            "محركات البحث ومعاينات اللينكات بتقرا أول رد ومابتشغّلش سكريبتات. ومع الـ SSR بالترتيب ومن غير head بيستنى بيانات، بيشوفوا بالظبط اللي الزبون بيشوفه.",
        ),
        used_in: &[
            used(
                Href::Product,
                "Product page: Product and BreadcrumbList JSON-LD",
                "صفحة المنتج: JSON-LD للمنتج ولمسار الصفحة",
            ),
            used(
                Href::Path("/"),
                "Home: OpenGraph per shop and language",
                "الرئيسية: OpenGraph لكل محل ولغة",
            ),
        ],
        note: None,
        sources: &["crates/app/src/pages/product/head.rs", "crates/app/src/pages/home.rs"],
    },
    Feature {
        id: "sim",
        cat: Cat::Server,
        title: ("Simulated latency and faults", "تأخير وأعطال تجريبية"),
        what: (
            "/__sim?ms=350 sets a cookie for you alone. Every widget's read then waits a multiple of that base: deals {{2.6}}×, related products {{1.8}}×, search {{0.25}}×. ?fail=<widget> breaks one read for one request, and /__sim?fail=deals breaks it for the whole session.",
            "‏‎/__sim?ms=350‎ بيحط كوكي ليك انت بس. بعدها قراءة كل widget بتستنى مضاعف من الرقم ده: العروض {{2.6}}×، والمنتجات المشابهة {{1.8}}×، والبحث {{0.25}}×. و ‎?fail=<widget>‎ بيبوّظ قراءة واحدة في طلب واحد، و ‎/__sim?fail=deals‎ بيبوّظها طول الجلسة.",
        ),
        why: (
            "On localhost everything answers at once, which hides loading states, slow phones and partial failures. It's off by default, and it's how every skeleton, timeout and error state here was checked.",
            "على localhost كل حاجة بترد في ساعتها، وده بيخبّي حالات التحميل والموبايلات البطيئة والأعطال الجزئية. هو مقفول افتراضيًا، وبيه اتجرّبت كل حالة تحميل وتأخير وخطأ هنا.",
        ),
        used_in: &[used(
            Href::Path("/"),
            "Every catalogue read in the app",
            "كل قراءة من الكتالوج في الأبلكيشن",
        )],
        note: None,
        sources: &["crates/app/src/server/sim.rs", "crates/server/src/main.rs"],
    },
    // ── The TypeScript client ──────────────────────────────────────────────────────────────────
    Feature {
        id: "router",
        cat: Cat::Client,
        title: ("The router and the page cache", "الراوتر وكاش الصفحات"),
        what: (
            "core/router.ts takes over same-origin links and forms. It fetches the next page's HTML, swaps the page's regions, and mounts the features in what came in. Pages are cached with a TTL and stale-while-revalidate, a hover ({{65}} ms) or a finger-down prefetches, and refresh() skips the cache.",
            "‏core/router.ts بيمسك اللينكات والفورمز اللي على نفس الموقع. بيجيب HTML الصفحة الجاية، ويبدّل أجزاء الصفحة، ويركّب الـ features في الجزء اللي وصل. الصفحات بتتخزن بـ TTL و stale-while-revalidate، والوقوف بالماوس ({{65}} مللي ثانية) أو لمسة الصباع بيعملوا prefetch، و refresh()‎ بيتخطّى الكاش.",
        ),
        why: (
            "Navigation feels instant without rendering pages in the browser. The server still renders every page, and the router only moves HTML. Without JavaScript the same links are ordinary page loads.",
            "التنقل بيبقى فوري من غير ما الصفحات تترسم في المتصفح. السيرفر لسه هو اللي بيرسم كل صفحة، والراوتر بس بينقل الـ HTML. ومن غير JavaScript نفس اللينكات بتبقى تحميل صفحة عادي.",
        ),
        used_in: &[used(
            Href::Path("/"),
            "Every link and form on every page",
            "كل لينك وكل فورم في كل صفحة",
        )],
        note: None,
        sources: &["client/src/core/router.ts", "client/src/core/cache.ts", "client/src/main.ts"],
    },
    Feature {
        id: "stores",
        cat: Cat::Client,
        title: ("One store per concern", "store واحد لكل حاجة"),
        what: (
            "createStore(initial) is the only state primitive: a value with subscribers. Each concern has exactly one store, created once in stores/*.ts: the cart, the toast queue, the router's pending state. Every feature that needs one imports that same store.",
            "‏createStore(initial)‎ هو الأداة الوحيدة للحالة: قيمة ومعاها مشتركين. كل حاجة ليها store واحد بس، بيتعمل مرة واحدة في stores/*.ts: السلة، وطابور التنبيهات، وحالة الراوتر وهو بيحمّل. وأي feature محتاجة واحد منهم بتستورد نفس الـ store ده.",
        ),
        why: (
            "The header badge, the drawer and the cart page can't disagree when they read the same store. There's also one path per action: the cart changes only through stores/cart.ts.",
            "علامة السلة في الهيدر والدرج وصفحة السلة مستحيل يختلفوا طالما بيقروا من نفس الـ store. وكمان فيه طريق واحد لكل عملية: السلة مابتتغيرش غير من stores/cart.ts.",
        ),
        used_in: &[used(
            Href::Path("/cart"),
            "Header badge, cart drawer and /cart",
            "علامة السلة في الهيدر والدرج وصفحة السلة",
        )],
        note: None,
        sources: &["client/src/core/store.ts", "client/src/stores/toasts.ts", "client/src/stores/cart.ts"],
    },
    Feature {
        id: "cart",
        cat: Cat::Client,
        title: ("The cart lives in a cookie", "السلة عايشة في كوكي"),
        what: (
            "The browser owns the cart: lines like 2307:1:2 in the cart cookie, written only by stores/cart.ts. Adding, removing and changing a quantity happen on the spot, with no request. The server reads the same cookie only when it has to price it: on /cart, for the drawer's quote and at checkout.",
            "المتصفح هو اللي ماسك السلة: سطور زي 2307:1:2 في كوكي cart، ومحدش بيكتبها غير stores/cart.ts. الإضافة والشيل وتغيير الكمية بيحصلوا عندك في ساعتها، من غير أي طلب. والسيرفر بيقرا نفس الكوكي بس لما يحتاج يسعّرها: في صفحة السلة، ولحساب الدرج، وعند الدفع.",
        ),
        why: (
            "Taps never wait for the network, the cart survives reloads and stays in step across tabs, and no-JS forms change the very same cookie. Prices, stock and totals still come from the server, formatted.",
            "اللمسات عمرها ما بتستنى النت، والسلة بتفضل بعد الريلود وبتفضل متظبطة بين التابات، والفورمز اللي من غير JS بتغيّر نفس الكوكي بالظبط. والأسعار والمخزون والإجمالي لسه جايين من السيرفر ومتنسقين.",
        ),
        used_in: &[
            used(Href::Path("/cart"), "/cart and checkout price the cookie", "صفحة السلة والدفع بيسعّروا الكوكي"),
            used(Href::Product, "Product page: add to cart", "صفحة المنتج: أضف للسلة"),
        ],
        note: None,
        sources: &["client/src/stores/cart.ts", "crates/domain/src/cart.rs"],
    },
    Feature {
        id: "templates",
        cat: Cat::Client,
        title: ("Templates filled by TypeScript", "قوالب بيملاها TypeScript"),
        what: (
            "When the client has to show data, such as a toast, a cart line or a search suggestion, it clones a server-rendered <template id=\"tpl-…\">, and fill() sets each [data-slot] as text. The markup and its translated labels are written once, in Rust.",
            "لما العميل يحتاج يعرض بيانات، زي تنبيه أو سطر في السلة أو اقتراح بحث، بياخد نسخة من <template id=\"tpl-…\"> جاي من السيرفر، و fill()‎ بيحط كل [data-slot] كنص. الـ markup والكلام المترجم بتاعه متكتبين مرة واحدة، في Rust.",
        ),
        why: (
            "There's no second copy of the UI in the client, no HTML built from strings, and nothing inserted as HTML. A change to a component's markup is made in one place, and the client picks it up.",
            "مفيش نسخة تانية من الواجهة في العميل، ولا HTML متركّب من نصوص، ولا حاجة بتتحط كـ HTML. وأي تعديل في شكل كومبوننت بيتعمل في مكان واحد، والعميل بياخده.",
        ),
        used_in: &[used(
            Href::Path("/"),
            "The toaster (tpl-toast) on every page",
            "التنبيهات (tpl-toast) في كل صفحة",
        )],
        note: None,
        sources: &["client/src/core/dom.ts", "crates/app/src/components/toast.rs"],
    },
    Feature {
        id: "data-cache",
        cat: Cat::Client,
        title: ("The client data cache", "كاش البيانات في العميل"),
        what: (
            "core/cache.ts keeps API reads and pages in memory: fresh for a TTL, then stale-while-revalidate (shown at once, refreshed in the background), then gone. Reads of one key at the same time share one request, and every key includes the shop, the language and the numerals.",
            "‏core/cache.ts بيحتفظ بقراءات الـ API والصفحات في الميموري: طازة لمدة الـ TTL، وبعدها stale-while-revalidate (بتظهر على طول وتتحدّث في الخلفية)، وبعدين تتشال. والقراءات لنفس المفتاح في نفس الوقت بتشترك في طلب واحد، وكل مفتاح فيه المحل واللغة ونوع الأرقام.",
        ),
        why: (
            "Going back to a page or reopening the drawer shows data immediately, and a language switch can never show a cached page in the other language. A new catalogue generation from the server (x-catalog-gen) drops everything.",
            "لما ترجع لصفحة أو تفتح الدرج تاني البيانات بتظهر على طول، وتغيير اللغة عمره ما يعرض صفحة متخزنة باللغة التانية. ولو السيرفر بعت generation جديد للكتالوج (x-catalog-gen)، كل حاجة بتتشال.",
        ),
        used_in: &[used(
            Href::Path("/cart"),
            "The cart quote: {{60}} s fresh, {{5}} min stale",
            "حساب السلة: طازة {{60}} ثانية، وقديم {{5}} دقايق",
        )],
        note: None,
        sources: &["client/src/core/cache.ts"],
    },
    Feature {
        id: "springs",
        cat: Cat::Client,
        title: ("Springs, from core/motion.ts", "حركة السبرينج من core/motion.ts"),
        what: (
            "SpringValue animates a number with a damped spring, solved exactly on every frame. It can be interrupted: a new target starts from the live value and velocity. A flick hands over its release velocity, project() says where it would come to rest, and rubberband() resists past an edge.",
            "‏SpringValue بيحرّك رقم بسبرينج متخمّد، بيتحسب بالظبط في كل فريم. وتقدر تقاطعه: أي هدف جديد بيبدأ من القيمة والسرعة اللي عليها دلوقتي. والزقّة بتسلّم سرعتها لحظة ما تسيب، و project()‎ بيقول هتقف فين، و rubberband()‎ بيقاوم بعد الطرف.",
        ),
        why: (
            "Drawers, sheets and the gallery follow the finger and leave the way they came, as on iOS. There's one implementation for every gesture, so nothing moves on a different curve.",
            "الأدراج والشيتات والجاليري بيمشوا مع صباعك وبيخرجوا من نفس السكة اللي دخلوا منها، زي iOS. وفيه تنفيذ واحد لكل الحركات، فمفيش حاجة بتتحرك بمنحنى مختلف.",
        ),
        used_in: &[
            used(Href::Path("/ui#overlays"), "The drawer and the sheet on /ui", "الدرج والشيت في ‎/ui‎"),
            used(Href::Product, "The product gallery's swipe", "سحب الصور في صفحة المنتج"),
        ],
        note: None,
        sources: &["client/src/core/motion.ts"],
    },
    Feature {
        id: "bundle",
        cat: Cat::Client,
        title: ("The size of the client", "حجم العميل"),
        what: (
            "All the browser code is one ES module. It is type-checked by tsc, bundled by esbuild from crates/app/build.rs, embedded in the server binary and served at a content-hashed /js/app-….js that browsers keep for good.",
            "كود المتصفح كله ES module واحد. بيتشيك عليه tsc، وبيجمّعه esbuild من crates/app/build.rs، وبيتحط جوه السيرفر نفسه، وبيتقدّم على ‎/js/app-….js‎ باسم فيه hash، فالمتصفح بيحتفظ بيه على طول.",
        ),
        why: (
            "The budget for the whole client is {{30}} KB gzip. The WASM islands it replaced cost {{379}}–{{769}} KB brotli on every page, before any page-specific chunk.",
            "الميزانية للعميل كله {{30}} كيلوبايت gzip. وجزر الـ WASM اللي جه مكانها كانت بتكلّف من {{379}} لـ {{769}} كيلوبايت brotli في كل صفحة، قبل أي جزء خاص بالصفحة.",
        ),
        used_in: &[used(Href::Path("/"), "Every page loads exactly this file", "كل صفحة بتحمّل الملف ده بالظبط")],
        note: None,
        sources: &["client/build.mjs", "crates/app/build.rs", "crates/app/src/client_js.rs"],
    },
    // ── Decisions ──────────────────────────────────────────────────────────────────────────────
    Feature {
        id: "why-not-islands",
        cat: Cat::Decisions,
        title: ("Why we left WASM islands", "ليه سبنا جزر الـ WASM"),
        what: (
            "The first version used Leptos islands: #[island] components compiled to WebAssembly and hydrated in the browser, with the islands router doing the navigation. It worked, and we measured it in Chromium against the release server.",
            "النسخة الأولى كانت بجزر Leptos: كومبوننتس ‎#[island]‎ بتتحول WebAssembly وتصحى في المتصفح، والـ islands router هو اللي بيعمل التنقل. كانت شغالة، وقسناها في Chromium على سيرفر الـ release.",
        ),
        why: (
            "Every page paid {{379}}–{{769}} KB of brotli WASM for a storefront that is mostly static, and hydration had race bugs that broke whole pages. Here are the measurements.",
            "كل صفحة كانت بتدفع من {{379}} لـ {{769}} كيلوبايت WASM ‏(brotli) في متجر أغلبه ثابت، والـ hydration كان فيه مشاكل توقيت بتبوّظ صفحات كاملة. ودي القياسات.",
        ),
        used_in: &[],
        note: Some((
            "Measured on {{26}} October {{2026}} with Leptos 0.8.22: release builds, bytes as sent. The full report is in README.md.",
            "اتقاس يوم {{26}} أكتوبر {{2026}} على Leptos 0.8.22: نسخ release، والبايتات زي ما اتبعتت. التقرير الكامل في README.md.",
        )),
        sources: &["README.md"],
    },
];

/// The feature's 1-based number in display order.
pub fn number(id: &str) -> usize {
    FEATURES.iter().position(|f| f.id == id).map(|i| i + 1).unwrap_or(0)
}
