//! `/orders` — find an order by its reference (tab bar "Orders", footer "Track an order").
//!
//! Works without JS: the form is a GET to `/orders?id=…`. The server tidies the reference
//! (`nil 48213`, `#NIL48213` → `NIL-48213`) and, if this shop has that order, answers with a
//! redirect to its tracking page; otherwise it renders the form again with the reference and an
//! inline error. Below the form: orders placed from this browser, with their current status.

use crate::components::button::{Button, ButtonKind, ButtonSize};
use crate::components::icons::Icon;
use crate::components::price::{Price, PriceSize};
use crate::components::status_pill::{StatusPill, Tone};
use crate::format;
use crate::prefs::{Lang, Numerals, tr};
use crate::request::req;
use domain::{OrderStatus, Piastres};
use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use leptos_router::hooks::use_query_map;

/// `nil 48213`, `#NIL48213`, `nil-48213` → `NIL-48213`. `None` if it can't be a reference.
pub fn normalize_ref(raw: &str) -> Option<String> {
    let s: String = raw
        .trim()
        .trim_start_matches('#')
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .to_ascii_uppercase();
    if s.is_empty() || s.len() > 16 || !s.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
        return None;
    }
    if s.contains('-') {
        return Some(s);
    }
    // Letters then digits with the dash left out.
    let split = s.find(|c: char| c.is_ascii_digit())?;
    let (prefix, digits) = s.split_at(split);
    (!prefix.is_empty() && digits.chars().all(|c| c.is_ascii_digit()))
        .then(|| format!("{prefix}-{digits}"))
}

/// One of the visitor's orders, for the list.
#[derive(Clone, Debug, PartialEq)]
struct Recent {
    id: String,
    status: OrderStatus,
    created_ms: i64,
    items: u32,
    total: Piastres,
}

fn tone(s: OrderStatus) -> (Tone, &'static str) {
    match s {
        OrderStatus::Delivered => (Tone::Success, "circle-check"),
        OrderStatus::Cancelled | OrderStatus::Rejected => (Tone::Danger, "circle-x"),
        OrderStatus::OutForDelivery => (Tone::Brand, "truck"),
        _ => (Tone::Info, "clock"),
    }
}

