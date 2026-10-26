//! Checkout validation, written once in plain Rust and compiled twice: into the server functions
//! (the authority — a request that skips the browser still meets these rules) and into the address
//! island (instant feedback on blur and submit, in the visitor's language). Same functions, same
//! messages, no JSON schema or JavaScript twin to drift.

use crate::format;
use crate::prefs::{Lang, Numerals};
use serde::{Deserialize, Serialize};

/// What the address form submits (field names are the form's `name`s and the server function's
/// arguments).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AddressInput {
    pub name: String,
    pub phone: String,
    pub city: String,
    pub area: String,
    pub street: String,
    pub building: String,
    pub floor: String,
    pub apartment: String,
    pub landmark: String,
    pub notes: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Field {
    Name,
    Phone,
    City,
    Area,
    Street,
    Building,
    Floor,
    Apartment,
    Landmark,
    Notes,
}

impl Field {
    /// Form order: the first invalid field in this order gets focus.
    pub const ALL: [Field; 10] = [
        Field::Name,
        Field::Phone,
        Field::City,
        Field::Area,
        Field::Street,
        Field::Building,
        Field::Floor,
        Field::Apartment,
        Field::Landmark,
        Field::Notes,
    ];

    /// The input's `name` (and the server function argument).
    pub fn name(self) -> &'static str {
        match self {
            Field::Name => "name",
            Field::Phone => "phone",
            Field::City => "city",
            Field::Area => "area",
            Field::Street => "street",
            Field::Building => "building",
            Field::Floor => "floor",
            Field::Apartment => "apartment",
            Field::Landmark => "landmark",
            Field::Notes => "notes",
        }
    }

    pub fn id(self) -> String {
        format!("co-{}", self.name())
    }

    pub fn error_id(self) -> String {
        format!("co-{}-err", self.name())
    }

    pub fn max_len(self) -> usize {
        match self {
            Field::Name => 60,
            Field::Phone => 20,
            Field::City | Field::Area => 40,
            Field::Street | Field::Landmark => 120,
            Field::Building | Field::Floor | Field::Apartment => 12,
            Field::Notes => NOTES_MAX,
        }
    }

    pub fn required(self) -> bool {
        matches!(
            self,
            Field::Name
                | Field::Phone
                | Field::City
                | Field::Area
                | Field::Street
                | Field::Building
        )
    }

    pub fn value(self, a: &AddressInput) -> &str {
        match self {
            Field::Name => &a.name,
            Field::Phone => &a.phone,
            Field::City => &a.city,
            Field::Area => &a.area,
            Field::Street => &a.street,
            Field::Building => &a.building,
            Field::Floor => &a.floor,
            Field::Apartment => &a.apartment,
            Field::Landmark => &a.landmark,
            Field::Notes => &a.notes,
        }
    }

    pub fn set(self, a: &mut AddressInput, v: String) {
        match self {
            Field::Name => a.name = v,
            Field::Phone => a.phone = v,
            Field::City => a.city = v,
            Field::Area => a.area = v,
            Field::Street => a.street = v,
            Field::Building => a.building = v,
            Field::Floor => a.floor = v,
            Field::Apartment => a.apartment = v,
            Field::Landmark => a.landmark = v,
            Field::Notes => a.notes = v,
        }
    }
}

