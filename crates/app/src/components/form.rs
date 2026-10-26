//! Form controls: 52px fields with 16px text (no iOS zoom), a label above, a hint and an inline
//! error below (wired with `aria-describedby`, `aria-invalid`), radio cards for delivery slots
//! and payment methods, and checkboxes. Native inputs throughout, so forms work without JS; the
//! browser code listens for `input`/`change` on them (features/*.ts). Server markup only: values
//! are read once.
//!
//! Ids: a control with `id = "phone"` describes itself with `phone-hint` and `phone-error`;
//! use the same convention when wrapping a custom control in [`Field`].
//!
//! The error line is always in the page — empty, its text slot hidden, while there's no error — so
//! browser checks show their message inline: `patch(document.getElementById("phone-error"),
//! { error: "…" })` (core/dom.ts) and set `aria-invalid`. Clear it with `{ error: "" }`, not
//! `null`: the control's `aria-describedby` always names the line, and screen readers read a
//! described-by element even when it is hidden, so it must be empty, not just hidden.

use super::icons::Icon;
use crate::prefs::{lang_now, tr};
use leptos::prelude::*;

/// `{id}-hint` (when there is one) and `{id}-error`, always: the error line is always rendered and
/// empty while there's no error, so it adds nothing to the description until there is one.
fn described_by(id: &str, hint: bool) -> String {
    if hint {
        format!("{id}-hint {id}-error")
    } else {
        format!("{id}-error")
    }
}

/// The inline error line, `{id}-error`: always rendered, its text in a `data-slot="error"` that is
/// hidden (and empty) while there's no error; CSS hides the line with it.
pub(super) fn error_line(id: Option<String>, error: Option<String>) -> impl IntoView {
    let none = error.is_none();
    view! {
        <p class="field__error" id=id>
            <Icon name="circle-alert" size=16/>
            <span data-slot="error" hidden=none>{error}</span>
        </p>
    }
}

/// The frame every control shares: label (with "(optional)"), the control, hint, error.
fn frame(
    id: String,
    label: TextProp,
    hint: Option<TextProp>,
    error: MaybeProp<String>,
    optional: bool,
    class: String,
    control: AnyView,
) -> impl IntoView {
    let cls = if class.is_empty() {
        "field".to_owned()
    } else {
        format!("field {class}")
    };
    let hint_id = format!("{id}-hint");
    let error_id = format!("{id}-error");
    let err = error.get_untracked().filter(|e| !e.is_empty());
    let opt = optional.then(|| tr(lang_now(), "(optional)", "(اختياري)"));
    view! {
        <div class=cls>
            <label class="field__label" for=id>
                {move || label.get()}
                {opt.map(|o| view! { <span class="field__optional">{o}</span> })}
            </label>
            {control}
            {hint.map(|h| view! { <p class="field__hint" id=hint_id>{move || h.get()}</p> })}
            {error_line(Some(error_id), err)}
        </div>
    }
}

/// Label + control + hint + error, for a custom control. The control (children) must carry
/// `id`, and `aria-describedby="{id}-hint {id}-error"` (the error line is always there) and
/// `aria-invalid` itself.
#[component]
pub fn Field(
    #[prop(into)] id: String,
    #[prop(into)] label: TextProp,
    #[prop(optional, into)] hint: Option<TextProp>,
    #[prop(optional, into)] error: MaybeProp<String>,
    /// Mark the field "(optional)" instead of starring the required ones.
    #[prop(optional)]
    optional: bool,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    frame(id, label, hint, error, optional, class, children())
}

