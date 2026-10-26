//! Numbers, money and times, in both languages and both numeral systems.
//!
//! Origin: Madar's one money shape (`MadarDashboard/src/lib/format.ts`, mirroring
//! `madar/rust-core/crates/madar-core/src/display.rs`):
//!
//! ```text
//! en  EGP 1,234.50     −EGP 50.00
//! ar  <LRI>1,234.50<PDI> ج.م   <LRI>−50.00<PDI> ج.م
//! ```
//!
//! Two decimals, thousands grouped, a true minus (U+2212), and in Arabic the figure is wrapped in
//! LRI…PDI (U+2066…U+2069) so the bidi algorithm never separates sign, digits and label. Madar forces Western
//! digits; the storefront adds the Arabic-Indic option (٠١٢٣٤٥٦٧٨٩ with ٬ and ٫ separators).

use crate::prefs::{Lang, Numerals};
use domain::Piastres;

pub const MINUS: char = '\u{2212}';
pub const LRI: char = '\u{2066}';
pub const PDI: char = '\u{2069}';
const AR_THOUSANDS: char = '\u{066C}'; // ٬
const AR_DECIMAL: char = '\u{066B}'; // ٫

/// Wrap a figure so it stays left-to-right inside RTL text.
pub fn ltr(s: &str) -> String {
    format!("{LRI}{s}{PDI}")
}

/// Rewrite ASCII digits (and `,` `.` separators) for the numeral system.
pub fn digits(s: &str, n: Numerals) -> String {
    match n {
        Numerals::Latn => s.to_owned(),
        Numerals::Arab => s
            .chars()
            .map(|c| match c {
                '0'..='9' => char::from_u32(0x0660 + (c as u32 - '0' as u32)).unwrap_or(c),
                ',' => AR_THOUSANDS,
                '.' => AR_DECIMAL,
                c => c,
            })
            .collect(),
    }
}

/// `1234567` → `1,234,567` (Western) or `١٬٢٣٤٬٥٦٧` (Arabic-Indic).
pub fn int(n: i64, numerals: Numerals) -> String {
    digits(&group(n.unsigned_abs()), numerals).pipe(|s| if n < 0 { format!("{MINUS}{s}") } else { s })
}

/// A count as plain digits, no grouping (quantities, ratings counts under 10k read better).
pub fn count(n: u32, numerals: Numerals) -> String {
    digits(&n.to_string(), numerals)
}

/// One decimal place, e.g. ratings `4.6` / `٤٫٦`.
pub fn dec1(x: f32, numerals: Numerals) -> String {
    digits(&format!("{x:.1}"), numerals)
}

fn group(mut n: u64) -> String {
    if n == 0 {
        return "0".into();
    }
    let mut parts = Vec::new();
    while n > 0 {
        parts.push(n % 1000);
        n /= 1000;
    }
    let mut out = parts.pop().map(|p| p.to_string()).unwrap_or_default();
    while let Some(p) = parts.pop() {
        out.push_str(&format!(",{p:03}"));
    }
    out
}

/// The currency label in a language: `EGP` / `ج.م`.
pub fn currency_label(lang: Lang) -> &'static str {
    match lang {
        Lang::En => "EGP",
        Lang::Ar => "ج.م",
    }
}

/// Money split into parts, so the price component can style the decimals and label separately.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MoneyParts {
    /// `""` or `"−"`.
    pub sign: &'static str,
    /// Grouped whole pounds in the chosen numerals, e.g. `1,234` / `١٬٢٣٤`.
    pub whole: String,
    /// Decimal separator + piastres, e.g. `.50` / `٫٥٠`.
    pub fraction: String,
    pub label: &'static str,
    pub lang: Lang,
}

impl MoneyParts {
    pub fn new(p: Piastres, lang: Lang, numerals: Numerals) -> Self {
        let sign = if p.0 < 0 { "\u{2212}" } else { "" };
        let abs = p.0.unsigned_abs();
        MoneyParts {
            sign,
            whole: digits(&group(abs / 100), numerals),
            fraction: digits(&format!(".{:02}", abs % 100), numerals),
            label: currency_label(lang),
            lang,
        }
    }

