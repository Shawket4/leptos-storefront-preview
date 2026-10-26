//! The UI strings the browser code needs, in one catalogue. Each area keeps its own list in
//! `strings/<area>.rs` (owned like that area's code); the shell sends the page's language to the
//! browser as JSON (`#app-strings`), read by `t(key)` in `client/src/core/page.ts`.
//!
//! A string that shows up in several places ("Try again", "You're offline") lives here once and
//! server markup uses it through [`get`] too; one-off server labels may stay next to their use with
//! `tr(lang, en, ar)`. Keys are `area.thing` (`cart.added`); `{name}` marks a placeholder.

use crate::prefs::Lang;

mod browse;
mod cart;
mod checkout;
mod core;
mod product;
mod showcase;

pub struct Entry {
    pub key: &'static str,
    pub en: &'static str,
    pub ar: &'static str,
}

pub const fn s(key: &'static str, en: &'static str, ar: &'static str) -> Entry {
    Entry { key, en, ar }
}

fn all() -> impl Iterator<Item = &'static Entry> {
    core::ALL
        .iter()
        .chain(cart::ALL)
        .chain(checkout::ALL)
        .chain(browse::ALL)
        .chain(product::ALL)
        .chain(showcase::ALL)
}

/// One string by key (server markup sharing a client string). An unknown key is a bug: it is
/// logged as an error and renders a visible marker — loud, but it never takes the page down
/// (a panic here emptied every product page's response).
pub fn get(key: &str, lang: Lang) -> &'static str {
    match all().find(|e| e.key == key) {
        Some(e) if lang.is_rtl() => e.ar,
        Some(e) => e.en,
        None => {
            #[cfg(feature = "ssr")]
            tracing::error!("unknown string key {key}: add it to crates/app/src/strings/");
            "⟦missing string⟧"
        }
    }
}

/// `#app-strings`: the strings in `lang` that the browser code on `path` can use, as a JSON object
/// safe inside `<script>`. Pages are `no-cache`, so this is sent with every page view: the
/// showcase's strings (a third of the catalogue) go only to the pages that run its code, /showcase
/// and /ui. The router copies each page's `#app-strings` over on navigation, so a visit there by
/// the router brings them along.
pub fn client_json(lang: Lang, path: &str) -> String {
    let path = path.split(['?', '#']).next().unwrap_or_default();
    let showcase = ["/showcase", "/ui"].iter().any(|p| {
        path == *p
            || path
                .strip_prefix(p)
                .is_some_and(|rest| rest.starts_with('/'))
    });
    let map: std::collections::BTreeMap<&str, &str> = all()
        .filter(|e| showcase || !e.key.starts_with("showcase."))
        .map(|e| (e.key, if lang.is_rtl() { e.ar } else { e.en }))
        .collect();
    serde_json::to_string(&map)
        .unwrap_or_else(|_| "{}".into())
        .replace('<', "\\u003c")
}

/// Duplicate keys are a bug (one would silently win): checked at startup in debug builds.
pub fn check() {
    let mut seen = std::collections::HashSet::new();
    for e in all() {
        assert!(seen.insert(e.key), "duplicate string key {}", e.key);
    }
}
