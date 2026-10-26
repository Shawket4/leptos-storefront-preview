//! Client bootstrap, run once from `frontend::hydrate()` right after `hydrate_islands()`.
//!
//! `hydrate_islands()` installs a root reactive owner and leaves it current; every island's owner
//! is created beneath it. Context provided here is therefore visible to *all* islands, which is how
//! sibling islands (header badge, cart drawer, add-to-cart buttons, toggles) share one store.

use crate::prefs::{Lang, Numerals, Prefs, Theme, Ui};

pub fn init() {
    let html = leptos::prelude::document()
        .document_element()
        .expect("<html> element");
    let attr = |name: &str| html.get_attribute(name).unwrap_or_default();
    let prefs = Prefs {
        lang: Lang::parse(&attr("lang")).unwrap_or_default(),
        numerals: Numerals::parse(&attr("data-numerals")).unwrap_or_default(),
        theme: Theme::parse(&attr("data-theme")).unwrap_or_default(),
    };
    Ui::provide(prefs);
}
