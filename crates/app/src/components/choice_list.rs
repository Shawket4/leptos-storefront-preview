//! A list of choices with a trailing figure: a product's add-ons ("Extra cheese  + EGP 15"), any
//! one-of or any-of list where each option has a price or a note. Native radios (`single`) or
//! checkboxes, so a plain form posts `name=value` for each choice without JS.
//!
//! Built on [`RadioGroup`] — the `<fieldset>`, legend, hint and inline error, wired with
//! `aria-describedby` / `aria-invalid` exactly as there — and [`Checkbox`] for the rows (a radio
//! row is the same row with a round dot). The rows sit in one inset list, divided by hairlines.
//!
//! Validating in the browser (min / max picks): put the rule in `hint` ("Choose up to 3") and
//! set `aria-invalid="true"` on the fieldset while it's broken — the list takes the danger edge
//! and the hint turns red. A server-side `error` renders the error line as well.
//!
//! ```ignore
//! let rows = g.options.iter().map(|o| ChoiceRow::new(&o.id, name(o)).meta(surcharge(o.price))).collect();
//! <ChoiceList legend=name(g) name="addon" items=rows id=format!("pp-g-{}", g.id) hint=rule(g)/>
//! ```

use super::attrs::{Attrs, spread};
use super::form::{Checkbox, RadioGroup};
use leptos::either::{Either, EitherOf4};
use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ChoiceRow {
    /// What the row posts.
    pub value: String,
    pub label: String,
    /// A line under the label.
    pub description: Option<String>,
    /// At the row's end: a price, a count (formatted).
    pub meta: Option<String>,
    pub checked: bool,
    pub disabled: bool,
    /// Extra attributes on the row (`<label>`).
    pub attrs: Attrs,
}

impl ChoiceRow {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        ChoiceRow {
            value: value.into(),
            label: label.into(),
            description: None,
            meta: None,
            checked: false,
            disabled: false,
            attrs: Vec::new(),
        }
    }
    pub fn description(mut self, d: impl Into<String>) -> Self {
        self.description = Some(d.into());
        self
    }
    pub fn meta(mut self, m: impl Into<String>) -> Self {
        self.meta = Some(m.into());
        self
    }
    pub fn checked(mut self, on: bool) -> Self {
        self.checked = on;
        self
    }
    pub fn disabled(mut self, off: bool) -> Self {
        self.disabled = off;
        self
    }
    /// One more attribute on the row: `.attr("data-price", "1500")`.
    pub fn attr(mut self, name: &'static str, value: impl Into<String>) -> Self {
        self.attrs.push((name, value.into()));
        self
    }
}

#[component]
pub fn ChoiceList(
    #[prop(into)] legend: TextProp,
    /// The inputs' form name; each chosen row posts its `value`.
    #[prop(into)]
    name: String,
    items: Vec<ChoiceRow>,
    /// One choice (radios) instead of any number (checkboxes).
    #[prop(optional)]
    single: bool,
    /// Needed with a hint or an error (they are `{id}-hint` / `{id}-error`).
    #[prop(optional, into)]
    id: Option<String>,
    #[prop(optional, into)] hint: Option<TextProp>,
    #[prop(optional, into)] error: MaybeProp<String>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let cls = if class.is_empty() {
        "choice-list".to_owned()
    } else {
        format!("choice-list {class}")
    };
    let rows = items
        .into_iter()
        .map(|r| {
            if single {
                Either::Left(view! {
                    <label class="check check--radio" data-press="" {..spread(r.attrs)}>
                        <input
                            class="check__input"
                            type="radio"
                            name=name.clone()
                            value=r.value
                            checked=r.checked
                            disabled=r.disabled
                        />
                        <span class="check__box is-round" aria-hidden="true"></span>
                        <span class="check__text">
                            <span class="check__label">{r.label}</span>
                            {r.description.map(|d| view! { <span class="check__desc">{d}</span> })}
                        </span>
                        {r.meta.map(|m| view! { <span class="check__meta">{m}</span> })}
                    </label>
                })
            } else {
                // Checkbox's optional props take a value, not an Option: an empty one renders
                // nothing visible (`.check__desc:empty`, `.check__meta:empty`).
                Either::Right(view! {
                    <Checkbox
                        name=name.clone()
                        value=r.value
                        label=r.label
                        description=r.description.unwrap_or_default()
                        meta=r.meta.unwrap_or_default()
                        checked=r.checked
                        disabled={r.disabled}
                        {..spread(r.attrs)}
                    />
                })
            }
        })
        .collect_view();
    let rows = view! { <div class="choice-list__rows">{rows}</div> };
    // RadioGroup's optional props take values, not Options: one call per combination.
    match (id, hint) {
        (Some(id), Some(hint)) => EitherOf4::A(view! {
            <RadioGroup legend id hint error class=cls>{rows}</RadioGroup>
        }),
        (Some(id), None) => EitherOf4::B(view! {
            <RadioGroup legend id error class=cls>{rows}</RadioGroup>
        }),
        (None, Some(hint)) => EitherOf4::C(view! {
            <RadioGroup legend hint error class=cls>{rows}</RadioGroup>
        }),
        (None, None) => EitherOf4::D(view! {
            <RadioGroup legend error class=cls>{rows}</RadioGroup>
        }),
    }
}
