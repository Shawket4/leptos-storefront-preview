//! One tenant's catalogue with the indexes the pages need. Built once at startup from
//! `data/catalog/<tenant>.json`; every query returns ready-to-render, serializable views.

use super::media::Media;
use super::search::{self, DocText, SearchIndex};
use crate::api::types::*;
use crate::components::product_card::CardData;
use domain::{Brand, Catalog, CategoryId, L10n, Piastres, Product, ProductId, Tag, Variant, VariantId};
use std::collections::HashMap;

pub struct TenantCatalog {
    pub tenant: String,
    catalog: Catalog,
    by_id: HashMap<ProductId, usize>,
    by_slug: HashMap<String, usize>,
    cards: Vec<CardData>,
    nav: Vec<NavCategory>,
    nav_by_id: HashMap<CategoryId, usize>,
    brands: HashMap<String, usize>,
    index: SearchIndex,
}

fn pounds_floor(p: Piastres) -> i64 {
    p.0.div_euclid(100)
}

impl TenantCatalog {
    pub fn new(mut catalog: Catalog, media: &Media) -> Self {
        catalog.categories.sort_by_key(|c| (c.display_order, c.id));
        let brands: HashMap<String, usize> =
            catalog.brands.iter().enumerate().map(|(i, b)| (b.id.clone(), i)).collect();
        let by_id = catalog.products.iter().enumerate().map(|(i, p)| (p.id, i)).collect();
        let by_slug = catalog.products.iter().enumerate().map(|(i, p)| (p.slug.clone(), i)).collect();

        let mut counts: HashMap<CategoryId, u32> = HashMap::new();
        for p in &catalog.products {
            *counts.entry(p.category).or_default() += 1;
        }
        let nav: Vec<NavCategory> = catalog
            .categories
            .iter()
            .map(|c| NavCategory {
                id: c.id.0,
                slug: c.slug.clone(),
                name: c.name.clone(),
                blurb: c.blurb.clone(),
                icon: c.icon.clone(),
                // A category without its own picture borrows its first product's packshot.
                img: c.image.as_deref().and_then(|k| media.try_img(k)).or_else(|| {
                    catalog
                        .products
                        .iter()
                        .filter(|p| p.category == c.id)
                        .find_map(|p| p.images.first().and_then(|k| media.try_img(k)))
                }),
                count: counts.get(&c.id).copied().unwrap_or(0),
            })
            .collect();
        let nav_by_id = catalog.categories.iter().enumerate().map(|(i, c)| (c.id, i)).collect();

        let brand_name = |id: &Option<String>| -> Option<L10n> {
            id.as_ref().and_then(|b| brands.get(b)).map(|&i| catalog.brands[i].name.clone())
        };
        let cards = catalog.products.iter().map(|p| card(p, brand_name(&p.brand), media)).collect();

        let empty = L10n::default();
        let index = SearchIndex::build(catalog.products.iter().map(|p| {
            let brand = p.brand.as_ref().and_then(|b| brands.get(b)).map(|&i| &catalog.brands[i].name);
            let cat = catalog.categories.iter().find(|c| c.id == p.category).map(|c| &c.name);
            let b = brand.unwrap_or(&empty);
            let c = cat.unwrap_or(&empty);
            DocText {
                names: [p.name.en.as_str(), p.name.ar.as_str()],
                brands: [b.en.as_str(), b.ar.as_str()],
                categories: [c.en.as_str(), c.ar.as_str()],
                extra: vec![p.subtitle.en.as_str(), p.subtitle.ar.as_str()],
            }
        }));

        TenantCatalog { tenant: catalog.tenant.clone(), catalog, by_id, by_slug, cards, nav, nav_by_id, brands, index }
    }

    pub fn products(&self) -> &[Product] {
        &self.catalog.products
    }

    pub fn product(&self, id: ProductId) -> Option<&Product> {
        self.by_id.get(&id).map(|&i| &self.catalog.products[i])
    }

    pub fn product_by_slug(&self, slug: &str) -> Option<&Product> {
        self.by_slug.get(slug).map(|&i| &self.catalog.products[i])
    }

    pub fn card(&self, id: ProductId) -> Option<&CardData> {
        self.by_id.get(&id).map(|&i| &self.cards[i])
    }

    pub fn brand(&self, id: &str) -> Option<&Brand> {
        self.brands.get(id).map(|&i| &self.catalog.brands[i])
    }

    pub fn nav(&self) -> &[NavCategory] {
        &self.nav
    }

    pub fn category(&self, id: CategoryId) -> Option<&NavCategory> {
        self.nav_by_id.get(&id).map(|&i| &self.nav[i])
    }

    pub fn category_by_slug(&self, slug: &str) -> Option<&NavCategory> {
        self.nav.iter().find(|c| c.slug == slug)
    }

