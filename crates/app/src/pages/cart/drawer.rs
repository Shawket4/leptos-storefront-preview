//! The cart drawer's content, inside core's one drawer (`components::drawer::Drawer`, opened by
//! `openDrawer("cart-drawer")`). Rendered empty in every page's shell (the cart is personal and
//! public pages are cached); `features/cart.ts` (`data-ts="cart-drawer"`) fills it from the cart
//! store at once and from the server's quote when it lands. It also carries the line templates
//! the drawer and /cart share.

use super::line::{CartSummary, GoneTemplate, LineData, LineRow, SummaryData};
use crate::components::button::{Button, ButtonKind};
use crate::components::drawer::{Drawer, DrawerSide};
use crate::components::states::{StateTone, StateView};
use crate::prefs::use_ui;
use crate::strings;
use leptos::prelude::*;

#[component]
pub fn CartDrawer() -> impl IntoView {
    let ui = use_ui();
    let (lang, num) = (ui.lang.get_untracked(), ui.numerals.get_untracked());
    let g = move |k: &str| strings::get(k, lang);
    let footer = Box::new(move || {
        view! { <CartSummary d=SummaryData { empty: true, ..Default::default() } lang num in_drawer=true/> }.into_any()
    });
    view! {
        <Drawer id="cart-drawer" side=DrawerSide::End title=g("cart.title").to_owned() class="cart-drawer" footer>
            <div class="cart-drawer__content" data-ts="cart-drawer">
                <div class="cart-empty" data-cart-empty="" hidden="">
                    <StateView tone=StateTone::Empty art="empty-cart" compact=true
                        title=g("cart.empty_title") body=g("cart.empty_body")>
                        <Button href="/" kind=ButtonKind::Primary attr:data-drawer-close="">{g("cart.start")}</Button>
                    </StateView>
                </div>
                <ul class="clines" data-cart-lines="" aria-label=g("cart.lines_label")></ul>
            </div>
        </Drawer>
        <template id="tpl-cart-line">
            <LineRow d=LineData { max: domain::cart::MAX_QTY, qty: 1, ..Default::default() } lang next=super::line::CART_PATH/>
        </template>
        <GoneTemplate lang/>
    }
}
