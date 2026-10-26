//! Buttons and round icon buttons.
//!
//! `Button` renders an `<a>` when given `href` (navigation is a link, so it works without JS and
//! the router takes it) and a `<button>` otherwise. Extra attributes go straight on the component
//! (`<Button attr:form="x" attr:data-drawer-close="">`); Leptos spreads them onto the element.

use super::icons::Icon;
use leptos::either::Either;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonKind {
    /// The tenant colour. One per screen.
    #[default]
    Primary,
    /// Sunk grey: the second action.
    Secondary,
    /// Hairline on transparent.
    Outline,
    /// Text only, a wash on hover.
    Ghost,
    Danger,
}

impl ButtonKind {
    fn class(self) -> &'static str {
        match self {
            ButtonKind::Primary => "btn--primary",
            ButtonKind::Secondary => "btn--secondary",
            ButtonKind::Outline => "btn--outline",
            ButtonKind::Ghost => "btn--ghost",
            ButtonKind::Danger => "btn--danger",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonSize {
    /// 36px.
    Sm,
    /// 44px (the minimum tap target).
    #[default]
    Md,
    /// 52px: the page's main call to action.
    Lg,
}

impl ButtonSize {
    fn class(self) -> &'static str {
        match self {
            ButtonSize::Sm => "btn--sm",
            ButtonSize::Md => "btn--md",
            ButtonSize::Lg => "btn--lg",
        }
    }
}

/// Icons that point along the reading direction mirror in Arabic.
pub(crate) fn directional(icon: &str) -> bool {
    matches!(
        icon,
        "arrow-left" | "arrow-right" | "chevron-left" | "chevron-right" | "arrow-up-right"
    )
}

#[component]
pub fn Button(
    #[prop(optional)] kind: ButtonKind,
    #[prop(optional)] size: ButtonSize,
    /// Render a link instead of a button.
    #[prop(optional, into)]
    href: Option<String>,
    /// `"button"` (default) or `"submit"`.
    #[prop(optional)]
    button_type: &'static str,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    /// Busy: a spinner replaces the leading icon and the button can't be pressed.
    #[prop(optional, into)]
    loading: MaybeProp<bool>,
    #[prop(optional)] full_width: bool,
    /// A lucide icon before the label.
    #[prop(optional)]
    icon: Option<&'static str>,
    /// A lucide icon after the label (arrows mirror in Arabic).
    #[prop(optional)]
    icon_end: Option<&'static str>,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let mut cls = format!("btn {} {}", kind.class(), size.class());
    if full_width {
        cls.push_str(" btn--block");
    }
    if !class.is_empty() {
        cls.push(' ');
        cls.push_str(&class);
    }
    let icon_size = if size == ButtonSize::Sm { 16 } else { 18 };
    let busy = move || loading.get().unwrap_or(false);
    let lead = move || {
        if busy() {
            Some(view! { <Icon name="loader-circle" size=icon_size class="spin"/> }.into_any())
        } else {
            icon.map(|i| {
                let c = if directional(i) { "flip-rtl" } else { "" };
                view! { <Icon name=i size=icon_size class=c/> }.into_any()
            })
        }
    };
    let trail = icon_end.map(|i| {
        let c = if directional(i) {
            "btn__end flip-rtl"
        } else {
            "btn__end"
        };
        view! { <Icon name=i size=icon_size class=c/> }
    });
    let inner = view! {
        {lead}
        <span class="btn__label">{children()}</span>
        {trail}
    };
    match href {
        Some(href) => {
            // A disabled link is not a link: drop the href so it can't be followed.
            let off = disabled.get_untracked().unwrap_or(false);
            Either::Left(view! {
                <a
                    class=cls
                    href=(!off).then_some(href)
                    aria-disabled=off.then_some("true")
                    data-press=""
                >
                    {inner}
                </a>
            })
        }
        None => {
            let t = if button_type.is_empty() {
                "button"
            } else {
                button_type
            };
            Either::Right(view! {
                <button
                    class=cls
                    type=t
                    disabled=move || disabled.get().unwrap_or(false) || busy()
                    aria-busy=move || busy().then_some("true")
                    data-press=""
                >
                    {inner}
                </button>
            })
        }
    }
}

/// A round, bordered icon button: 36px to the eye, 44px to the finger (Madar's header buttons).
///
/// `kind`: `Outline` (default: surface disc with a hairline), `Primary` (tenant fill), `Ghost`
/// (no disc until hovered). Children render inside after the icon (e.g. a count badge,
/// `<span class="icon-btn__badge">3</span>`).
#[component]
pub fn IconButton(
    /// Lucide icon name.
    icon: &'static str,
    /// The accessible name (`aria-label`); also the tooltip.
    #[prop(into)]
    label: TextProp,
    #[prop(optional, into)] href: Option<String>,
    #[prop(optional)] kind: Option<ButtonKind>,
    #[prop(optional)] button_type: &'static str,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(optional, into)] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let kind_class = match kind {
        Some(ButtonKind::Primary) => "icon-btn--primary",
        Some(ButtonKind::Ghost) => "icon-btn--ghost",
        Some(ButtonKind::Danger) => "icon-btn--danger",
        _ => "icon-btn--outline",
    };
    let cls = if class.is_empty() {
        format!("icon-btn {kind_class}")
    } else {
        format!("icon-btn {kind_class} {class}")
    };
    let icon_class = if directional(icon) { "flip-rtl" } else { "" };
    let title = label.clone();
    let inner = view! {
        <Icon name=icon size=20 class=icon_class/>
        {children.map(|c| c())}
    };
    match href {
        Some(href) => Either::Left(view! {
            <a class=cls href=href aria-label=label title=title data-press="">
                {inner}
            </a>
        }),
        None => {
            let t = if button_type.is_empty() {
                "button"
            } else {
                button_type
            };
            Either::Right(view! {
                <button
                    class=cls
                    type=t
                    aria-label=label
                    title=title
                    disabled=move || disabled.get().unwrap_or(false)
                    data-press=""
                >
                    {inner}
                </button>
            })
        }
    }
}
