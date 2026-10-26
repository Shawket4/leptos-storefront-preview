//! `/orders` — find an order by its reference (tab bar "Orders", footer "Track an order").
//! (Stub — the commerce agent owns it; works without JS: GET form → `/orders?id=…` → redirect.)

use crate::prefs::tr;
use crate::request::req;
use leptos::prelude::*;

#[component]
pub fn OrdersLookupPage() -> impl IntoView {
    let rq = req();
    let lang = rq.prefs.lang;
    #[cfg(feature = "ssr")]
    {
        crate::server::cache::private();
        let id = rq
            .path_and_query
            .split_once("id=")
            .map(|(_, v)| v.split('&').next().unwrap_or("").trim().to_ascii_uppercase())
            .filter(|v| !v.is_empty() && v.len() <= 16 && v.chars().all(|c| c.is_ascii_alphanumeric() || c == '-'));
        if let Some(id) = id {
            leptos_axum::redirect(&format!("/orders/{id}"));
        }
    }
    view! {
        <section class="container orders-lookup">
            <h1>{tr(lang, "Track an order", "تتبّع طلبك")}</h1>
            <form method="get" action="/orders">
                <label for="order-id">{tr(lang, "Order number", "رقم الطلب")}</label>
                <input id="order-id" name="id" placeholder=if lang.is_rtl() { "مثال: NIL-48213" } else { "e.g. NIL-48213" } autocomplete="off"/>
                <button type="submit">{tr(lang, "Track", "تتبّع")}</button>
            </form>
        </section>
    }
}
