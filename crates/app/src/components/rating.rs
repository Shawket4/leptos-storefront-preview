//! Star ratings: five lucide stars with a precise partial fill (clipped from the reading start, so
//! it fills right-to-left in Arabic), the figure in mono, the review count. One `role="img"` with
//! the whole sentence as its name.

use super::icons::Icon;
use crate::format::{self, plural_ar, plural_en};
use crate::prefs::{Lang, Numerals};
use leptos::prelude::*;

/// "Rated 4.6 out of 5 from 312 reviews" / "التقييم 4.6 من 5 من 312 تقييم".
pub fn rating_label(value: f32, count: u32, lang: Lang, numerals: Numerals) -> String {
    let v = format::dec1(value, numerals);
    let five = format::count(5, numerals);
    let n = format::int(count as i64, numerals);
    match lang {
        Lang::En => format!(
            "Rated {v} out of {five} from {n} {}",
            plural_en(count, "review", "reviews")
        ),
        Lang::Ar => match count {
            1 => format!("التقييم {v} من {five}، تقييم واحد"),
            2 => format!("التقييم {v} من {five}، تقييمان"),
            _ => format!(
                "التقييم {v} من {five}، {n} {}",
                plural_ar(count, "تقييم", "تقييمان", "تقييمات", "تقييمًا")
            ),
        },
    }
}

/// Width of the filled layer in `em`: whole stars plus the gaps between them, then the fraction
/// of the next star (stars are 1em with a 0.125em gap, see rating.css).
fn fill_em(value: f32) -> f32 {
    let v = value.clamp(0.0, 5.0);
    let whole = v.floor();
    whole * 1.125 + (v - whole)
}

#[component]
pub fn Rating(
    value: f32,
    count: u32,
    lang: Lang,
    numerals: Numerals,
    /// One star, the figure and the count: for cards.
    #[prop(optional)]
    compact: bool,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let cls = format!(
        "rating{}{}{}",
        if compact { " rating--compact" } else { "" },
        if count == 0 { " rating--none" } else { "" },
        if class.is_empty() {
            String::new()
        } else {
            format!(" {class}")
        }
    );
    if count == 0 {
        let none = crate::prefs::tr(lang, "No reviews yet", "لسه مفيش تقييمات");
        // Same box as a rated row, so cards and lists keep their rhythm.
        return view! {
            <span class=cls>
                <span class="rating__none">{(!compact).then_some(none)}</span>
            </span>
        }
        .into_any();
    }
    let label = rating_label(value, count, lang, numerals);
    let figure = format::dec1(value, numerals);
    let n = format::int(count as i64, numerals);
    let stars = if compact {
        view! {
            <span class="rating__stars rating__stars--one" aria-hidden="true">
                <Icon name="star" size=14 class="rating__star is-on"/>
            </span>
        }
        .into_any()
    } else {
        let fill = format!("--fill: {:.3}em", fill_em(value));
        view! {
            <span class="rating__stars" aria-hidden="true" style=fill>
                <span class="rating__row rating__row--base">
                    <Icon name="star" size=16 class="rating__star"/>
                    <Icon name="star" size=16 class="rating__star"/>
                    <Icon name="star" size=16 class="rating__star"/>
                    <Icon name="star" size=16 class="rating__star"/>
                    <Icon name="star" size=16 class="rating__star"/>
                </span>
                <span class="rating__row rating__row--fill">
                    <Icon name="star" size=16 class="rating__star is-on"/>
                    <Icon name="star" size=16 class="rating__star is-on"/>
                    <Icon name="star" size=16 class="rating__star is-on"/>
                    <Icon name="star" size=16 class="rating__star is-on"/>
                    <Icon name="star" size=16 class="rating__star is-on"/>
                </span>
            </span>
        }
        .into_any()
    };
    view! {
        <span class=cls role="img" aria-label=label>
            {stars}
            <span class="rating__value mono" aria-hidden="true">{figure}</span>
            <span class="rating__count" aria-hidden="true">"(" {n} ")"</span>
        </span>
    }
    .into_any()
}
