//! Every state that isn't "here is your content": a whole view for empty, error, offline,
//! not-found and coming-soon ([`StateView`]), and the "not in this preview yet" view for
//! unbuilt features ([`ComingSoon`]).
//!
//! The error views built with the showcase — [`SectionError`] (one failed section, server
//! components and islands), [`ErrorPage`] (route-level 500) and [`NotFound`] (the router's 404) —
//! live in `pages::errors` and are re-exported here, so pages import every state from one place.
//!
//! All of these work in server components and inside islands (text follows `use_ui()`) and
//! without JavaScript (every action is a link). Errors are worded in clear Modern Standard
//! Arabic; the rest in the shop's warm Egyptian register (AGENTS.md).
//!
//! ```ignore
//! <StateView tone=StateTone::Offline title=… body=…><Button href=…>…</Button></StateView>
//! <ComingSoon feature=tr(lang, "Card payments", "الدفع بالكارت") description=…/>
//! ```

pub use crate::pages::errors::{ErrorPage, NotFound, SectionError};

use super::button::{Button, ButtonKind};
use super::icons::Icon;
use crate::media::MEDIA_BASE;
use crate::prefs::{tr, use_ui};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

// ─── Tones and their art ─────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum StateTone {
    /// Nothing here yet (empty cart, no results).
    #[default]
    Empty,
    /// Something failed on our side.
    Error,
    /// The visitor's connection is down.
    Offline,
    /// Nothing at this address.
    NotFound,
    /// Not built in this preview yet.
    ComingSoon,
}

impl StateTone {
    /// The brand illustration a tone wears (`data/media/brand/<art>-light|dark`), if it has one.
    pub fn default_art(self) -> Option<&'static str> {
        match self {
            StateTone::Empty => Some("empty-search"),
            StateTone::Error => Some("error"),
            StateTone::Offline => None,
            StateTone::NotFound => Some("not-found"),
            StateTone::ComingSoon => Some("delivery"),
        }
    }
    fn glyph(self) -> &'static str {
        match self {
            StateTone::Empty => "search",
            StateTone::Error => "triangle-alert",
            StateTone::Offline => "wifi-off",
            StateTone::NotFound => "search",
            StateTone::ComingSoon => "construction",
        }
    }
    fn class(self) -> &'static str {
        match self {
            StateTone::Empty => "state--empty",
            StateTone::Error => "state--error",
            StateTone::Offline => "state--offline",
            StateTone::NotFound => "state--not-found",
            StateTone::ComingSoon => "state--soon",
        }
    }
}

/// One brand illustration in both themes. The files sit on one fixed ladder
/// (`brand/<art>-light|dark-{240,400,640}.webp`), so no manifest lookup is needed and this works
/// in islands; CSS shows the one the theme wants and `loading=lazy` keeps the other unfetched.
fn art_pair(art: &str) -> impl IntoView + use<> {
    let img = |variant: &'static str| {
        let base = format!("{MEDIA_BASE}/brand/{art}-{variant}");
        view! {
            <img
                class=format!("state__art state__art--{variant}")
                src=format!("{base}-400.webp")
                srcset=format!("{base}-240.webp 240w, {base}-400.webp 400w, {base}-640.webp 640w")
                sizes="(min-width: 600px) 240px, 50vw"
                width="240"
                height="240"
                alt=""
                loading="lazy"
                decoding="async"
            />
        }
    };
    view! {
        <div class="state__figure" aria-hidden="true">
            {img("light")}
            {img("dark")}
        </div>
    }
}

// ─── StateView ───────────────────────────────────────────────────────────────────────────────

/// A whole state: illustration (or the tone's glyph), an optional kicker ("Error 404"), a title,
/// a line of explanation, the actions (children: buttons or links), and an optional footnote
/// (a reference code).
#[component]
pub fn StateView(
    #[prop(optional)] tone: StateTone,
    /// Overrides the tone's illustration (`empty-cart`, `order-placed`, …).
    #[prop(optional)]
    art: Option<&'static str>,
    #[prop(optional, into)] kicker: Option<TextProp>,
    #[prop(into)] title: TextProp,
    #[prop(into)] body: TextProp,
    /// The page is only this state: the title is the page's `<h1>`.
    #[prop(optional)]
    page: bool,
    /// Smaller art and spacing (inside a card, a drawer, a section).
    #[prop(optional)]
    compact: bool,
    #[prop(optional, into)] footnote: Option<TextProp>,
    #[prop(optional, into)] class: String,
    /// The actions, most important first.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let mut cls = format!("state {}", tone.class());
    if compact {
        cls.push_str(" state--compact");
    }
    if page {
        cls.push_str(" state--page");
    }
    if !class.is_empty() {
        cls.push(' ');
        cls.push_str(&class);
    }
    let figure = match art.or(tone.default_art()) {
        Some(a) => art_pair(a).into_any(),
        None => view! {
            <div class="state__glyph" aria-hidden="true">
                <Icon name=tone.glyph() size=36/>
            </div>
        }
        .into_any(),
    };
    let heading = if page {
        view! { <h1 class="state__title">{move || title.get()}</h1> }.into_any()
    } else {
        view! { <h2 class="state__title">{move || title.get()}</h2> }.into_any()
    };
    view! {
        <div class=cls>
            {figure}
            <div class="state__copy">
                {kicker.map(|k| view! { <p class="state__kicker">{move || k.get()}</p> })}
                {heading}
                <p class="state__body">{move || body.get()}</p>
            </div>
            {children.map(|c| view! { <div class="state__actions">{c()}</div> })}
            {footnote.map(|f| view! { <p class="state__footnote mono">{move || f.get()}</p> })}
        </div>
    }
}

// ─── ComingSoon ──────────────────────────────────────────────────────────────────────────────

/// A feature this preview doesn't have yet (card or wallet payment, accounts…): says so plainly,
/// says what it will do, and offers the way back. Never a dead end.
#[component]
pub fn ComingSoon(
    /// The feature's name ("Card payments").
    #[prop(into)]
    feature: TextProp,
    /// What it will do, a sentence or two.
    #[prop(into)]
    description: TextProp,
    #[prop(optional, into)] back_href: Option<String>,
    #[prop(optional, into)] back_label: Option<TextProp>,
    /// Inline (inside a checkout step or a drawer) instead of a page.
    #[prop(optional)]
    compact: bool,
    #[prop(optional)] page: bool,
) -> impl IntoView {
    let ui = use_ui();
    let back = back_href.unwrap_or_else(|| "/".to_owned());
    let back_label = back_label.unwrap_or_else(|| {
        TextProp::from(move || tr(ui.lang.get(), "Back to the shop", "ارجع للمتجر"))
    });
    let kicker = move || {
        tr(
            ui.lang.get(),
            "Not in this preview yet",
            "مش موجودة في المعاينة دي لسه",
        )
    };
    view! {
        <StateView tone=StateTone::ComingSoon kicker=kicker title=feature body=description compact=compact page=page>
            <Button kind=ButtonKind::Outline href=back icon="arrow-left">
                {move || back_label.get()}
            </Button>
        </StateView>
    }
}
