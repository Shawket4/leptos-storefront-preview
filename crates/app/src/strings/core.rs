//! Page-wide strings: network, navigation, toasts. Owner: core.
use super::{Entry, s};

pub const ALL: &[Entry] = &[
    s("toast.dismiss", "Dismiss", "إغلاق"),
    s("net.offline", "You're offline.", "لا يوجد اتصال بالإنترنت."),
    s("net.back_online", "You're back online.", "رجع الاتصال."),
    s("net.failed", "Something went wrong.", "حصلت مشكلة."),
    s("net.retry", "Try again", "حاول تاني"),
    s("nav.failed", "This page didn't load.", "الصفحة ما فتحتش."),
    s("stepper.quantity", "Quantity", "الكمية"),
    s("stepper.decrease", "Decrease", "قلّل"),
    s("stepper.increase", "Increase", "زوّد"),
    s("stepper.remove", "Remove", "شيل"),
];
