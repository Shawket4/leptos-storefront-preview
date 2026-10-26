//! Error views shared by every page.
//!
//! - [`NotFound`] — the router's fallback (status 404): illustration, search, popular categories.
//! - [`SectionError`] — the per-section fallback for an `<ErrorBoundary>` around one rail, widget
//!   or panel; the rest of the page renders normally.
//! - [`ErrorPage`] — the full-page view for a route-level `<ErrorBoundary>` (status 500 when the
//!   failure is known before the first byte).
//!
//! ```ignore
//! <ErrorBoundary fallback=|errors| view! { <SectionError errors/> }> … </ErrorBoundary>
//! <ErrorBoundary fallback=|errors| view! { <SectionError errors what=tr(lang, "Related products", "منتجات مشابهة")/> }> … </ErrorBoundary>
//! <ErrorBoundary fallback=|errors| view! { <ErrorPage errors/> }> {page} </ErrorBoundary>
//! ```
//!
//! Copy is clear Modern Standard Arabic (AGENTS.md: errors in MSA).

use crate::components::button::{Button, ButtonKind, ButtonSize};
use crate::components::icons::Icon;
use crate::media::Img;
use crate::prefs::{Lang, Theme, tr, use_ui};
use leptos::error::Errors;
use leptos::prelude::*;
use leptos_meta::{Meta, Title};

// ─── Brand illustration ────────────────────────────────────────────────────────────────────────

/// A `brand/<name>-light` / `brand/<name>-dark` illustration pair, resolved on the server.
#[derive(Clone, Debug, Default)]
struct Art {
    light: Option<Img>,
    dark: Option<Img>,
}

#[cfg(feature = "ssr")]
fn art(name: &str) -> Art {
    let st = crate::server::state();
    let m = st.media();
    Art {
        light: m.try_img(&format!("brand/{name}-light")),
        dark: m.try_img(&format!("brand/{name}-dark")),
    }
}

#[cfg(not(feature = "ssr"))]
fn art(_: &str) -> Art {
    Art::default()
}

/// The illustration for the visitor's theme: the `<source media>` picks the dark art for Dark (or
/// System on a dark device), so one element tree serves every theme.
#[component]
fn Illustration(
    art: Art,
    theme: Theme,
    lang: Lang,
    #[prop(into)] fallback_glyph: String,
) -> impl IntoView {
    let Some(light) = art.light.clone().or_else(|| art.dark.clone()) else {
        // No art yet (the brand pipeline hasn't produced it): a typographic stand-in, same box.
        return view! {
            <div class="err-art err-art--glyph" aria-hidden="true">
                <span class="mono">{fallback_glyph}</span>
            </div>
        }
        .into_any();
    };
    let dark = art.dark.clone().unwrap_or_else(|| light.clone());
    let media = match theme {
        Theme::Light => "not all",
        Theme::Dark => "all",
        Theme::System => "(prefers-color-scheme: dark)",
    };
    let sizes = "(min-width: 768px) 340px, 68vw";
    let w = 640u32;
    let h = (w as f32 / light.aspect.max(0.1)).round() as u32;
    view! {
        <div class="err-art" style=format!("--img-color:{}", light.color)>
            <picture>
                <source media=media srcset=dark.srcset() sizes=sizes/>
                <img
                    src=light.src_near(640)
                    srcset=light.srcset()
                    sizes=sizes
                    alt=light.alt(lang).to_owned()
                    width=w
                    height=h
                    decoding="async"
                />
            </picture>
        </div>
    }
    .into_any()
}

// ─── 404 ───────────────────────────────────────────────────────────────────────────────────────

