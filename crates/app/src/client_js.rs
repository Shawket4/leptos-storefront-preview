//! The browser code: `client/` (TypeScript) as bundled by `build.rs` — ES modules with code
//! splitting (the entry, on-demand feature chunks, shared chunks), every file named by its content
//! hash so browsers can keep it forever (`immutable`). The server serves `/js/{file}` from
//! [`FILES`]; the shell loads [`APP_JS_PATH`] and preloads [`PRELOAD`].

include!(concat!(env!("OUT_DIR"), "/client_js.rs"));

/// A file of the bundle by name (`app-<hash>.js`, `chunk-<hash>.js`).
pub fn file(name: &str) -> Option<&'static [u8]> {
    FILES.iter().find(|(n, _)| *n == name).map(|(_, bytes)| *bytes)
}

/// Total bytes of every file (the whole client, before compression).
pub fn total_len() -> usize {
    FILES.iter().map(|(_, b)| b.len()).sum()
}
