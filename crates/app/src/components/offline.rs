//! The offline banner: when the device loses its connection, a calm pill at the top says so and
//! offers a retry; when it comes back, a "Back online" toast — and the page is refreshed in place
//! so whatever failed meanwhile loads. Mounted once by the shell.

use super::icons::Icon;
use crate::prefs::{tr, use_ui};
use leptos::prelude::*;

#[component]
pub fn OfflineBanner() -> impl IntoView {
    let ui = use_ui();
    let offline = RwSignal::new(false);
    let still = RwSignal::new(false);

    // Re-fetch the current page through the islands router (it patches in place).
    #[cfg(feature = "hydrate")]
    let refresh = move || {
        let doc = document();
        if let (Ok(a), Some(body)) = (doc.create_element("a"), doc.body()) {
            let href = window().location().href().unwrap_or_default();
            let _ = a.set_attribute("href", &href);
            let _ = a.set_attribute("hidden", "");
            let _ = body.append_child(&a);
            if let Ok(a) = wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlElement>(a) {
                a.click();
                a.remove();
            }
        }
    };

    #[cfg(feature = "hydrate")]
    {
        // After hydration (the server rendered "online"), so the markup always matches first.
        Effect::new(move |_| offline.set(!super::net::is_online()));
        let lang = ui.lang;
        // The island lives as long as the page: its listeners never need removing.
        std::mem::forget(window_event_listener_untyped("offline", move |_| {
            still.set(false);
            offline.set(true);
        }));
        std::mem::forget(window_event_listener_untyped("online", move |_| {
            if offline.get_untracked() {
                offline.set(false);
                super::toast::show(
                    tr(lang.get_untracked(), "Back online", "عاد الاتصال").to_owned(),
                    super::toast::ToastKind::Success,
                );
                refresh();
            }
        }));
    }

    let on_retry = move |_| {
        #[cfg(feature = "hydrate")]
        {
            if super::net::is_online() {
                offline.set(false);
                refresh();
            } else {
                still.set(true);
            }
        }
    };

    view! {
        <div class="offline" data-open=move || if offline.get() { "true" } else { "false" } role="status">
            {move || {
                offline.get().then(|| {
                    let lang = ui.lang.get();
                    view! {
                        <Icon name="wifi-off" size=20 class="offline__icon"/>
                        <p class="offline__text">
                            <strong>{tr(lang, "You're offline", "أنت غير متصل بالإنترنت")}</strong>
                            <span>
                                {move || {
                                    if still.get() {
                                        tr(ui.lang.get(), "Still no connection — we'll keep trying.", "ما زال الاتصال مقطوعًا، وسنواصل المحاولة.")
                                    } else {
                                        tr(ui.lang.get(), "We'll reconnect automatically.", "سنعيد الاتصال تلقائيًا.")
                                    }
                                }}
                            </span>
                        </p>
                        <button type="button" class="offline__retry" on:click=on_retry>
                            {tr(lang, "Retry", "إعادة المحاولة")}
                        </button>
                    }
                })
            }}
        </div>
    }
}
