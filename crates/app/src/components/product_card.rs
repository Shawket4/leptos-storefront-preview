//! Product cards, their skeleton, and the responsive product grid.
//!
//! The card is one link (the title, stretched over the card), so it reads as a single item to a
//! screen reader and the quick-add button can sit above it without nesting interactive elements.
//! Every row has a fixed height (see product-card.css), so cards align in a grid and the
//! skeleton matches the card exactly.
//!
//! View transitions: the image box carries `data-vt="p{id}"`, and the shell's script names only
//! the card that was clicked (`view-transition-name`), because a product can appear twice on one
//! page and duplicate names abort the transition. The product page names its main image the same
//! (`Picture vt_name=format!("p{id}")`), so the packshot travels from the card to the page.

use super::badge::{Badge, BadgeKind};
use super::image::Picture;
use super::price::{Price, PriceSize};
use super::rating::Rating;
use crate::media::Img;
use crate::prefs::{Lang, Numerals};
use domain::{L10n, Piastres, ProductId, Tag, VariantId};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// Everything a product card needs, ready to render.
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

/// `sizes` for a card image in [`ProductGrid`] / `Rail`: 2 columns on phones, 3 ≥ 600px,
/// 4 ≥ 960px, 5 ≥ 1280px (content max 1320px).
pub const CARD_SIZES: &str =
    "(min-width: 1280px) 236px, (min-width: 960px) 22vw, (min-width: 600px) 30vw, 44vw";

#[component]
pub fn ProductCard(
    card: CardData,
    lang: Lang,
    numerals: Numerals,
    /// The first cards of an above-the-fold row: load the image eagerly.
    #[prop(optional)]
    priority: bool,
) -> impl IntoView {
    let ar = lang.is_rtl();
    let href = format!("/p/{}", card.slug);
    let name = card.name.pick(ar).to_owned();
    let brand = card
        .brand
        .as_ref()
        .map(|b| b.pick(ar).to_owned())
        .unwrap_or_default();
    let oos = !card.in_stock;
    let has = |t: Tag| card.tags.contains(&t);

    // Badges in fixed slots (an Option each), never a list: slots keep cards structurally alike.
    let express = (!oos && has(Tag::Express)).then_some(BadgeKind::Express);
    let second = if oos {
        Some(BadgeKind::OutOfStock)
    } else if let Some(n) = card.low_stock.filter(|n| *n > 0) {
        Some(BadgeKind::LowStock(n))
    } else if has(Tag::Bestseller) {
        Some(BadgeKind::Bestseller)
    } else if has(Tag::New) {
        Some(BadgeKind::New)
    } else {
        None
    };
    let quick = (card.single_variant && card.in_stock).then(|| {
        view! { <crate::pages::cart::QuickAdd card=card.clone() lang numerals/> }
    });
    let vt = format!("p{}", card.id.0);
    let cls = if oos { "pcard pcard--oos" } else { "pcard" };

    view! {
        <article class=cls>
            <div class="pcard__top">
                <div class="pcard__media" data-vt=vt>
                    <Picture img=card.img.clone() sizes=CARD_SIZES priority=priority lang=lang/>
                </div>
                {oos.then(|| view! { <span class="pcard__veil" aria-hidden="true"></span> })}
                <div class="pcard__badges">
                    {express.map(|k| view! { <Badge kind=k lang=lang numerals=numerals/> })}
                    {second.map(|k| view! { <Badge kind=k lang=lang numerals=numerals/> })}
                </div>
                <div class="pcard__quick">{quick}</div>
            </div>
            <div class="pcard__body">
                <p class="pcard__brand">{brand}</p>
                <h3 class="pcard__name">
                    <a class="pcard__link" href=href>{name}</a>
                </h3>
                <div class="pcard__rating">
                    <Rating value=card.rating count=card.reviews lang=lang numerals=numerals compact=true/>
                </div>
                <div class="pcard__price">
                    <Price
                        amount=card.price
                        compare_at=card.compare_at
                        from=card.from
                        size=PriceSize::Md
                        lang=lang
                        numerals=numerals
                    />
                </div>
            </div>
        </article>
    }
}

/// The card's exact box, pulsing. Use while a grid or rail streams in.
#[component]
pub fn ProductCardSkeleton() -> impl IntoView {
    view! {
        <div class="pcard pcard--skeleton" aria-hidden="true">
            <div class="pcard__top">
                <div class="pcard__media"></div>
            </div>
            <div class="pcard__body">
                <p class="pcard__brand"><span class="skeleton"></span></p>
                <div class="pcard__name">
                    <span class="skeleton"></span>
                    <span class="skeleton"></span>
                </div>
                <div class="pcard__rating"><span class="skeleton"></span></div>
                <div class="pcard__price"><span class="skeleton"></span></div>
            </div>
        </div>
    }
}

/// The responsive product grid: 2 columns ≤ 599px, 3 ≥ 600, 4 ≥ 960, 5 ≥ 1280.
///
/// Render the cards inside it; the listing's sort, filters and pages are router swaps of `<main>`.
#[component]
pub fn ProductGrid(
    children: Children,
    #[prop(optional, into)] class: String,
    /// `aria-label` for the list of results, e.g. "Products".
    #[prop(optional, into)]
    label: Option<String>,
) -> impl IntoView {
    let cls = if class.is_empty() {
        "pgrid".to_owned()
    } else {
        format!("pgrid {class}")
    };
    view! {
        <div class=cls aria-label=label>
            {children()}
        </div>
    }
}

/// A grid of skeleton cards (default 10: two full rows at 5 columns).
#[component]
pub fn ProductGridSkeleton(
    #[prop(optional)] count: Option<usize>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let cls = if class.is_empty() {
        "pgrid".to_owned()
    } else {
        format!("pgrid {class}")
    };
    view! {
        <div class=cls aria-hidden="true">
            {(0..count.unwrap_or(10)).map(|_| view! { <ProductCardSkeleton/> }).collect_view()}
        </div>
    }
}
