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
