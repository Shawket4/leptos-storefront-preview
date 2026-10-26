//! Product card. (Placeholder holding only the data type — the design-system agent owns this file.)

use crate::media::Img;
use domain::{L10n, Piastres, ProductId, Tag, VariantId};
use serde::{Deserialize, Serialize};

/// Everything a product card needs, ready to render (server or island).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CardData {
    pub id: ProductId,
    pub slug: String,
    pub name: L10n,
    pub brand: Option<L10n>,
    pub img: Img,
    pub price: Piastres,
    pub compare_at: Option<Piastres>,
    /// Variants have different prices: show "from".
    pub from: bool,
    pub rating: f32,
    pub reviews: u32,
    pub tags: Vec<Tag>,
    pub in_stock: bool,
    pub low_stock: Option<u32>,
    pub default_variant: VariantId,
    /// One variant and no required add-ons: the card can add straight to the cart.
    pub single_variant: bool,
}

use crate::prefs::{Lang, Numerals};
use leptos::prelude::*;

/// Placeholder card until the design system lands.
#[component]
pub fn ProductCard(card: CardData, lang: Lang, numerals: Numerals, #[prop(optional)] priority: bool) -> impl IntoView {
    let ar = lang.is_rtl();
    let img = card.img.clone();
    view! {
        <a class="pcard" href=format!("/p/{}", card.slug)>
            <span class="pcard__well" style=format!("background-image:url({})", img.lqip)>
                <img src=img.src_near(400) srcset=img.srcset() sizes="(max-width: 600px) 45vw, 220px" alt=img.alt(lang).to_owned()
                    width="400" height="400" loading=if priority { "eager" } else { "lazy" } decoding="async"/>
            </span>
            {card.brand.map(|b| view! { <span class="pcard__brand">{b.pick(ar).to_owned()}</span> })}
            <span class="pcard__name">{card.name.pick(ar).to_owned()}</span>
            <span class="pcard__price">{crate::format::money(card.price, lang, numerals)}</span>
        </a>
    }
}

#[component]
pub fn ProductCardSkeleton() -> impl IntoView {
    view! { <span class="pcard pcard--skeleton" aria-hidden="true"><span class="pcard__well skeleton"></span><span class="skeleton skeleton--text"></span><span class="skeleton skeleton--text"></span></span> }
}
