//! The cart: the `/cart` page, the header button, the slide-over drawer, quick-add, and the
//! client cart store.
//!
//! Two progressive-enhancement styles, side by side:
//!
//! - **`/cart` is a server component** (`page`). Its steppers and remove buttons are stock
//!   `<ActionForm>`s: plain POSTs without JS; with JS, posts the islands router follows and diffs.
//! - **The drawer, the header badge and quick-add are islands** (`islands`), sharing one store
//!   (`stores::CartStore` + `store::CartOps`): optimistic, sequenced, rolled back on error. Their
//!   controls are still real forms that post to the same server functions without JS.
//!
//! Names mounted by other agents: [`CartButton`] (header), [`CartDrawer`] (once, by the shell),
//! [`QuickAdd`] (product cards), [`provide_store`] (from `stores::provide_all`).

mod copy;
mod islands;
pub mod math;
mod page;
pub mod shim;
pub mod store;

pub use islands::{CartButton, CartDrawer, CartForm, FreeDeliveryMeter, QuickAdd};
pub use page::{Art, ArtImage, CartPage, ShipMeter, Totals, art};

/// Provide the client cart store (called on the server per request and once on the client).
pub fn provide_store() {
    store::provide();
}