    /// The figure only (sign + whole + fraction).
    pub fn figure(&self) -> String {
        format!("{}{}{}", self.sign, self.whole, self.fraction)
    }
}

/// Full money string in Madar's shape. For text contexts (aria-label, `<title>`, toasts).
pub fn money(p: Piastres, lang: Lang, numerals: Numerals) -> String {
    let m = MoneyParts::new(p, lang, numerals);
    match lang {
        Lang::En => format!("{}{} {}{}", m.sign, m.label, m.whole, m.fraction),
        Lang::Ar => format!("{} {}", ltr(&m.figure()), m.label),
    }
}

/// Money for marketing copy ("Free delivery over EGP 1,500"): drops `.00` on whole pounds. Prices
/// themselves always use [`money`] (two decimals, Madar's shape).
pub fn money_short(p: Piastres, lang: Lang, numerals: Numerals) -> String {
    if p.cents() != 0 {
        return money(p, lang, numerals);
    }
    let m = MoneyParts::new(p, lang, numerals);
    match lang {
        Lang::En => format!("{}{} {}", m.sign, m.label, m.whole),
        Lang::Ar => format!("{} {}", ltr(&format!("{}{}", m.sign, m.whole)), m.label),
    }
}

/// Machine price for JSON-LD / OpenGraph: `1234.50`.
pub fn money_plain(p: Piastres) -> String {
    let abs = p.0.unsigned_abs();
    let sign = if p.0 < 0 { "-" } else { "" };
    format!("{sign}{}.{:02}", abs / 100, abs % 100)
}

/// Arabic plural forms (CLDR: zero, one, two, few 3–10, many 11–99, other). English: one/other.
/// `forms = [one, two, few, many]` for Arabic; English uses `[one, other]` via [`plural_en`].
pub fn plural_ar(n: u32, one: &'static str, two: &'static str, few: &'static str, many: &'static str) -> &'static str {
    match n % 100 {
        _ if n == 1 => one,
        _ if n == 2 => two,
        3..=10 => few,
        11..=99 => many,
        _ => one, // 100, 200, … take the singular-like "other" form: ١٠٠ منتج
    }
}

pub fn plural_en(n: u32, one: &'static str, other: &'static str) -> &'static str {
    if n == 1 { one } else { other }
}

/// "3 items" / "٣ منتجات" — count + noun with the right plural form.
pub fn items(n: u32, lang: Lang, numerals: Numerals) -> String {
    let c = count(n, numerals);
    match lang {
        Lang::En => format!("{c} {}", plural_en(n, "item", "items")),
        Lang::Ar => match n {
            1 => "منتج واحد".into(),
            2 => "منتجان".into(),
            _ => format!("{c} {}", plural_ar(n, "منتج", "منتجان", "منتجات", "منتجًا")),
        },
    }
}

/// Cairo's UTC offset in hours at a UTC instant. Egypt observes DST again since 2023: from the
/// last Friday of April 00:00 to the last Thursday of October 24:00 local time.
pub fn cairo_offset_hours(utc_ms: i64) -> i64 {
    let (y, m, d) = civil_from_days((utc_ms + 2 * 3_600_000).div_euclid(86_400_000));
    let start = last_weekday_of(y, 4, 5); // Friday
    let end = last_weekday_of(y, 10, 4); // Thursday
    let doy = |mm: u32, dd: u32| days_from_civil(y, mm, dd);
    let today = doy(m, d);
    if today >= doy(4, start) && today <= doy(10, end) { 3 } else { 2 }
}

