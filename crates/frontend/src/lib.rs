//! Empty on purpose: cargo-leptos expects a WASM lib package, but no page loads it. The browser
//! code is TypeScript (`client/`), bundled by `crates/app/build.rs` and served by the server.

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn unused() {}
