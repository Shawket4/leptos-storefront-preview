//! Responsive images with no layout shift.
//!
//! The box reserves the image's aspect ratio before a byte arrives; photos show the 20px LQIP
//! (blurred) over their dominant colour until the WebP (`srcset` ladder) paints over it, and
//! transparent packshots rest on the flat well. The image is never hidden while it loads: no
//! opacity gate waits on the inline `onload` (pages the islands router patches in come from
//! DOMParser, where inline handlers may never fire); `onload` only lets CSS drop the LQIP.

use super::icons::Icon;
use crate::media::Img;
use crate::prefs::Lang;
use leptos::either::Either;
use leptos::prelude::*;

#[component]
pub fn Picture(
    img: Img,
    /// The `sizes` attribute: how wide the box renders at each breakpoint.
    sizes: &'static str,
    /// Above-the-fold hero or LCP image: load eagerly with high priority.
    #[prop(optional)]
    priority: bool,
    #[prop(optional, into)] class: String,
    /// `view-transition-name` for a card → product morph. Must be unique on the page: use it on
    /// a page's main image (product cards name theirs only when clicked, see `ProductCard`).
    #[prop(optional)]
    vt_name: Option<String>,
    lang: Lang,
    /// Override the box's ratio (width / height); the image then covers it.
    #[prop(optional)]
    aspect: Option<f32>,
) -> impl IntoView {
    let ratio = aspect.unwrap_or(img.aspect).max(0.1);
    let mut style = format!(
        "aspect-ratio:{ratio:.4};--img-color:{}",
        sanitize_color(&img.color)
    );
    if let Some(name) = vt_name.as_deref().filter(|n| is_ident(n)) {
        style.push_str(&format!(
            ";view-transition-name:{name};view-transition-class:product-media"
        ));
    }
    let mut cls = String::from("pic");
    cls.push_str(if img.is_missing() {
        " pic--missing"
    } else if img.alpha {
        " pic--alpha"
    } else {
        " pic--photo"
    });
    if aspect.is_some() {
        cls.push_str(" pic--cover");
    }
    if !class.is_empty() {
        cls.push(' ');
        cls.push_str(&class);
    }

    if img.is_missing() {
        return Either::Left(view! {
            <div class=cls style=style role="img" aria-label=img.alt(lang).to_owned()>
                <Icon name="image" size=28 class="pic__missing-icon"/>
            </div>
        });
    }

    if !img.lqip.is_empty()
        && img.lqip.starts_with("data:image/")
        && !img.lqip.contains(['"', '\'', ')', '('])
    {
        style.push_str(&format!(";--lqip:url({})", img.lqip));
    }
    let largest = img.widths.iter().copied().max().unwrap_or(640);
    let height = (largest as f32 / img.aspect.max(0.1)).round() as u32;
    let alt = img.alt(lang).to_owned();
    let srcset = img.srcset();
    let src = img.src_near(640);
    view! {
        <div class=cls style=style>
            <img
                src=src
                srcset=srcset
                sizes=sizes
                width=largest
                height=height
                alt=alt
                loading=if priority { "eager" } else { "lazy" }
                decoding=(!priority).then_some("async")
                fetchpriority=priority.then_some("high")
                onload="this.dataset.loaded=''"
            />
        </div>
    }
    .pipe(Either::Right)
}

/// Only `#rgb`/`#rrggbb` reach an inline style.
fn sanitize_color(c: &str) -> &str {
    let ok = c.starts_with('#')
        && matches!(c.len(), 4 | 7)
        && c[1..].chars().all(|ch| ch.is_ascii_hexdigit());
    if ok { c } else { "transparent" }
}

/// A CSS custom ident (what `view-transition-name` accepts): letters, digits, `-`, `_`.
fn is_ident(s: &str) -> bool {
    !s.is_empty()
        && !s.starts_with(|c: char| c.is_ascii_digit())
        && s.chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

trait Pipe: Sized {
    fn pipe<R>(self, f: impl FnOnce(Self) -> R) -> R {
        f(self)
    }
}
impl<T> Pipe for T {}
