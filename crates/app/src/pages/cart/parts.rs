//! The header's cart button and the quick-add "+" on product cards.

use crate::components::button::{ButtonKind, IconButton};
use crate::components::icons::Icon;
use crate::components::product_card::CardData;
use crate::format;
use crate::prefs::{Lang, Numerals, use_ui};
use crate::strings;
use leptos::prelude::*;

/// The bag in the header: a link to /cart that the browser code turns into the drawer's opener
/// (except on /cart and /checkout) and fills with the count from the `cart` cookie. Public pages
/// are cached, so the server always renders it empty.
#[component]
pub fn CartButton() -> impl IntoView {
    let lang = use_ui().lang.get_untracked();
    view! {
        <IconButton icon="shopping-bag" label=strings::get("cart.title", lang) href="/cart"
            kind=ButtonKind::Ghost class="cart-btn" attr:data-ts="cart-badge" attr:data-count="0"
            attr:aria-haspopup="dialog" attr:aria-controls="cart-drawer">
            <span class="icon-btn__badge cart-btn__badge" aria-hidden="true"></span>
        </IconButton>
    }
}

/// The round "+" on a product card (one variant, nothing required, in stock). Without JS a plain
/// `POST /cart/add` that comes back to this page; with JS `addAndConfirm` adds it instantly, with
/// the card's name, image, price and link as the drawer's first picture of the line.
#[component]
pub fn QuickAdd(card: CardData, lang: Lang, numerals: Numerals) -> impl IntoView {
    let ar = lang.is_rtl();
    let name = card.name.pick(ar).to_owned();
    let label = crate::pages::cart::tx("cart.add_item", lang, &[("name", &name)]);
    let href = format!("/p/{}", card.slug);
    let img = (!card.img.is_missing()).then(|| card.img.src_near(192));
    let price = format::money(card.price, lang, numerals);
    // Come back to this page (without a previous add's notice).
    let next = crate::request::req()
        .path_and_query
        .split('#')
        .next()
        .unwrap_or("/")
        .to_owned();
    view! {
        <form class="quick-add" method="post" action="/cart/add" data-ts="quick-add"
            data-product=card.id.0 data-variant=card.default_variant.0 data-name=name.clone()
            data-img=img data-price=price data-href=href>
            <input type="hidden" name="product" value=card.id.0/>
            <input type="hidden" name="variant" value=card.default_variant.0/>
            <input type="hidden" name="qty" value="1"/>
            <input type="hidden" name="next" value=next/>
            <button type="submit" class="quick-add__btn" data-state="idle" aria-label=label>
                <Icon name="plus" size=20 class="quick-add__plus"/>
                <Icon name="check" size=20 class="quick-add__check"/>
            </button>
        </form>
    }
}
