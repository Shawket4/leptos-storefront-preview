//! `HeaderSearch`: the instant-search box (in the site header, and at the top of `/search`).
//!
//! Without JS it is a plain `GET /search?q=` form. With JS (`data-ts="search-box"`,
//! `features/browse.ts`) it is an ARIA combobox: typing asks `GET /api/search/suggest?q=`
//! (`suggest::search_suggest`) after a short pause and fills the listbox from the templates below
//! — markup and labels stay here, the browser only puts the server's data in. Keyboard: ↑/↓ move
//! through the options, Enter opens the active one or searches, Escape closes, then clears.
//!
//! On phones (≤ 600 px) the header shows a search button; with JS it opens this form as a
//! full-screen overlay, without JS it is a link to `/search`, which has the form on the page.

use crate::api::types::ListingQuery;
use crate::components::icons::Icon;
use crate::prefs::{Lang, tr};
use crate::request::req;
use crate::strings;
use leptos::prelude::*;

/// Where the box sits: the site header (with the phone overlay) or the top of `/search`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SearchBox {
    #[default]
    Header,
    Page,
}

/// The `q` of the current URL, when it is a search page (so the box shows what was searched).
fn url_query() -> Option<String> {
    let pq = req().path_and_query;
    let (path, qs) = pq.split_once('?').unwrap_or((&pq, ""));
    (path == "/search")
        .then(|| ListingQuery::from_query(None, qs).q)
        .flatten()
}

#[component]
pub fn HeaderSearch(#[prop(optional)] kind: Option<SearchBox>) -> impl IntoView {
    let lang = req().prefs.lang;
    let header = kind.unwrap_or_default() == SearchBox::Header;
    // Ids: the header box and the page's box can be on one page.
    let p: &'static str = if header { "hs" } else { "ps" };
    let initial = url_query().unwrap_or_default();
    let placeholder = tr(lang, "Search products and brands", "دوّر على منتج أو ماركة");
    let empty = initial.is_empty();
    let cls = if header { "hs" } else { "hs hs--page" };

    view! {
        <div class=cls data-ts="search-box">
            {header.then(|| view! {
                <a class="hs-trigger" href="/search" aria-haspopup="dialog">
                    <Icon name="search" size=20/>
                    <span class="hs-trigger__text" class:is-filled=!empty>
                        {if empty { placeholder.to_owned() } else { initial.clone() }}
                    </span>
                </a>
            })}
            <form class="hs-form" action="/search" method="get" role="search">
                <div class="hs-field">
                    {header.then(|| view! {
                        <button type="button" class="hs-back" aria-label=tr(lang, "Close search", "اقفل البحث")>
                            <Icon name="arrow-left" size=20 class="flip-rtl"/>
                        </button>
                    })}
                    <span class="hs-lead" aria-hidden="true">
                        <Icon name="search" size=18 class="hs-glass"/>
                        <Icon name="loader-circle" size=18 class="hs-busy spin"/>
                    </span>
                    <input
                        class="hs-input"
                        id=format!("{p}-input")
                        type="search"
                        name="q"
                        role="combobox"
                        aria-autocomplete="list"
                        aria-controls=format!("{p}-lb")
                        aria-expanded="false"
                        aria-label=tr(lang, "Search products", "دوّر في المنتجات")
                        autocomplete="off"
                        autocapitalize="off"
                        spellcheck="false"
                        enterkeyhint="search"
                        maxlength="80"
                        placeholder=placeholder
                        value=initial
                    />
                    <button type="button" class="hs-clear" hidden=empty aria-label=strings::get("browse.clear", lang)>
                        <Icon name="x" size=18/>
                    </button>
                    {(!header).then(|| view! {
                        // The page's box names its action; the header's submits with Enter (the
                        // phone keyboard shows "Search") and stays one quiet field.
                        <button type="submit" class="hs-submit">
                            <span class="hs-submit__label">{tr(lang, "Search", "دوّر")}</span>
                        </button>
                    })}
                </div>
                <div class="hs-panel" hidden>
                    <div class="hs-list" role="listbox" id=format!("{p}-lb") aria-label=tr(lang, "Suggestions", "اقتراحات")>
                        <div class="hs-group" role="group" data-group="cats" hidden
                            aria-label=tr(lang, "Categories", "الأقسام") data-label=tr(lang, "Categories", "الأقسام")></div>
                        <div class="hs-group" role="group" data-group="hits" hidden
                            aria-label=tr(lang, "Products", "المنتجات") data-label=tr(lang, "Products", "المنتجات")></div>
                        <div class="hs-all" data-group="all"></div>
                    </div>
                    <div class="hs-list hs-list--skel" data-state="loading" aria-hidden="true" hidden>
                        {(0..3).map(|_| view! {
                            <div class="hs-opt hs-opt--hit hs-opt--skel">
                                <span class="hs-thumb skeleton"></span>
                                <span class="hs-opt__main">
                                    <span class="skeleton hs-skel-line"></span>
                                    <span class="skeleton hs-skel-line hs-skel-line--short"></span>
                                </span>
                            </div>
                        }).collect_view()}
                    </div>
                    <div class="hs-note" data-state="empty" hidden>
                        <p class="hs-note__title" data-slot="title"></p>
                        <p>{tr(lang, "Check the spelling or try a different word.", "اتأكد من الكلمة أو جرّب كلمة تانية.")}</p>
                    </div>
                    <div class="hs-note" data-state="offline" role="alert" hidden>
                        <p class="hs-note__title">{strings::get("net.offline", lang)}</p>
                        <p>{tr(lang, "We'll search as soon as you're back online.", "سيتم البحث فور عودة الاتصال.")}</p>
                    </div>
                    <div class="hs-note" data-state="error" role="alert" hidden>
                        <p class="hs-note__title">{tr(lang, "Search isn't available right now.", "البحث غير متاح الآن.")}</p>
                        <button type="button" class="hs-retry" data-retry="">
                            <Icon name="rotate-ccw" size=16/>
                            <span>{strings::get("net.retry", lang)}</span>
                        </button>
                    </div>
                </div>
                <p class="sr-only" role="status" data-status=""></p>
            </form>
            // Once per page: the header's box is on every page; the /search box uses these too.
            {header.then(|| view! { <Templates lang/> })}
        </div>
    }
}

