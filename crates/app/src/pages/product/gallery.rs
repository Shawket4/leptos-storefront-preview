//! The product gallery's markup. Without JS it is a native scroll-snap strip (swipe, momentum and
//! RTL come from the browser) with anchor thumbnails. With JS (`data-ts="gallery"`,
//! `features/product.ts`) the gesture is taken over on the design system's springs: the strip
//! follows the finger 1:1, the release velocity decides next/previous (a slow release lands where
//! its momentum projects), the ends rubber-band, and every movement starts from the live position.
//! Desktop adds arrows, thumbnails and hover zoom-to-cursor; everywhere a tap opens the fullscreen
//! viewer (`<dialog class="pv">`) with double-tap and pinch zoom and pan. A variant whose photo
//! differs moves the strip to it (an event from the purchase panel, no request).

use super::model::{STAGE_SIZES, text};
use crate::components::button::IconButton;
use crate::components::icons::Icon;
use crate::components::image::Picture;
use crate::format;
use crate::media::Img;
use crate::prefs::{Lang, Numerals, tr};
use domain::L10n;
use leptos::prelude::*;

const THUMB_SIZES: &str = "72px";

/// "2 of 5" / "٢ من ٥".
fn slide_label(i: usize, n: usize, lang: Lang, num: Numerals) -> String {
    let (a, b) = (
        format::count(i as u32 + 1, num),
        format::count(n as u32, num),
    );
    match lang {
        Lang::En => format!("{a} of {b}"),
        Lang::Ar => format!("{a} من {b}"),
    }
}

#[component]
pub fn Gallery(
    product: u32,
    name: L10n,
    images: Vec<Img>,
    start: usize,
    lang: Lang,
    num: Numerals,
) -> impl IntoView {
    // A product without photos still gets one (neutral) slide, so the layout holds.
    let images = if images.is_empty() {
        vec![Img::missing("")]
    } else {
        images
    };
    let n = images.len();
    let start = start.min(n - 1);
    let name = text(&name, lang);
    let multi = n > 1;
    let count = |i: usize| {
        format!(
            "{} / {}",
            format::count(i as u32 + 1, num),
            format::count(n as u32, num)
        )
    };
    let photo = tr(lang, "Photo", "صورة");
    let unavailable = tr(lang, "Photo unavailable", "الصورة غير متاحة");

    let slides = images
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, img)| {
            let cls = if i == start { "pg__slide is-start" } else { "pg__slide" };
            view! {
                <div
                    class=cls
                    id=format!("pg{product}-{i}")
                    data-i=i
                    role="group"
                    aria-roledescription=tr(lang, "photo", "صورة")
                    aria-label=slide_label(i, n, lang, num)
                >
                    <span class="pg__fallback" aria-hidden="true">
                        <Icon name="image" size=28/>
                        <span>{unavailable}</span>
                    </span>
                    {if i == start {
                        // The card → page morph lands on the photo the page opens with.
                        view! { <Picture img=img sizes=STAGE_SIZES priority=true vt_name=format!("p{product}") lang=lang/> }.into_any()
                    } else {
                        view! { <Picture img=img sizes=STAGE_SIZES lang=lang/> }.into_any()
                    }}
                </div>
            }
        })
        .collect_view();

    let thumbs = multi.then(|| {
        let items = images
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, img)| {
                view! {
                    <a
                        class="pg__thumb"
                        href=format!("#pg{product}-{i}")
                        data-i=i
                        aria-label=format!("{photo} {}", slide_label(i, n, lang, num))
                        aria-current=(i == start).then_some("true")
                    >
                        <Picture img=img sizes=THUMB_SIZES lang=lang/>
                        <span class="pg__fallback pg__fallback--sm" aria-hidden="true"><Icon name="image" size=16/></span>
                    </a>
                }
            })
            .collect_view();
        view! { <div class="pg__thumbs">{items}</div> }
    });

    let dots = multi.then(|| {
        view! {
            <div class="pg__dots" aria-hidden="true">
                {(0..n).map(|i| view! { <span class=if i == start { "pg__dot is-on" } else { "pg__dot" }></span> }).collect_view()}
            </div>
        }
    });

    let viewer_slides = images
        .iter()
        .enumerate()
        .map(|(i, img)| {
            let missing = img.is_missing();
            let (src, srcset, alt) = (img.largest(), img.srcset(), img.alt(lang).to_owned());
            view! {
                <div class="pv__slide" data-i=i>
                    <span class="pg__fallback" aria-hidden="true">
                        <Icon name="image" size=32/>
                        <span>{unavailable}</span>
                    </span>
                    <div class="pv__zoom">
                        // Lazy inside a closed dialog: fetched only once the viewer shows it.
                        {(!missing).then(|| view! {
                            <img class="pv__img" src=src srcset=srcset sizes="100vw" alt=alt
                                loading="lazy" decoding="async" draggable="false"/>
                        })}
                    </div>
                </div>
            }
        })
        .collect_view();

    let label = format!("{} — {name}", tr(lang, "Photos", "الصور"));
    let viewer_label = label.clone();
    view! {
        <section class="pg" data-ts="gallery" data-start=start data-count=n
            aria-roledescription=tr(lang, "gallery", "معرض صور") aria-label=label>
            <div class="pg__stage">
                <div
                    class="pg__viewport"
                    tabindex="0"
                    aria-label=tr(
                        lang,
                        "Product photos. Use arrow keys to browse, Enter to enlarge.",
                        "صور المنتج. استخدم الأسهم للتنقل وEnter للتكبير.",
                    )
                >
                    <div class="pg__track">{slides}</div>
                </div>
                {multi.then(|| view! {
                    <IconButton
                        icon="chevron-left"
                        label=tr(lang, "Previous photo", "الصورة السابقة")
                        class="pg__nav pg__nav--prev"
                        disabled={start == 0}
                        attr:data-step="-1"
                    />
                    <IconButton
                        icon="chevron-right"
                        label=tr(lang, "Next photo", "الصورة التالية")
                        class="pg__nav pg__nav--next"
                        disabled={start + 1 >= n}
                        attr:data-step="1"
                    />
                    // `dir` on an inner span: on the positioned element it would flip its inset.
                    <p class="pg__count mono" aria-hidden="true">
                        <span dir="ltr" data-count="">{count(start)}</span>
                    </p>
                })}
                <IconButton
                    icon="zoom-in"
                    label=tr(lang, "View full screen", "عرض بملء الشاشة")
                    class="pg__expand"
                    attr:data-open=""
                />
                <p class="sr-only" aria-live="polite" data-live=""></p>
            </div>
            {dots}
            {thumbs}
            <dialog class="pv" aria-label=viewer_label>
                <div class="pv__bar">
                    <p class="pv__count mono">
                        <span dir="ltr" data-count="">{multi.then(|| count(start))}</span>
                    </p>
                    <IconButton icon="x" label=tr(lang, "Close", "إغلاق") class="pv__close" attr:data-close=""/>
                </div>
                <div class="pv__viewport">
                    <div class="pv__track">{viewer_slides}</div>
                </div>
                <p class="pv__hint" aria-hidden="true">
                    <Icon name="zoom-in" size=14/>
                    <span class="pv__hint-touch">
                        {tr(lang, "Double-tap or pinch to zoom", "دوس مرتين أو قرّب بصباعين للتكبير")}
                    </span>
                    <span class="pv__hint-mouse">
                        {tr(lang, "Double-click to zoom · drag to move", "دوس مرتين للتكبير · اسحب للتحريك")}
                    </span>
                </p>
            </dialog>
        </section>
    }
}
