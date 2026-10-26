//! Per-visitor display preferences: language, numeral system, colour theme.
//!
//! They live in cookies so the server renders the right `<html lang dir data-theme>` before the
//! first byte (no flash). A change is a `/__prefs` link: the router swaps the whole frame in place.

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

pub const COOKIE_LANG: &str = "lang";
pub const COOKIE_NUMERALS: &str = "num";
pub const COOKIE_THEME: &str = "theme";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Lang {
    #[default]
    En,
    Ar,
}

impl Lang {
    pub fn code(self) -> &'static str {
        match self {
            Lang::En => "en",
            Lang::Ar => "ar",
        }
    }
    pub fn dir(self) -> &'static str {
        match self {
            Lang::En => "ltr",
            Lang::Ar => "rtl",
        }
    }
    pub fn is_rtl(self) -> bool {
        self == Lang::Ar
    }
    pub fn parse(s: &str) -> Option<Lang> {
        match s {
            "en" => Some(Lang::En),
            "ar" => Some(Lang::Ar),
            _ => None,
        }
    }
    pub fn other(self) -> Lang {
        match self {
            Lang::En => Lang::Ar,
            Lang::Ar => Lang::En,
        }
    }
    /// The language's own name, as shown on the toggle.
    pub fn endonym(self) -> &'static str {
        match self {
            Lang::En => "English",
            Lang::Ar => "العربية",
        }
    }
}

/// Which digits numbers are written with. Arabic text in Egypt uses both, so it is a setting.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Numerals {
    /// 0123456789
    #[default]
    Latn,
    /// ٠١٢٣٤٥٦٧٨٩ (Arabic-Indic)
    Arab,
}

impl Numerals {
    pub fn code(self) -> &'static str {
        match self {
            Numerals::Latn => "latn",
            Numerals::Arab => "arab",
        }
    }
    pub fn parse(s: &str) -> Option<Numerals> {
        match s {
            "latn" => Some(Numerals::Latn),
            "arab" => Some(Numerals::Arab),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Theme {
    /// Follow `prefers-color-scheme` (only when the visitor picks it).
    System,
    /// The default — owner's call: the storefront opens light even on a dark-mode phone.
    #[default]
    Light,
    Dark,
}

impl Theme {
    pub fn code(self) -> &'static str {
        match self {
            Theme::System => "system",
            Theme::Light => "light",
            Theme::Dark => "dark",
        }
    }
    pub fn parse(s: &str) -> Option<Theme> {
        match s {
            "system" => Some(Theme::System),
            "light" => Some(Theme::Light),
            "dark" => Some(Theme::Dark),
            _ => None,
        }
    }
}

/// A snapshot of the preferences for one request (server) or page load (client).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Prefs {
    pub lang: Lang,
    pub numerals: Numerals,
    pub theme: Theme,
}

/// The request's preferences as context, provided per request from cookies (see
/// `request::RequestCtx`); components read them with [`use_ui`] / [`lang_now`].
#[derive(Clone, Copy, Debug)]
pub struct Ui {
    pub lang: RwSignal<Lang>,
    pub numerals: RwSignal<Numerals>,
    pub theme: RwSignal<Theme>,
}

impl Ui {
    pub fn new(p: Prefs) -> Self {
        Ui {
            lang: RwSignal::new(p.lang),
            numerals: RwSignal::new(p.numerals),
            theme: RwSignal::new(p.theme),
        }
    }

    pub fn provide(p: Prefs) -> Self {
        let ui = Ui::new(p);
        provide_context(ui);
        ui
    }
}

/// The [`Ui`] store. Falls back to defaults if nothing provided it (e.g. an isolated test).
pub fn use_ui() -> Ui {
    use_context::<Ui>().unwrap_or_else(|| Ui::provide(Prefs::default()))
}

/// Current language, untracked. Use in server components, where it never changes mid-render.
pub fn lang_now() -> Lang {
    use_ui().lang.get_untracked()
}

/// Pick the English or Arabic literal (one-off server labels; shared strings live in `strings`).
#[inline]
pub fn tr(lang: Lang, en: &'static str, ar: &'static str) -> &'static str {
    match lang {
        Lang::En => en,
        Lang::Ar => ar,
    }
}
