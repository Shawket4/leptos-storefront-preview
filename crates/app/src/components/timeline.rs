//! A vertical timeline of stages: a dot per step on a line that fills up to the current one, each
//! with its label and an optional time ("Placed · 14:02"). Order tracking. Each step says its state
//! to screen readers ("done", "in progress", "not yet"); the current one is `aria-current="step"`.
//!
//! Browser code moves it while it polls with `setTimeline(el, {progress, states, times})`
//! (`client/src/core/timeline.ts`): states are `data-state` on each step, times fill
//! `data-slot="t{i}"` (always present with `slots`, hidden while empty).
//!
//! ```ignore
//! <Timeline steps=t.steps.iter().map(|s| TimelineStep::new(s.label, s.state).time(s.time)).collect()
//!     progress=t.progress stopped=t.cancelled slots=true/>
//! ```

use super::icons::Icon;
use crate::prefs::lang_now;
use crate::strings;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StepState {
    Done,
    Current,
    #[default]
    Todo,
}

impl StepState {
    /// `data-state` (and `setTimeline`'s states): `done`, `current`, `todo`.
    pub fn code(self) -> &'static str {
        match self {
            StepState::Done => "done",
            StepState::Current => "current",
            StepState::Todo => "todo",
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TimelineStep {
    pub label: String,
    pub state: StepState,
    /// When it happened (or is expected), formatted.
    pub time: Option<String>,
}

impl TimelineStep {
    pub fn new(label: impl Into<String>, state: StepState) -> Self {
        TimelineStep {
            label: label.into(),
            state,
            time: None,
        }
    }
    pub fn time(mut self, time: impl Into<Option<String>>) -> Self {
        self.time = time.into();
        self
    }
}

#[component]
pub fn Timeline(
    steps: Vec<TimelineStep>,
    /// How far the line has grown, 0..=1 (to the current step).
    #[prop(optional)]
    progress: f32,
    /// A stopped run (a cancelled order): the line doesn't fill.
    #[prop(optional)]
    stopped: bool,
    /// `data-slot="t{i}"` on every step's time, hidden while empty (for `setTimeline`).
    #[prop(optional)]
    slots: bool,
    /// Names the list for screen readers ("Order progress").
    #[prop(optional, into)]
    label: Option<String>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let lang = lang_now();
    let cls = if class.is_empty() {
        "track".to_owned()
    } else {
        format!("track {class}")
    };
    let said = move |s: StepState| strings::get(&format!("timeline.{}", s.code()), lang);
    view! {
        <ol class=cls style=format!("--p:{:.4}", progress.clamp(0.0, 1.0)) data-stopped=stopped.to_string() aria-label=label>
            {steps
                .into_iter()
                .enumerate()
                .map(|(i, s)| {
                    let time = match (s.time, slots) {
                        (Some(t), _) => Some(view! { <span class="track__time" data-slot=slots.then(|| format!("t{i}"))>{t}</span> }.into_any()),
                        (None, true) => Some(view! { <span class="track__time" data-slot=format!("t{i}") hidden=true></span> }.into_any()),
                        (None, false) => None,
                    };
                    view! {
                        <li class="track__step" data-step=i data-state=s.state.code()
                            aria-current=if s.state == StepState::Current { "step" } else { "false" }>
                            <span class="track__dot" aria-hidden="true"><Icon name="check" size=12/></span>
                            <span class="track__text">
                                <span class="track__label">{s.label}</span>
                                <span class="sr-only" data-said="">{format!(", {}", said(s.state))}</span>
                                {time}
                            </span>
                        </li>
                    }
                })
                .collect_view()}
        </ol>
    }
}
