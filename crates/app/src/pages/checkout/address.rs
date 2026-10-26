//! The address step: one plain form (a POST to `checkout_address`), rendered with what the shopper
//! typed and the server's messages next to the fields. Its HTML5 constraints (`required`,
//! `minlength`, `maxlength`) let `features/checkout.ts` say what's missing before the round-trip,
//! inline and in the page's language (without JS the browser's own checks apply); the server
//! checks everything again with `rules::validate`. The area list carries every city's areas in
//! `<optgroup data-city>`s; the browser code narrows it to the chosen city.

use super::CheckoutAddress;
use super::rules::{AddressInput, CITIES, Field, FieldError, NOTES_MAX};
use crate::components::button::{Button, ButtonKind, ButtonSize};
use crate::components::icons::Icon;
use crate::format;
use crate::prefs::{Lang, Numerals, tr, use_ui};
use leptos::prelude::*;
use leptos::server_fn::ServerFn;

#[component]
pub fn AddressForm(
    initial: AddressInput,
    errors: Vec<FieldError>,
    form_error: Option<String>,
) -> impl IntoView {
    let ui = use_ui();
    let (lang, num) = (ui.lang.get_untracked(), ui.numerals.get_untracked());
    let err = |f: Field| {
        errors
            .iter()
            .find(|e| e.field == f)
            .map(|e| e.message(lang, num))
    };
    let t = move |en: &'static str, ar: &'static str| tr(lang, en, ar);
    let text = |field: Field, label: (&'static str, &'static str), opts: TextOpts| {
        view! { <TextField field value=field.value(&initial).to_owned() error=err(field) label opts lang/> }
    };
    let city_err = err(Field::City);
    let area_err = err(Field::Area);
    let notes_err = err(Field::Notes);
    let used = initial.notes.chars().count() as u32;
    view! {
        <div id="co-address" class="co-address">
            <div class="co-step-head">
                <h2 id="co-step-title" class="co-step-title" tabindex="-1">{t("Where should we deliver?", "نوصّل فين؟")}</h2>
                <p class="co-step-sub">{t("We deliver across Cairo, Giza and Alexandria.", "بنوصّل في القاهرة والجيزة والإسكندرية.")}</p>
            </div>
            {form_error.map(|m| view! {
                <div class="notice notice--error" role="alert" tabindex="-1" data-autofocus="">
                    <Icon name="circle-alert" size=20/>
                    <span>{m}</span>
                </div>
            })}
            <form id="co-address-form" class="co-form co-fields" method="post" action=CheckoutAddress::url()
                data-ts="checkout-form">
                {text(Field::Name, ("Full name", "الاسم بالكامل"), TextOpts {
                    hint: Some(("The person receiving the order", "اسم اللي هيستلم الطلب")),
                    autocomplete: Some("name"), minlength: Some(2), ..Default::default()
                })}
                {text(Field::Phone, ("Mobile number", "رقم الموبايل"), TextOpts {
                    hint: Some(("The courier will call this number", "المندوب هيكلمك على الرقم ده")),
                    placeholder: Some("01X XXXX XXXX"), kind: Some("tel"), autocomplete: Some("tel"),
                    inputmode: Some("tel"), ltr: true, minlength: Some(10), ..Default::default()
                })}
                <fieldset class="field field--wide" aria-describedby=Field::City.error_id()>
                    <legend class="field__label">{t("City", "المدينة")}</legend>
                    <div class="choice-row">
                        {CITIES.iter().map(|c| view! {
                            <label class="choice">
                                <input class="choice__input" type="radio" name="city" value=c.id required=true
                                    checked=initial.city == c.id aria-invalid=city_err.is_some().to_string()/>
                                <span class="choice__label">{c.name(lang)}</span>
                            </label>
                        }).collect_view()}
                    </div>
                    <FieldErrorView field=Field::City error=city_err/>
                </fieldset>
                <div class="field">
                    <label class="field__label" for="co-area">{t("Area", "المنطقة")}</label>
                    <span class="select-wrap">
                        <select class="field__input field__select" id="co-area" name="area" autocomplete="address-level2"
                            required=true aria-invalid=area_err.is_some().to_string() aria-describedby=Field::Area.error_id()>
                            <option value="" selected=initial.area.is_empty()>{t("Choose your area", "اختار منطقتك")}</option>
                            {CITIES.iter().map(|c| view! {
                                <optgroup label=c.name(lang) data-city=c.id>
                                    {c.areas.iter().map(|(id, en, ar)| view! {
                                        <option value=*id selected=initial.area == *id>{if lang.is_rtl() { *ar } else { *en }}</option>
                                    }).collect_view()}
                                </optgroup>
                            }).collect_view()}
                        </select>
                    </span>
                    <FieldErrorView field=Field::Area error=area_err/>
                </div>
                {text(Field::Street, ("Street", "الشارع"), TextOpts {
                    placeholder: Some(if lang.is_rtl() { "مثلًا: شارع ٢٦ يوليو" } else { "e.g. 26th of July St." }),
                    autocomplete: Some("address-line1"), ..Default::default()
                })}
                <div class="field-row">
                    {text(Field::Building, ("Building", "رقم العمارة"), TextOpts { autocomplete: Some("address-line2"), ..Default::default() })}
                    {text(Field::Floor, ("Floor", "الدور"), TextOpts { optional: true, inputmode: Some("numeric"), ..Default::default() })}
                    {text(Field::Apartment, ("Apartment", "الشقة"), TextOpts { optional: true, ..Default::default() })}
                </div>
                {text(Field::Landmark, ("Nearby landmark", "علامة مميزة"), TextOpts {
                    optional: true, wide: true,
                    placeholder: Some(if lang.is_rtl() { "جنب الصيدلية" } else { "Next to the pharmacy" }),
                    ..Default::default()
                })}
                <div class="field field--wide">
                    <label class="field__label" for="co-notes">
                        {t("Notes for the courier", "ملاحظات للمندوب")}
                        <span class="field__opt">{t("optional", "اختياري")}</span>
                    </label>
                    <textarea class="field__input field__textarea" id="co-notes" name="notes" rows="3"
                        maxlength=NOTES_MAX.to_string()
                        placeholder=t("Ring twice, or leave it with the doorman…", "اضرب الجرس مرتين، أو سيبه مع البواب…")
                        aria-invalid=notes_err.is_some().to_string() aria-describedby=format!("co-notes-count {}", Field::Notes.error_id())
                        inner_html=escape_text(&initial.notes)>
                    </textarea>
                    <p class="field__hint field__count" id="co-notes-count">
                        <span class="mono" data-notes-count="">{count_text(used, num)}</span>
                    </p>
                    <FieldErrorView field=Field::Notes error=notes_err/>
                </div>
                <div class="co-actions field--wide">
                    <Button button_type="submit" kind=ButtonKind::Primary size=ButtonSize::Lg icon_end="arrow-right">
                        {t("Continue to delivery time", "كمّل لميعاد التوصيل")}
                    </Button>
                </div>
            </form>
        </div>
    }
}

