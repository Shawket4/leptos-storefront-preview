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
