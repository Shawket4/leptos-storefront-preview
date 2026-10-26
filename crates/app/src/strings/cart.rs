//! Strings for the cart area (badge, quick add, drawer, /cart). Owner: the cart agent.
//! Shop copy in warm Egyptian colloquial; errors in clear MSA.
//!
//! `cart.added` and `cart.err.<code>` are the notices for the no-JS cart forms' `?added=` /
//! `?cart_err=` (AGENTS.md, "The cart (contract)"): any page may render them, so they carry no
//! placeholders.
use super::{Entry, s};

pub const ALL: &[Entry] = &[
    // Add to cart
    s("cart.added", "Added to your cart.", "اتضاف لسلتك."),
    s("cart.added_item", "Added to your cart: {name}", "اتضاف لسلتك: {name}"),
    s("cart.view", "View cart", "شوف السلة"),
    // No-JS form errors (`?cart_err=<code>`), also the client's toasts for the same cases
    s(
        "cart.err.out_of_stock",
        "Sorry, that item just sold out.",
        "عذرًا، نفدت كمية هذا المنتج.",
    ),
    s(
        "cart.err.max_qty",
        "You already have the most of this item we can send.",
        "وصلت للحد الأقصى المتاح من هذا المنتج.",
    ),
    s(
        "cart.err.full",
        "Your cart is full. Check out or remove something first.",
        "السلة ممتلئة. أكمل الطلب أو احذف منتجًا أولًا.",
    ),
    s(
        "cart.err.addon_required",
        "Choose the required options on the product page first.",
        "اختر الإضافات المطلوبة من صفحة المنتج أولًا.",
    ),
    s(
        "cart.err.bad_addon",
        "One of the chosen options isn't available.",
        "أحد الخيارات المختارة غير متاح.",
    ),
    s(
        "cart.err.unknown_product",
        "This product isn't available any more.",
        "هذا المنتج لم يعد متاحًا.",
    ),
    s(
        "cart.err.bad_request",
        "We couldn't update your cart. Please try again.",
        "تعذّر تحديث السلة. يرجى المحاولة مرة أخرى.",
    ),
    // The cart (drawer and /cart)
    s("cart.title", "Your cart", "سلتك"),
    s("cart.lines_label", "Items in your cart", "المنتجات في سلتك"),
    s("cart.remove", "Remove", "شيل"),
    s("cart.remove_item", "Remove {name}", "شيل {name}"),
    s("cart.update", "Update", "حدّث"),
    s("cart.qty_of", "Quantity of {name}", "كمية {name}"),
    s("cart.breakdown", "{qty} × {unit}", "{qty} × {unit}"),
    s("cart.low_stock", "Only {n} in stock", "المتاح {n} بس"),
    s(
        "cart.issue.out_of_stock",
        "Sold out since you added it. Remove it to check out.",
        "نفدت الكمية بعد إضافته. احذفه لإتمام الطلب.",
    ),
    s("cart.issue.capped", "Only {n} left now.", "المتوفر الآن {n} فقط."),
    s(
        "cart.issue.unavailable",
        "This item isn't sold any more. Remove it to check out.",
        "هذا المنتج لم يعد متاحًا. احذفه لإتمام الطلب.",
    ),
    s("cart.unavailable_name", "An item that's no longer sold", "منتج لم يعد متاحًا"),
    s("cart.fix", "Change to {n}", "خليها {n}"),
    s("cart.removed", "Removed {name}", "شيلنا {name}"),
    s("cart.undo", "Undo", "رجّعه"),
    s("cart.subtotal", "Subtotal", "المجموع"),
    s("cart.savings", "You save", "وفّرت"),
    s("cart.delivery", "Delivery", "التوصيل"),
    s("cart.free", "Free", "مجاني"),
    s("cart.total", "Total", "الإجمالي"),
    s("cart.vat", "Prices include {pct} VAT ({vat}).", "الأسعار شاملة ضريبة القيمة المضافة {pct} ({vat})."),
    s("cart.free_hint", "Add {amount} more for free delivery.", "ضيف بـ {amount} كمان والتوصيل يبقى مجاني."),
    s("cart.free_done", "Delivery is on us.", "التوصيل علينا."),
    s("cart.checkout", "Checkout", "كمّل الطلب"),
    s(
        "cart.blocked",
        "Fix the items marked above to check out.",
        "عدّل المنتجات المحددة فوق علشان تكمّل الطلب.",
    ),
    s("cart.quote_failed", "We couldn't update the totals.", "تعذّر تحديث الإجمالي."),
    s(
        "cart.quote_offline",
        "You're offline. The totals will update when you're back.",
        "أنت غير متصل بالإنترنت. سيتم تحديث الإجمالي عند عودة الاتصال.",
    ),
    s("cart.retry", "Try again", "حاول تاني"),
    s("cart.empty_title", "Your cart is empty", "سلتك فاضية"),
    s("cart.empty_body", "Add something you like — it'll wait for you here.", "ضيف اللي يعجبك، وهيستناك هنا."),
    s("cart.start", "Start shopping", "ابدأ التسوق"),
    s("cart.continue", "Continue shopping", "كمّل تسوق"),
    s("cart.add_item", "Add to cart: {name}", "أضف للسلة: {name}"),
    s("cart.picks", "Popular right now", "الأكثر طلبًا دلوقتي"),
    // The header badge
    s("cart.badge", "Cart, {items}", "السلة، {items}"),
    s("cart.badge_empty", "Cart, empty", "السلة، فاضية"),
    // "{n} items" by plural form (Intl.PluralRules; Arabic zero uses "other")
    s("cart.items.one", "{n} item", "منتج واحد"),
    s("cart.items.two", "{n} items", "منتجين"),
    s("cart.items.few", "{n} items", "{n} منتجات"),
    s("cart.items.many", "{n} items", "{n} منتج"),
    s("cart.items.other", "{n} items", "{n} منتج"),
];
