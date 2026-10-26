//! `/core-scratch?case=…` — the core agent's scratch route (dev checks; the real specimens are /ui):
//! `intent`, `now` and `idle` exercise `lazyOnIntent` with dummy features (declared in dev builds
//! only); `components` (the default) shows the shared components in their states.
//!
//! Debug builds only: in a release build the route renders the ordinary 404 page and none of the
//! scratch code, styles or browser code is compiled in.

use leptos::prelude::*;

#[component]
pub fn CoreScratchPage() -> impl IntoView {
    #[cfg(debug_assertions)]
    {
        dev::page().into_any()
    }
    #[cfg(not(debug_assertions))]
    {
        view! { <super::NotFound/> }.into_any()
    }
}

#[cfg(debug_assertions)]
mod dev {
    use crate::prefs::lang_now;
    use leptos::prelude::*;
    use leptos_meta::Title;

    /// The page's own styles, inline (dev only, never in the shared stylesheet).
    const CSS: &str = r#".core-scratch {
  padding-block: var(--sp-6);
}
.core-scratch__nav {
  display: flex;
  flex-wrap: wrap;
  gap: var(--sp-3);
  margin-block: var(--sp-3) var(--sp-5);
}
.core-scratch__grid {
  display: grid;
  gap: var(--sp-4);
}
@media (min-width: 960px) {
  .core-scratch__grid {
    grid-template-columns: repeat(2, minmax(0, 1fr));
    align-items: start;
  }
}
.core-scratch__card {
  display: flex;
  flex-direction: column;
  gap: var(--sp-3);
  padding: var(--sp-4);
  border-radius: var(--r-xl);
  corner-shape: squircle;
  background: var(--surface);
  box-shadow: var(--shadow-sm);
}
.core-scratch__card h2 {
  font-size: var(--fs-md);
}
.core-scratch__card h3 {
  font-size: var(--fs-sm);
  color: var(--text-3);
}
.core-scratch__row {
  display: flex;
  flex-wrap: wrap;
  gap: var(--sp-2);
}
.core-scratch__list {
  display: flex;
  flex-direction: column;
  gap: var(--sp-3);
  margin: 0;
  padding: 0;
  list-style: none;
}"#;

    pub fn page() -> impl IntoView {
        let rq = crate::request::req();
        let query = rq
            .path_and_query
            .split_once('?')
            .map(|(_, q)| q.to_owned())
            .unwrap_or_default();
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
        _ => super::super::core_scratch_components::Specimens().into_any(),
    };
        view! {
            <Title text="Core scratch"/>
            <style>{CSS}</style>
            <div class="container core-scratch">
                <h1>"Core scratch"</h1>
                <nav class="core-scratch__nav">{nav}</nav>
                {body}
            </div>
        }
    }
}
