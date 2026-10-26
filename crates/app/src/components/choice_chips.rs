//! Choice chips: pick one value — a product's size or colour, a city. A labelled radio group of
//! 44px squircle chips ([`ChoiceKind::Chip`]) or colour swatches ([`ChoiceKind::Swatch`]).
//!
//! Two ways to work without JS:
//! - **Links** (no `name`): every pickable choice is a link (`href`) to the page with it chosen —
//!   `role="radio"` with `aria-checked`, in a `role="radiogroup"`. Browser code may take them
//!   over (arrow keys, a roving `tabindex`, choosing in place): it sets `aria-checked` /
//!   `aria-disabled` and the look follows — the styles key off those attributes, not classes.
//! - **Radios** (`name`): native `<input type="radio">`s posting `name=value` in a form.
//!
//! An unavailable choice keeps its place, greyed (chips struck through, swatches crossed), with
//! `title` saying why; it points at the group's `note` (`aria-describedby`), the line under the
//! choices that explains. The head shows the label and, for swatches or with `show_value`, the
//! chosen value — `data-slot="value"`, like the note's `data-slot="note"`, for `patch()`.
//!
//! ```ignore
//! let sizes = values.map(|v| ChoiceChip::new(v.label, v.code).href(variant_href(v)).state(state(v)).attr("data-index", i));
//! <ChoiceChips label=tr(lang, "Size", "المقاس") id="pp-size" items=sizes show_value=true note=why/>
//! <ChoiceChips label=t("City", "المدينة") id="co-city" name="city" items=cities required=true full=true error=city_err/>
//! ```

use super::attrs::{Attrs, spread};
use super::icons::Icon;
use leptos::either::Either;
use leptos::prelude::*;

/// Chips with words, or colour swatches (the word is then the name read out and the tooltip).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ChoiceKind {
    #[default]
    Chip,
    Swatch,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ChoiceState {
    /// The one chosen.
    Selected,
    /// Can be picked.
    #[default]
    Available,
    /// Can't be picked now (sold out in this combination); shown greyed, not a link.
    Unavailable,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ChoiceChip {
    pub label: String,
    /// The radio's value (radios); for links, data for the browser code if it wants it.
    pub value: String,
    /// Links: the page with this choice made.
    pub href: Option<String>,
    pub state: ChoiceState,
    /// A tooltip: why it can't be picked.
    pub title: Option<String>,
    /// Swatches: the colour, `#rgb` or `#rrggbb` (anything else draws the neutral swatch).
    pub swatch: Option<String>,
    /// Extra attributes on the chip (`data-*`).
    pub attrs: Attrs,
}

impl ChoiceChip {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        ChoiceChip {
            label: label.into(),
            value: value.into(),
            href: None,
            state: ChoiceState::Available,
            title: None,
            swatch: None,
            attrs: Vec::new(),
        }
    }
    pub fn href(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        self
    }
    pub fn state(mut self, state: ChoiceState) -> Self {
        self.state = state;
        self
    }
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
    pub fn swatch(mut self, colour: impl Into<String>) -> Self {
        self.swatch = Some(colour.into());
        self
    }
    /// One more attribute on the chip: `.attr("data-index", "2")`.
    pub fn attr(mut self, name: &'static str, value: impl Into<String>) -> Self {
        self.attrs.push((name, value.into()));
        self
    }
}

/// `--swatch:#rrggbb` for a well-formed colour (never anything else into a style attribute).
fn swatch_style(c: Option<&str>) -> Option<String> {
    let c = c?;
    let hex = c.strip_prefix('#')?;
    ((hex.len() == 3 || hex.len() == 6) && hex.chars().all(|x| x.is_ascii_hexdigit()))
        .then(|| format!("--swatch:{c}"))
}

