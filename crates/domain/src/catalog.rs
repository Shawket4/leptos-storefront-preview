//! Catalogue types. JSON fixtures in `data/catalog/<tenant>.json` deserialize straight into these
//! (see `data/SCHEMA.md`).

use crate::{L10n, Piastres};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CategoryId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ProductId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct VariantId(pub u32);

/// Origin: MadarRust `DeliveryMenuCategory { id, name, name_translations, image_url, display_order }`
/// (`src/delivery/public.rs:379`). Flat, like Madar's (no parent). Added for the web: `slug`,
/// `blurb`, `icon`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Category {
    pub id: CategoryId,
    pub slug: String,
    pub name: L10n,
    #[serde(default)]
    pub blurb: L10n,
    /// A lucide icon name (see `app::components::icons`).
    #[serde(default)]
    pub icon: String,
    /// Media key (Madar: `image_url`).
    #[serde(default)]
    pub image: Option<String>,
    pub display_order: i32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Brand {
    pub id: String,
    pub name: L10n,
}

/// Merchandising flags that drive the home page sections and badges.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Tag {
    Featured,
    Deal,
    New,
    Bestseller,
    /// Delivered within hours (dark-store stock).
    Express,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Spec {
    pub k: L10n,
    pub v: L10n,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OptionKind {
    /// Colour dots (values carry `swatch`).
    Swatch,
    /// Text chips (sizes, capacities, weights).
    Chip,
}

/// One variant axis (colour, storage, weight …).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProductOption {
    pub id: String,
    pub name: L10n,
    pub kind: OptionKind,
    pub values: Vec<OptionValue>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OptionValue {
    pub id: String,
    pub label: L10n,
    /// `#RRGGBB` for swatch options.
    #[serde(default)]
    pub swatch: Option<String>,
}

/// A purchasable combination.
///
/// Origin: Madar `DeliveryMenuSize { label, price }`, where `price` is the *absolute* price in
/// piastres (not a delta from the base). Extended with what an online shop needs that a café menu
/// doesn't: a SKU (Madar has none), stock (Madar tracks stock per ingredient, never per item), a
/// compare-at price, and the option values it stands for.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Variant {
    pub id: VariantId,
    pub sku: String,
    /// One option value id per axis, in `Product::options` order. Empty when there are no options.
    #[serde(default)]
    pub opts: Vec<String>,
    pub price: Piastres,
    #[serde(default)]
    pub compare_at: Option<Piastres>,
    pub stock: u32,
    /// Index into `Product::images` to show when this variant is picked.
    #[serde(default)]
    pub image: Option<usize>,
}

impl Variant {
    pub fn in_stock(&self) -> bool {
        self.stock > 0
    }
    pub fn on_sale(&self) -> bool {
        self.compare_at.is_some_and(|c| c > self.price)
    }
}

/// Origin: Madar `DeliveryModifierGroup` (`src/delivery/public.rs:304`) — kept as-is: `single` or
/// `multi`, `min`/`max` selections, surcharge options. Used for add-ons (gift wrap, warranty …).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModifierGroup {
    pub id: String,
    pub name: L10n,
    pub selection: SelectionType,
    #[serde(default)]
    pub min: u32,
    #[serde(default)]
    pub max: Option<u32>,
    #[serde(default)]
    pub required: bool,
    pub options: Vec<ModifierOption>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SelectionType {
    Single,
    Multi,
}

/// Origin: Madar `DeliveryModifierOption { option_id, name, price }`; `price` is a surcharge.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModifierOption {
    pub id: String,
    pub name: L10n,
    pub price: Piastres,
}

/// Origin: MadarRust `DeliveryMenuItem` (`src/delivery/public.rs:322`): `id, category_id, name,
/// name_translations, description, image_url, price, sizes, modifier_groups`. Madar's
/// `price` + `sizes` become `variants`; `image_url` becomes an image gallery; brand, specs,
/// rating and tags are new for retail.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Product {
    pub id: ProductId,
    pub slug: String,
    pub category: CategoryId,
    #[serde(default)]
    pub brand: Option<String>,
    pub name: L10n,
    #[serde(default)]
    pub subtitle: L10n,
    pub description: L10n,
    #[serde(default)]
    pub highlights: Vec<L10n>,
    #[serde(default)]
    pub specs: Vec<Spec>,
    /// Media keys; the first is the primary image.
    pub images: Vec<String>,
    #[serde(default)]
    pub rating: f32,
    #[serde(default)]
    pub reviews: u32,
    #[serde(default)]
    pub tags: Vec<Tag>,
    #[serde(default)]
    pub options: Vec<ProductOption>,
    pub variants: Vec<Variant>,
    #[serde(default)]
    pub modifier_groups: Vec<ModifierGroup>,
}

impl Product {
    /// The variant shown by default: the cheapest one in stock, else the first.
    pub fn default_variant(&self) -> &Variant {
        self.variants
            .iter()
            .filter(|v| v.in_stock())
            .min_by_key(|v| v.price)
            .unwrap_or(&self.variants[0])
    }
    pub fn variant(&self, id: VariantId) -> Option<&Variant> {
        self.variants.iter().find(|v| v.id == id)
    }
    /// Lowest price across variants (the "from" price on cards).
    pub fn price_from(&self) -> Piastres {
        self.variants.iter().map(|v| v.price).min().unwrap_or_default()
    }
    pub fn total_stock(&self) -> u32 {
        self.variants.iter().map(|v| v.stock).sum()
    }
    pub fn has(&self, tag: Tag) -> bool {
        self.tags.contains(&tag)
    }
    /// Something to choose besides the quantity (more than one variant, or add-ons): its cart line
    /// can be edited.
    pub fn has_choices(&self) -> bool {
        self.variants.len() > 1 || !self.modifier_groups.is_empty()
    }
}

/// A whole tenant catalogue, as stored in `data/catalog/<tenant>.json`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Catalog {
    pub tenant: String,
    #[serde(default)]
    pub brands: Vec<Brand>,
    pub categories: Vec<Category>,
    pub products: Vec<Product>,
}