pub const NOTES_MAX: usize = 200;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Problem {
    Required,
    TooShort(u8),
    TooLong(u16),
    /// Not an Egyptian mobile number.
    Phone,
    /// Not one of the cities we deliver to.
    City,
    /// Not an area we know.
    Area,
    /// An area of another city.
    AreaCity,
    /// Letters expected (a name made of digits or symbols).
    Letters,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldError {
    pub field: Field,
    pub problem: Problem,
}

impl FieldError {
    /// The message next to the field: clear Modern Standard Arabic, like Madar's error copy.
    pub fn message(&self, lang: Lang, n: Numerals) -> String {
        let num = |x: u32| format::count(x, n);
        match (self.field, self.problem, lang) {
            (Field::Name, Problem::Required, Lang::En) => {
                "Enter the name of the person receiving the order.".into()
            }
            (Field::Name, Problem::Required, Lang::Ar) => "أدخل اسم مستلم الطلب.".into(),
            (Field::Name, Problem::TooShort(m), Lang::En) => {
                format!("Enter at least {} letters.", num(m as u32))
            }
            (Field::Name, Problem::TooShort(m), Lang::Ar) => {
                format!("أدخل {} أحرف على الأقل.", num(m as u32))
            }
            (Field::Name, Problem::Letters, Lang::En) => "Use letters for the name.".into(),
            (Field::Name, Problem::Letters, Lang::Ar) => "يجب أن يتكون الاسم من حروف.".into(),
            (Field::Phone, Problem::Required, Lang::En) => {
                "Enter a mobile number so the courier can reach you.".into()
            }
            (Field::Phone, Problem::Required, Lang::Ar) => {
                "أدخل رقم الهاتف المحمول ليتمكن المندوب من التواصل معك.".into()
            }
            (Field::Phone, Problem::Phone, Lang::En) => {
                "Enter an Egyptian mobile number: 11 digits starting with 010, 011, 012 or 015."
                    .into()
            }
            (Field::Phone, Problem::Phone, Lang::Ar) => {
                format!(
                    "أدخل رقم هاتف محمول مصريًا من {} رقمًا يبدأ بـ 010 أو 011 أو 012 أو 015.",
                    num(11)
                )
            }
            (Field::City, _, Lang::En) => "Choose the city we're delivering to.".into(),
            (Field::City, _, Lang::Ar) => "اختر مدينة التوصيل.".into(),
            (Field::Area, Problem::AreaCity, Lang::En) => {
                "This area isn't in the city you chose. Choose an area in that city.".into()
            }
            (Field::Area, Problem::AreaCity, Lang::Ar) => {
                "هذه المنطقة لا تتبع المدينة المختارة. اختر منطقة داخلها.".into()
            }
            (Field::Area, _, Lang::En) => "Choose your area.".into(),
            (Field::Area, _, Lang::Ar) => "اختر المنطقة.".into(),
            (Field::Street, Problem::Required, Lang::En) => "Enter the street name.".into(),
            (Field::Street, Problem::Required, Lang::Ar) => "أدخل اسم الشارع.".into(),
            (Field::Building, Problem::Required, Lang::En) => "Enter the building number.".into(),
            (Field::Building, Problem::Required, Lang::Ar) => "أدخل رقم العقار.".into(),
            (Field::Notes, Problem::TooLong(m), Lang::En) => {
                format!("Keep delivery notes under {} characters.", num(m as u32))
            }
            (Field::Notes, Problem::TooLong(m), Lang::Ar) => {
                format!("يجب ألا تزيد ملاحظات التوصيل على {} حرف.", num(m as u32))
            }
            (_, Problem::TooLong(m), Lang::En) => {
                format!("Use at most {} characters.", num(m as u32))
            }
            (_, Problem::TooLong(m), Lang::Ar) => {
                format!("يجب ألا يزيد على {} حرفًا.", num(m as u32))
            }
            (_, _, Lang::En) => "Check this field.".into(),
            (_, _, Lang::Ar) => "راجع هذا الحقل.".into(),
        }
    }
}

/// A city we deliver to, with its areas (`id`, English, Arabic).
pub struct City {
    pub id: &'static str,
    pub en: &'static str,
    pub ar: &'static str,
    pub areas: &'static [(&'static str, &'static str, &'static str)],
}

impl City {
    pub fn name(&self, lang: Lang) -> &'static str {
        if lang.is_rtl() { self.ar } else { self.en }
    }
}

