//! Totals: label–amount rows (subtotal, savings, delivery, total) and an optional note under them
//! ("VAT included"). The cart's summary, checkout's summary and review, the order page.
//!
//! Amounts arrive formatted (`format::money`). A row with a `slot` is written by browser code
//! (`patch()`, core/dom.ts): `data-slot=<slot>` on its value. A row whose value is hidden is hidden
//! whole, so the client shows and hides rows by patching `null` (savings when there are none) —
//! e.g. delivery as two rows, an amount (`slot("delivery")`) and `Free` (`slot("free")`), one of
//! them hidden. Works inside a `<template>`.
//!
//! ```ignore
//! <Totals rows=vec![
//!     TotalRow::new(g("cart.subtotal"), subtotal).slot("subtotal"),
//!     TotalRow::new(g("cart.savings"), savings).tone(TotalTone::Saving).slot("savings").hidden(none),
//!     TotalRow::new(g("cart.delivery"), g("cart.free")).tone(TotalTone::Free).slot("free"),
//!     TotalRow::new(g("cart.total"), total).tone(TotalTone::Total).slot("total"),
//! ] note=vat/>
//! ```

use leptos::prelude::*;

/// How a row reads.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TotalTone {
    /// A plain amount.
    #[default]
    Normal,
    /// Money saved (success colour, the minus is in the formatted amount).
    Saving,
    /// A word instead of an amount ("Free"), success colour.
    Free,
    /// The bottom line: a rule above, bold, larger.
    Total,
}

impl TotalTone {
    fn class(self) -> &'static str {
        match self {
            TotalTone::Normal => "sums__row",
            TotalTone::Saving => "sums__row sums__row--saving",
            TotalTone::Free => "sums__row sums__row--free",
            TotalTone::Total => "sums__row sums__row--total",
        }
    }
}

/// One row: a label and its (formatted) value.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TotalRow {
    pub label: String,
    pub value: String,
    pub tone: TotalTone,
    /// `data-slot` on the value, for browser code that writes it.
    pub slot: Option<&'static str>,
    /// Render it hidden (the client shows it by patching a value).
    pub hidden: bool,
}

impl TotalRow {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        TotalRow {
            label: label.into(),
            value: value.into(),
            ..Default::default()
        }
    }
    pub fn tone(mut self, tone: TotalTone) -> Self {
        self.tone = tone;
        self
    }
    pub fn slot(mut self, slot: &'static str) -> Self {
        self.slot = Some(slot);
        self
    }
    pub fn hidden(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }
}

#[component]
pub fn Totals(
    rows: Vec<TotalRow>,
    /// A quiet line under the rows ("VAT included").
    #[prop(optional, into)]
    note: Option<TextProp>,
    /// `data-slot` on the note.
    #[prop(optional)]
    note_slot: Option<&'static str>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let cls = if class.is_empty() {
        "sums".to_owned()
    } else {
        format!("sums {class}")
    };
    view! {
        <div class=cls>
            <dl class="sums__rows">
                {rows
                    .into_iter()
                    .map(|r| {
                        // Amounts are figures (Plex Mono); a word ("Free") stays in the text face.
                        let value_cls = if r.tone == TotalTone::Free { "sums__value" } else { "sums__value mono" };
                        view! {
                            <div class=r.tone.class()>
                                <dt class="sums__label">{r.label}</dt>
                                <dd class=value_cls data-slot=r.slot hidden=r.hidden>{r.value}</dd>
                            </div>
                        }
                    })
                    .collect_view()}
            </dl>
            {note.map(|n| view! { <p class="sums__note" data-slot=note_slot>{move || n.get()}</p> })}
        </div>
    }
}
