//! The browser code: `client/` (TypeScript) as bundled by `build.rs` — one ES module, served at a
//! content-hashed path so browsers can keep it forever (`immutable`).

pub const APP_JS: &str = include_str!(concat!(env!("OUT_DIR"), "/app.js"));
pub const APP_JS_PATH: &str = concat!("/js/app-", env!("APP_JS_HASH"), ".js");
