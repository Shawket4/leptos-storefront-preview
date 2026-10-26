//! WASM entry point. Only `#[island]` components (and what they use) end up in this bundle.

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_islands();
    // Must run after hydrate_islands(): it provides the shared stores under the root owner.
    app::client::init();
}
