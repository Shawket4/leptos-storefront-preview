//! The one quantity stepper: − input +, 44px targets, the figure in Plex Mono.
//!
//! Server markup only; `client/src/core/stepper.ts` (`data-ts="stepper"`) makes the buttons work,
//! clamps what is typed to `min..=max` and fires `change` on the input — features listen for that
//! (`input.valueAsNumber`) and never build steppers of their own. The − and + are `type="button"`
//! and the input is a real `<input type="number" name=…>`, so in a plain form without JS the typed
//! quantity is posted (the product page posts `name="qty"`).
//!
//! `removable` (cart lines): at the minimum the − turns into a bin, and pressing it sets the value
//! to 0 and fires `change` — the listener removes the line.

use super::icons::Icon;
use crate::prefs::use_ui;
use crate::strings;
use leptos::prelude::*;

#[component]
pub fn Stepper(
    /// The input's form name (`qty`).
    #[prop(optional, into)]
    name: Option<String>,
    #[prop(into)] value: Signal<u32>,
    min: u32,
    max: u32,
    /// Names the group and the input ("Quantity of Galaxy S25"); defaults to "Quantity".
    #[prop(optional, into)]
    label: MaybeProp<String>,
    /// At the minimum the − becomes a bin that sets the value to 0.
    #[prop(optional)]
    removable: bool,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    /// The input's id (for a `<label for>`).
    #[prop(optional, into)]
    id: Option<String>,
    #[prop(optional, into)] class: String,
    /// Islands-era props, ignored (listen for `change` on the input instead). Remove them.
    #[prop(optional)]
    on_change: Option<Callback<u32>>,
    #[prop(optional)] on_remove: Option<Callback<()>>,
) -> impl IntoView {
    let _ = on_change;
    let removable = removable || on_remove.is_some();
    let lang = use_ui().lang.get_untracked();
    let max = max.max(min);
    let v = value
        .get_untracked()
        .clamp(if removable { 0 } else { min }, max);
    let off = disabled.get_untracked().unwrap_or(false);
    let label = label
        .get_untracked()
        .unwrap_or_else(|| strings::get("stepper.quantity", lang).to_owned());
    let group_label = label.clone();
    let bin = removable && v <= min;
    let cls = if class.is_empty() {
        "stepper".to_owned()
    } else {
        format!("stepper {class}")
    };
    let minus_label = strings::get(
        if bin {
            "stepper.remove"
        } else {
            "stepper.decrease"
        },
        lang,
    );
    view! {
        <div
            class=cls
            role="group"
            aria-label=group_label
            data-ts="stepper"
            data-min=min
            data-max=max
            data-removable=removable.then_some("")
            data-bin=bin.then_some("")
        >
            <button
                type="button"
                class="stepper__btn"
                data-step="-1"
                aria-label=minus_label
                disabled=(off || (v <= min && !removable))
            >
                <Icon name="minus" size=18 class="stepper__minus"/>
                {removable.then(|| view! { <Icon name="trash-2" size=18 class="stepper__bin"/> })}
            </button>
            <input
                class="stepper__value mono"
                type="number"
                inputmode="numeric"
                name=name
                id=id
                value=v
                min=if removable { 0 } else { min }
                max=max
                step="1"
                aria-label=label
                autocomplete="off"
                enterkeyhint="done"
                disabled=off
            />
            <button
                type="button"
                class="stepper__btn"
                data-step="1"
                aria-label=strings::get("stepper.increase", lang)
                disabled=(off || v >= max)
            >
                <Icon name="plus" size=18/>
            </button>
        </div>
    }
}
