//! Product detail page, `/p/:slug?v=<variant>`, rendered in order.
//!
//! 1. **Synchronously**, before anything is sent: the slug is looked up in the in-memory catalogue.
//!    Unknown → status 404 and a designed not-found view. Known → `Cache-Control`/`Cache-Tag`
//!    (product + category tags) and every `<head>` tag (title, description, canonical, OpenGraph
//!    with `product:*` price tags, Twitter card, LCP preload, JSON-LD), from in-memory data.
//! 2. The body comes from `api::product_detail` (`?fail=product` shows the error view), for the
//!    variant in `?v=`; related products from `api::related_products` (`?fail=related`).
//! 3. In the browser (`features/product.ts`): the gallery's gestures and viewer, switching variants
//!    from the page's own JSON (no request), add-ons, quantity and add to cart (the cookie cart).
//!
//! `?added=` / `?cart_err=` are the no-JS add's outcome (the cart's 303 back): rendered as a notice,
//! never cached, and dropped from the address by the browser code.

mod gallery;
mod head;
mod model;
mod panel;
mod parts;

use crate::api;
use crate::components::breadcrumbs::Breadcrumbs;
use crate::request::req;
use domain::VariantId;
use head::{HeadData, NotFoundHead, ProductHead};
use leptos::either::Either;
use leptos::prelude::*;
use leptos_router::hooks::{use_params_map, use_query_map};
use model::{Notice, text};
use parts::{NotFound, ProductBody, ProductError, Related};

#[component]
pub fn ProductPage() -> impl IntoView {
    let slug = use_params_map().with_untracked(|p| p.get("slug").unwrap_or_default());
    let (v_param, notice, one_off) = use_query_map().with_untracked(|q| {
        let v = q.get("v").and_then(|v| v.parse::<u32>().ok());
        let added = q
            .get("added")
            .and_then(|n| n.parse::<u32>().ok())
            .filter(|n| *n > 0);
        let err = q.get("cart_err").filter(|c| {
            !c.is_empty() && c.len() <= 32 && c.bytes().all(|b| b.is_ascii_lowercase() || b == b'_')
        });
        let notice = err.map(Notice::Error).or(added.map(Notice::Added));
        let one_off = notice.is_some() || q.get("fail").is_some();
        (v, notice, one_off)
    });
    let rq = req();
    let (lang, num) = (rq.prefs.lang, rq.prefs.numerals);

    let Some(pv) = lookup(&slug) else {
        if let Some(res) = use_context::<leptos_axum::ResponseOptions>() {
            res.set_status(http::StatusCode::NOT_FOUND);
        }
        crate::server::cache::public(&[]);
        let tenant = text(&rq.tenant.name, lang);
        return Either::Left(view! {
            <NotFoundHead lang=lang tenant=tenant/>
            <NotFound slug=slug lang=lang numerals=num/>
        });
    };

    let p = &pv.product;
    {
        use crate::server::cache;
        if one_off {
            // A confirmation, an error or a fault demo is one visitor's moment, not the page.
            cache::private();
        } else {
            cache::public(&[
                cache::tag_product(&rq.tenant.id, p.id.0),
                cache::tag_category(&rq.tenant.id, p.category.0),
            ]);
        }
    }

    let selected = v_param
        .and_then(|v| p.variant(VariantId(v)))
        .unwrap_or_else(|| p.default_variant());
    let start = selected.image.filter(|i| *i < pv.images.len()).unwrap_or(0);
    let selected = selected.id.0;
    let id = p.id.0;
    let head = HeadData::build(&pv, &rq, start);
    let crumbs = vec![
        (
            crate::prefs::tr(lang, "Home", "الرئيسية").to_owned(),
            Some("/".to_owned()),
        ),
        (
            text(&pv.category.name, lang),
            (!pv.category.slug.is_empty()).then(|| format!("/c/{}", pv.category.slug)),
        ),
        (text(&p.name, lang), None),
    ];
    let category = pv.category.clone();

    Either::Right(view! {
        <ProductHead h=head/>
        <div class="pdp" data-product=id>
            <div class="container">
                <div class="pdp-crumbs">
                    <Breadcrumbs items=crumbs/>
                </div>
                <ErrorBoundary fallback=move |_| view! { <ProductError lang=lang/> }>
                    {Suspend::new(async move {
                        api::product_detail(slug).await.map(|found| match found {
                            Some(pv) => Either::Left(view! { <ProductBody pv selected start notice/> }),
                            None => Either::Right(view! { <ProductError lang=lang/> }),
                        })
                    })}
                </ErrorBoundary>
            </div>
        </div>
        <div class="container pdp-after">
            <Related id=id category=category lang=lang numerals=num/>
        </div>
    })
}

/// The product, synchronously, from the in-memory catalogue (status, headers and `<head>`).
fn lookup(slug: &str) -> Option<api::types::ProductView> {
    let st = crate::server::state();
    crate::server::current_catalog().product_view(slug, st.media())
}
