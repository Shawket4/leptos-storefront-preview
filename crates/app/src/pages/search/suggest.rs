//! What to offer when a search finds nothing: a respelling that does find something ("did you
//! mean"), popular searches, and the busiest aisles.
//!
//! A minimal local addition (the data layer has no suggestions yet): computed from the tenant's
//! own catalogue with the data layer's Arabic-aware folding, so `ارز` / `أرز` and Western or
//! Arabic-Indic digits are the same word here as they are in search.

use domain::L10n;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// An aisle to suggest (light enough for island props).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CatLink {
    pub slug: String,
    pub name: L10n,
    pub icon: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Suggestions {
    /// The search these are for.
    pub q: String,
    /// The whole query respelt, when the respelling has results (`samsong` → `samsung`).
    pub did_you_mean: Option<String>,
    /// Popular searches, in both languages.
    pub popular: Vec<L10n>,
}

/// Suggestions for a search with no results.
#[server]
pub async fn search_suggestions(q: String) -> Result<Suggestions, ServerFnError> {
    crate::request::init();
    Ok(suggestions_now(q.trim()))
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
