use crate::prefs::{tr, tr_signal, use_ui};
use crate::request::req;
use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn HomePage() -> impl IntoView {
    let rq = req();
    let lang = rq.prefs.lang;
    let name = rq.tenant.name.pick(lang.is_rtl()).to_owned();
    view! {
        <Title text=name.clone()/>
        <h1>{name}</h1>
        <p>{rq.tenant.tagline.pick(lang.is_rtl()).to_owned()}</p>
        <p>{tr(lang, "Host: ", "المضيف: ")} <code>{rq.host.clone()}</code></p>
        <p><a href="/__prefs?lang=ar&next=/">"العربية"</a> " · " <a href="/__prefs?lang=en&next=/">"English"</a></p>
        <ProbeIsland start=3/>
    }
}

/// Scaffold probe: proves islands in `app` hydrate through the `frontend` cdylib and share the
/// root `Ui` store.
#[island]
pub fn ProbeIsland(start: i32) -> impl IntoView {
    let count = RwSignal::new(start);
    let ui = use_ui();
    let label = tr_signal("Clicks", "نقرات");
    view! {
        <button class="probe" on:click=move |_| count.update(|n| *n += 1)>
            {label} ": " {count} " (" {move || ui.lang.get().code()} ")"
        </button>
    }
}

#[component]
pub fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        if let Some(resp) = use_context::<leptos_axum::ResponseOptions>() {
            resp.set_status(http::StatusCode::NOT_FOUND);
        }
    }
    view! { <h1>"404"</h1> }
}