    /// The label of a variant: its option values joined (`Titanium Black · 256 GB`).
    pub fn variant_label(p: &Product, v: &Variant) -> L10n {
        let mut en = Vec::new();
        let mut ar = Vec::new();
        for (axis, val) in p.options.iter().zip(&v.opts) {
            if let Some(ov) = axis.values.iter().find(|x| &x.id == val) {
                en.push(ov.label.en.clone());
                ar.push(ov.label.ar.clone());
            }
        }
        L10n::new(en.join(" · "), ar.join(" · "))
    }

    /// A home-page rail, up to `limit` cards.
    pub fn rail(&self, rail: Rail, limit: usize) -> Vec<CardData> {
        let mut picks: Vec<usize> = (0..self.cards.len())
            .filter(|&i| {
                let p = &self.catalog.products[i];
                match rail {
                    Rail::Featured => p.has(Tag::Featured),
                    Rail::Deals => p.variants.iter().any(Variant::on_sale),
                    Rail::New => p.has(Tag::New),
                    Rail::Bestsellers => p.has(Tag::Bestseller),
                    Rail::Express => p.has(Tag::Express),
                }
            })
            .collect();
        let score = |i: &usize| -> i64 {
            let p = &self.catalog.products[*i];
            let c = &self.cards[*i];
            let stock = if c.in_stock { 1_000_000 } else { 0 };
            stock
                + match rail {
                    Rail::Deals => discount_pct(c) as i64 * 1000 + p.reviews as i64,
                    Rail::Bestsellers => p.reviews as i64,
                    _ => (p.rating * 100.0) as i64 * 10 + (p.reviews as i64).min(9),
                }
        };
        picks.sort_by_key(|i| std::cmp::Reverse(score(i)));
        picks.truncate(limit);
        picks.into_iter().map(|i| self.cards[i].clone()).collect()
    }

