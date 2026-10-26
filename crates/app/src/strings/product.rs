//! Strings for the product page. Owner: the browse agent.
use super::{Entry, s};

pub const ALL: &[Entry] = &[
    s("product.add", "Add to cart", "أضف للسلة"),
    s("product.out", "Out of stock", "غير متوفر"),
    s("product.added_short", "Added", "اتضاف"),
    s(
        "product.photo_of",
        "Photo {n} of {total}",
        "صورة {n} من {total}",
    ),
    // Used only by the local addAndConfirm shim (features/product.ts) until the cart's lands.
    s(
        "product.added",
        "Added to your cart · {name}",
        "اتضاف لسلتك · {name}",
    ),
    s("product.cart_full", "Your cart is full.", "السلة مليانة."),
    s(
        "product.max_qty",
        "That's the most you can add of this item.",
        "ده أقصى عدد تقدر تضيفه من المنتج ده.",
    ),
];