pub const CITIES: [City; 3] = [
    City {
        id: "cairo",
        en: "Cairo",
        ar: "القاهرة",
        areas: &[
            ("nasr-city", "Nasr City", "مدينة نصر"),
            ("heliopolis", "Heliopolis", "مصر الجديدة"),
            ("new-cairo", "New Cairo", "القاهرة الجديدة"),
            ("maadi", "Maadi", "المعادي"),
            ("zamalek", "Zamalek", "الزمالك"),
            ("downtown", "Downtown", "وسط البلد"),
            ("garden-city", "Garden City", "جاردن سيتي"),
            ("mokattam", "Mokattam", "المقطم"),
            ("shubra", "Shubra", "شبرا"),
            ("abbassia", "Abbassia", "العباسية"),
            ("ain-shams", "Ain Shams", "عين شمس"),
            ("helwan", "Helwan", "حلوان"),
            ("rehab", "El Rehab", "الرحاب"),
            ("madinaty", "Madinaty", "مدينتي"),
            ("shorouk", "El Shorouk", "الشروق"),
        ],
    },
    City {
        id: "giza",
        en: "Giza",
        ar: "الجيزة",
        areas: &[
            ("dokki", "Dokki", "الدقي"),
            ("mohandessin", "Mohandessin", "المهندسين"),
            ("agouza", "Agouza", "العجوزة"),
            ("haram", "Haram", "الهرم"),
            ("faisal", "Faisal", "فيصل"),
            ("october", "6th of October", "السادس من أكتوبر"),
            ("sheikh-zayed", "Sheikh Zayed", "الشيخ زايد"),
            ("imbaba", "Imbaba", "إمبابة"),
            ("omraneya", "Omraneya", "العمرانية"),
            ("hadayek-ahram", "Hadayek El Ahram", "حدائق الأهرام"),
        ],
    },
    City {
        id: "alexandria",
        en: "Alexandria",
        ar: "الإسكندرية",
        areas: &[
            ("smouha", "Smouha", "سموحة"),
            ("sidi-gaber", "Sidi Gaber", "سيدي جابر"),
            ("stanley", "Stanley", "ستانلي"),
            ("roushdy", "Roushdy", "رشدي"),
            ("sporting", "Sporting", "سبورتنج"),
            ("gleem", "Gleem", "جليم"),
            ("louran", "Louran", "لوران"),
            ("sidi-bishr", "Sidi Bishr", "سيدي بشر"),
            ("miami", "Miami", "ميامي"),
            ("mandara", "Mandara", "المندرة"),
            ("montaza", "Montaza", "المنتزه"),
            ("raml-station", "Raml Station", "محطة الرمل"),
            ("agami", "Agami", "العجمي"),
        ],
    },
];

pub fn city(id: &str) -> Option<&'static City> {
    CITIES.iter().find(|c| c.id == id)
}

/// The city an area belongs to.
pub fn city_of_area(area: &str) -> Option<&'static City> {
    CITIES
        .iter()
        .find(|c| c.areas.iter().any(|(id, _, _)| *id == area))
}

pub fn area_name(area: &str, lang: Lang) -> Option<&'static str> {
    CITIES
        .iter()
        .flat_map(|c| c.areas.iter())
        .find(|(id, _, _)| *id == area)
        .map(|(_, en, ar)| if lang.is_rtl() { *ar } else { *en })
}

/// Digits as typed on an Egyptian phone (Arabic-Indic `٠١٢`, Persian `۰۱۲`) → ASCII; spaces,
/// dashes, dots and brackets dropped; `+20` / `0020` / `20` country prefix folded to the local `0`.
pub fn normalize_phone(raw: &str) -> String {
    let digits: String = raw
        .chars()
        .filter_map(|c| match c {
            '0'..='9' => Some(c),
            '\u{0660}'..='\u{0669}' => char::from_u32('0' as u32 + (c as u32 - 0x0660)),
            '\u{06F0}'..='\u{06F9}' => char::from_u32('0' as u32 + (c as u32 - 0x06F0)),
            _ => None,
        })
        .collect();
    if let Some(rest) = digits.strip_prefix("0020") {
        return format!("0{rest}");
    }
    if digits.len() == 12 && digits.starts_with("201") {
        return format!("0{}", &digits[2..]);
    }
    digits
}