/// A single-line text field.
#[component]
pub fn TextInput(
    #[prop(into)] id: String,
    #[prop(into)] name: String,
    #[prop(into)] label: TextProp,
    #[prop(optional, into)] hint: Option<TextProp>,
    #[prop(optional, into)] error: MaybeProp<String>,
    /// `text` (default), `email`, `tel`, `search`, `number`, `password`.
    #[prop(optional)]
    input_type: &'static str,
    #[prop(optional, into)] value: MaybeProp<String>,
    #[prop(optional, into)] placeholder: Option<TextProp>,
    #[prop(optional)] autocomplete: &'static str,
    #[prop(optional)] inputmode: &'static str,
    #[prop(optional)] required: bool,
    #[prop(optional)] optional: bool,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    /// `ltr` for phone numbers, emails and codes inside Arabic forms.
    #[prop(optional)]
    dir: Option<&'static str>,
    /// A lucide icon inside the field, at the start.
    #[prop(optional)]
    icon: Option<&'static str>,
    /// The browser's length checks, in characters.
    #[prop(optional)]
    minlength: Option<u32>,
    #[prop(optional)] maxlength: Option<u32>,
) -> impl IntoView {
    let has_hint = hint.is_some();
    let describe = described_by(&id, has_hint);
    let invalid = move || error.get().is_some_and(|e| !e.is_empty()).then_some("true");
    let t = if input_type.is_empty() {
        "text"
    } else {
        input_type
    };
    let input_id = id.clone();
    let cls = if icon.is_some() {
        "input input--icon"
    } else {
        "input"
    };
    let initial = value.get_untracked();
    let control = view! {
            // The wrapper takes the field's direction too, so the icon sits at the field's own start.
            <div class="input-wrap" dir=dir>
                {icon.map(|i| view! { <Icon name=i size=20 class="input-wrap__icon"/> })}
                <input
                    class=cls
                    id=input_id
                    name=name
                    type=t
                    value=initial
                    placeholder=move || placeholder.as_ref().map(|p| p.get())
                    autocomplete=(!autocomplete.is_empty()).then_some(autocomplete)
                    inputmode=(!inputmode.is_empty()).then_some(inputmode)
                    required=required
                    minlength=minlength
                    maxlength=maxlength
                    disabled=move || disabled.get().unwrap_or(false)
                    dir=dir
                    aria-invalid=invalid
                    aria-describedby=describe
                />
            </div>
    }
    .into_any();
    frame(id, label, hint, error, optional, String::new(), control)
}

/// A multi-line text field (delivery notes).
#[component]
pub fn TextArea(
    #[prop(into)] id: String,
    #[prop(into)] name: String,
    #[prop(into)] label: TextProp,
    #[prop(optional, into)] hint: Option<TextProp>,
    #[prop(optional, into)] error: MaybeProp<String>,
    #[prop(optional, into)] value: MaybeProp<String>,
    #[prop(optional, into)] placeholder: Option<TextProp>,
    #[prop(optional)] rows: Option<u32>,
    #[prop(optional)] optional: bool,
    /// The browser's length checks, in characters.
    #[prop(optional)]
    minlength: Option<u32>,
    #[prop(optional)] maxlength: Option<u32>,
) -> impl IntoView {
    let has_hint = hint.is_some();
    let describe = described_by(&id, has_hint);
    let invalid = move || error.get().is_some_and(|e| !e.is_empty()).then_some("true");
    let input_id = id.clone();
    let initial = value.get_untracked().unwrap_or_default();
    let control = view! {
            <textarea
                class="input input--area"
                id=input_id
                name=name
                rows=rows.unwrap_or(3)
                minlength=minlength
                maxlength=maxlength
                placeholder=move || placeholder.as_ref().map(|p| p.get())
                aria-invalid=invalid
                aria-describedby=describe
            >
                {initial}
            </textarea>
    }
    .into_any();
    frame(id, label, hint, error, optional, String::new(), control)
}

/// A native select, styled (its menu stays the platform's, which is what people expect on
/// phones). Options first (a "Choose…" with an empty value), then any labelled groups.
#[component]
pub fn Select(
    #[prop(into)] id: String,
    #[prop(into)] name: String,
    #[prop(into)] label: TextProp,
    /// `(value, label)` pairs.
    #[prop(optional)]
    options: Vec<(String, String)>,
    /// `(label, options)`: options under a heading (`<optgroup>`), after `options`.
    #[prop(optional)]
    groups: Vec<(String, Vec<(String, String)>)>,
    #[prop(optional, into)] value: MaybeProp<String>,
    #[prop(optional, into)] hint: Option<TextProp>,
    #[prop(optional, into)] error: MaybeProp<String>,
    #[prop(optional)] required: bool,
    #[prop(optional)] optional: bool,
) -> impl IntoView {
    let has_hint = hint.is_some();
    let describe = described_by(&id, has_hint);
    let invalid = move || error.get().is_some_and(|e| !e.is_empty()).then_some("true");
    let input_id = id.clone();
    let current = value.get_untracked().unwrap_or_default();
    let control = view! {
            <div class="input-wrap input-wrap--select">
                <select
                    class="input input--select"
                    id=input_id
                    name=name
                    required=required
                    aria-invalid=invalid
                    aria-describedby=describe
                >
                    {option_list(options, &current)}
                    {groups
                        .into_iter()
                        .map(|(label, options)| {
                            view! { <optgroup label=label>{option_list(options, &current)}</optgroup> }
                        })
                        .collect_view()}
                </select>
                <Icon name="chevron-down" size=20 class="input-wrap__chevron"/>
            </div>
    }
    .into_any();
    frame(id, label, hint, error, optional, String::new(), control)
}

