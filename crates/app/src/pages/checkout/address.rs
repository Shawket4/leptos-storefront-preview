//! The address step, as one island.
//!
//! It renders the whole form, so the server-rendered HTML *is* the no-JS form (a plain POST to
//! `checkout_address`, errors and the typed values coming back from the server). Once hydrated:
//! - each field is checked with `rules::check` when you leave it, and again as you type once it
//!   has shown an error (so a fixed field clears immediately);
//! - the city narrows the area list;
//! - submitting runs `rules::validate` in a capture-phase listener on the wrapper, which sees the
//!   event before the stock `<ActionForm>` does: an invalid form is stopped there (focus moves to
//!   the first problem and a live region says how many), a valid one goes on to `ActionForm`,
//!   which dispatches the `ServerAction`; the server validates again with the same function.
//!   A server-side rejection comes back as the action's `Err` and lands next to the same fields.

use super::rules::{self, AddressInput, CITIES, Field, FieldError};
use super::{CheckoutAddress, CheckoutError};
use crate::components::button::{Button, ButtonKind, ButtonSize};
use crate::components::icons::Icon;
use crate::format;
use crate::pages::cart::store::{arrived_by_navigation, focus_soon};
use crate::prefs::{Lang, Numerals, tr, use_ui};
use leptos::form::{ActionForm, FromFormData};
use leptos::prelude::*;

#[derive(Clone, Copy)]
struct Form {
    values: RwSignal<AddressInput>,
    /// The problems on screen, in form order.
    shown: RwSignal<Vec<FieldError>>,
    submitted: RwSignal<bool>,
    mounted: RwSignal<bool>,
}

impl Form {
    fn error(self, f: Field) -> Option<FieldError> {
        self.shown
            .with(|v| v.iter().find(|e| e.field == f).copied())
    }

    /// Re-run the rule for one field and update what's shown.
    fn recheck(self, f: Field) {
        let problem = self.values.with_untracked(|v| rules::check(f, v));
        self.shown.update(|v| {
            v.retain(|e| e.field != f);
            if let Some(problem) = problem {
                v.push(FieldError { field: f, problem });
                v.sort_by_key(|e| Field::ALL.iter().position(|x| *x == e.field));
            }
        });
    }
}

/// Where focus goes for a field's problem.
fn selector(f: Field) -> &'static str {
    match f {
        Field::Name => "#co-name",
        Field::Phone => "#co-phone",
        Field::City => "#co-address input[name=city]",
        Field::Area => "#co-area",
        Field::Street => "#co-street",
        Field::Building => "#co-building",
        Field::Floor => "#co-floor",
        Field::Apartment => "#co-apartment",
        Field::Landmark => "#co-landmark",
        Field::Notes => "#co-notes",
    }
}

/// "2 fields need attention" with the right Arabic number agreement.
fn count_message(n: usize, lang: Lang, num: Numerals) -> String {
    let c = format::count(n as u32, num);
    match lang {
        Lang::En if n == 1 => "1 field needs attention.".into(),
        Lang::En => format!("{c} fields need attention."),
        Lang::Ar => match n {
            1 => "يوجد حقل يحتاج إلى مراجعة.".into(),
            2 => "يوجد حقلان يحتاجان إلى مراجعة.".into(),
            3..=10 => format!("توجد {c} حقول تحتاج إلى مراجعة."),
            _ => format!("يوجد {c} حقلًا يحتاج إلى مراجعة."),
        },
    }
}

fn input_from(f: &CheckoutAddress) -> AddressInput {
    AddressInput {
        name: f.name.clone(),
        phone: f.phone.clone(),
        city: f.city.clone().unwrap_or_default(),
        area: f.area.clone().unwrap_or_default(),
        street: f.street.clone(),
        building: f.building.clone(),
        floor: f.floor.clone().unwrap_or_default(),
        apartment: f.apartment.clone().unwrap_or_default(),
        landmark: f.landmark.clone().unwrap_or_default(),
        notes: f.notes.clone().unwrap_or_default(),
    }
}

