//! The address step: one plain form (a POST to `checkout_address`) built from the design system's
//! controls (`components::form`), rendered with what the shopper typed and the server's messages
//! next to the fields. `features/checkout.ts` says what's missing before the round-trip (the
//! constraint validation API, messages from the catalogue); without JS the browser's own checks
//! apply; the server checks everything again with `rules::validate`.

use super::CheckoutAddress;
use super::rules::{AddressInput, CITIES, Field, FieldError, NOTES_MAX};
use crate::components::button::{Button, ButtonKind, ButtonSize};
use crate::components::form::{RadioCard, RadioGroup, Select, TextArea, TextInput};
use crate::components::notice::{Notice, Tone};
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
    // The areas, grouped by city (the server checks that the pair matches).
    let choose = vec![(
        String::new(),
        t("Choose your area", "اختار منطقتك").to_owned(),
    )];
    let areas: Vec<(String, Vec<(String, String)>)> = CITIES
        .iter()
        .map(|c| {
            let list = c
                .areas
                .iter()
                .map(|(id, en, a)| ((*id).to_owned(), (if ar { *a } else { *en }).to_owned()))
                .collect();
            (c.name(lang).to_owned(), list)
        })
        .collect();
    let used = initial.notes.chars().count() as u32;
    view! {
        <div id="co-address" class="co-address">
            <div class="co-step-head">
                <h2 id="co-step-title" class="co-step-title" tabindex="-1">{t("Where should we deliver?", "نوصّل فين؟")}</h2>
                <p class="co-step-sub">{t("We deliver across Cairo, Giza and Alexandria.", "بنوصّل في القاهرة والجيزة والإسكندرية.")}</p>
            </div>
            {form_error.map(|m| view! { <Notice tone=Tone::Danger title=m attr:tabindex="-1" attr:data-autofocus=""/> })}
            <form id="co-address-form" class="co-form co-fields" method="post" action=CheckoutAddress::url()
                data-ts="checkout-form">
                <TextInput id="co-name" maxlength=Field::Name.max_len() as u32 minlength=2 name="name" label=t("Full name", "الاسم بالكامل")
                    hint=t("The person receiving the order", "اسم اللي هيستلم الطلب")
                    value=initial.name.clone() error=err(Field::Name) autocomplete="name" required=true/>
                <TextInput id="co-phone" maxlength=Field::Phone.max_len() as u32 minlength=10 name="phone" label=t("Mobile number", "رقم الموبايل")
                    hint=t("The courier will call this number", "المندوب هيكلمك على الرقم ده")
                    value=initial.phone.clone() error=err(Field::Phone) input_type="tel" inputmode="tel"
                    autocomplete="tel" dir="ltr" placeholder="01X XXXX XXXX" required=true/>
                <div class="co-span">
                    <RadioGroup legend=t("City", "المدينة") id="co-city" error=err(Field::City) class="co-cities">
                        {CITIES.iter().map(|c| view! {
                            <RadioCard name="city" value=c.id label=c.name(lang) checked=initial.city == c.id required=true/>
                        }).collect_view()}
                    </RadioGroup>
                </div>
                <div class="co-span">
                    <Select id="co-area" name="area" label=t("Area", "المنطقة") options=choose groups=areas required=true
                        value=initial.area.clone() error=err(Field::Area)/>
                </div>
                <div class="co-span">
                    <TextInput id="co-street" maxlength=Field::Street.max_len() as u32 name="street" label=t("Street", "الشارع")
                        value=initial.street.clone() error=err(Field::Street) autocomplete="address-line1" required=true
                        placeholder=t("e.g. 26th of July St.", "مثلًا: شارع ٢٦ يوليو")/>
                </div>
                <div class="co-span co-row">
                    <TextInput id="co-building" maxlength=Field::Building.max_len() as u32 name="building" label=t("Building", "رقم العمارة")
                        value=initial.building.clone() error=err(Field::Building) autocomplete="address-line2" required=true/>
                    <TextInput id="co-floor" maxlength=Field::Floor.max_len() as u32 name="floor" label=t("Floor", "الدور") value=initial.floor.clone()
                        error=err(Field::Floor) inputmode="numeric" optional=true/>
                    <TextInput id="co-apartment" maxlength=Field::Apartment.max_len() as u32 name="apartment" label=t("Apartment", "الشقة")
                        value=initial.apartment.clone() error=err(Field::Apartment) optional=true/>
                </div>
                <div class="co-span">
                    <TextInput id="co-landmark" maxlength=Field::Landmark.max_len() as u32 name="landmark" label=t("Nearby landmark", "علامة مميزة")
                        value=initial.landmark.clone() error=err(Field::Landmark) optional=true
                        placeholder=t("Next to the pharmacy", "جنب الصيدلية")/>
                </div>
                <div class="co-span">
                    <TextArea id="co-notes" maxlength=NOTES_MAX as u32 name="notes" label=t("Notes for the courier", "ملاحظات للمندوب")
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
