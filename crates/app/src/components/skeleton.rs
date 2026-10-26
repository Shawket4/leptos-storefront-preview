//! Loading placeholders: solid blocks that pulse, sized exactly like what replaces them, so data
//! landing moves nothing. All `aria-hidden`; announce loading with the region's own
//! `aria-busy` if it matters.

use leptos::prelude::*;

/// One block. Size it with `width`/`height` (any CSS length) or with a class.
#[component]
pub fn Skeleton(
    #[prop(optional)] width: Option<&'static str>,
    #[prop(optional)] height: Option<&'static str>,
    /// A circle (avatars, icon discs).
    #[prop(optional)]
    circle: bool,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let mut style = String::new();
    if let Some(w) = width {
        style.push_str(&format!("inline-size:{w};"));
    }
    if let Some(h) = height {
        style.push_str(&format!("block-size:{h};"));
    }
    let cls = format!(
        "skeleton{}{}",
        if circle { " skeleton--circle" } else { "" },
        if class.is_empty() {
            String::new()
        } else {
            format!(" {class}")
        }
    );
    view! { <span class=cls style=(!style.is_empty()).then_some(style) aria-hidden="true"></span> }
}

/// Lines of text, sized from the text they stand for: give it that text's class (`t-h2`,
/// `pcard__name`, …) and each line box is exactly one of its lines (`1lh` at that font size and
/// line height), in either script. The last of several lines is shorter (62%); `last_width`
/// sets the last line's width (of a single line too: a title).
#[component]
pub fn SkeletonLines(
    lines: u8,
    #[prop(optional, into)] class: String,
    /// The last line's width (any CSS length or percentage).
    #[prop(optional)]
    last_width: Option<&'static str>,
) -> impl IntoView {
    let n = lines.max(1);
    let cls = if class.is_empty() {
        "skeleton-text".to_owned()
    } else {
        format!("skeleton-text {class}")
    };
    view! {
        <span class=cls aria-hidden="true">
            {(0..n)
                .map(|i| {
                    let style = last_width
                        .filter(|_| i + 1 == n)
                        .map(|w| format!("inline-size:{w}"));
                    view! {
                        <span class="skeleton-text__line">
                            <span class="skeleton skeleton--text" style=style></span>
                        </span>
                    }
                })
                .collect_view()}
        </span>
    }
}

/// [`SkeletonLines`] under its first name: 2 lines unless told otherwise.
#[component]
pub fn SkeletonText(
    #[prop(optional)] lines: Option<u8>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! { <SkeletonLines lines=lines.unwrap_or(2) class=class/> }
}

/// A media box with its ratio reserved (width / height), like the `Picture` it stands for.
#[component]
pub fn SkeletonMedia(
    /// Width / height (1.0 square packshots, 21/9 heroes, 4/3 tiles).
    aspect: f32,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let cls = if class.is_empty() {
        "skeleton skeleton-media".to_owned()
    } else {
        format!("skeleton skeleton-media {class}")
    };
    view! { <span class=cls style=format!("aspect-ratio:{:.4}", aspect.max(0.1)) aria-hidden="true"></span> }
}

/// A form field: the label line, the 52px box (taller for a text area), and optionally the hint
/// line — the same box as `form::TextInput` / `TextArea`.
#[component]
pub fn SkeletonField(
    /// A text area instead of a single line.
    #[prop(optional)]
    multiline: bool,
    #[prop(optional)] hint: bool,
) -> impl IntoView {
    let cls = if multiline {
        "skeleton skeleton-field__box skeleton-field__box--area"
    } else {
        "skeleton skeleton-field__box"
    };
    view! {
        <div class="field skeleton-field" aria-hidden="true">
            <span class="field__label"><span class="skeleton-text__line"><span class="skeleton skeleton--text" style="inline-size:7rem"></span></span></span>
            <span class=cls></span>
            {hint.then(|| view! {
                <span class="field__hint"><span class="skeleton-text__line"><span class="skeleton skeleton--text" style="inline-size:70%"></span></span></span>
            })}
        </div>
    }
}

/// A rail of skeleton cards: the same track, item widths and card box as `Rail` +
/// `ProductCard`, so the real rail lands without a pixel moving. Give it the `class` the real
/// `Rail` gets (it may set the column count).
#[component]
pub fn SkeletonRail(
    /// Cards (default 6: enough to fill the widest rail).
    #[prop(optional)]
    count: Option<usize>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let cls = if class.is_empty() {
        "rail".to_owned()
    } else {
        format!("rail {class}")
    };
    view! {
        <div class=cls data-at="none" aria-hidden="true">
            <div class="rail__track">
                {(0..count.unwrap_or(6))
                    .map(|_| view! { <super::product_card::ProductCardSkeleton/> })
                    .collect_view()}
            </div>
        </div>
    }
}
