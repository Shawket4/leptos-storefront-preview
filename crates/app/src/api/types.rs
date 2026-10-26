//! Data shapes exchanged between server functions, server components and islands. Plain serde
//! types, compiled on both sides.

use crate::components::product_card::CardData;
use crate::media::Img;
use domain::{Brand, L10n, Piastres, Product, ProductId, VariantId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NavCategory {
    pub id: u32,
    pub slug: String,
    pub name: L10n,
    pub blurb: L10n,
    pub icon: String,
    pub img: Option<Img>,
    pub count: u32,
}

/// Home-page merchandising rails.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Rail {
    Featured,
    Deals,
    New,
    Bestsellers,
    Express,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Sort {
    #[default]
    Relevance,
    PriceAsc,
    PriceDesc,
    Newest,
    Rating,
    Discount,
}

impl Sort {
    pub const ALL: [Sort; 6] =
        [Sort::Relevance, Sort::PriceAsc, Sort::PriceDesc, Sort::Newest, Sort::Rating, Sort::Discount];

    pub fn code(self) -> &'static str {
        match self {
            Sort::Relevance => "relevance",
            Sort::PriceAsc => "price-asc",
            Sort::PriceDesc => "price-desc",
            Sort::Newest => "newest",
            Sort::Rating => "rating",
            Sort::Discount => "discount",
        }
    }
    pub fn parse(s: &str) -> Sort {
        Sort::ALL.into_iter().find(|x| x.code() == s).unwrap_or_default()
    }
    pub fn label(self) -> L10n {
        let (en, ar) = match self {
            Sort::Relevance => ("Recommended", "المقترح"),
            Sort::PriceAsc => ("Price: low to high", "السعر: من الأقل"),
            Sort::PriceDesc => ("Price: high to low", "السعر: من الأعلى"),
            Sort::Newest => ("Newest", "الأحدث"),
            Sort::Rating => ("Top rated", "الأعلى تقييمًا"),
            Sort::Discount => ("Biggest discount", "أكبر خصم"),
        };
        L10n::new(en, ar)
    }
}

pub const PER_PAGE: u32 = 24;

/// A product listing request. Lives in the URL query string so it works without JS, can be
/// shared, and is cacheable at the edge:
/// `/c/phones?min=1000&max=20000&brand=samsung&brand=apple&stock=1&deal=1&sort=price-asc&page=2`.
// `default`: server-function args travel URL-encoded (serde_qs), which omits empty Vecs and
// `false`s, so every field must have a default on the way back in.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(default)]
pub struct ListingQuery {
    /// Category slug (`None` = all products, e.g. a deals listing).
    pub category: Option<String>,
    /// Free-text filter (search results reuse the listing).
    pub q: Option<String>,
    /// Price bounds in whole pounds.
    pub min: Option<i64>,
    pub max: Option<i64>,
    pub brands: Vec<String>,
    pub in_stock: bool,
    pub deal: bool,
    pub sort: Sort,
    /// 1-based.
    pub page: u32,
}

impl ListingQuery {
    /// Parse the query-string part (without `?`). Unknown keys are ignored.
    pub fn from_query(category: Option<String>, qs: &str) -> Self {
        let mut q = ListingQuery { category, page: 1, ..Default::default() };
        for pair in qs.trim_start_matches('?').split('&').filter(|s| !s.is_empty()) {
            let (k, v) = pair.split_once('=').unwrap_or((pair, ""));
            // Figures may arrive in Arabic-Indic digits from a no-JS form (٥٠٠ → 500).
            let v = ascii_digits(&url_decode(v));
            match k {
                "q" if !v.trim().is_empty() => q.q = Some(v.trim().to_owned()),
                "min" => q.min = v.parse().ok().filter(|n: &i64| *n >= 0),
                "max" => q.max = v.parse().ok().filter(|n: &i64| *n > 0),
                "brand" if !v.is_empty() => {
                    if !q.brands.contains(&v) {
                        q.brands.push(v)
                    }
                }
                "stock" => q.in_stock = v == "1" || v == "on" || v == "true",
                "deal" => q.deal = v == "1" || v == "on" || v == "true",
                "sort" => q.sort = Sort::parse(&v),
                "page" => q.page = v.parse().ok().filter(|n| *n >= 1).unwrap_or(1),
                _ => {}
            }
        }
        q.brands.sort();
        q
    }

    /// Canonical query string (stable key order, defaults omitted), without `?`.
    pub fn to_query(&self) -> String {
        let mut parts = Vec::new();
        if let Some(q) = &self.q {
            parts.push(format!("q={}", url_encode(q)));
        }
        if let Some(v) = self.min {
            parts.push(format!("min={v}"));
        }
        if let Some(v) = self.max {
            parts.push(format!("max={v}"));
        }
        for b in &self.brands {
            parts.push(format!("brand={}", url_encode(b)));
        }
        if self.in_stock {
            parts.push("stock=1".into());
        }
        if self.deal {
            parts.push("deal=1".into());
        }
        if self.sort != Sort::Relevance {
            parts.push(format!("sort={}", self.sort.code()));
        }
        if self.page > 1 {
            parts.push(format!("page={}", self.page));
        }
        parts.join("&")
    }

