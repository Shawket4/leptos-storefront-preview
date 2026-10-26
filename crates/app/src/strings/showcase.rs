//! Strings for the showcase area (/showcase, /showcase/stream, /ui, error pages) that the browser
//! code shows. Owner: the showcase agent. `{name}` marks a placeholder; numbers in them arrive
//! through `digits()` in the visitor's numerals.
use super::{Entry, s};

pub const ALL: &[Entry] = &[
    // Units and generic outcomes.
    s("showcase.ms", "{n} ms", "{n} مللي ثانية"),
    s("showcase.kb", "{n} KB", "{n} كيلوبايت"),
    s("showcase.px", "{n} px", "{n} بكسل"),
    s("showcase.pxs", "{n} px/s", "{n} بكسل/ثانية"),
    s("showcase.n_of", "{n} of {total}", "{n} من {total}"),
    s("showcase.bytes_pair", "{sent} sent · {raw} unpacked", "{sent} اتبعتوا · {raw} بعد الفك"),
    s("showcase.calling", "Asking the server…", "بنسأل السيرفر…"),
    s("showcase.measuring", "Measuring…", "بنقيس…"),
    s("showcase.loading", "Loading…", "بيحمّل…"),
    s("showcase.failed", "Failed: {why}", "فشل: {why}"),
    s("showcase.undo", "Undo", "تراجع"),
    // Read cache (server).
    s("showcase.bumped", "Generation {gen}: every cached read of this shop is gone.", "الـ generation بقى {gen}: كل القرايات المتخزنة للمحل ده اتشالت."),
    // Router.
    s("showcase.prefetched", "prefetch {url}: in the page cache now", "prefetch لـ {url}: بقت في كاش الصفحات"),
    s("showcase.navigated", "Last navigation: {url} in {ms}", "آخر تنقل: {url} في {ms}"),
    // Stores and toasts.
    s("showcase.store_cart", "{count} units in {lines} lines", "{count} قطعة في {lines} سطر"),
    s("showcase.store_toasts", "{n} on screen", "{n} ظاهرين"),
    s("showcase.store_idle", "idle", "واقف"),
    s("showcase.store_pending", "loading {to}", "بيحمّل {to}"),
    s("showcase.toast_success", "Added to your cart", "اتضاف للسلة"),
    s("showcase.toast_info", "Prices include {n}% VAT", "الأسعار شاملة ضريبة {n}٪"),
    s("showcase.toast_error", "We couldn't reach the store. Try again.", "مقدرناش نوصل للمتجر. جرّب تاني."),
    s("showcase.toast_removed", "Removed from your cart", "اتشال من السلة"),
    s("showcase.toast_back", "Back in your cart", "رجع للسلة"),
    // Cart cookie.
    s("showcase.cart_empty", "(empty)", "(فاضية)"),
    s("showcase.cart_full", "Your cart is full.", "السلة اتملت."),
    s("showcase.cart_max", "That's the most you can add of this.", "ده أقصى عدد تقدر تضيفه من ده."),
    s("showcase.requests_none", "none: the cookie and the store changed on the spot", "ولا طلب: الكوكي والـ store اتغيروا في ساعتها"),
    s("showcase.removed", "Removed {name}", "اتشال {name}"),
    s("showcase.quote_line", "{subtotal} + delivery {delivery} = {total} ({ms})", "{subtotal} + توصيل {delivery} = {total} ({ms})"),
    s("showcase.quote_empty", "Nothing to price: the cart is empty.", "مفيش حاجة تتسعّر: السلة فاضية."),
    // Templates.
    s("showcase.filled", "{n} rows, each a clone of the template with data put in as text.", "{n} سطر، كل واحد نسخة من القالب والبيانات متحطوطة كنص."),
    // Client data cache.
    s("showcase.cache_fresh", "fresh: from memory, no request ({ms})", "طازة: من الميموري، من غير طلب ({ms})"),
    s("showcase.cache_stale", "stale: shown at once, refreshing in the background ({ms})", "قديمة: ظهرت على طول، وبتتحدّث في الخلفية ({ms})"),
    s("showcase.cache_miss", "miss: fetched from the server ({ms})", "مش موجودة: اتجابت من السيرفر ({ms})"),
    s("showcase.cache_refreshed", "…the background refresh landed", "…التحديث اللي في الخلفية وصل"),
    s("showcase.cache_dropped", "invalidated: the next read fetches", "اتشالت: القراية الجاية هتجيب من السيرفر"),
    s("showcase.cache_age", "{s} s old", "عمرها {s} ثانية"),
    s("showcase.cache_none", "not cached", "مش متخزنة"),
    s("showcase.cache_counts", "{hits} fresh · {stale} stale · {misses} misses, all reads in this tab", "{hits} طازة · {stale} قديمة · {misses} مش موجودة، لكل القرايات في التاب ده"),
    // Bundle size.
    s("showcase.wire_sent", "{size}, compressed by the server", "{size}، مضغوط من السيرفر"),
    s("showcase.wire_cached", "{size}, from the browser cache", "{size}، من كاش المتصفح"),
    s("showcase.of_budget", "{p}% of the budget", "{p}٪ من الميزانية"),
    // /ui: a notice revealed by the browser, the error line filled by a check.
    s("showcase.notice_title", "Saved for later", "اتحفظ لبعدين"),
    s("showcase.notice_body", "Find it under Saved, on any device.", "هتلاقيه في المحفوظات، من أي جهاز."),
    s("showcase.check_error", "Enter the street and the building number.", "اكتب اسم الشارع ورقم العمارة."),
    // /ui skeletons.
    s("showcase.cls", "Layout shift while loading: {n}", "تحرّك الصفحة وقت التحميل: {n}"),
];