/// `12/200` in the visitor's numerals.
fn count_text(used: u32, num: Numerals) -> String {
    format!(
        "{}/{}",
        format::count(used, num),
        format::count(NOTES_MAX as u32, num)
    )
}

#[derive(Clone, Copy, Default)]
struct TextOpts {
    hint: Option<(&'static str, &'static str)>,
    placeholder: Option<&'static str>,
    kind: Option<&'static str>,
    autocomplete: Option<&'static str>,
    inputmode: Option<&'static str>,
    minlength: Option<u32>,
    /// Figures typed left to right (phone numbers) even in Arabic.
    ltr: bool,
    optional: bool,
    /// Span the whole row.
    wide: bool,
}

/// The message under a field: the server's, or one the browser code puts in (`data-field-error`).
#[component]
fn FieldErrorView(field: Field, error: Option<String>) -> impl IntoView {
    let hidden = error.is_none();
    view! {
        <p class="field__error" id=field.error_id() data-field-error="" hidden=hidden>
            <Icon name="circle-alert" size=16/>
            <span data-slot="error">{error}</span>
        </p>
    }
}

#[component]
fn TextField(
    field: Field,
    value: String,
    error: Option<String>,
    label: (&'static str, &'static str),
    opts: TextOpts,
    lang: Lang,
) -> impl IntoView {
    let id = field.id();
    let hint_id = format!("{id}-hint");
    let described = if opts.hint.is_some() {
        format!("{hint_id} {}", field.error_id())
    } else {
        field.error_id()
    };
    let invalid = error.is_some();
    view! {
        <div class={if opts.wide { "field field--wide" } else { "field" }}>
            <label class="field__label" for=id.clone()>
                {tr(lang, label.0, label.1)}
                {opts.optional.then(|| view! { <span class="field__opt">{tr(lang, "optional", "اختياري")}</span> })}
            </label>
            <input class="field__input" id=id name=field.name() type=opts.kind.unwrap_or("text") value=value
                autocomplete=opts.autocomplete inputmode=opts.inputmode dir=opts.ltr.then_some("ltr")
                maxlength=field.max_len().to_string() minlength=opts.minlength.map(|n| n.to_string())
                placeholder=opts.placeholder required=!opts.optional
                aria-invalid=invalid.to_string() aria-describedby=described/>
            {opts.hint.map(|h| view! { <p class="field__hint" id=hint_id>{tr(lang, h.0, h.1)}</p> })}
            <FieldErrorView field error/>
        </div>
    }
}

/// A `<textarea>`'s initial value as raw content.
fn escape_text(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}
