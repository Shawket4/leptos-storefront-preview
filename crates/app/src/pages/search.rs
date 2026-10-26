//! Search results — `/search?q=…` plus the listing's filters and sort — and the header's instant
//! search box ([`HeaderSearch`], in `instant.rs`).
//!
//! `/search` is the no-JS fallback for the instant search and its "see all results" page, and,
//! without `q`, the all-products listing the home page links to (`?deal=1&sort=discount` for the
//! deals, `?sort=newest` for new arrivals). It renders the category page's listing with
//! `ListingQuery { category: None, q }`, which names itself from the query; a search that finds
//! nothing comes with "did you mean" and popular searches (`suggest.rs`).

mod instant;
pub(crate) mod suggest;

pub use instant::{HeaderSearch, SearchBox};

use crate::api::types::ListingQuery;
use crate::pages::category::{ListingResults, listing};
use crate::request::req;
use leptos::prelude::*;
use leptos_meta::{Link, Meta, Title};
use leptos_router::hooks::use_location;

#[component]
pub fn SearchPage() -> impl IntoView {
    let rq = req();
    let (lang, num) = (rq.prefs.lang, rq.prefs.numerals);
    let ar = lang.is_rtl();
    let query = ListingQuery::from_query(None, &use_location().search.get_untracked());
    #[cfg(feature = "ssr")]
    crate::server::cache::public(&[]);

    let site = rq.tenant.name.pick(ar).to_owned();
    let title = listing::doc_title(&query, None, &site, lang, num);
    let heading = listing::search_heading(&query, lang);
    let description = match (&query.q, heading.sub) {
        (Some(text), _) => match lang {
            crate::prefs::Lang::En => format!(
                "{} {} at {site}.",
                heading.kicker.unwrap_or_default(),
                listing::quoted(text, lang)
            ),
            crate::prefs::Lang::Ar => format!(
                "{} {} في {site}.",
                heading.kicker.unwrap_or_default(),
                listing::quoted(text, lang)
            ),
        },
        (None, Some(sub)) => format!("{} — {sub}.", heading.title),
        (None, None) => format!("{} · {}", heading.title, rq.tenant.tagline.pick(ar)),
    };
    let canonical = rq.absolute(&query.href());
    // Result pages for typed queries are thin and endless: keep them out of the index, follow links.
    let noindex = query.q.is_some();

    view! {
        <Title text=title/>
        <Meta name="description" content=description/>
        <Link rel="canonical" href=canonical/>
        {noindex.then(|| view! { <Meta name="robots" content="noindex, follow"/> })}

        <div class="listing container search-page">
            <div class="search-page__box">
                <HeaderSearch kind=SearchBox::Page/>
            </div>
            <ListingResults query aisles=crate::pages::category::aisles(None)/>
        </div>
    }
}
