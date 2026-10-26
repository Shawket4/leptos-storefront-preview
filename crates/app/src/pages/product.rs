//! Product detail page, `/p/:slug` (route mode `PartiallyBlocked`, set in `lib.rs`).
//!
//! A request, in order:
//! 1. **Synchronously**, before anything streams: the slug is looked up in the in-memory catalogue.
//!    Unknown → status 404 and a designed not-found view. Known → `Cache-Control`/`Cache-Tag`
//!    (product + category tags) and every `<head>` tag (title, description, canonical, OpenGraph
//!    with `product:*` price tags, Twitter card, LCP preload, JSON-LD). leptos_meta only injects
//!    tags registered before the first chunk; in Leptos 0.8.22 a tag inside a `<Suspense>` misses
//!    `<head>` even over a blocking resource, so the head never depends on the resource.
//! 2. The page data comes from `api::product_detail` through `Resource::new_blocking` inside
//!    `<Suspense>` + `<ErrorBoundary>` (`?fail=product` shows the error view).
//! 3. Related products stream on their own (`<Suspense>` + `<ErrorBoundary>`, `?fail=related`).
//!
//! Streaming facts this relies on (verified with curl on 0.8.22):
//! - leptos_axum doesn't implement PartiallyBlocked's server-side block replacement
//!   (`replace_blocks` is a TODO): a blocking resource only delays the first chunk, and a plain
//!   `<Suspense>` over it still ships as an out-of-order `<template>` + script (no JS → fallback).
//! - A keyed `<For>` item is *resolved before it is rendered*. The product subtree sits in one
//!   (for the islands router, below), so its Suspense renders inline — the page works without JS.
//!   A Suspense nested in it would freeze on its fallback, hence Related lives outside.
//! - The Suspense fallback is therefore not emitted today; it is the same product rendered
//!   statically (the islands' bodies as plain HTML) so that, should this ever stream again, no-JS
//!   visitors still get the page and JS visitors see zero layout shift when it swaps.
//!
//! Islands-router: it diffs pages but never touches islands, so the product subtree is a `<For>`
//! keyed by product id — product → product *replaces* it (new islands, hydrated), while a language
//! toggle on the same product keeps the islands and their state.

mod gallery;
mod head;
mod model;
mod motion;
mod panel;
mod parts;
mod shim;

pub use model::{GalleryData, PanelData};
pub use panel::AddToCart;

use crate::api;
use crate::request::req;
use domain::VariantId;
use head::{HeadData, NotFoundHead, ProductHead};
use leptos::either::Either;
use leptos::prelude::*;
use leptos_router::hooks::{use_params_map, use_query_map};
use model::{Notice, text};
use parts::{NotFound, ProductBody, ProductError, Related};
use serde::{Deserialize, Serialize};
use shim::{Breadcrumbs, Crumb};

