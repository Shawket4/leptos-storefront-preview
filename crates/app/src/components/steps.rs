//! A step indicator: numbered dots on a track that fills up to the current step, each with its
//! label under it. Finished steps (and the ones before the current) are links back when they have
//! an `href`; the current one is `aria-current="step"`. Checkout's address → slot → payment →
//! review. Numbers follow the visitor's numerals; the fill mirrors in Arabic.
//!
//! ```ignore
//! <StepIndicator
//!     label=tr(lang, "Checkout steps", "خطوات إتمام الطلب")
//!     steps=Step::ALL.iter().map(|s| (s.label(lang).to_owned(), Some(s.href()))).collect()
//!     current=step.index()
//!     done=done.to_vec()
//! />
//! ```

use super::icons::Icon;
use crate::format;
use crate::prefs::use_ui;
use leptos::either::Either;
use leptos::prelude::*;

#[component]
pub fn StepIndicator(
    /// Names the indicator for screen readers ("Checkout steps").
    #[prop(into)]
    label: String,
    /// `(label, href)` per step, in order.
    steps: Vec<(String, Option<String>)>,
    /// The current step's index.
    current: usize,
    /// Which steps are finished (missing entries: not finished).
    #[prop(optional)]
    done: Vec<bool>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let num = use_ui().numerals.get_untracked();
    let n = steps.len().max(1);
    let progress = if n > 1 {
        current.min(n - 1) as f32 / (n - 1) as f32
    } else {
        0.0
    };
    let cls = if class.is_empty() {
        "steps".to_owned()
    } else {
        format!("steps {class}")
    };
    view! {
        <nav class=cls aria-label=label>
            <ol class="steps__list" style=format!("--n:{n};--p:{progress:.4}")>
                {steps
                    .into_iter()
                    .enumerate()
                    .map(|(i, (text, href))| {
                        let is_done = done.get(i).copied().unwrap_or(false);
                        let is_current = i == current;
                        let state = if is_current {
                            "current"
                        } else if is_done {
                            "done"
                        } else {
                            "next"
                        };
                        let dot = if is_done && !is_current {
                            Either::Left(view! { <Icon name="check" size=14/> })
                        } else {
                            Either::Right(view! { <span class="mono">{format::count(i as u32 + 1, num)}</span> })
                        };
                        let inner = view! {
                            <span class="steps__dot" aria-hidden="true">{dot}</span>
                            <span class="steps__label">{text}</span>
                        };
                        // Back to a finished step (or one before the current): a link.
                        let href = href.filter(|_| !is_current && (is_done || i < current));
                        view! {
                            <li class="steps__step" data-state=state aria-current=if is_current { "step" } else { "false" }>
                                {match href {
                                    Some(h) => Either::Left(view! { <a class="steps__link" href=h>{inner}</a> }),
                                    None => Either::Right(view! { <span class="steps__link">{inner}</span> }),
                                }}
                            </li>
                        }
                    })
                    .collect_view()}
            </ol>
        </nav>
    }
}
