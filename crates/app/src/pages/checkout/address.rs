//! The address step: one plain form (a POST to `checkout_address`) built from the design system's
//! controls (`components::form`), rendered with what the shopper typed and the server's messages
//! next to the fields. `features/checkout.ts` says what's missing before the round-trip (the
//! constraint validation API, messages from the catalogue); without JS the browser's own checks
//! apply; the server checks everything again with `rules::validate`.

use super::CheckoutAddress;
use super::rules::{AddressInput, CITIES, Field, FieldError, NOTES_MAX};
use crate::components::button::{Button, ButtonKind, ButtonSize};
use crate::components::form::{RadioCard, RadioGroup, Select, TextArea, TextInput};
use crate::components::icons::Icon;
use crate::format;
use crate::prefs::{Numerals, tr, use_ui};
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
    let ar = lang.is_rtl();
    // Every city's areas, each named with its city (one flat list; the server checks the pair).
    let mut areas = vec![(
        String::new(),
        t("Choose your area", "اختار منطقتك").to_owned(),
    )];
    for c in CITIES.iter() {
        for (id, en, a) in c.areas {
            areas.push((
                (*id).to_owned(),
                format!("{} — {}", if ar { *a } else { *en }, c.name(lang)),
            ));
        }
    }
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
                <TextInput id="co-name" name="name" label=t("Full name", "الاسم بالكامل")
                    hint=t("The person receiving the order", "اسم اللي هيستلم الطلب")
                    value=initial.name.clone() error=err(Field::Name) autocomplete="name" required=true/>
                <TextInput id="co-phone" name="phone" label=t("Mobile number", "رقم الموبايل")
                    hint=t("The courier will call this number", "المندوب هيكلمك على الرقم ده")
                    value=initial.phone.clone() error=err(Field::Phone) input_type="tel" inputmode="tel"
                    autocomplete="tel" dir="ltr" placeholder="01X XXXX XXXX" required=true/>
                <div class="co-span">
                    <RadioGroup legend=t("City", "المدينة") id="co-city" error=err(Field::City) class="co-cities">
                        {CITIES.iter().map(|c| view! {
                            <RadioCard name="city" value=c.id label=c.name(lang) checked=initial.city == c.id/>
                        }).collect_view()}
                    </RadioGroup>
                </div>
                <div class="co-span">
                    <Select id="co-area" name="area" label=t("Area", "المنطقة") options=areas
                        value=initial.area.clone() error=err(Field::Area)/>
                </div>
                <div class="co-span">
                    <TextInput id="co-street" name="street" label=t("Street", "الشارع")
                        value=initial.street.clone() error=err(Field::Street) autocomplete="address-line1" required=true
                        placeholder=t("e.g. 26th of July St.", "مثلًا: شارع ٢٦ يوليو")/>
                </div>
                <div class="co-span co-row">
                    <TextInput id="co-building" name="building" label=t("Building", "رقم العمارة")
                        value=initial.building.clone() error=err(Field::Building) autocomplete="address-line2" required=true/>
                    <TextInput id="co-floor" name="floor" label=t("Floor", "الدور") value=initial.floor.clone()
                        error=err(Field::Floor) inputmode="numeric" optional=true/>
                    <TextInput id="co-apartment" name="apartment" label=t("Apartment", "الشقة")
                        value=initial.apartment.clone() error=err(Field::Apartment) optional=true/>
                </div>
                <div class="co-span">
                    <TextInput id="co-landmark" name="landmark" label=t("Nearby landmark", "علامة مميزة")
                        value=initial.landmark.clone() error=err(Field::Landmark) optional=true
                        placeholder=t("Next to the pharmacy", "جنب الصيدلية")/>
                </div>
                <div class="co-span">
                    <TextArea id="co-notes" name="notes" label=t("Notes for the courier", "ملاحظات للمندوب")
                        value=initial.notes.clone() error=err(Field::Notes) optional=true
                        placeholder=t("Ring twice, or leave it with the doorman…", "اضرب الجرس مرتين، أو سيبه مع البواب…")/>
                    <p class="co-count mono" data-notes-count="" data-max=NOTES_MAX>{count_text(used, num)}</p>
                </div>
                <div class="co-actions co-span">
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