fn option_list(options: Vec<(String, String)>, current: &str) -> impl IntoView + use<> {
    options
        .into_iter()
        .map(|(v, l)| {
            let selected = v == current;
            view! { <option value=v selected=selected>{l}</option> }
        })
        .collect_view()
}

/// A group of [`RadioCard`]s (delivery slots, payment methods): a `<fieldset>` with its legend, an
/// optional hint under it and an inline error after the cards, wired with `aria-describedby`.
/// Give it an `id` when it has a hint or an error (they're `{id}-hint` / `{id}-error`); with an
/// `id` the error line is always rendered, for browser checks. `required` goes on the cards.
#[component]
pub fn RadioGroup(
    #[prop(into)] legend: TextProp,
    #[prop(optional, into)] id: Option<String>,
    #[prop(optional, into)] hint: Option<TextProp>,
    #[prop(optional, into)] error: MaybeProp<String>,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let err = error.get_untracked().filter(|e| !e.is_empty());
    let base = id.clone().unwrap_or_default();
    let describe = (!base.is_empty()).then(|| described_by(&base, hint.is_some()));
    let cls = if class.is_empty() {
        "radio-cards".to_owned()
    } else {
        format!("radio-cards {class}")
    };
    let invalid = err.is_some().then_some("true");
    let hint_id = (!base.is_empty()).then(|| format!("{base}-hint"));
    let error_id = (!base.is_empty()).then(|| format!("{base}-error"));
    view! {
        <fieldset class=cls id=id aria-describedby=describe aria-invalid=invalid>
            <legend>{move || legend.get()}</legend>
            {hint.map(|h| view! { <p class="field__hint" id=hint_id>{move || h.get()}</p> })}
            {children()}
            {(error_id.is_some() || err.is_some()).then(|| error_line(error_id, err))}
        </fieldset>
    }
}

/// A radio as a card: delivery slots, payment methods. Put them in a [`RadioGroup`].
#[component]
pub fn RadioCard(
    #[prop(into)] name: String,
    #[prop(into)] value: String,
    #[prop(into)] label: TextProp,
    #[prop(optional, into)] description: Option<TextProp>,
    #[prop(optional)] icon: Option<&'static str>,
    /// Extra at the end: a price, "Free", a time.
    #[prop(optional, into)]
    trailing: Option<TextProp>,
    #[prop(optional, into)] checked: MaybeProp<bool>,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    /// One of the group must be chosen (put it on every card of the group).
    #[prop(optional)]
    required: bool,
) -> impl IntoView {
    view! {
        <label class="radio-card" data-press="">
            <input
                class="radio-card__input"
                type="radio"
                name=name
                value=value
                required=required
                checked=checked.get_untracked().unwrap_or(false)
                disabled=move || disabled.get().unwrap_or(false)
            />
            <span class="radio-card__body">
                {icon.map(|i| view! { <span class="radio-card__icon"><Icon name=i size=22/></span> })}
                <span class="radio-card__text">
                    <span class="radio-card__label">{move || label.get()}</span>
                    {description.map(|d| view! { <span class="radio-card__desc">{move || d.get()}</span> })}
                </span>
                {trailing.map(|t| view! { <span class="radio-card__trailing">{move || t.get()}</span> })}
                <span class="radio-card__dot is-round" aria-hidden="true"></span>
            </span>
        </label>
    }
}

/// A checkbox with its label (and an optional line under it, and an optional figure at the end).
#[component]
pub fn Checkbox(
    #[prop(into)] name: String,
    #[prop(into)] label: TextProp,
    #[prop(optional, into)] description: Option<TextProp>,
    /// Trailing, quiet: a count ("12") or a price ("+ EGP 10"), formatted.
    #[prop(optional, into)]
    meta: Option<TextProp>,
    #[prop(optional, into)] value: Option<String>,
    #[prop(optional, into)] checked: MaybeProp<bool>,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
) -> impl IntoView {
    view! {
        <label class="check" data-press="">
            <input
                class="check__input"
                type="checkbox"
                name=name
                value=value.unwrap_or_else(|| "on".to_owned())
                checked=checked.get_untracked().unwrap_or(false)
                disabled=move || disabled.get().unwrap_or(false)
            />
            <span class="check__box" aria-hidden="true">
                <Icon name="check" size=16/>
            </span>
            <span class="check__text">
                <span class="check__label">{move || label.get()}</span>
                {description.map(|d| view! { <span class="check__desc">{move || d.get()}</span> })}
            </span>
            {meta.map(|m| view! { <span class="check__meta">{move || m.get()}</span> })}
        </label>
    }
}