#[component]
pub fn OrdersLookupPage() -> impl IntoView {
    #[cfg(feature = "ssr")]
    crate::server::cache::private();
    let rq = req();
    let (lang, num) = (rq.prefs.lang, rq.prefs.numerals);
    let raw = use_query_map()
        .with_untracked(|q| q.get("id"))
        .unwrap_or_default();
    let wanted = normalize_ref(&raw);

    // Found → straight to tracking (decided before the first byte, so it's a real 302).
    #[cfg(feature = "ssr")]
    let found = wanted.as_deref().is_some_and(|id| {
        crate::server::state()
            .orders()
            .get(&rq.tenant.id, id)
            .is_some()
    });
    #[cfg(not(feature = "ssr"))]
    let found = false;
    #[cfg(feature = "ssr")]
    if found {
        if let Some(id) = &wanted {
            leptos_axum::redirect(&format!("/orders/{id}"));
        }
    }

    let error = (!raw.trim().is_empty() && !found).then(|| match (lang, wanted.as_deref()) {
        (Lang::En, Some(id)) => {
            format!("We couldn't find order {id}. Check the number in your confirmation message.")
        }
        (Lang::Ar, Some(id)) => format!("لم نعثر على الطلب {id}. تحقّق من الرقم في رسالة التأكيد."),
        (Lang::En, None) => {
            "That doesn't look like an order number. It looks like NIL-48213.".to_owned()
        }
        (Lang::Ar, None) => "هذا لا يبدو رقم طلب. رقم الطلب يكون بهذا الشكل: NIL-48213.".to_owned(),
    });

    #[cfg(feature = "ssr")]
    let recent: Vec<Recent> = {
        let st = crate::server::state();
        let mut v: Vec<Recent> = rq
            .cart_id
            .as_deref()
            .map(crate::pages::checkout::draft::orders_of)
            .unwrap_or_default()
            .into_iter()
            .filter_map(|id| st.orders().get(&rq.tenant.id, &id))
            .map(|o| Recent {
                id: o.id.clone(),
                status: o.status,
                created_ms: o.created_at_ms,
                items: o.lines.iter().map(|l| l.quantity).sum(),
                total: o.total,
            })
            .collect();
        v.sort_by_key(|r| std::cmp::Reverse(r.created_ms));
        v.truncate(5);
        v
    };
    #[cfg(not(feature = "ssr"))]
    let recent: Vec<Recent> = Vec::new();

    let example = format!("{}-48213", rq.tenant.order_prefix);
    let title = format!(
        "{} · {}",
        tr(lang, "Track an order", "تتبّع طلبك"),
        rq.tenant.name.pick(lang.is_rtl())
    );
    let value = wanted.clone().unwrap_or_else(|| raw.trim().to_owned());
    let invalid = error.is_some();

    view! {
        <Title text=title/>
        <Meta name="robots" content="noindex"/>
        <section class="container container--narrow orders-lookup">
            <header class="orders-lookup__head">
                <span class="orders-lookup__icon" aria-hidden="true"><Icon name="package" size=28/></span>
                <h1 class="t-h1">{tr(lang, "Track an order", "تتبّع طلبك")}</h1>
                <p class="orders-lookup__sub">
                    {tr(lang, "Enter the order number from your confirmation message.", "اكتب رقم الطلب اللي وصلك في رسالة التأكيد.")}
                </p>
            </header>
            <form class="lookup card" method="get" action="/orders" role="search">
                <label class="field__label" for="order-id">{tr(lang, "Order number", "رقم الطلب")}</label>
                <div class="lookup__row">
                    <input class="field__input lookup__input mono" id="order-id" name="id" value=value dir="ltr"
                        placeholder=example autocomplete="off" autocapitalize="characters" spellcheck="false"
                        enterkeyhint="go" maxlength="20" aria-invalid=invalid.to_string()
                        aria-describedby="order-id-err" autofocus=invalid/>
                    <Button button_type="submit" kind=ButtonKind::Primary size=ButtonSize::Lg icon_end="arrow-right">
                        {tr(lang, "Track", "تتبّع")}
                    </Button>
                </div>
                {error.map(|e| view! {
                    <p class="field__error" id="order-id-err" role="alert"><Icon name="circle-alert" size=16/><span>{e}</span></p>
                })}
            </form>
            {(!recent.is_empty()).then(|| view! { <RecentOrders recent lang num/> })}
        </section>
    }
}

#[component]
fn RecentOrders(recent: Vec<Recent>, lang: Lang, num: Numerals) -> impl IntoView {
    view! {
        <section class="recent" aria-labelledby="recent-title">
            <h2 class="t-h3" id="recent-title">{tr(lang, "Your recent orders", "طلباتك الأخيرة")}</h2>
            <ul class="recent__list" role="list">
                {recent.into_iter().map(|r| {
                    let (tone, icon) = tone(r.status);
                    let label = r.status.label().pick(lang.is_rtl()).to_owned();
                    let when = format!("{} · {}", format::date_cairo(r.created_ms, lang, num), format::time_cairo(r.created_ms, lang, num));
                    view! {
                        <li>
                            <a class="recent__item card" href=format!("/orders/{}", r.id) data-press="">
                                <span class="recent__top">
                                    <span class="recent__id mono" dir="ltr">{r.id.clone()}</span>
                                    <StatusPill tone label icon=icon/>
                                </span>
                                <span class="recent__meta">
                                    <span>{when}</span>
                                    <span>{format::items(r.items, lang, num)}</span>
                                </span>
                                <span class="recent__total"><Price amount=r.total size=PriceSize::Sm no_badge=true lang numerals=num/></span>
                                <Icon name="chevron-right" size=18 class="flip-rtl recent__chev"/>
                            </a>
                        </li>
                    }
                }).collect_view()}
            </ul>
        </section>
    }
}