#[island]
pub fn AddressForm(
    initial: AddressInput,
    errors: Vec<FieldError>,
    form_error: Option<String>,
) -> impl IntoView {
    let ui = use_ui();
    let save = ServerAction::<CheckoutAddress>::new();
    let first_error = errors.first().map(|e| e.field);
    let form = Form {
        values: RwSignal::new(initial.clone()),
        shown: RwSignal::new(errors),
        submitted: RwSignal::new(first_error.is_some()),
        mounted: RwSignal::new(false),
    };
    let general = RwSignal::new(form_error);
    let announce = RwSignal::new(String::new());

    let focus_first = move || {
        if let Some(f) = form.shown.with_untracked(|v| v.first().map(|e| e.field)) {
            focus_soon(selector(f));
        }
    };

    Effect::new(move |_| {
        form.mounted.set(true);
        // Arrived from another step through the islands router: land on the heading.
        if arrived_by_navigation() && first_error.is_none() {
            focus_soon("#co-step-title");
        }
    });

    // The server's answer to a submit from here (an `Ok` already navigated via the redirect hook).
    Effect::new(move |_| {
        let Some(Err(e)) = save.value().get() else {
            return;
        };
        let (lang, num) = (ui.lang.get_untracked(), ui.numerals.get_untracked());
        match e {
            CheckoutError::Invalid(es) => {
                announce.set(count_message(es.len(), lang, num));
                form.shown.set(es);
                form.submitted.set(true);
                focus_first();
            }
            other => {
                general.set(Some(other.message(lang)));
                focus_soon("#co-address .notice");
            }
        }
    });

    let on_input = move |ev: leptos::ev::Event| {
        let Some(name) = event_target::<web_sys::Element>(&ev).get_attribute("name") else {
            return;
        };
        let Some(f) = Field::ALL.into_iter().find(|f| f.name() == name) else {
            return;
        };
        let value = event_target_value(&ev);
        form.values.update(|v| {
            f.set(v, value);
            // A new city: an area from the old one no longer applies.
            if f == Field::City && rules::city_of_area(&v.area).is_some_and(|c| c.id != v.city) {
                v.area.clear();
            }
        });
        if form.error(f).is_some() || (f == Field::City && form.submitted.get_untracked()) {
            form.recheck(f);
            if f == Field::City {
                form.recheck(Field::Area);
            }
        }
    };
    let on_focusout = move |ev: leptos::ev::FocusEvent| {
        let Some(name) = event_target::<web_sys::Element>(&ev).get_attribute("name") else {
            return;
        };
        let Some(f) = Field::ALL.into_iter().find(|f| f.name() == name) else {
            return;
        };
        // Don't nag about a field the shopper merely tabbed through before trying to submit.
        let touched = form
            .values
            .with_untracked(|v| !f.value(v).trim().is_empty());
        if touched || form.submitted.get_untracked() {
            form.recheck(f);
        }
    };
    // Runs before `<ActionForm>`'s own submit handler (capture phase on an ancestor).
    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        let Ok(raw) = CheckoutAddress::from_event(&ev) else {
            return;
        };
        let input = input_from(&raw);
        let errs = rules::validate(&input);
        form.values.set(input);
        form.submitted.set(true);
        general.set(None);
        if errs.is_empty() {
            form.shown.set(vec![]);
            announce.set(String::new());
        } else {
            // Stops `ActionForm` and the islands router alike (both skip prevented events).
            ev.prevent_default();
            announce.set(count_message(
                errs.len(),
                ui.lang.get_untracked(),
                ui.numerals.get_untracked(),
            ));
            form.shown.set(errs);
            focus_first();
        }
    };

    let t = move |en: &'static str, ar: &'static str| move || tr(ui.lang.get(), en, ar);
    let city_now = initial.city.clone();

    view! {
        <div id="co-address" class="co-address" on:submit:capture=on_submit on:input=on_input on:focusout=on_focusout>
            <div class="co-step-head">
                <h2 id="co-step-title" class="co-step-title" tabindex="-1">{t("Where should we deliver?", "نوصّل فين؟")}</h2>
                <p class="co-step-sub">{t("We deliver across Cairo, Giza and Alexandria.", "بنوصّل في القاهرة والجيزة والإسكندرية.")}</p>
            </div>
            {move || general.get().map(|m| view! {
                <p class="notice notice--error" role="alert" tabindex="-1"><Icon name="circle-alert" size=20/><span>{m}</span></p>
            })}
            <ActionForm action=save attr:class="co-form co-fields" attr:novalidate="">
                <TextField form field=Field::Name label=("Full name", "الاسم بالكامل")
                    hint=("The person receiving the order", "اسم اللي هيستلم الطلب")
                    autocomplete="name" autofocus={first_error == Some(Field::Name)}/>
                <TextField form field=Field::Phone kind="tel" label=("Mobile number", "رقم الموبايل")
                    hint=("The courier will call this number", "المندوب هيكلمك على الرقم ده")
                    placeholder=("01X XXXX XXXX", "01X XXXX XXXX") autocomplete="tel" inputmode="tel" ltr=true
                    autofocus={first_error == Some(Field::Phone)}/>
                <div class="field field--wide" role="radiogroup" aria-labelledby="co-city-label"
                    aria-describedby="co-city-err" aria-invalid=move || { form.error(Field::City).is_some().to_string() }>
                    <span class="field__label" id="co-city-label">{t("City", "المدينة")}</span>
                    <div class="choice-row">
                        {CITIES.iter().map(|c| {
                            let checked = city_now == c.id;
                            let autofocus = first_error == Some(Field::City) && c.id == CITIES[0].id;
                            view! {
                                <label class="choice">
                                    <input class="choice__input" type="radio" name="city" value=c.id checked=checked autofocus=autofocus/>
                                    <span class="choice__label">{move || c.name(ui.lang.get())}</span>
                                </label>
                            }
                        }).collect_view()}
                    </div>
                    <FieldErrorView form field=Field::City/>
                </div>
                <div class="field">
                    <label class="field__label" for="co-area">{t("Area", "المنطقة")}</label>
                    <AreaSelect form autofocus={first_error == Some(Field::Area)}/>
                    <FieldErrorView form field=Field::Area/>
                </div>
                <TextField form field=Field::Street label=("Street", "الشارع")
                    placeholder=("e.g. 26th of July St.", "مثلًا: شارع ٢٦ يوليو") autocomplete="address-line1"
                    autofocus={first_error == Some(Field::Street)}/>
                <div class="field-row">
                    <TextField form field=Field::Building label=("Building", "رقم العمارة") autocomplete="address-line2"
                        autofocus={first_error == Some(Field::Building)}/>
                    <TextField form field=Field::Floor label=("Floor", "الدور") optional=true inputmode="numeric"/>
                    <TextField form field=Field::Apartment label=("Apartment", "الشقة") optional=true/>
                </div>
                <TextField form field=Field::Landmark label=("Nearby landmark", "علامة مميزة") optional=true wide=true
                    placeholder=("Next to the pharmacy", "جنب الصيدلية")/>
                <NotesField form/>
                <p class="sr-only" aria-live="assertive">
                    {move || { let a = announce.get(); (!a.is_empty()).then(|| view! { <span>{a}</span> }) }}
                </p>
                <div class="co-actions field--wide">
                    <Button button_type="submit" kind=ButtonKind::Primary size=ButtonSize::Lg icon_end="arrow-right"
                        loading=Signal::derive(move || save.pending().get())>
                        {t("Continue to delivery time", "كمّل لميعاد التوصيل")}
                    </Button>
                </div>
            </ActionForm>
        </div>
    }
}

