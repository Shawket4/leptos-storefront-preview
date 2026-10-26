//! Pagination: links only (works without JS; the router follows them). Previous / next with
//! chevrons that mirror in Arabic, then page numbers with ellipses for the gaps, the current page
//! filled and `aria-current="page"`. A previous or next that doesn't exist keeps its place,
//! greyed, so nothing shifts between pages.
//!
//! It fits its own width (a container query, not the viewport): a narrow pager shows the first,
//! the current and the last page — previous and next reach the neighbours — and words on the
//! steps only where there is room for them (screen readers always hear them).
//!
//! ```ignore
//! <Pager page=cur pages=total href=move |n| format!("{}#results", q.with_page(n).href())/>
//! ```

use super::icons::Icon;
use crate::format;
use crate::prefs::{tr, use_ui};
use leptos::either::Either;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Slot {
    /// A page link. `wide`: shown only when the pager has room.
    Page { n: u32, wide: bool },
    /// An ellipsis. `narrow`: only where a narrow pager skips pages a wide one shows.
    Gap { narrow: bool },
}

/// Wide: every page up to 7, else the first, the last and two either side of the current one.
/// Narrow: every page up to 4, else the first, the current and the last.
fn slots(cur: u32, pages: u32) -> Vec<Slot> {
    let wide: Vec<u32> = if pages <= 7 {
        (1..=pages).collect()
    } else {
        let lo = cur.saturating_sub(2).max(2);
        let hi = (cur + 2).min(pages - 1);
        std::iter::once(1)
            .chain(lo..=hi)
            .chain(std::iter::once(pages))
            .collect()
    };
    let narrow = |n: u32| pages <= 4 || n == 1 || n == cur || n == pages;
    let mut out = Vec::new();
    let (mut last, mut last_narrow, mut gap_since) = (0, 0, false);
    for n in wide {
        if last != 0 && n > last + 1 {
            out.push(Slot::Gap { narrow: false });
            gap_since = true;
        }
        if narrow(n) {
            if last_narrow != 0 && n > last_narrow + 1 && !gap_since {
                out.push(Slot::Gap { narrow: true });
            }
            out.push(Slot::Page { n, wide: false });
            last_narrow = n;
            gap_since = false;
        } else {
            out.push(Slot::Page { n, wide: true });
        }
        last = n;
    }
    out
}

#[component]
pub fn Pager<F>(
    /// The current page (1-based).
    page: u32,
    /// How many pages there are. One or none: nothing is rendered.
    pages: u32,
    /// The address of page `n` (a fragment included, e.g. `#results`).
    href: F,
    /// Names the navigation for screen readers; "Pages" by default.
    #[prop(optional, into)]
    label: Option<String>,
    #[prop(optional, into)] class: String,
) -> impl IntoView
where
    F: Fn(u32) -> String + 'static,
{
    if pages <= 1 {
        return Either::Right(());
    }
    let ui = use_ui();
    let lang = ui.lang.get_untracked();
    let num = ui.numerals.get_untracked();
    let cur = page.clamp(1, pages);
    let cls = if class.is_empty() {
        "pager".to_owned()
    } else {
        format!("pager {class}")
    };
    let label = label.unwrap_or_else(|| tr(lang, "Pages", "الصفحات").to_owned());
    let page_word = tr(lang, "Page", "صفحة");
    let step = |to: Option<String>, next: bool| {
        let (icon, word, rel) = if next {
            ("chevron-right", tr(lang, "Next", "التالي"), "next")
        } else {
            ("chevron-left", tr(lang, "Previous", "السابق"), "prev")
        };
        let side = if next {
            "pager__step pager__step--next"
        } else {
            "pager__step pager__step--prev"
        };
        // "‹ Previous", "Next ›": the chevron on the outside, pointing the way it goes.
        let glyph = view! { <Icon name=icon size=18 class="flip-rtl"/> };
        let word = view! { <span class="pager__step-label">{word}</span> };
        let inner = if next {
            Either::Left(view! { {word}{glyph} })
        } else {
            Either::Right(view! { {glyph}{word} })
        };
        match to {
            Some(h) => {
                Either::Left(view! { <a class=side href=h rel=rel data-press="">{inner}</a> })
            }
            None => Either::Right(view! {
                <span class=format!("{side} is-off") aria-hidden="true">{inner}</span>
            }),
        }
    };
    let prev = step((cur > 1).then(|| href(cur - 1)), false);
    let next = step((cur < pages).then(|| href(cur + 1)), true);
    let items = slots(cur, pages)
        .into_iter()
        .map(|s| match s {
            Slot::Gap { narrow } => Either::Left(view! {
                <li class="pager__gap" class:pager__gap--narrow=narrow aria-hidden="true">"…"</li>
            }),
            Slot::Page { n, wide } => {
                let aria = format!("{page_word} {}", format::count(n, num));
                let current = (n == cur).then_some("page");
                Either::Right(view! {
                    <li class:pager__wide=wide>
                        <a class="pager__page" href=href(n) aria-label=aria aria-current=current data-press="">
                            {format::count(n, num)}
                        </a>
                    </li>
                })
            }
        })
        .collect_view();
    Either::Left(view! {
        <nav class=cls aria-label=label>
            {prev}
            <ol class="pager__list" role="list">{items}</ol>
            {next}
        </nav>
    })
}
