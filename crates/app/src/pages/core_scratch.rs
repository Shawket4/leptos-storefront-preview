//! `/core-scratch?case=…` — the core agent's scratch route (dev checks; the real specimens are /ui):
//! `intent`, `now` and `idle` exercise `lazyOnIntent` with dummy features (declared in dev builds
//! only); `components` (the default) shows the shared components in their states.

use crate::prefs::lang_now;
use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn CoreScratchPage() -> impl IntoView {
    let rq = crate::request::req();
    let query = rq.path_and_query.split_once('?').map(|(_, q)| q.to_owned()).unwrap_or_default();
    let case = query
        .split('&')
        .find_map(|kv| kv.strip_prefix("case="))
        .unwrap_or("components")
        .to_owned();
    let _ = lang_now();
    let nav = ["components", "intent", "now", "idle"].map(|c| {
        let href = format!("/core-scratch?case={c}");
        view! { <a class="link" href=href aria-current=if c == case { "page" } else { "false" }>{c}</a> }
    });
    let body = match case.as_str() {
        "intent" => view! {
            <section id="intent">
                <p>"Box (lazyOnIntent: hover/touch starts the import; focus, press or a key loads and mounts):"</p>
                <div data-ts="scratch-intent" id="box">
                    <label>"Field " <input id="field" name="q" autocomplete="off"/></label>
                </div>
                <p><a id="trigger" class="link" href="/core-scratch?case=intent&n=2" data-scratch-trigger="">"A trigger outside the box (a link to this page)"</a></p>
                <p><a id="away" class="link" href="/c/phones">"Away (a category)"</a></p>
            </section>
        }
        .into_any(),
        "now" => view! {
            <section id="now">
                <div data-ts="scratch-intent" data-now="" id="box">"now(el) is true: loads and mounts at once"</div>
            </section>
        }
        .into_any(),
        "idle" => view! {
            <section id="idle">
                <div data-ts="scratch-idle" id="idlebox">"idle: imports when the page has settled"</div>
            </section>
        }
        .into_any(),
        _ => view! { <p>"Shared components: coming next."</p> }.into_any(),
    };
    view! {
        <Title text="Core scratch"/>
        <div class="container core-scratch">
            <h1>"Core scratch"</h1>
            <nav class="core-scratch__nav">{nav}</nav>
            {body}
        </div>
    }
}
