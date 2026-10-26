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

/// Lines of text: each line box is exactly one line of the text it stands for (`1lh` at the
/// wrapper's font size and line height), the last line shorter.
#[component]
pub fn SkeletonText(
    /// Number of lines (default 2).
    #[prop(optional)]
    lines: Option<u8>,
    /// Give it the class of the text it replaces (font size and line height carry over).
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    let n = lines.unwrap_or(2).max(1);
    let cls = if class.is_empty() {
        "skeleton-text".to_owned()
    } else {
        format!("skeleton-text {class}")
    };
    view! {
        <span class=cls aria-hidden="true">
            {(0..n)
                .map(|_| {
                    view! {
                        <span class="skeleton-text__line">
                            <span class="skeleton skeleton--text"></span>
                        </span>
                    }
                })
                .collect_view()}
        </span>
    }
}
