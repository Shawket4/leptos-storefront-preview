//! The browser code: `client/` (TypeScript) as bundled by `build.rs` — ES modules with code
//! splitting (the entry, on-demand feature chunks, shared chunks), every file named by its content
//! hash so browsers can keep it forever (`immutable`). The server serves `/js/{file}` from
//! [`FILES`]; the shell loads [`APP_JS_PATH`] and preloads [`PRELOAD`]; a page whose feature lives
//! in a chunk preloads that chunk and the shared ones it imports ([`PreloadFeature`]).

include!(concat!(env!("OUT_DIR"), "/client_js.rs"));

/// A file of the bundle by name (`app-<hash>.js`, `chunk-<hash>.js`).
pub fn file(name: &str) -> Option<&'static [u8]> {
    FILES
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, bytes)| *bytes)
}

/// Total bytes of every file (the whole client, before compression).
pub fn total_len() -> usize {
    FILES.iter().map(|(_, b)| b.len()).sum()
}

/// The chunks an on-demand module needs beyond the first load (`features/product.ts`).
pub fn feature_chunks(module: &str) -> &'static [&'static str] {
    FEATURE_CHUNKS
        .iter()
        .find(|(m, _)| *m == module)
        .map(|(_, f)| *f)
        .unwrap_or(&[])
}

/// `<link rel="modulepreload">` (in `<head>`, through leptos_meta) for a feature this page loads:
/// its chunk and the shared chunks it imports are fetched at once, with the page, instead of each
/// shared chunk only once the one naming it is in. On a client-side navigation the router puts the
/// new page's head tags in, which starts them as the page is shown.
#[leptos::component]
pub fn PreloadFeature(module: &'static str) -> impl leptos::IntoView {
    use leptos::prelude::*;
    feature_chunks(module)
        .iter()
        .map(|href| view! { <leptos_meta::Link rel="modulepreload" href=*href/> })
        .collect_view()
}
