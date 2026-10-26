//! Strings for checkout and order tracking (`features/checkout.ts`). Owner: the cart agent.
//! Validation messages in clear MSA, like the server's (`pages/checkout/rules.rs`).
use super::{Entry, s};

pub const ALL: &[Entry] = &[
    // Constraint messages shown before the form is sent (the server checks again)
    s("checkout.v.required", "Fill this in.", "هذا الحقل مطلوب."),
    s(
        "checkout.v.choose",
        "Choose one of these.",
        "اختر أحد الخيارات.",
    ),
    s(
        "checkout.v.too_short",
        "Enter at least {n} characters.",
        "أدخل {n} أحرف على الأقل.",
    ),
    s(
        "checkout.v.too_long",
        "Use at most {n} characters.",
        "يجب ألا يزيد على {n} حرفًا.",
    ),
    s(
        "checkout.v.summary",
        "Some details need attention.",
        "بعض البيانات تحتاج إلى مراجعة.",
    ),
    // Order tracking
    s(
        "order.update",
        "Order update: {status}",
        "تحديث الطلب: {status}",
    ),
    s(
        "order.retrying",
        "Connection lost. Retrying in {s} s…",
        "انقطع الاتصال. هنحاول تاني خلال {s} ث…",
    ),
    s(
        "order.offline",
        "You're offline. Updates resume when you're back.",
        "إنت مش متصل بالإنترنت. التحديثات هترجع أول ما ترجع.",
    ),
];