#[component]
pub fn ProductPage() -> impl IntoView {
    let slug = use_params_map().with_untracked(|p| p.get("slug").unwrap_or_default());
    let (v_param, notice, one_off) = use_query_map().with_untracked(|q| {
        let v = q.get("v").and_then(|v| v.parse::<u32>().ok());
        let added = q
            .get("added")
            .and_then(|n| n.parse::<u32>().ok())
            .filter(|n| *n > 0);
        let err = q.get("__err").filter(|_| {
            q.get("__path")
                .is_some_and(|p| p.ends_with("product_add_to_cart"))
        });
        let notice = err
            .map(|e| Notice::Error(decode_error(&e)))
            .or(added.map(Notice::Added));
        let one_off = notice.is_some() || q.get("fail").is_some();
        (v, notice, one_off)
    });
    let rq = req();
    let (lang, num) = (rq.prefs.lang, rq.prefs.numerals);

    let Some(pv) = lookup(&slug) else {
        #[cfg(feature = "ssr")]
        {
            if let Some(res) = use_context::<leptos_axum::ResponseOptions>() {
                res.set_status(http::StatusCode::NOT_FOUND);
            }
            crate::server::cache::public(&[]);
        }
        let tenant = text(&rq.tenant.name, lang);
        return Either::Left(view! {
            <NotFoundHead lang=lang tenant=tenant/>
            <NotFound slug=slug lang=lang/>
        });
    };

    let p = &pv.product;
    #[cfg(feature = "ssr")]
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
    #[cfg(not(feature = "ssr"))]
    let _ = one_off;

    let selected = v_param
        .and_then(|v| p.variant(VariantId(v)))
        .unwrap_or_else(|| p.default_variant());
    let start = selected.image.filter(|i| *i < pv.images.len()).unwrap_or(0);
    let selected = selected.id.0;
    let id = p.id.0;
    let head = HeadData::build(&pv, &rq, start);
    let crumbs = vec![
        Crumb {
            label: crate::prefs::tr(lang, "Home", "الرئيسية").to_owned(),
            href: Some("/".into()),
        },
        Crumb {
            label: text(&pv.category.name, lang),
            href: (!pv.category.slug.is_empty()).then(|| format!("/c/{}", pv.category.slug)),
        },
        Crumb {
            label: text(&p.name, lang),
            href: None,
        },
    ];
    let category = pv.category.clone();
    let data = Resource::new_blocking(move || slug.clone(), api::product_detail);

    Either::Right(view! {
        <ProductHead h=head/>
        <For
            each=move || [id]
            key=|id| *id
            children=move |_| {
                let (pv, crumbs, notice) = (pv.clone(), crumbs.clone(), notice.clone());
                let fallback_pv = pv.clone();
                let fallback_notice = notice.clone();
                view! {
                    <div class="pdp" data-product=id>
                        <div class="container">
                            <div class="pdp-crumbs">
                                <Breadcrumbs items=crumbs lang=lang/>
                            </div>
                            <Suspense fallback=move || view! {
                                <ProductBody pv=fallback_pv.clone() selected=selected start=start notice=fallback_notice.clone() live=false/>
                            }>
                                <ErrorBoundary fallback=move |_| view! { <ProductError lang=lang/> }>
                                    {move || {
                                        let notice = notice.clone();
                                        Suspend::new(async move {
                                            data.await.map(|found| match found {
                                                Some(pv) => Either::Left(view! {
                                                    <ProductBody pv=pv selected=selected start=start notice=notice live=true/>
                                                }),
                                                None => Either::Right(view! { <ProductError lang=lang/> }),
                                            })
                                        })
                                    }}
                                </ErrorBoundary>
                            </Suspense>
                        </div>
                    </div>
                }
            }
        />
        // Outside the keyed `<For>`: Leptos resolves a `<For>` item completely before rendering
        // it, which would freeze this Suspense on its skeleton. Here it streams on its own; its
        // cards are keyed so the islands router patches them one by one.
        <div class="container">
            <Related id=id category=category lang=lang numerals=num/>
        </div>
    })
}

/// The product, synchronously (server only; the page component never runs in the browser).
fn lookup(slug: &str) -> Option<api::types::ProductView> {
    #[cfg(feature = "ssr")]
    {
        let st = crate::server::state();
        crate::server::current_catalog().product_view(slug, st.media())
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = slug;
        None
    }
}

/// The error code a no-JS add came back with (server_fn encodes it into `?__err=`).
fn decode_error(raw: &str) -> String {
    let e: ServerFnError =
        leptos::server_fn::error::ServerFnUrlError::<ServerFnError>::decode_err(raw);
    model::error_code(&e)
}

// ── The page's store: what the purchase panel has picked, for the gallery ───────────────────

/// The current pick, tagged with its product so a store left over from an earlier product page
/// (the islands router keeps the root owner alive) is ignored.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Picked {
    pub product: u32,
    pub variant: u32,
    pub image: Option<usize>,
}

#[derive(Clone, Copy, Debug)]
pub struct ProductStore {
    pub picked: RwSignal<Option<Picked>>,
}

/// Provide the product page's store (called on the server per request and once on the client,
/// from `stores::provide_all`). Starts empty on both sides so islands hydrate identically; the
/// panel fills it after hydration.
pub fn provide_store() {
    provide_context(ProductStore {
        picked: RwSignal::new(None),
    });
}

pub fn use_product_store() -> ProductStore {
    use_context::<ProductStore>().unwrap_or_else(|| {
        let s = ProductStore {
            picked: RwSignal::new(None),
        };
        provide_context(s);
        s
    })
}
