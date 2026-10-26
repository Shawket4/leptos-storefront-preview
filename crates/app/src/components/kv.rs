//! A key–value list: an icon and a label, the value on one or more lines, and an optional action
//! at the row's end ("Edit", "Change"). Checkout's review (address, delivery time, payment) and the
//! order page's details. The action's link names its row for screen readers ("Edit — Deliver to").
//!
//! ```ignore
//! <KeyValueList rows=vec![
//!     KvRow::new("map-pin", tr(lang, "Deliver to", "التوصيل على"), address_lines)
//!         .action(tr(lang, "Edit", "تعديل"), Step::Address.href()),
//!     KvRow::new("clock", tr(lang, "Delivery time", "ميعاد التوصيل"), vec![slot_text]),
//! ]/>
//! ```

use super::icons::Icon;
use leptos::prelude::*;

/// One row: icon, label, value lines (the first reads as the headline), optional action.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct KvRow {
    /// A lucide icon name.
    pub icon: &'static str,
    pub label: String,
    pub lines: Vec<String>,
    /// `(label, href)`: a link at the row's end.
    pub action: Option<(String, String)>,
}

impl KvRow {
    pub fn new(icon: &'static str, label: impl Into<String>, lines: Vec<String>) -> Self {
        KvRow {
            icon,
            label: label.into(),
            lines,
            action: None,
        }
    }
    pub fn action(mut self, label: impl Into<String>, href: impl Into<String>) -> Self {
        self.action = Some((label.into(), href.into()));
        self
    }
}

#[component]
pub fn KeyValueList(rows: Vec<KvRow>, #[prop(optional, into)] class: String) -> impl IntoView {
    let cls = if class.is_empty() {
        "kv".to_owned()
    } else {
        format!("kv {class}")
    };
    view! {
        <dl class=cls>
            {rows
                .into_iter()
                .map(|r| {
                    let key = r.label.clone();
                    view! {
                        <div class="kv__row">
                            <dt class="kv__key">
                                <Icon name=r.icon size=18/>
                                <span>{r.label}</span>
                            </dt>
                            <dd class="kv__value">
                                // <bdi>: each line takes its direction from its own text (a phone
                                // number in an Arabic address stays in order) and still aligns with the list.
                                {r.lines.into_iter().map(|l| view! { <span class="kv__line"><bdi>{l}</bdi></span> }).collect_view()}
                            </dd>
                            {r.action.map(|(label, href)| {
                                view! {
                                    <a class="kv__action" href=href>
                                        {label}
                                        <span class="sr-only">{format!(" — {key}")}</span>
                                    </a>
                                }
                            })}
                        </div>
                    }
                })
                .collect_view()}
        </dl>
    }
}