    /// The listing's own URL (`/c/<slug>?…`, or `/search?…` when there is no category).
    pub fn href(&self) -> String {
        let base = match &self.category {
            Some(slug) => format!("/c/{slug}"),
            None => "/search".to_owned(),
        };
        let qs = self.to_query();
        if qs.is_empty() { base } else { format!("{base}?{qs}") }
    }

    pub fn with_page(&self, page: u32) -> Self {
        ListingQuery { page, ..self.clone() }
    }

    /// Any filter beyond category/sort/page applied?
    pub fn has_filters(&self) -> bool {
        self.min.is_some() || self.max.is_some() || !self.brands.is_empty() || self.in_stock || self.deal
    }
}

/// `٠-٩` / `۰-۹` → `0-9`, and the Arabic thousands/decimal separators dropped/normalised, so
/// numbers typed with Arabic-Indic digits parse like Western ones.
pub fn ascii_digits(s: &str) -> String {
    s.chars()
        .filter_map(|c| match c {
            '٠'..='٩' => char::from_u32('0' as u32 + (c as u32 - '٠' as u32)),
            '۰'..='۹' => char::from_u32('0' as u32 + (c as u32 - '۰' as u32)),
            '\u{066C}' => None,      // Arabic thousands separator
            '\u{066B}' => Some('.'), // Arabic decimal separator
            c => Some(c),
        })
        .collect()
}

pub fn url_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => out.push(b as char),
            b' ' => out.push('+'),
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

pub fn url_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if b == b'+' {
            out.push(b' ');
            i += 1;
            continue;
        }
        if b == b'%' {
            let hex = bytes.get(i + 1..i + 3).and_then(|h| std::str::from_utf8(h).ok());
            if let Some(v) = hex.and_then(|h| u8::from_str_radix(h, 16).ok()) {
                out.push(v);
                i += 3;
                continue;
            }
        }
        out.push(b);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BrandFacet {
    pub id: String,
    pub name: L10n,
    pub count: u32,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Facets {
    pub brands: Vec<BrandFacet>,
    /// Whole pounds across the unfiltered set (for the price inputs' placeholders).
    pub price_min: i64,
    pub price_max: i64,
    pub in_stock_count: u32,
    pub deal_count: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListingPage {
    pub query: ListingQuery,
    pub category: Option<NavCategory>,
    pub items: Vec<CardData>,
    pub total: u32,
    pub page: u32,
    pub pages: u32,
    pub facets: Facets,
}

/// Everything the product page needs.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProductView {
    pub product: Product,
    /// `product.images`, resolved, same order.
    pub images: Vec<Img>,
    pub brand: Option<Brand>,
    pub category: NavCategory,
}

/// A run of text, highlighted or not (search-match highlighting without client-side matching).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Seg {
    pub text: String,
    pub hit: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchHit {
    pub card: CardData,
    pub name_en: Vec<Seg>,
    pub name_ar: Vec<Seg>,
    /// Category name, for the suggestion's second line.
    pub category: L10n,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchResults {
    pub q: String,
    pub hits: Vec<SearchHit>,
    pub total: u32,
    /// Categories whose name matches, for "search in …" shortcuts.
    pub categories: Vec<NavCategory>,
}

/// One line in the cart, priced by Madar's rules (`madar-catalog::price_line`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CartLineView {
    /// Stable key of the line: product, variant and add-ons.
    pub key: String,
    pub product: ProductId,
    pub variant: VariantId,
    pub slug: String,
    pub name: L10n,
    pub variant_label: L10n,
    pub addons: Vec<L10n>,
    pub img: Img,
    pub qty: u32,
    /// Stock available (cap for the quantity stepper).
    pub max_qty: u32,
    pub unit_price: Piastres,
    pub compare_at: Option<Piastres>,
    pub line_total: Piastres,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CartView {
    pub lines: Vec<CartLineView>,
    /// Total quantity across lines (the badge).
    pub count: u32,
    pub subtotal: Piastres,
    /// What the shopper saves against compare-at prices.
    pub savings: Piastres,
    pub delivery_fee: Piastres,
    /// Subtotal at which delivery becomes free.
    pub free_delivery_over: Piastres,
    /// VAT included in the total (14%, prices are tax-inclusive) — from `madar-money::tax`.
    pub tax_included: Piastres,
    pub total: Piastres,
}

/// A delivery window offered at checkout.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliverySlot {
    pub id: String,
    pub day: L10n,
    pub window: L10n,
    pub fee: Piastres,
    pub express: bool,
    pub available: bool,
}

/// Read-cache counters, for the showcase.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CacheInfo {
    pub entries: u64,
    pub hits: u64,
    pub misses: u64,
    pub generation: u64,
    pub base_latency_ms: u64,
}