/// `01[0125]` + 8 digits: Vodafone 010, Etisalat (e&) 011, Orange 012, WE 015.
pub fn is_egyptian_mobile(normalized: &str) -> bool {
    let b = normalized.as_bytes();
    b.len() == 11
        && b[0] == b'0'
        && b[1] == b'1'
        && matches!(b[2], b'0' | b'1' | b'2' | b'5')
        && b.iter().all(u8::is_ascii_digit)
}

/// `010 1234 5678`, for the review step and the order page.
pub fn pretty_phone(normalized: &str) -> String {
    if normalized.len() == 11 && normalized.is_ascii() {
        format!(
            "{} {} {}",
            &normalized[..3],
            &normalized[3..7],
            &normalized[7..]
        )
    } else {
        normalized.to_owned()
    }
}

/// Validate one field (the island calls this on blur; `validate` calls it for each field).
pub fn check(field: Field, a: &AddressInput) -> Option<Problem> {
    let raw = field.value(a);
    let v = raw.trim();
    if v.is_empty() {
        return field.required().then_some(Problem::Required);
    }
    if v.chars().count() > field.max_len() {
        return Some(Problem::TooLong(field.max_len() as u16));
    }
    match field {
        Field::Name => {
            let letters = v.chars().filter(|c| c.is_alphabetic()).count();
            if letters == 0 {
                Some(Problem::Letters)
            } else if letters < 2 {
                Some(Problem::TooShort(2))
            } else {
                None
            }
        }
        Field::Phone => (!is_egyptian_mobile(&normalize_phone(v))).then_some(Problem::Phone),
        Field::City => city(v).is_none().then_some(Problem::City),
        Field::Area => match (city(a.city.trim()), city_of_area(v)) {
            (_, None) => Some(Problem::Area),
            (Some(c), Some(of)) if c.id != of.id => Some(Problem::AreaCity),
            _ => None,
        },
        _ => None,
    }
}

/// Every problem, in form order.
pub fn validate(a: &AddressInput) -> Vec<FieldError> {
    Field::ALL
        .iter()
        .filter_map(|&field| check(field, a).map(|problem| FieldError { field, problem }))
        .collect()
}

/// The stored form of a valid address: trimmed, phone normalised.
pub fn clean(a: &AddressInput) -> AddressInput {
    AddressInput {
        name: a.name.trim().to_owned(),
        phone: normalize_phone(&a.phone),
        city: a.city.trim().to_owned(),
        area: a.area.trim().to_owned(),
        street: a.street.trim().to_owned(),
        building: a.building.trim().to_owned(),
        floor: a.floor.trim().to_owned(),
        apartment: a.apartment.trim().to_owned(),
        landmark: a.landmark.trim().to_owned(),
        notes: a.notes.trim().to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phones() {
        for ok in [
            "01012345678",
            "011 2345 6789",
            "+20 122 345 6789",
            "0020-155-555-1234",
            "٠١٠١٢٣٤٥٦٧٨",
        ] {
            assert!(is_egyptian_mobile(&normalize_phone(ok)), "{ok}");
        }
        for bad in ["0101234567", "01312345678", "1234", "+1 415 555 0100"] {
            assert!(!is_egyptian_mobile(&normalize_phone(bad)), "{bad}");
        }
    }

    #[test]
    fn area_must_match_city() {
        let a = AddressInput {
            city: "giza".into(),
            area: "maadi".into(),
            ..Default::default()
        };
        assert_eq!(check(Field::Area, &a), Some(Problem::AreaCity));
    }
}