/// The router's fallback. Sets status 404 synchronously (before the first chunk).
#[component]
pub fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        if let Some(resp) = use_context::<leptos_axum::ResponseOptions>() {
            resp.set_status(http::StatusCode::NOT_FOUND);
        }
        crate::server::cache::private();
    }
    let rq = crate::request::req();
    let lang = rq.prefs.lang;
    let ar = lang.is_rtl();
    let tenant_name = rq.tenant.name.pick(ar).to_owned();
    let back = same_origin_referer(&rq.origin);
    // The categories are in memory: read synchronously, so the 404 is complete in the first chunk.
    let cats = popular_categories(ar);

    view! {
        <Title text=format!("{} · {}", tr(lang, "Page not found", "الصفحة غير موجودة"), tenant_name)/>
        <Meta name="robots" content="noindex"/>
        <section class="err-page container" aria-labelledby="err-title">
            <Illustration art=art("not-found") theme=rq.prefs.theme lang fallback_glyph="404"/>
            <div class="err-page__copy">
                <p class="err-page__code mono">{tr(lang, "Error 404", "خطأ 404")}</p>
                <h1 id="err-title" class="err-page__title">
                    {tr(lang, "We couldn't find that page", "لم نعثر على هذه الصفحة")}
                </h1>
                <p class="err-page__body">
                    {tr(
                        lang,
                        "The link may be old or mistyped. Try a search, or pick a category below.",
                        "ربما يكون الرابط قديمًا أو به خطأ. جرّب البحث، أو اختر قسمًا من الأقسام التالية.",
                    )}
                </p>
                <form class="err-search" action="/search" method="get" role="search">
                    <label class="sr-only" for="nf-q">{tr(lang, "Search the shop", "ابحث في المتجر")}</label>
                    <span class="err-search__icon" aria-hidden="true"><Icon name="search" size=20/></span>
                    <input
                        id="nf-q"
                        class="err-search__input"
                        type="search"
                        name="q"
                        autocomplete="off"
                        enterkeyhint="search"
                        placeholder=tr(lang, "Search products, brands…", "ابحث عن منتج أو ماركة…")
                    />
                    <Button button_type="submit" size=ButtonSize::Sm>{tr(lang, "Search", "بحث")}</Button>
                </form>
                <div class="err-page__links">
                    {back.map(|href| view! {
                        <a class="err-page__home" href=href>
                            <Icon name="arrow-left" size=18 class="flip-rtl"/>
                            <span>{tr(lang, "Back to the previous page", "العودة إلى الصفحة السابقة")}</span>
                        </a>
                    })}
                    <a class="err-page__home" href="/">
                        <Icon name="house" size=18/>
                        <span>{tr(lang, "Back to the home page", "العودة إلى الصفحة الرئيسية")}</span>
                    </a>
                </div>
            </div>
        </section>
        <section class="err-cats container" aria-labelledby="err-cats-title">
            <h2 id="err-cats-title" class="err-cats__title">{tr(lang, "Popular categories", "أقسام يبحث عنها الناس")}</h2>
            <ul class="err-cats__list">
                {cats
                    .into_iter()
                    .map(|(slug, icon, name)| view! {
                        <li>
                            <a class="err-cat" href=format!("/c/{slug}")>
                                <Icon name=icon size=20/>
                                <span>{name}</span>
                            </a>
                        </li>
                    })
                    .collect_view()}
            </ul>
        </section>
    }
}

/// The first eight categories of this shop: (slug, icon, name in the visitor's language).
#[cfg(feature = "ssr")]
fn popular_categories(ar: bool) -> Vec<(String, String, String)> {
    if use_context::<crate::server::AppState>().is_none() {
        return Vec::new();
    }
    crate::server::current_catalog()
        .nav()
        .iter()
        .take(8)
        .map(|c| (c.slug.clone(), c.icon.clone(), c.name.pick(ar).to_owned()))
        .collect()
}

#[cfg(not(feature = "ssr"))]
fn popular_categories(_: bool) -> Vec<(String, String, String)> {
    Vec::new()
}

/// The page the visitor came from, as a path, when it is on this site (and isn't this 404).
#[cfg(feature = "ssr")]
fn same_origin_referer(origin: &str) -> Option<String> {
    let parts = use_context::<http::request::Parts>()?;
    let referer = parts.headers.get(http::header::REFERER)?.to_str().ok()?;
    let path = referer.strip_prefix(origin)?;
    let here = parts.uri.path_and_query().map(|p| p.as_str()).unwrap_or("");
    (path.starts_with('/') && !path.starts_with("//") && path != here).then(|| path.to_owned())
}

