//! Search data for the browser and for empty results:
//!
//! - [`search_suggest`], `GET /api/search/suggest?q=` — the header's instant search. It wraps the
//!   data layer's `api::search_products` (Arabic-aware folding, synonyms, highlight segments) and
//!   formats everything for the visitor: prices in their numerals, the "see all" label, the
//!   status line. The browser fills `<template id="tpl-suggest-*">` with it (`features/browse.ts`).
//! - What to offer when a search finds nothing: a respelling that does find something ("did you
//!   mean"), popular searches, and the busiest aisles. Computed from the tenant's own catalogue
//!   with the same folding, so `ارز` / `أرز` and Western or Arabic-Indic digits are one word here
//!   as they are in search.

use crate::api::types::{ListingQuery, Seg};
use crate::components::icons;
use crate::format;
use crate::pages::category::listing::quoted;
use crate::prefs::{Lang, Numerals};
use domain::L10n;
use leptos::prelude::*;
use leptos::server_fn::codec::GetUrl;
use serde::{Deserialize, Serialize};

/// Products in the instant-search panel.
pub const HITS: u32 = 8;

/// `GET /api/search/suggest` (`client/src/api/types.ts`: `Suggest`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Suggest {
    /// The question as the server read it (the client drops answers to older questions).
    pub q: String,
    pub hits: Vec<SuggestHit>,
    pub categories: Vec<SuggestCategory>,
    /// "See all 37 results for “phone”"; `None` when nothing matched.
    pub all: Option<SuggestAll>,
    /// "8 suggestions" (the live region), or the empty state's title.
    pub status: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SuggestHit {
    pub href: String,
    /// The name in the visitor's language, matched runs marked.
    pub name: Vec<Seg>,
    pub category: String,
    /// Thumbnail (`src`, `srcset`), `None` without a photo.
    pub img: Option<String>,
    pub srcset: Option<String>,
    /// The placeholder tint behind the packshot (`#rrggbb`).
    pub tint: String,
    /// Formatted price, and the struck "was" price when on sale.
    pub price: String,
    pub was: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SuggestCategory {
    /// The search inside that aisle.
    pub href: String,
    pub name: String,
    /// `“phone”` / `«تليفون»`, as the row shows it.
    pub quoted: String,
    /// The aisle's lucide icon: the `d` of its one `<path>` (see `components::icons`).
    pub icon: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SuggestAll {
    pub href: String,
    pub label: String,
}

/// The instant search's answer for `q`, in the visitor's language and numerals. Public catalogue
/// data (cacheable at the edge like a listing); `?fail=search` and the simulated latency apply.
#[server(endpoint = "search/suggest", input = GetUrl)]
pub async fn search_suggest(q: String) -> Result<Suggest, ServerFnError> {
    let rq = crate::request::init();
    let (lang, num) = (rq.prefs.lang, rq.prefs.numerals);
    crate::server::cache::public(&[]);
    let r = crate::api::search_products(q, HITS).await?;
    Ok(build(&r, lang, num))
}

/// The server's answer shaped for the panel (no I/O: the formatting lives here once).
pub fn build(r: &crate::api::types::SearchResults, lang: Lang, num: Numerals) -> Suggest {
    let ar = lang.is_rtl();
    let quoted_q = quoted(&r.q, lang);
    let hits = r
        .hits
        .iter()
        .map(|h| {
            let img = &h.card.img;
            let live = !img.is_missing();
            SuggestHit {
                href: format!("/p/{}", h.card.slug),
                name: if ar {
                    h.name_ar.clone()
                } else {
                    h.name_en.clone()
                },
                category: h.category.pick(ar).to_owned(),
                img: live.then(|| img.src_near(96)),
                srcset: live.then(|| img.srcset()),
                tint: img.color.clone(),
                price: format::money(h.card.price, lang, num),
                was: h
                    .card
                    .compare_at
                    .filter(|w| *w > h.card.price)
                    .map(|w| format::money(w, lang, num)),
            }
        })
        .collect::<Vec<_>>();
    let categories = r
        .categories
        .iter()
        .map(|c| SuggestCategory {
            href: ListingQuery {
                category: Some(c.slug.clone()),
                q: Some(r.q.clone()),
                page: 1,
                ..Default::default()
            }
            .href(),
            name: c.name.pick(ar).to_owned(),
            quoted: quoted_q.clone(),
            icon: icons::path(&c.icon)
                .or_else(|| icons::path("layout-grid"))
                .unwrap_or_default()
                .to_owned(),
        })
        .collect::<Vec<_>>();
    let all = (r.total > 0).then(|| SuggestAll {
        href: ListingQuery {
            q: Some(r.q.clone()),
            page: 1,
            ..Default::default()
        }
        .href(),
        label: match lang {
            Lang::En => format!(
                "See all {} results for {quoted_q}",
                format::int(r.total as i64, num)
            ),
            Lang::Ar => format!(
                "شوف كل النتائج لـ{quoted_q} ({})",
                format::int(r.total as i64, num)
            ),
        },
    });
    let n = hits.len() as u32;
    let status = if hits.is_empty() && categories.is_empty() {
        format!(
            "{} {quoted_q}",
            crate::prefs::tr(lang, "No matches for", "مفيش نتائج لـ")
        )
    } else {
        match lang {
            Lang::En => format!(
                "{} {}",
                format::count(n, num),
                format::plural_en(n, "suggestion", "suggestions")
            ),
            Lang::Ar => format!(
                "{} {}",
                format::count(n, num),
                format::plural_ar(n, "اقتراح", "اقتراحان", "اقتراحات", "اقتراحًا")
            ),
        }
    };
    Suggest {
        q: r.q.clone(),
        hits,
        categories,
        all,
        status,
    }
}

/// An aisle to suggest.
#[derive(Clone, Debug, PartialEq)]
pub struct CatLink {
    pub slug: String,
    pub name: L10n,
    pub icon: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Suggestions {
    /// The search these are for.
    pub q: String,
    /// The whole query respelt, when the respelling has results (`samsong` → `samsung`).
    pub did_you_mean: Option<String>,
    /// Popular searches, in both languages.
    pub popular: Vec<L10n>,
}

/// The busiest aisles, for empty states and the 404 (synchronous: in-memory catalogue).
#[cfg(feature = "ssr")]
pub fn popular_categories(limit: usize, except: Option<&str>) -> Vec<CatLink> {
    let catalog = crate::server::current_catalog();
    let mut cats: Vec<_> = catalog
        .nav()
        .iter()
        .filter(|c| Some(c.slug.as_str()) != except && c.count > 0)
        .collect();
    cats.sort_by_key(|c| std::cmp::Reverse(c.count));
    cats.into_iter()
        .take(limit)
        .map(|c| CatLink {
            slug: c.slug.clone(),
            name: c.name.clone(),
            icon: c.icon.clone(),
        })
        .collect()
}

#[cfg(feature = "ssr")]
pub fn suggestions_now(q: &str) -> Suggestions {
    use crate::server::search::{fold, tokens};
    use std::collections::HashMap;

    let catalog = crate::server::current_catalog();
    // Vocabulary: every word of product, brand and aisle names, folded → first spelling seen.
    let mut vocab: HashMap<String, String> = HashMap::new();
    let mut add = |text: &str| {
        for word in text
            .split(|c: char| !c.is_alphanumeric())
            .filter(|w| w.chars().count() >= 3)
        {
            vocab.entry(fold(word)).or_insert_with(|| word.to_owned());
        }
    };
    for p in catalog.products() {
        add(&p.name.en);
        add(&p.name.ar);
        if let Some(b) = p.brand.as_deref().and_then(|b| catalog.brand(b)) {
            add(&b.name.en);
            add(&b.name.ar);
        }
    }
    for c in catalog.nav() {
        add(&c.name.en);
        add(&c.name.ar);
    }

    let mut changed = false;
    let fixed: Vec<String> = tokens(q)
        .into_iter()
        .map(|t| {
            if vocab.keys().any(|w| w.starts_with(&t)) {
                return t;
            }
            let budget = if t.chars().count() <= 4 { 1 } else { 2 };
            let best = vocab
                .iter()
                .map(|(folded, word)| (distance(&t, folded), folded.len(), word))
                .filter(|(d, _, _)| *d <= budget)
                .min_by_key(|(d, len, _)| (*d, *len));
            match best {
                Some((_, _, word)) => {
                    changed = true;
                    word.clone()
                }
                None => t,
            }
        })
        .collect();
    let respelt = fixed.join(" ");
    let did_you_mean = (changed && catalog.search(&respelt, 1).total > 0).then_some(respelt);

    // Popular: the brands of the most-reviewed products; where brands are scarce (fresh
    // groceries), the first word of the product's name ("Tomatoes", "طماطم").
    let mut products: Vec<_> = catalog.products().iter().collect();
    products.sort_by_key(|p| std::cmp::Reverse(p.reviews));
    let mut popular: Vec<L10n> = Vec::new();
    let mut push = |label: L10n| {
        if popular.len() < 6
            && !label.en.is_empty()
            && !popular.iter().any(|x| x.en.eq_ignore_ascii_case(&label.en))
        {
            popular.push(label);
        }
    };
    for p in products.iter().take(60) {
        if let Some(b) = p.brand.as_deref().and_then(|b| catalog.brand(b)) {
            push(b.name.clone());
        }
    }
    let first = |s: &str| s.split_whitespace().next().unwrap_or_default().to_owned();
    for p in &products {
        push(L10n::new(first(&p.name.en), first(&p.name.ar)));
    }
    Suggestions {
        q: q.to_owned(),
        did_you_mean,
        popular,
    }
}

/// Damerau–Levenshtein (optimal string alignment) distance over chars.
#[cfg(feature = "ssr")]
fn distance(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    if a.len().abs_diff(b.len()) > 2 {
        return usize::MAX;
    }
    let mut d = vec![vec![0usize; b.len() + 1]; a.len() + 1];
    for (i, row) in d.iter_mut().enumerate() {
        row[0] = i;
    }
    for j in 0..=b.len() {
        d[0][j] = j;
    }
    for i in 1..=a.len() {
        for j in 1..=b.len() {
            let cost = usize::from(a[i - 1] != b[j - 1]);
            d[i][j] = (d[i - 1][j] + 1)
                .min(d[i][j - 1] + 1)
                .min(d[i - 1][j - 1] + cost);
            if i > 1 && j > 1 && a[i - 1] == b[j - 2] && a[i - 2] == b[j - 1] {
                d[i][j] = d[i][j].min(d[i - 2][j - 2] + 1);
            }
        }
    }
    d[a.len()][b.len()]
}
