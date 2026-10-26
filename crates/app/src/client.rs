//! Client bootstrap, run once from `frontend::hydrate()` right after `hydrate_islands()`.
//!
//! `hydrate_islands()` installs a root reactive owner and leaves it current; every island's owner
//! is created beneath it. Context provided here is therefore visible to *all* islands, which is how
//! sibling islands (header badge, cart drawer, add-to-cart buttons, toasts) share one store.

use crate::prefs::{Lang, Numerals, Prefs, Theme, Ui};
use leptos::prelude::*;
use wasm_bindgen::{JsCast, closure::Closure};

fn read_prefs(html: &web_sys::Element) -> Prefs {
    let attr = |name: &str| html.get_attribute(name).unwrap_or_default();
    Prefs {
        lang: Lang::parse(&attr("lang")).unwrap_or_default(),
        numerals: Numerals::parse(&attr("data-numerals")).unwrap_or_default(),
        theme: Theme::parse(&attr("data-theme")).unwrap_or_default(),
    }
}

pub fn init() {
    let html = document().document_element().expect("<html> element");
    let ui = Ui::provide(read_prefs(&html));
    crate::stores::provide_all();

    // The language/numeral/theme controls are plain links to `/__prefs?…&next=…`. The islands
    // router follows them and diffs the server's new HTML into the page, which rewrites the
    // `<html>` attributes. Mirror those attributes into the store so islands — which the diff
    // deliberately leaves alone — re-render their own text too. No reload, no JS in the toggles.
    let on_change = Closure::<dyn FnMut(js_sys::Array, web_sys::MutationObserver)>::new(
        move |_records: js_sys::Array, _observer: web_sys::MutationObserver| {
            let Some(html) = document().document_element() else { return };
            let p = read_prefs(&html);
            if ui.lang.get_untracked() != p.lang {
                ui.lang.set(p.lang);
            }
            if ui.numerals.get_untracked() != p.numerals {
                ui.numerals.set(p.numerals);
            }
            if ui.theme.get_untracked() != p.theme {
                ui.theme.set(p.theme);
            }
        },
    );
    if let Ok(observer) = web_sys::MutationObserver::new(on_change.as_ref().unchecked_ref()) {
        let opts = web_sys::MutationObserverInit::new();
        opts.set_attributes(true);
        let filter = js_sys::Array::of3(&"lang".into(), &"data-numerals".into(), &"data-theme".into());
        opts.set_attribute_filter(&filter);
        let _ = observer.observe_with_options(&html, &opts);
    }
    on_change.forget();
}