#[component]
fn FieldErrorView(form: Form, field: Field) -> impl IntoView {
    let ui = use_ui();
    move || {
        form.error(field).map(|e| {
            view! {
                <p class="field__error" id=field.error_id()>
                    <Icon name="circle-alert" size=16/>
                    <span>{e.message(ui.lang.get(), ui.numerals.get())}</span>
                </p>
            }
        })
    }
}

#[component]
fn TextField(
    form: Form,
    field: Field,
    label: (&'static str, &'static str),
    #[prop(optional)] hint: Option<(&'static str, &'static str)>,
    #[prop(optional)] placeholder: Option<(&'static str, &'static str)>,
    #[prop(optional)] kind: Option<&'static str>,
    #[prop(optional)] autocomplete: Option<&'static str>,
    #[prop(optional)] inputmode: Option<&'static str>,
    /// Figures typed left to right (phone numbers) even in Arabic.
    #[prop(optional)]
    ltr: bool,
    #[prop(optional)] optional: bool,
    /// Span the whole row.
    #[prop(optional)]
    wide: bool,
    #[prop(optional)] autofocus: bool,
) -> impl IntoView {
    let ui = use_ui();
    let id = field.id();
    let hint_id = format!("{id}-hint");
    let described = if hint.is_some() {
        format!("{hint_id} {}", field.error_id())
    } else {
        field.error_id()
    };
    let value = form.values.with_untracked(|v| field.value(v).to_owned());
    let invalid = move || form.error(field).is_some();
    view! {
        <div class={if wide { "field field--wide" } else { "field" }}>
            <label class="field__label" for=id.clone()>
                {move || tr(ui.lang.get(), label.0, label.1)}
                {optional.then(|| view! { <span class="field__opt">{move || tr(ui.lang.get(), "optional", "اختياري")}</span> })}
            </label>
            <input class="field__input" id=id name=field.name() type=kind.unwrap_or("text") value=value
                autocomplete=autocomplete inputmode=inputmode dir=ltr.then_some("ltr") maxlength=field.max_len().to_string()
                placeholder=move || { placeholder.map(|p| tr(ui.lang.get(), p.0, p.1)) }
                aria-invalid=move || { invalid().to_string() } aria-describedby=described
                aria-required={(!optional).to_string()} autofocus=autofocus/>
            {hint.map(|h| view! { <p class="field__hint" id=hint_id>{move || tr(ui.lang.get(), h.0, h.1)}</p> })}
            <FieldErrorView form field/>
        </div>
    }
}

