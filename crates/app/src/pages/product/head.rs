//! Everything the product page puts in `<head>`: title, description, canonical, OpenGraph (with
//! `product:*` price tags), Twitter card, a preload for the LCP image, and JSON-LD (`Product` with
//! an `Offer` or an `AggregateOffer` of per-variant offers, plus a `BreadcrumbList`).
//!
//! Built synchronously from the in-memory catalogue, before anything streams: leptos_meta writes
//! into `<head>` only the tags registered before the first chunk is sent, and in Leptos 0.8.22 a
//! tag rendered inside a `<Suspense>` — even over a blocking resource — misses it.

use super::model::{STAGE_SIZES, text, variant_href};
use crate::api::types::ProductView;
use crate::format::{self, LRI, PDI};
use crate::prefs::{Lang, Numerals, tr};
use crate::request::RequestCtx;
use leptos::prelude::*;
use leptos_meta::{Link, Meta, Script, Title};
use serde_json::{Value, json};

#[derive(Clone, Debug, PartialEq)]
pub struct HeadData {
    title: String,
    description: String,
    canonical: String,
    site_name: String,
    locale: &'static str,
    alt_locale: &'static str,
    image: Option<OgImage>,
    price: String,
    in_stock: bool,
    sku: String,
    json_ld: String,
    /// `imagesrcset` of the gallery's first visible image (same `sizes` as the gallery).
    preload: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
struct OgImage {
    url: String,
    width: u32,
    height: u32,
    alt: String,
}

/// Money for machine-ish text (meta descriptions): Western digits, no bidi isolates.
fn money_text(p: domain::Piastres, lang: Lang) -> String {
    format::money(p, lang, Numerals::Latn).replace([LRI, PDI], "")
}

/// At most `max` characters, cut at a word boundary, with an ellipsis.
fn clip(s: &str, max: usize) -> String {
    let s = s.split_whitespace().collect::<Vec<_>>().join(" ");
    if s.chars().count() <= max {
        return s;
    }
    let cut: String = s.chars().take(max - 1).collect();
    let cut = cut.rsplit_once(' ').map(|(a, _)| a).unwrap_or(&cut);
    format!(
        "{}…",
        cut.trim_end_matches([',', '،', '.', ';', ':', '·', '-', '—'])
    )
}

/// JSON safe inside `<script>`: `<`, `>`, `&` (and the JS line separators) as `\uXXXX` escapes.
/// They can only occur inside JSON strings, where the escapes mean the same characters.
pub fn script_json(v: &Value) -> String {
    let s = v.to_string();
    let mut out = String::with_capacity(s.len() + 16);
    for c in s.chars() {
        match c {
            '<' => out.push_str("\\u003c"),
            '>' => out.push_str("\\u003e"),
            '&' => out.push_str("\\u0026"),
            '\u{2028}' => out.push_str("\\u2028"),
            '\u{2029}' => out.push_str("\\u2029"),
            c => out.push(c),
        }
    }
    out
}

impl HeadData {
    pub fn build(pv: &ProductView, rq: &RequestCtx, start_image: usize) -> HeadData {
        let lang = rq.prefs.lang;
        let p = &pv.product;
        let name = text(&p.name, lang);
        let tenant = text(&rq.tenant.name, lang);
        let canonical = rq.absolute(&format!("/p/{}", p.slug));
        let dv = p.default_variant();
        let price_text = money_text(dv.price, lang);
        let brand = pv.brand.as_ref().map(|b| text(&b.name, lang));
        let category = text(&pv.category.name, lang);
        let desc = text(&p.description, lang);

        let lead = match lang {
            Lang::En => format!("{name} for {price_text} at {tenant}. "),
            Lang::Ar => format!("{name} بسعر {price_text} من {tenant}. "),
        };
        let description = clip(&format!("{lead}{desc}"), 160);

        let real: Vec<&crate::media::Img> = pv.images.iter().filter(|i| !i.is_missing()).collect();
        let image = real.first().map(|img| {
            let width = img.widths.iter().copied().max().unwrap_or(960) as u32;
            OgImage {
                url: rq.absolute(&img.largest()),
                width,
                height: (width as f32 / img.aspect.max(0.1)).round() as u32,
                alt: img.alt(lang).to_owned(),
            }
        });
        let preload = pv
            .images
            .get(start_image)
            .or(pv.images.first())
            .filter(|i| !i.is_missing())
            .map(|i| i.srcset());

        // ── JSON-LD ──
        let availability = |stock: u32| {
            if stock > 0 {
                "https://schema.org/InStock"
            } else {
                "https://schema.org/OutOfStock"
            }
        };
        let seller = json!({ "@type": "Organization", "name": tenant, "url": rq.absolute("/") });
        let offer = |v: &domain::Variant| {
            json!({
                "@type": "Offer",
                "url": rq.absolute(&variant_href(&p.slug, v.id.0)),
                "sku": v.sku,
                "price": format::money_plain(v.price),
                "priceCurrency": "EGP",
                "availability": availability(v.stock),
                "seller": seller,
            })
        };
        let offers = if p.variants.len() == 1 {
            offer(&p.variants[0])
        } else {
            let lo = p.variants.iter().map(|v| v.price).min().unwrap_or_default();
            let hi = p.variants.iter().map(|v| v.price).max().unwrap_or_default();
            json!({
                "@type": "AggregateOffer",
                "priceCurrency": "EGP",
                "lowPrice": format::money_plain(lo),
                "highPrice": format::money_plain(hi),
                "offerCount": p.variants.len(),
                "availability": availability(p.total_stock()),
                "offers": p.variants.iter().map(offer).collect::<Vec<_>>(),
            })
        };
        let mut product = json!({
            "@type": "Product",
            "@id": format!("{canonical}#product"),
            "name": name,
            "description": desc,
            "image": real.iter().map(|i| rq.absolute(&i.largest())).collect::<Vec<_>>(),
            "sku": dv.sku,
            "url": canonical,
            "category": category,
            "offers": offers,
        });
        if let Some(b) = &brand {
            product["brand"] = json!({ "@type": "Brand", "name": b });
        }
        if p.reviews > 0 {
            product["aggregateRating"] = json!({
                "@type": "AggregateRating",
                "ratingValue": format!("{:.1}", p.rating),
                "reviewCount": p.reviews,
                "bestRating": "5",
                "worstRating": "1",
            });
        }
        let crumbs = json!({
            "@type": "BreadcrumbList",
            "itemListElement": [
                { "@type": "ListItem", "position": 1, "name": tr(lang, "Home", "الرئيسية"), "item": rq.absolute("/") },
                { "@type": "ListItem", "position": 2, "name": category, "item": rq.absolute(&format!("/c/{}", pv.category.slug)) },
                { "@type": "ListItem", "position": 3, "name": name, "item": canonical },
            ],
        });
        let json_ld =
            script_json(&json!({ "@context": "https://schema.org", "@graph": [product, crumbs] }));

        HeadData {
            title: format!("{name} · {tenant}"),
            description,
            canonical,
            site_name: tenant,
            locale: if lang.is_rtl() { "ar_EG" } else { "en_EG" },
            alt_locale: if lang.is_rtl() { "en_EG" } else { "ar_EG" },
            image,
            price: format::money_plain(dv.price),
            in_stock: dv.stock > 0,
            sku: dv.sku.clone(),
            json_ld,
            preload,
        }
    }
}

/// The tags. The set is the same for every product (no optional tags but the image group), so the
/// islands router's lockstep diff of `<head>` lines up when going from one product to another.
#[component]
pub fn ProductHead(h: HeadData) -> impl IntoView {
    let og_image = h.image.clone().map(|i| {
        view! {
            <Meta property="og:image" content=i.url.clone()/>
            <Meta property="og:image:type" content="image/webp"/>
            <Meta property="og:image:width" content=i.width.to_string()/>
            <Meta property="og:image:height" content=i.height.to_string()/>
            <Meta property="og:image:alt" content=i.alt.clone()/>
            <Meta name="twitter:image" content=i.url/>
            <Meta name="twitter:image:alt" content=i.alt/>
        }
    });
    let preload = h.preload.clone().map(|srcset| {
        view! { <Link rel="preload" as_="image" imagesrcset=srcset imagesizes=STAGE_SIZES fetchpriority="high"/> }
    });
    view! {
        <Title text=h.title.clone()/>
        <Meta name="description" content=h.description.clone()/>
        <Link rel="canonical" href=h.canonical.clone()/>
        {preload}
        <Meta property="og:type" content="product"/>
        <Meta property="og:site_name" content=h.site_name.clone()/>
        <Meta property="og:title" content=h.title.clone()/>
        <Meta property="og:description" content=h.description.clone()/>
        <Meta property="og:url" content=h.canonical.clone()/>
        <Meta property="og:locale" content=h.locale/>
        <Meta property="og:locale:alternate" content=h.alt_locale/>
        {og_image}
        <Meta property="product:price:amount" content=h.price.clone()/>
        <Meta property="product:price:currency" content="EGP"/>
        <Meta property="product:availability" content=if h.in_stock { "in stock" } else { "out of stock" }/>
        <Meta property="product:retailer_item_id" content=h.sku.clone()/>
        <Meta name="twitter:card" content="summary_large_image"/>
        <Meta name="twitter:title" content=h.title/>
        <Meta name="twitter:description" content=h.description/>
        <Script type_="application/ld+json">{h.json_ld}</Script>
    }
}

/// `<head>` of the not-found view.
#[component]
pub fn NotFoundHead(lang: Lang, tenant: String) -> impl IntoView {
    view! {
        <Title text=format!("{} · {tenant}", tr(lang, "Product not found", "المنتج غير موجود"))/>
        <Meta name="robots" content="noindex"/>
    }
}
