//! A quantity stepper: − value +, 44px targets, the figure in Plex Mono (in the visitor's
//! numerals), buttons disabled at the bounds. Optionally the minus turns into a bin at the
//! minimum (cart lines) and removes the item. For islands: the value is a signal, changes go out
//! through `on_change`.

use super::icons::Icon;
use crate::format;
use crate::prefs::{tr, use_ui};
use leptos::prelude::*;

#[component]
pub fn Stepper(
    #[prop(into)] value: Signal<u32>,
    on_change: Callback<u32>,
    min: u32,
    max: u32,
    /// Names the group ("Quantity of Galaxy S25"); defaults to "Quantity".
    #[prop(optional, into)]
    label: MaybeProp<String>,
    /// At the minimum, the minus becomes a bin that calls this.
    #[prop(optional)]
    on_remove: Option<Callback<()>>,
    /// The whole control is busy (a request in flight).
    #[prop(optional, into)]
    disabled: MaybeProp<bool>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let ui = use_ui();
    let busy = move || disabled.get().unwrap_or(false);
    let at_min = move || value.get() <= min;
    let at_max = move || value.get() >= max;
    let removable = on_remove.is_some();
    let group = move || {
        label
            .get()
            .unwrap_or_else(|| tr(ui.lang.get(), "Quantity", "الكمية").to_owned())
    };
    let minus_label = move || {
        if removable && at_min() {
            tr(ui.lang.get(), "Remove", "شيل")
        } else {
            tr(ui.lang.get(), "Decrease", "قلّل")
        }
    };
    let plus_label = move || tr(ui.lang.get(), "Increase", "زوّد");
    let shown = move || format::count(value.get(), ui.numerals.get());
    let cls = if class.is_empty() {
        "stepper".to_owned()
    } else {
        format!("stepper {class}")
    };
    view! {
        <div class=cls role="group" aria-label=group>
            <button
                type="button"
                class="stepper__btn"
                aria-label=minus_label
                disabled=move || busy() || (at_min() && !removable)
                on:click=move |_| {
                    let v = value.get_untracked();
                    if v <= min {
                        if let Some(cb) = on_remove {
                            cb.run(());
                        }
                    } else {
                        on_change.run(v - 1);
                    }
                }
            >
                {move || {
                    let name = if removable && at_min() { "trash-2" } else { "minus" };
                    view! { <Icon name=name size=18/> }
                }}
            </button>
            <output class="stepper__value mono" aria-live="polite">{shown}</output>
            <button
                type="button"
                class="stepper__btn"
                aria-label=plus_label
                disabled=move || busy() || at_max()
                on:click=move |_| {
                    let v = value.get_untracked();
                    if v < max {
                        on_change.run(v + 1);
                    }
                }
            >
                <Icon name="plus" size=18/>
            </button>
        </div>
    }
}