#[component]
pub fn ChoiceChips(
    /// Names the group ("Size", "Colour"): shown above the choices, read by screen readers.
    #[prop(into)]
    label: String,
    items: Vec<ChoiceChip>,
    #[prop(optional)] kind: ChoiceKind,
    /// Native radios posting under this name (in a form). Without it, choices are links.
    #[prop(optional, into)]
    name: Option<String>,
    /// Show the chosen value after the label ("Size  M"). Swatches always do.
    #[prop(optional)]
    show_value: bool,
    /// The group's id; the note and the error hang off it (`{id}-note`, `{id}-error`).
    #[prop(optional, into)]
    id: Option<String>,
    /// A line under the choices: why some can't be picked (needs `id`).
    #[prop(optional, into)]
    note: Option<String>,
    /// An inline error (radios: none chosen yet); marks the group invalid (needs `id`).
    #[prop(optional, into)]
    error: MaybeProp<String>,
    /// Radios: one must be chosen.
    #[prop(optional)]
    required: bool,
    /// Share the row equally (a few short choices: cities).
    #[prop(optional)]
    full: bool,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let swatch = kind == ChoiceKind::Swatch;
    let mut cls = String::from("choice-chips");
    if swatch {
        cls.push_str(" choice-chips--swatch");
    }
    if full {
        cls.push_str(" choice-chips--full");
    }
    if !class.is_empty() {
        cls.push(' ');
        cls.push_str(&class);
    }
    let err = error.get_untracked().filter(|e| !e.is_empty());
    let note = note.filter(|n| !n.is_empty());
    let note_id = id.as_ref().map(|i| format!("{i}-note"));
    let error_id = id.as_ref().map(|i| format!("{i}-error"));
    let describe = {
        let mut v = Vec::new();
        if note.is_some() {
            v.extend(note_id.clone());
        }
        if err.is_some() {
            v.extend(error_id.clone());
        }
        (!v.is_empty()).then(|| v.join(" "))
    };
    let chosen = items
        .iter()
        .find(|c| c.state == ChoiceState::Selected)
        .map(|c| c.label.clone())
        .unwrap_or_default();
    let value = (swatch || show_value)
        .then(|| view! { <span class="choice-chips__value" data-slot="value">{chosen}</span> });
    let chips = items
        .into_iter()
        .map(|c| {
            let on = c.state == ChoiceState::Selected;
            let off = c.state == ChoiceState::Unavailable;
            let face = match swatch_style(c.swatch.as_deref()) {
                _ if !swatch => Either::Left(
                    view! { <span class="choice-chip__label">{c.label.clone()}</span> },
                ),
                style => Either::Right(
                    view! { <span class="choice-chip__dot" style=style aria-hidden="true"></span> },
                ),
            };
            let title = c.title.clone().or_else(|| swatch.then(|| c.label.clone()));
            let named = swatch.then(|| c.label.clone());
            let why = off.then(|| note_id.clone()).flatten();
            let chip_cls = if swatch {
                "choice-chip choice-chip--swatch"
            } else {
                "choice-chip"
            };
            match &name {
                None => Either::Left(view! {
                    <a
                        class=chip_cls
                        role="radio"
                        href=c.href.filter(|_| !off)
                        aria-checked=if on { "true" } else { "false" }
                        aria-disabled=off.then_some("true")
                        aria-describedby=why
                        aria-label=named
                        title=title
                        data-value=c.value
                        data-press=""
                        {..spread(c.attrs)}
                    >
                        {face}
                    </a>
                }),
                Some(n) => Either::Right(view! {
                    <label class=chip_cls title=title data-press="" {..spread(c.attrs)}>
                        <input
                            class="choice-chip__input"
                            type="radio"
                            name=n.clone()
                            value=c.value
                            checked=on
                            disabled=off
                            required=required
                            aria-label=named
                            aria-describedby=why
                        />
                        {face}
                    </label>
                }),
            }
        })
        .collect_view();
    let group_label = label.clone();
    let invalid = err.is_some().then_some("true");
    view! {
        <div
            class=cls
            id=id
            role="radiogroup"
            aria-label=group_label
            aria-describedby=describe
            aria-invalid=invalid
            aria-required=required.then_some("true")
        >
            // The visible head repeats the group's name (and the chosen value, which the checked
            // choice already says), so screen readers skip it.
            <p class="choice-chips__head" aria-hidden="true">
                <span class="choice-chips__label">{label}</span>
                {value}
            </p>
            <div class="choice-chips__items">{chips}</div>
            {note_id.map(|nid| {
                let empty = note.is_none();
                view! { <p class="choice-chips__note" id=nid data-slot="note" hidden=empty>{note.clone()}</p> }
            })}
            {err.map(|e| {
                view! {
                    <p class="field__error" id=error_id>
                        <Icon name="circle-alert" size=16/>
                        <span>{e}</span>
                    </p>
                }
            })}
        </div>
    }
}
