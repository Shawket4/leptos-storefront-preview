//! The cart: the `/cart` page, the header button, the drawer's content and quick add — server
//! markup that the browser code (`client/src/features/cart.ts`) fills and keeps current. The cart
//! itself is the browser's (`cart` cookie); the server prices it (`server::cart`).
//!
//! Names mounted by other agents: [`CartButton`] (header), [`CartDrawer`] (once, by the shell),
//! [`QuickAdd`] (product cards).

mod drawer;
mod line;
mod page;
mod parts;

pub use drawer::CartDrawer;
pub use line::{LineData, LineRow, SummaryData, CartSummary};
pub use page::{Art, ArtImage, CartPage, Totals, art, vat_note};
pub use parts::{CartButton, QuickAdd};

use crate::prefs::Lang;

/// A catalogue string with `{name}` placeholders filled (server side of `t(key, vars)`).
pub fn tx(key: &str, lang: Lang, vars: &[(&str, &str)]) -> String {
    let mut s = crate::strings::get(key, lang).to_owned();
    for (k, v) in vars {
        s = s.replace(&format!("{{{k}}}"), v);
    }
    s
}