/// The area list. The server renders every city's areas in `<optgroup>`s (without JS the city
/// can't narrow it; the server checks the pair). After hydration it shows only the chosen city's.
#[component]
fn AreaSelect(form: Form, autofocus: bool) -> impl IntoView {
    let ui = use_ui();
    let area = move || form.values.with(|v| v.area.clone());
    let options = move || {
        let (lang, current) = (ui.lang.get(), area());
        let city = form.values.with(|v| rules::city(&v.city).map(|c| c.id));
        let only = city.filter(|_| form.mounted.get());
        let placeholder = tr(lang, "Choose your area", "اختار منطقتك");
        let groups = CITIES
            .iter()
            .filter(|c| only.is_none_or(|id| id == c.id))
            .map(|c| {
                let opts = c
                    .areas
                    .iter()
                    .map(|(id, en, ar)| {
                        let name = if lang.is_rtl() { *ar } else { *en };
                        view! { <option value=*id selected={current == *id}>{name}</option> }
                    })
                    .collect_view();
                view! { <optgroup label=c.name(lang)>{opts}</optgroup> }
            })
            .collect_view();
        view! {
            <option value="" selected={current.is_empty()}>{placeholder}</option>
            {groups}
        }
    };
    view! {
        <select class="field__input field__select" id="co-area" name="area" autocomplete="address-level2"
            prop:value=area aria-invalid=move || { form.error(Field::Area).is_some().to_string() }
            aria-describedby=Field::Area.error_id() aria-required="true" autofocus=autofocus>
            {options}
        </select>
    }
}

/// Text for an element's raw content (a `<textarea>`'s initial value: a text child would render
/// as a single space when empty, because tachys keeps a text node for hydration).
fn escape_text(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

#[component]
fn NotesField(form: Form) -> impl IntoView {
    let ui = use_ui();
    let f = Field::Notes;
    let value = form.values.with_untracked(|v| v.notes.clone());
    let used = move || form.values.with(|v| v.notes.chars().count());
    view! {
        <div class="field field--wide">
            <label class="field__label" for="co-notes">
                {move || tr(ui.lang.get(), "Notes for the courier", "ملاحظات للمندوب")}
                <span class="field__opt">{move || tr(ui.lang.get(), "optional", "اختياري")}</span>
            </label>
            <textarea class="field__input field__textarea" id="co-notes" name="notes" rows="3" maxlength=rules::NOTES_MAX.to_string()
                placeholder=move || { tr(ui.lang.get(), "Ring twice, or leave it with the doorman…", "اضرب الجرس مرتين، أو سيبه مع البواب…") }
                aria-invalid=move || { form.error(f).is_some().to_string() } aria-describedby=format!("co-notes-count {}", f.error_id())
                inner_html=escape_text(&value)>
            </textarea>
            <p class="field__hint field__count" id="co-notes-count">
                <span class="mono">{move || format!("{}/{}", format::count(used() as u32, ui.numerals.get()), format::count(rules::NOTES_MAX as u32, ui.numerals.get()))}</span>
            </p>
            <FieldErrorView form field=f/>
        </div>
    }
}
