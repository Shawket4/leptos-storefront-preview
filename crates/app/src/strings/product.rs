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
];