#[cfg(not(feature = "ssr"))]
fn same_origin_referer(_: &str) -> Option<String> {
    None
}

// ─── Per-section fallback ──────────────────────────────────────────────────────────────────────

/// One section failed: say so, offer a retry, keep the rest of the page.
///
/// Same look as the home page's original `WidgetError`. Rendered on the server, in the request's
/// language.
///
/// - `errors` — pass the boundary's signal to show the error text in debug builds (never in
///   release).
/// - `what` — the section's name in the visitor's language, e.g. "Related products".
/// - `retry_href` — defaults to the current URL without `fail=` (the simulated-fault parameter).
/// - `compact` — a single line, for small slots.
#[component]
pub fn SectionError(
    #[prop(optional)] errors: Option<ArcRwSignal<Errors>>,
    #[prop(optional, into)] what: Option<String>,
    #[prop(optional, into)] retry_href: Option<String>,
    #[prop(optional)] compact: bool,
) -> impl IntoView {
    let lang = use_ui().lang.get_untracked();
    let retry = retry_href.unwrap_or_else(current_url_without_fail);
    let sim_faults = sim_faults_active();
    let detail = errors.filter(|_| cfg!(debug_assertions)).map(error_text);
    let headline = match &what {
        Some(w) => match lang {
            Lang::En => format!("{w} didn't load."),
            Lang::Ar => format!("تعذّر تحميل {w}."),
        },
        None => tr(lang, "This section didn't load.", "تعذّر تحميل هذا القسم.").to_owned(),
    };
    view! {
        <div class="section-error" class:section-error--compact=compact role="status">
            <span class="section-error__icon" aria-hidden="true"><Icon name="triangle-alert" size=20/></span>
            <div class="section-error__text">
                <p>
                    <strong>{headline}</strong>
                    " "
                    <span>{tr(lang, "Everything else on the page still works.", "باقي الصفحة يعمل بشكل طبيعي.")}</span>
                </p>
                {detail.map(|d| view! {
                    <details class="section-error__detail">
                        <summary>{tr(lang, "Details (debug build)", "التفاصيل (نسخة التطوير)")}</summary>
                        <code dir="ltr">{d}</code>
                    </details>
                })}
            </div>
            <span class="section-error__actions">
                <a class="section-error__retry" href=retry>
                    <Icon name="rotate-ccw" size=16/>
                    <span>{tr(lang, "Try again", "إعادة المحاولة")}</span>
                </a>
                {sim_faults.map(|href| view! {
                    <a class="section-error__clear" href=href>
                        {tr(lang, "Clear simulated faults", "إلغاء الأعطال التجريبية")}
                    </a>
                })}
            </span>
        </div>
    }
}

/// The boundary's errors as one line (debug builds only show it).
fn error_text(errors: ArcRwSignal<Errors>) -> String {
    errors
        .get_untracked()
        .iter()
        .map(|(_, e)| e.to_string())
        .collect::<Vec<_>>()
        .join(" · ")
}

/// `path?query` of the current page with any `fail=` pair removed.
fn current_url_without_fail() -> String {
    strip_param(&current_path_and_query(), "fail")
}

#[cfg(feature = "ssr")]
fn current_path_and_query() -> String {
    use_context::<crate::request::RequestCtx>()
        .map(|rq| rq.path_and_query)
        .unwrap_or_else(|| "/".into())
}

#[cfg(not(feature = "ssr"))]
fn current_path_and_query() -> String {
    "/".into()
}

/// Remove every `name=…` pair from a `path?query` string.
pub fn strip_param(path_and_query: &str, name: &str) -> String {
    let Some((path, q)) = path_and_query.split_once('?') else {
        return path_and_query.to_owned();
    };
    let prefix = format!("{name}=");
    let kept: Vec<&str> = q
        .split('&')
        .filter(|kv| !kv.is_empty() && !kv.starts_with(&prefix) && *kv != name)
        .collect();
    if kept.is_empty() {
        path.to_owned()
    } else {
        format!("{path}?{}", kept.join("&"))
    }
}