/// The rows the browser fills (`fill()` in `core/dom.ts`): text goes into `data-slot`s, links and
/// images into `data-attr-*`. A product name's matched runs become `<span data-hit>`s.
#[component]
fn Templates(lang: Lang) -> impl IntoView {
    view! {
        <template id="tpl-suggest-row">
            <a role="option" class="hs-opt hs-opt--hit" tabindex="-1" aria-selected="false" data-attr-href="href">
                <span class="hs-thumb">
                    <img data-attr-src="img" data-attr-srcset="srcset" sizes="44px" alt="" width="44" height="44" loading="lazy" decoding="async"/>
                </span>
                <span class="hs-opt__main">
                    <span class="hs-name" data-name=""></span>
                    <span class="hs-meta" data-slot="category"></span>
                </span>
                <span class="hs-price">
                    <span class="hs-price__now mono" data-slot="price"></span>
                    <s class="hs-price__was mono" data-slot="was"></s>
                </span>
            </a>
        </template>
        <template id="tpl-suggest-cat">
            <a role="option" class="hs-opt hs-opt--cat" tabindex="-1" aria-selected="false" data-attr-href="href">
                <span class="hs-opt__ic" aria-hidden="true">
                    <svg class="icon" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                        stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" focusable="false">
                        <path data-attr-d="icon"></path>
                    </svg>
                </span>
                <span class="hs-opt__main">
                    <span data-slot="quoted"></span>
                    " "{tr(lang, "in", "في")}" "
                    <strong data-slot="name"></strong>
                </span>
            </a>
        </template>
        <template id="tpl-suggest-all">
            <a role="option" class="hs-opt hs-opt--all" tabindex="-1" aria-selected="false" data-attr-href="href">
                <span class="hs-opt__ic" aria-hidden="true"><Icon name="search" size=18/></span>
                <span class="hs-opt__main" data-slot="label"></span>
                <Icon name="arrow-right" size=18 class="flip-rtl hs-opt__go"/>
            </a>
        </template>
    }
}
