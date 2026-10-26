//! Listing helpers shared by the category and search pages: the URL each control leads to (every
//! one through `ListingQuery`), and the labels and titles they show.

use crate::api::types::{ListingQuery, NavCategory, PER_PAGE, Sort};
use crate::format;
use crate::prefs::{Lang, Numerals, tr};
use domain::L10n;

/// Whole pounds as a label: `EGP 1,250` / `1,250 ج.م` (figure isolated LTR).
pub fn pounds(n: i64, lang: Lang, num: Numerals) -> String {
    let fig = format::int(n, num);
    match lang {
        Lang::En => format!("{} {fig}", format::currency_label(lang)),
        Lang::Ar => format!("{} {}", format::ltr(&fig), format::currency_label(lang)),
    }
}

/// The price filter as a chip: `EGP 1,000–5,000`, `From EGP 1,000`, `Up to EGP 5,000`.
pub fn price_label(min: Option<i64>, max: Option<i64>, lang: Lang, num: Numerals) -> String {
    let cur = format::currency_label(lang);
    let fig = |n| format::int(n, num);
    match (min, max, lang) {
        (Some(a), Some(b), Lang::En) => format!("{cur} {}–{}", fig(a), fig(b)),
        (Some(a), Some(b), Lang::Ar) => {
            format!("{} {cur}", format::ltr(&format!("{}–{}", fig(a), fig(b))))
        }
        (Some(a), None, _) => format!("{} {}", tr(lang, "From", "من"), pounds(a, lang, num)),
        (None, Some(b), _) => format!("{} {}", tr(lang, "Up to", "لحد"), pounds(b, lang, num)),
        (None, None, _) => String::new(),
    }
}

/// Which results are on screen: `1–24 of 187 items` / `١–٢٤ من ١٨٧ منتجًا`; just the count
/// when they all fit on one page.
pub fn range_label(page: u32, total: u32, lang: Lang, num: Numerals) -> String {
    if total == 0 {
        return tr(lang, "No items", "مفيش منتجات").to_owned();
    }
    // One page holds them all: the range would only repeat the count.
    if total <= PER_PAGE {
        return format::items(total, lang, num);
    }
    let first = (page.max(1) - 1) * PER_PAGE + 1;
    let last = (page.max(1) * PER_PAGE).min(total);
    let span = format!(
        "{}–{}",
        format::int(first as i64, num),
        format::int(last as i64, num)
    );
    let all = format::items(total, lang, num);
    match lang {
        Lang::En => format!("{span} of {all}"),
        Lang::Ar => format!("{} من {all}", format::ltr(&span)),
    }
}

fn first_page(mut q: ListingQuery) -> ListingQuery {
    q.page = 1;
    q
}

/// Same listing and sort, no filters (what "Clear all" goes to).
pub fn cleared(q: &ListingQuery) -> ListingQuery {
    ListingQuery {
        category: q.category.clone(),
        q: q.q.clone(),
        sort: q.sort,
        page: 1,
        ..Default::default()
    }
}

pub fn with_brand(q: &ListingQuery, id: &str, on: bool) -> ListingQuery {
    let mut n = first_page(q.clone());
    n.brands.retain(|b| b != id);
    if on {
        n.brands.push(id.to_owned());
        n.brands.sort();
    }
    n
}

pub fn with_price(q: &ListingQuery, min: Option<i64>, max: Option<i64>) -> ListingQuery {
    ListingQuery {
        min,
        max,
        ..first_page(q.clone())
    }
}

pub fn with_stock(q: &ListingQuery, on: bool) -> ListingQuery {
    ListingQuery {
        in_stock: on,
        ..first_page(q.clone())
    }
}

pub fn with_deal(q: &ListingQuery, on: bool) -> ListingQuery {
    ListingQuery {
        deal: on,
        ..first_page(q.clone())
    }
}

pub fn with_sort(q: &ListingQuery, sort: Sort) -> ListingQuery {
    ListingQuery {
        sort,
        ..first_page(q.clone())
    }
}

pub fn without_text(q: &ListingQuery) -> ListingQuery {
    ListingQuery {
        q: None,
        ..first_page(q.clone())
    }
}

/// Filters set in the panel (the badge on the "Filters" button).
pub fn panel_filter_count(q: &ListingQuery) -> u32 {
    (q.min.is_some() || q.max.is_some()) as u32
        + q.brands.len() as u32
        + q.in_stock as u32
        + q.deal as u32
}

/// What a search-kind listing is called, from its query: a search, the deals, new arrivals …
pub struct Heading {
    pub kicker: Option<&'static str>,
    pub title: String,
    pub sub: Option<&'static str>,
}

/// Quote search text the way each language does: “rice” / «رز».
pub fn quoted(text: &str, lang: Lang) -> String {
    match lang {
        Lang::En => format!("“{text}”"),
        Lang::Ar => format!("«{text}»"),
    }
}

pub fn search_heading(q: &ListingQuery, lang: Lang) -> Heading {
    let (kicker, title, sub) = match &q.q {
        Some(text) => (
            Some(tr(lang, "Results for", "نتائج البحث عن")),
            quoted(text, lang),
            None,
        ),
        None if q.deal => (
            None,
            tr(lang, "Deals", "العروض").to_owned(),
            Some(tr(
                lang,
                "Everything on offer right now",
                "كل اللي عليه خصم دلوقتي",
            )),
        ),
        None if q.sort == Sort::Newest => (
            None,
            tr(lang, "New arrivals", "وصل حديثًا").to_owned(),
            Some(tr(lang, "Just landed on our shelves", "لسه نازلين الرفوف")),
        ),
        None if q.sort == Sort::Rating => (
            None,
            tr(lang, "Top rated", "الأعلى تقييمًا").to_owned(),
            Some(tr(
                lang,
                "What our shoppers love most",
                "اللي زباينّا بيحبوه أكتر",
            )),
        ),
        None => (
            None,
            tr(lang, "All products", "كل المنتجات").to_owned(),
            None,
        ),
    };
    Heading { kicker, title, sub }
}

/// `<title>` for a listing: `Phones · Nile`, `Phones — page 2 · Nile`, `Results for “rice” · Souq`.
pub fn doc_title(
    q: &ListingQuery,
    category: Option<&L10n>,
    site: &str,
    lang: Lang,
    num: Numerals,
) -> String {
    let name = match category {
        Some(c) => c.pick(lang.is_rtl()).to_owned(),
        None => {
            let h = search_heading(q, lang);
            match h.kicker {
                Some(k) => format!("{k} {}", h.title),
                None => h.title,
            }
        }
    };
    let page = if q.page > 1 {
        format!(
            " — {} {}",
            tr(lang, "page", "صفحة"),
            format::count(q.page, num)
        )
    } else {
        String::new()
    };
    format!("{name}{page} · {site}")
}

/// Meta description for a category listing.
pub fn category_description(c: &NavCategory, site: &str, lang: Lang, num: Numerals) -> String {
    let ar = lang.is_rtl();
    let blurb = c.blurb.pick(ar).trim_end_matches(['.', '،', ' ']);
    let count = format::items(c.count, lang, num);
    match lang {
        Lang::En => format!(
            "{} at {site}: {count}. {blurb}. Delivered across Egypt, cash on delivery.",
            c.name.en
        ),
        Lang::Ar => format!(
            "{} من {site}: {count}. {blurb}. توصيل لكل مصر والدفع عند الاستلام.",
            c.name.ar
        ),
    }
}