    pub fn listing(&self, q: &ListingQuery) -> ListingPage {
        let cat = q.category.as_deref().and_then(|s| self.category_by_slug(s)).cloned();
        let scores = q.q.as_deref().map(|text| self.index.matches(text));
        let base: Vec<usize> = (0..self.cards.len())
            .filter(|&i| cat.as_ref().is_none_or(|c| self.catalog.products[i].category.0 == c.id))
            .filter(|i| scores.as_ref().is_none_or(|s| s.contains_key(i)))
            .collect();

        // Facets come from the unfiltered set, so the shopper can see what each filter would give.
        let mut brand_counts: HashMap<&str, u32> = HashMap::new();
        let (mut lo, mut hi) = (i64::MAX, 0i64);
        let mut facets = Facets::default();
        for &i in &base {
            let p = &self.catalog.products[i];
            let c = &self.cards[i];
            if let Some(b) = &p.brand {
                *brand_counts.entry(b.as_str()).or_default() += 1;
            }
            lo = lo.min(pounds_floor(c.price));
            hi = hi.max(pounds_floor(c.price) + 1);
            facets.in_stock_count += c.in_stock as u32;
            facets.deal_count += c.compare_at.is_some() as u32;
        }
        let mut brands: Vec<BrandFacet> = brand_counts
            .into_iter()
            .filter_map(|(id, count)| {
                self.brand(id).map(|b| BrandFacet { id: b.id.clone(), name: b.name.clone(), count })
            })
            .collect();
        brands.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.name.en.cmp(&b.name.en)));
        facets.brands = brands;
        facets.price_min = if lo == i64::MAX { 0 } else { lo };
        facets.price_max = hi;

        let mut rows: Vec<usize> = base
            .into_iter()
            .filter(|&i| {
                let p = &self.catalog.products[i];
                let c = &self.cards[i];
                let pounds = pounds_floor(c.price);
                q.min.is_none_or(|m| pounds >= m)
                    && q.max.is_none_or(|m| pounds <= m)
                    && (q.brands.is_empty() || p.brand.as_ref().is_some_and(|b| q.brands.contains(b)))
                    && (!q.in_stock || c.in_stock)
                    && (!q.deal || c.compare_at.is_some())
            })
            .collect();

        let relevance = |i: &usize| -> i64 {
            let p = &self.catalog.products[*i];
            let c = &self.cards[*i];
            let text = scores.as_ref().and_then(|s| s.get(i)).copied().unwrap_or(0) as i64 * 100_000;
            let boost = (p.has(Tag::Featured) as i64 * 3 + p.has(Tag::Bestseller) as i64 * 2) * 1_000;
            text + c.in_stock as i64 * 50_000 + boost + (p.rating * 100.0) as i64
        };
        match q.sort {
            Sort::Relevance => rows.sort_by_key(|i| std::cmp::Reverse(relevance(i))),
            Sort::PriceAsc => rows.sort_by_key(|&i| (self.cards[i].price, i)),
            Sort::PriceDesc => rows.sort_by_key(|&i| (std::cmp::Reverse(self.cards[i].price), i)),
            Sort::Newest => rows.sort_by_key(|&i| {
                let p = &self.catalog.products[i];
                (!p.has(Tag::New), std::cmp::Reverse(p.id))
            }),
            Sort::Rating => rows.sort_by_key(|&i| {
                let p = &self.catalog.products[i];
                (std::cmp::Reverse((p.rating * 100.0) as i64), std::cmp::Reverse(p.reviews))
            }),
            Sort::Discount => rows.sort_by_key(|&i| (std::cmp::Reverse(discount_pct(&self.cards[i])), i)),
        }

        let total = rows.len() as u32;
        let pages = total.div_ceil(PER_PAGE).max(1);
        let page = q.page.clamp(1, pages);
        let items = rows
            .into_iter()
            .skip(((page - 1) * PER_PAGE) as usize)
            .take(PER_PAGE as usize)
            .map(|i| self.cards[i].clone())
            .collect();
        ListingPage { query: ListingQuery { page, ..q.clone() }, category: cat, items, total, page, pages, facets }
    }

    pub fn product_view(&self, slug: &str, media: &Media) -> Option<ProductView> {
        let p = self.product_by_slug(slug)?;
        Some(ProductView {
            product: p.clone(),
            images: p.images.iter().map(|k| media.img(k)).collect(),
            brand: p.brand.as_deref().and_then(|b| self.brand(b)).cloned(),
            category: self.category(p.category).cloned().unwrap_or_else(|| NavCategory {
                id: p.category.0,
                slug: String::new(),
                name: L10n::default(),
                blurb: L10n::default(),
                icon: String::new(),
                img: None,
                count: 0,
            }),
        })
    }

    /// Same category first (closest price), then same brand elsewhere, then top rated.
    pub fn related(&self, id: ProductId, limit: usize) -> Vec<CardData> {
        let Some(me) = self.product(id) else { return vec![] };
        let my_price = self.card(id).map(|c| c.price.0).unwrap_or(0);
        let mut scored: Vec<(i64, usize)> = self
            .catalog
            .products
            .iter()
            .enumerate()
            .filter(|(_, p)| p.id != id)
            .map(|(i, p)| {
                let same_cat = (p.category == me.category) as i64;
                let same_brand = (p.brand.is_some() && p.brand == me.brand) as i64;
                let price_gap = (self.cards[i].price.0 - my_price).abs() / 100;
                let stock = self.cards[i].in_stock as i64;
                (same_cat * 10_000_000 + same_brand * 2_000_000 + stock * 1_000_000 - price_gap.min(999_999), i)
            })
            .collect();
        scored.sort_by_key(|(s, _)| std::cmp::Reverse(*s));
        scored.into_iter().take(limit).map(|(_, i)| self.cards[i].clone()).collect()
    }

    pub fn search(&self, q: &str, limit: usize) -> SearchResults {
        let scores = self.index.matches(q);
        let mut hits: Vec<(usize, u32)> = scores.into_iter().collect();
        hits.sort_by_key(|&(i, s)| {
            let c = &self.cards[i];
            (std::cmp::Reverse(s), !c.in_stock, std::cmp::Reverse(c.reviews))
        });
        let total = hits.len() as u32;
        let qt = search::tokens(q);
        let categories = self
            .nav
            .iter()
            .filter(|c| {
                let words = format!("{} {}", search::fold(&c.name.en), search::fold(&c.name.ar));
                !qt.is_empty() && qt.iter().all(|t| words.split(' ').any(|w| w.starts_with(t.as_str())))
            })
            .take(3)
            .cloned()
            .collect();
        let hits = hits
            .into_iter()
            .take(limit)
            .map(|(i, _)| {
                let p = &self.catalog.products[i];
                SearchHit {
                    card: self.cards[i].clone(),
                    name_en: search::highlight(&p.name.en, q),
                    name_ar: search::highlight(&p.name.ar, q),
                    category: self.category(p.category).map(|c| c.name.clone()).unwrap_or_default(),
                }
            })
            .collect();
        SearchResults { q: q.to_owned(), hits, total, categories }
    }

    /// Resolve a (product, variant) pair.
    pub fn variant(&self, product: ProductId, variant: VariantId) -> Option<(&Product, &Variant)> {
        let p = self.product(product)?;
        Some((p, p.variant(variant)?))
    }
}

fn discount_pct(c: &CardData) -> u8 {
    c.compare_at.map(|was| was.percent_off(c.price)).unwrap_or(0)
}

fn card(p: &Product, brand: Option<L10n>, media: &Media) -> CardData {
    let dv = p.default_variant();
    let prices: Vec<Piastres> = p.variants.iter().map(|v| v.price).collect();
    let total = p.total_stock();
    CardData {
        id: p.id,
        slug: p.slug.clone(),
        name: p.name.clone(),
        brand,
        img: p.images.first().map(|k| media.img(k)).unwrap_or_else(|| crate::media::Img::missing("")),
        price: dv.price,
        compare_at: dv.compare_at.filter(|c| *c > dv.price),
        from: prices.iter().any(|x| *x != dv.price),
        rating: p.rating,
        reviews: p.reviews,
        tags: p.tags.clone(),
        in_stock: total > 0,
        low_stock: (total > 0 && total <= 5).then_some(total),
        default_variant: dv.id,
        single_variant: p.variants.len() == 1 && p.modifier_groups.iter().all(|g| !g.required),
    }
}
