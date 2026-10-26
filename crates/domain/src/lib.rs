//! Storefront domain types.
//!
//! The entity types are minimal copies of Madar's (each notes its origin), because the crate that
//! owns them — MadarRust — pulls in actix, sqlx and tokio. Madar's pricing and money rules live in
//! the clean `madar-shared` crates, which the server depends on by path instead (see `app::pricing`).

pub mod cart;
pub mod catalog;
pub mod money;
pub mod order;
pub mod text;

pub use catalog::*;
pub use money::Piastres;
pub use order::*;
pub use text::L10n;
