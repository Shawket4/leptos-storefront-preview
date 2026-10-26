//! Islands-era stores, provided per request on the server (inside `request::init`) for the pages
//! that still read them while they move to the browser code (`client/src/stores/*`). Owners
//! implement their `provide` functions in their files.

use crate::api::types::CartView;
use leptos::prelude::*;

pub fn provide_all() {
    provide_context(CartStore::new());
    crate::components::toast::provide();
}

/// The cart, as one store every island reads: the header badge, the drawer, add-to-cart buttons.
///
/// `count` is the single signal behind the header badge. Public (cacheable) pages never render a
/// visitor's cart into HTML, so on the server it starts at 0 and on the client it starts from the
/// readable `cart_n` cookie; `view` (lines and totals) is fetched lazily with `api::cart::cart_get`
/// the first time something needs it, and replaced by every cart mutation's response.
#[derive(Clone, Copy, Debug)]
pub struct CartStore {
    pub count: RwSignal<u32>,
    pub view: RwSignal<Option<CartView>>,
    pub open: RwSignal<bool>,
    /// Bumped on every successful add — the badge watches it to play its "pop".
    pub pulse: RwSignal<u32>,
}

impl CartStore {
    fn new() -> Self {
        CartStore {
            count: RwSignal::new(initial_count()),
            view: RwSignal::new(None),
            open: RwSignal::new(false),
            pulse: RwSignal::new(0),
        }
    }

    /// Adopt a server response (every cart server function returns the full `CartView`).
    pub fn apply(&self, v: CartView) {
        self.count.set(v.count);
        self.view.set(Some(v));
    }
}

pub fn use_cart() -> CartStore {
    use_context::<CartStore>().unwrap_or_else(|| {
        let s = CartStore::new();
        provide_context(s);
        s
    })
}

/// Public pages never render a visitor's cart: the badge is filled by the browser code.
fn initial_count() -> u32 {
    0
}
