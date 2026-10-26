//! Page-wide strings: network, navigation, toasts. Owner: core.
use super::{Entry, s};

pub const ALL: &[Entry] = &[
    s("toast.dismiss", "Dismiss", "إغلاق"),
    s("net.offline", "You're offline.", "لا يوجد اتصال بالإنترنت."),
    s("net.back_online", "You're back online.", "رجع الاتصال."),
    s("net.failed", "Something went wrong.", "حصلت مشكلة."),
    s("net.retry", "Try again", "حاول تاني"),
    s("nav.failed", "This page didn't load.", "الصفحة ما فتحتش."),
    s("nav.why.timeout", "It took too long to answer.", "استغرق الرد وقتًا طويلًا."),
    s("nav.why.network", "The connection dropped.", "انقطع الاتصال."),
    s("nav.why.server", "The store had a problem.", "حدثت مشكلة في المتجر."),
    s("net.offline_hint", "We'll reconnect automatically.", "سنعيد الاتصال تلقائيًا."),
    s("net.offline_still", "Still no connection — we'll keep trying.", "ما زال الاتصال مقطوعًا، وسنواصل المحاولة."),
    s("stepper.quantity", "Quantity", "الكمية"),
    s("stepper.decrease", "Decrease", "قلّل"),
    s("stepper.increase", "Increase", "زوّد"),
    s("stepper.remove", "Remove", "شيل"),
    // components::timeline (what each step says to screen readers) — set again by core/timeline.ts.
    s("timeline.done", "done", "تم"),
    s("timeline.current", "in progress", "جارٍ الآن"),
    s("timeline.todo", "not yet", "لم يبدأ بعد"),
    // components::live_status (core/live-status.ts writes the countdown).
    s("live.on", "Live updates", "تحديثات مباشرة"),
    s("live.failed", "Couldn't update.", "تعذّر التحديث."),
    s("live.retrying", "Couldn't update. Retrying in {s} s", "تعذّر التحديث. إعادة المحاولة خلال {s} ث"),
    s("live.retrying_said", "Couldn't update. Retrying shortly.", "تعذّر التحديث، وستُعاد المحاولة بعد قليل."),
    s("live.offline", "You're offline. Updates resume when you're back.", "لا يوجد اتصال، وتُستأنف التحديثات عند عودته."),
];