/// When the visitor has session-wide simulated faults on (the `fail` cookie that `/__sim` sets),
/// a retry alone can't help: offer the link that clears them. Server only.
#[cfg(feature = "ssr")]
fn sim_faults_active() -> Option<String> {
    let parts = use_context::<http::request::Parts>()?;
    let header = parts.headers.get(http::header::COOKIE)?.to_str().ok()?;
    let on = crate::request::parse_cookies(header).any(|(k, v)| k == "fail" && !v.is_empty());
    on.then(|| {
        let next = crate::api::types::url_encode(&current_url_without_fail());
        format!("/__sim?fail=&next={next}")
    })
}

#[cfg(not(feature = "ssr"))]
fn sim_faults_active() -> Option<String> {
    None
}

// ─── Full-page error ───────────────────────────────────────────────────────────────────────────

/// The whole page failed (route-level boundary). Sets 500 when called before the first chunk; an
/// error that arrives mid-stream can't change the status any more (headers are gone), but the view
/// is the same.
#[component]
pub fn ErrorPage(
    #[prop(optional)] errors: Option<ArcRwSignal<Errors>>,
    /// Where "Try again" goes (defaults to the current URL without `fail=`/`crash=`).
    #[prop(optional, into)]
    retry_href: Option<String>,
    /// `true`: 503 Service Unavailable (maintenance, a backend that is down) with a Retry-After
    /// hint, instead of 500.
    #[prop(optional)]
    unavailable: bool,
) -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        if let Some(resp) = use_context::<leptos_axum::ResponseOptions>() {
            if unavailable {
                resp.set_status(http::StatusCode::SERVICE_UNAVAILABLE);
                resp.insert_header(
                    http::header::RETRY_AFTER,
                    http::HeaderValue::from_static("30"),
                );
            } else {
                resp.set_status(http::StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
        crate::server::cache::private();
    }
    let ui = use_ui();
    let lang = ui.lang.get_untracked();
    let theme = ui.theme.get_untracked();
    let retry = retry_href.unwrap_or_else(|| strip_param(&current_url_without_fail(), "crash"));
    let detail = errors.filter(|_| cfg!(debug_assertions)).map(error_text);
    let (code, title, body) = if unavailable {
        (
            tr(lang, "Error 503", "خطأ 503"),
            tr(lang, "We'll be right back", "سنعود خلال لحظات"),
            tr(
                lang,
                "The shop is briefly unavailable — usually for a minute or two. Your cart is saved; try again shortly.",
                "المتجر غير متاح لفترة قصيرة — عادةً لدقيقة أو اثنتين. سلّتك محفوظة؛ حاول مرة أخرى بعد قليل.",
            ),
        )
    } else {
        (
            tr(lang, "Error 500", "خطأ 500"),
            tr(
                lang,
                "Something went wrong on our side",
                "حدث خطأ من جانبنا",
            ),
            tr(
                lang,
                "It's not you. Please try again in a moment — your cart is saved.",
                "المشكلة ليست عندك. يُرجى المحاولة مرة أخرى بعد قليل — سلّتك محفوظة.",
            ),
        )
    };
    view! {
        <Title text=title/>
        <Meta name="robots" content="noindex"/>
        <section class="err-page err-page--500 container" aria-labelledby="err500-title" role="alert">
            <Illustration art=art("error") theme lang fallback_glyph={if unavailable { "503" } else { "500" }}/>
            <div class="err-page__copy">
                <p class="err-page__code mono">{code}</p>
                <h1 id="err500-title" class="err-page__title">{title}</h1>
                <p class="err-page__body">{body}</p>
                <div class="err-page__actions">
                    <Button size=ButtonSize::Lg href=retry icon="rotate-ccw">{tr(lang, "Try again", "إعادة المحاولة")}</Button>
                    <Button kind=ButtonKind::Secondary size=ButtonSize::Lg href="/" icon="house">{tr(lang, "Home page", "الصفحة الرئيسية")}</Button>
                </div>
                {detail.map(|d| view! {
                    <details class="err-page__detail">
                        <summary>{tr(lang, "Details (debug build)", "التفاصيل (نسخة التطوير)")}</summary>
                        <code dir="ltr">{d}</code>
                    </details>
                })}
            </div>
        </section>
    }
}