/// `06:02 PM` / `06:02 م` in Cairo time (Madar shows 12-hour time in both languages).
pub fn time_cairo(utc_ms: i64, lang: Lang, numerals: Numerals) -> String {
    let local = utc_ms + cairo_offset_hours(utc_ms) * 3_600_000;
    let mins = local.div_euclid(60_000).rem_euclid(24 * 60);
    let (h24, m) = (mins / 60, mins % 60);
    let h12 = if h24 % 12 == 0 { 12 } else { h24 % 12 };
    let pm = h24 >= 12;
    let hm = digits(&format!("{h12:02}:{m:02}"), numerals);
    match lang {
        Lang::En => format!("{hm} {}", if pm { "PM" } else { "AM" }),
        Lang::Ar => format!("{} {}", ltr(&hm), if pm { "م" } else { "ص" }),
    }
}

/// Weekday + day + month in Cairo time, e.g. `Tue 27 Oct` / `الثلاثاء ٢٧ أكتوبر`.
pub fn date_cairo(utc_ms: i64, lang: Lang, numerals: Numerals) -> String {
    let local = utc_ms + cairo_offset_hours(utc_ms) * 3_600_000;
    let days = local.div_euclid(86_400_000);
    let (_, m, d) = civil_from_days(days);
    let wd = ((days + 4).rem_euclid(7)) as usize; // 1970-01-01 was a Thursday
    const WD_EN: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    const WD_AR: [&str; 7] = ["الأحد", "الإثنين", "الثلاثاء", "الأربعاء", "الخميس", "الجمعة", "السبت"];
    const MO_EN: [&str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    // Egyptian month names (Madar's Arabic register uses these, not the Levantine ones).
    const MO_AR: [&str; 12] = [
        "يناير", "فبراير", "مارس", "أبريل", "مايو", "يونيو", "يوليو", "أغسطس", "سبتمبر", "أكتوبر", "نوفمبر", "ديسمبر",
    ];
    let dd = digits(&d.to_string(), numerals);
    match lang {
        Lang::En => format!("{} {dd} {}", WD_EN[wd], MO_EN[m as usize - 1]),
        Lang::Ar => format!("{} {dd} {}", WD_AR[wd], MO_AR[m as usize - 1]),
    }
}

// Howard Hinnant's civil-date algorithms (days since 1970-01-01 <-> y/m/d), no chrono in WASM.
pub fn days_from_civil(y: i64, m: u32, d: u32) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let mp = (m as i64 + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d as i64 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

pub fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

/// Day-of-month of the last `weekday` (0 = Sunday) in a month.
fn last_weekday_of(y: i64, m: u32, weekday: i64) -> u32 {
    let next_first = if m == 12 { days_from_civil(y + 1, 1, 1) } else { days_from_civil(y, m + 1, 1) };
    let last = next_first - 1;
    let wd = (last + 4).rem_euclid(7);
    let back = (wd - weekday).rem_euclid(7);
    civil_from_days(last - back).2
}

trait Pipe: Sized {
    fn pipe<R>(self, f: impl FnOnce(Self) -> R) -> R {
        f(self)
    }
}
impl<T> Pipe for T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn money_shapes() {
        let p = Piastres(123_450);
        assert_eq!(money(p, Lang::En, Numerals::Latn), "EGP 1,234.50");
        assert_eq!(money(p, Lang::Ar, Numerals::Latn), "\u{2066}1,234.50\u{2069} ج.م");
        assert_eq!(money(Piastres(-5000), Lang::En, Numerals::Latn), "\u{2212}EGP 50.00");
        assert_eq!(money(p, Lang::Ar, Numerals::Arab), "\u{2066}١٬٢٣٤٫٥٠\u{2069} ج.م");
        assert_eq!(money_plain(p), "1234.50");
    }

    #[test]
    fn cairo_dst() {
        // 2026-10-26 12:00 UTC: DST still on (ends last Thursday of Oct = 29th) → 15:00.
        let t = days_from_civil(2026, 10, 26) * 86_400_000 + 12 * 3_600_000;
        assert_eq!(cairo_offset_hours(t), 3);
        assert_eq!(time_cairo(t, Lang::En, Numerals::Latn), "03:00 PM");
        let jan = days_from_civil(2026, 1, 10) * 86_400_000;
        assert_eq!(cairo_offset_hours(jan), 2);
    }
}
