//! Cart error codes (see the top of `api/cart.rs`) → what the shopper reads. Errors are in clear
//! Modern Standard Arabic, like Madar's error copy.

use crate::prefs::Lang;
use leptos::prelude::ServerFnError;

/// The code a cart server function failed with (`ServerFnError::new("out_of_stock")`), or a
/// transport-level stand-in.
pub fn code(e: &ServerFnError) -> String {
    match e {
        ServerFnError::ServerError(m) if m.starts_with("simulated failure") => "unavailable".into(),
        ServerFnError::ServerError(m) => m.clone(),
        ServerFnError::Request(_) | ServerFnError::Response(_) => "network".into(),
        _ => "unavailable".into(),
    }
}

pub fn message(code: &str, lang: Lang) -> String {
    let (en, ar) = match code {
        "out_of_stock" => (
            "Sorry, that one just sold out.",
            "عذرًا، نفدت كمية هذا المنتج.",
        ),
        "unknown_product" => (
            "This product isn't available any more.",
            "هذا المنتج لم يعد متاحًا.",
        ),
        "addon_required" => (
            "Choose the required options on the product page first.",
            "اختر الإضافات المطلوبة من صفحة المنتج أولًا.",
        ),
        "bad_addon" => (
            "One of the chosen options isn't available.",
            "أحد الخيارات المختارة غير متاح.",
        ),
        "network" => (
            "You seem to be offline. Your cart wasn't changed.",
            "يبدو أنك غير متصل بالإنترنت. لم تتغير السلة.",
        ),
        _ => (
            "We couldn't update your cart. Please try again.",
            "تعذّر تحديث السلة. حاول مرة أخرى.",
        ),
    };
    match lang {
        Lang::En => en.into(),
        Lang::Ar => ar.into(),
    }
}
