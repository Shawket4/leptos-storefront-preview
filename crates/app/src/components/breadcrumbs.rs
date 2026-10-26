//! Breadcrumbs: where am I, and the way back up. `items` are `(label, href)`; the last item is
//! the current page (no link, `aria-current="page"`). Chevrons mirror in Arabic.

use super::icons::Icon;
use crate::prefs::{lang_now, tr};
use leptos::prelude::*;

#[component]
pub fn Breadcrumbs(
    items: Vec<(String, Option<String>)>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let cls = if class.is_empty() {
        "crumbs".to_owned()
    } else {
        format!("crumbs {class}")
    };
    let n = items.len();
    view! {
        <nav class=cls aria-label=tr(lang_now(), "Breadcrumb", "مسار التنقل")>
            <ol class="crumbs__list" role="list">
                {items
                    .into_iter()
                    .enumerate()
                    .map(|(i, (label, href))| {
                        let last = i + 1 == n;
                        let link = match (href, last) {
                            (Some(h), false) => view! { <a class="crumbs__link" href=h>{label}</a> }.into_any(),
                            _ => view! { <span class="crumbs__current" aria-current="page">{label}</span> }.into_any(),
                        };
                        view! {
                            <li class="crumbs__item">
                                {link}
                                {(!last).then(|| view! { <Icon name="chevron-right" size=14 class="crumbs__sep flip-rtl"/> })}
                            </li>
                        }
                    })
                    .collect_view()}
            </ol>
        </nav>
    }
}

/// `BreadcrumbList` JSON-LD for the same trail (absolute URLs from `origin`, e.g.
/// `request::req().origin`). Put it in `<Script type_="application/ld+json">`.
pub fn json_ld(items: &[(String, Option<String>)], origin: &str) -> String {
    let list = items
        .iter()
        .enumerate()
        .map(|(i, (name, href))| {
            let mut item =
                serde_json::json!({ "@type": "ListItem", "position": i + 1, "name": name });
            if let Some(h) = href {
                item["item"] = serde_json::Value::String(format!("{origin}{h}"));
            }
            item
        })
        .collect::<Vec<_>>();
    serde_json::json!({ "@context": "https://schema.org", "@type": "BreadcrumbList", "itemListElement": list }).to_string()
}
