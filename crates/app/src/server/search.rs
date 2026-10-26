//! Bilingual product search with Arabic-aware folding.
//!
//! Arabic shoppers type the same word many ways (أرز / ارز / إرز, طحينة / طحينه, with or without
//! diacritics), and digits in either system. Text and queries are folded to one form before
//! matching; highlights are mapped back onto the original text, so what the shopper sees is the
//! catalogue's spelling with the matched part marked.
//!
//! Every word of a query has to be found in the product, in either language. A query word finds a
//! catalogue word when:
//! - it is the start of it ("sams" → Samsung, "ايفون" → آيفون), as typed or without the Arabic
//!   article and the clitics riding on it (الرز, بالتمر, والطحينة → رز, تمر, طحينه);
//! - both share a light stem (سماعات / سماعة, phones / phone, berries / berry), whole words only;
//! - they are listed as synonyms (رز / أرز / rice, عيش / خبز / bread), whole words only.
//!
//! Names weigh most, then brand, aisle, subtitle and variant labels ("256 GB"). Highlights and
//! spec values count least and only on whole words, so a prefix like "blue" doesn't pull in every
//! product with Bluetooth in its specs. Descriptions aren't indexed: they're prose that names
//! other products ("serve with tahini").

use crate::api::types::Seg;
use std::{collections::HashMap, sync::LazyLock};

/// Fold one character, or drop it (diacritics, tatweel). Separators become `' '`.
pub fn fold_char(c: char) -> Option<char> {
    Some(match c {
        '\u{064B}'..='\u{065F}' | '\u{0670}' | '\u{0640}' => return None,
        'أ' | 'إ' | 'آ' | 'ٱ' => 'ا',
        'ة' => 'ه',
        'ى' => 'ي',
        'ؤ' => 'و',
        'ئ' => 'ي',
        '٠'..='٩' => char::from_u32('0' as u32 + (c as u32 - '٠' as u32)).unwrap_or(c),
        '۰'..='۹' => char::from_u32('0' as u32 + (c as u32 - '۰' as u32)).unwrap_or(c),
        c if c.is_alphanumeric() => c.to_lowercase().next().unwrap_or(c),
        _ => ' ',
    })
}

pub fn fold(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut last_space = true;
    for f in s.chars().filter_map(fold_char) {
        if f == ' ' {
            if !last_space {
                out.push(' ');
            }
            last_space = true;
        } else {
            out.push(f);
            last_space = false;
        }
    }
    out.trim_end().to_owned()
}

pub fn tokens(q: &str) -> Vec<String> {
    fold(q).split(' ').filter(|t| !t.is_empty()).map(str::to_owned).collect()
}

/// The Arabic article and the clitics that ride on it (و / ب / ف / ك + ال, and لل = ل + ال),
/// longest first. Single-letter clitics alone aren't stripped: too many words start with و, ب, ل.
const ARTICLES: [&str; 6] = ["وال", "بال", "فال", "كال", "لل", "ال"];

/// `word` without its article, and the article's length in chars.
fn strip_article(word: &str) -> Option<(&str, usize)> {
    ARTICLES.iter().find_map(|a| {
        let rest = word.strip_prefix(a)?;
        (rest.chars().count() >= 2).then(|| (rest, a.chars().count()))
    })
}

fn is_arabic(c: char) -> bool {
    ('\u{0600}'..='\u{06FF}').contains(&c)
}

fn drop_last(word: &str, n: usize) -> String {
    let keep = word.chars().count() - n;
    word.chars().take(keep).collect()
}

/// English plurals that aren't the plural of what they look like ("glasses" isn't more glass).
const NO_STEM: [&str; 5] = ["glasses", "sunglasses", "shorts", "series", "news"];

/// A light stem: the Arabic feminine plural and tā' marbūṭa (سماعات / سماعه → سماع) and English
/// plurals (phones → phone, watches → watch, berries → berry, tomatoes → tomato). Both sides of a
/// match are stemmed the same way, so a stem only has to be consistent, not a dictionary word.
fn stem(word: &str) -> Option<String> {
    let n = word.chars().count();
    if word.chars().any(is_arabic) {
        return if n >= 5 && word.ends_with("ات") {
            Some(drop_last(word, 2))
        } else if n >= 4 && word.ends_with('ه') {
            Some(drop_last(word, 1))
        } else {
            None
        };
    }
    if !word.bytes().all(|b| b.is_ascii_lowercase()) || NO_STEM.contains(&word) {
        return None;
    }
    let base = |cut: usize| word[..word.len() - cut].to_owned();
    if n >= 5 && word.ends_with("ies") {
        Some(format!("{}y", base(3)))
    } else if n >= 6 && word.ends_with("oes") {
        Some(base(2))
    } else if n >= 5 && word.ends_with("es") && ["s", "x", "z", "ch", "sh"].iter().any(|e| base(2).ends_with(e)) {
        Some(base(2))
    } else if n >= 4 && word.ends_with('s') && !["ss", "us", "is"].iter().any(|e| word.ends_with(e)) {
        Some(base(1))
    } else {
        None
    }
}

/// The forms a folded word is indexed and matched under: as written, without its article, and
/// the stems of both.
fn word_keys(word: &str) -> Vec<String> {
    let mut keys = vec![word.to_owned()];
    if let Some((rest, _)) = strip_article(word) {
        keys.push(rest.to_owned());
    }
    let stems: Vec<String> = keys.iter().filter_map(|k| stem(k)).collect();
    keys.extend(stems);
    keys.dedup();
    keys
}

/// Words that say nothing about the product. Dropped from a query unless it has nothing else.
const STOP_WORDS: [&str; 22] = [
    "the", "a", "an", "and", "or", "of", "for", "with", "in", "on", "to", "by", "from", "في", "من",
    "مع", "علي", "عن", "الي", "و", "او", "اللي",
];

/// Same thing, different word: Egyptian everyday Arabic, standard Arabic and English. Matched on
/// whole words only, in both directions. Single words; grow it by adding a line.
const SYNONYMS: &[&[&str]] = &[
    // Groceries
    &["rice", "أرز", "رز"],
    &["bread", "خبز", "عيش"],
    &["milk", "لبن", "حليب"],
    &["yogurt", "yoghurt", "زبادي"],
    &["cheese", "جبنة", "جبن"],
    &["eggs", "بيض"],
    &["butter", "زبدة"],
    &["chicken", "فراخ", "فرخة", "دجاج", "دجاجة"],
    &["meat", "beef", "لحمة", "لحم", "لحوم"],
    &["fish", "سمك", "أسماك"],
    &["shrimp", "prawns", "جمبري", "روبيان"],
    &["tomatoes", "طماطم", "قوطة", "بندورة"],
    &["potatoes", "بطاطس", "بطاطا"],
    &["onions", "بصل"],
    &["garlic", "ثوم", "توم"],
    &["dates", "تمر", "بلح", "تمور"],
    &["mango", "mangoes", "مانجو", "مانجة"],
    &["coffee", "قهوة", "بن"],
    &["tea", "شاي"],
    &["water", "مياه", "مية", "ماء"],
    &["juice", "juices", "عصير", "عصائر"],
    &["oil", "زيت", "زيوت"],
    &["sugar", "سكر"],
    &["salt", "ملح"],
    &["flour", "دقيق"],
    &["honey", "عسل"],
    &["pasta", "macaroni", "مكرونة", "معكرونة"],
    &["chocolate", "شوكولاتة", "شيكولاتة", "شوكولا"],
    &["tahini", "tahina", "طحينة"],
    &["spices", "بهارات", "توابل"],
    &["nuts", "مكسرات"],
    &["sweets", "desserts", "حلويات"],
    // Electronics, fashion, home
    &["phone", "phones", "mobile", "smartphone", "موبايل", "تليفون", "هاتف", "جوال"],
    &["headphones", "earphones", "earbuds", "headset", "سماعة", "سماعات"],
    &["laptop", "notebook", "لابتوب", "كمبيوتر", "حاسوب"],
    &["tablet", "تابلت"],
    &["tv", "television", "تلفزيون", "تليفزيون"],
    &["charger", "شاحن"],
    &["cable", "كابل", "سلك"],
    &["case", "cover", "جراب", "كفر"],
    &["games", "gaming", "لعبة", "ألعاب"],
    &["watch", "watches", "ساعة", "ساعات"],
    &["shoes", "sneakers", "حذاء", "أحذية", "جزمة", "كوتشي"],
    &["bag", "handbag", "شنطة", "حقيبة", "شنط"],
    &["perfume", "fragrance", "cologne", "عطر", "برفان", "برفيوم"],
    &["glasses", "sunglasses", "نظارة", "نضارة"],
    &["dress", "فستان"],
    &["shirt", "قميص"],
    &["earrings", "أقراط", "حلق"],
    &["ball", "كرة", "كورة"],
    &["racket", "مضرب"],
    &["blender", "خلاط"],
    &["pan", "مقلاة", "طاسة"],
    &["chair", "كرسي"],
];

/// Each synonym group as the keys of all its words.
static SYNONYM_KEYS: LazyLock<Vec<Vec<String>>> = LazyLock::new(|| {
    SYNONYMS
        .iter()
        .map(|group| group.iter().flat_map(|w| word_keys(&fold(w))).collect())
        .collect()
});

/// One query word, ready to match: `prefix` forms may match the start of a catalogue word,
/// `whole` forms (stems, synonyms) only a whole one.
struct Term {
    prefix: Vec<String>,
    whole: Vec<String>,
}

impl Term {
    fn new(word: &str) -> Term {
        let mut prefix = vec![word.to_owned()];
        if let Some((rest, _)) = strip_article(word) {
            prefix.push(rest.to_owned());
        }
        let mut whole: Vec<String> = prefix.iter().filter_map(|p| stem(p)).collect();
        for group in SYNONYM_KEYS.iter() {
            if group.iter().any(|k| prefix.contains(k) || whole.contains(k)) {
                whole.extend(group.iter().filter(|k| !prefix.contains(k)).cloned());
            }
        }
        whole.sort();
        whole.dedup();
        Term { prefix, whole }
    }

    /// This word's score in one field (0 = not found): the field's weight, plus a little when it
    /// is a whole word rather than the start of one.
    fn score(&self, field: Field, keys: &Keys) -> u32 {
        let w = field.weight() * 3;
        if self.prefix.iter().chain(&self.whole).any(|t| keys.has(t)) {
            w + 2
        } else if field.prefix() && self.prefix.iter().any(|t| keys.has_prefix(t)) {
            w
        } else {
            0
        }
    }

    /// The part of a folded catalogue word this term marks, in chars: the matched start of the
    /// word (past its article), or the whole word for a stem or synonym match.
    fn span(&self, word: &str) -> Option<(usize, usize)> {
        let bare = strip_article(word);
        let mut best: Option<(usize, usize)> = None;
        let mut take = |s: (usize, usize)| {
            if best.is_none_or(|b| s.1 - s.0 > b.1 - b.0) {
                best = Some(s);
            }
        };
        for p in &self.prefix {
            let len = p.chars().count();
            if word.starts_with(p.as_str()) {
                take((0, len));
            }
            if let Some((rest, off)) = bare
                && rest.starts_with(p.as_str())
            {
                take((off, off + len));
            }
        }
        if !self.whole.is_empty() && word_keys(word).iter().any(|k| self.whole.contains(k)) {
            take((bare.map_or(0, |(_, off)| off), word.chars().count()));
        }
        best
    }
}

/// A parsed query: one [`Term`] per meaningful word.
pub struct Query {
    terms: Vec<Term>,
}

impl Query {
    pub fn parse(q: &str) -> Query {
        let words = tokens(q);
        let meaningful: Vec<&String> = words.iter().filter(|w| !STOP_WORDS.contains(&w.as_str())).collect();
        let words: Vec<&String> = if meaningful.is_empty() { words.iter().collect() } else { meaningful };
        Query { terms: words.into_iter().map(|w| Term::new(w)).collect() }
    }

    pub fn is_empty(&self) -> bool {
        self.terms.is_empty()
    }

    /// Does every word of the query match somewhere in `text`? (Aisle shortcuts.)
    pub fn matches_text(&self, text: &str) -> bool {
        let keys = Keys::of([text]);
        !self.is_empty() && self.terms.iter().all(|t| t.score(Field::Category, &keys) > 0)
    }
}

#[derive(Clone, Copy)]
enum Field {
    Name,
    Brand,
    Category,
    Subtitle,
    Variant,
    Detail,
}

impl Field {
    fn weight(self) -> u32 {
        match self {
            Field::Name => 12,
            Field::Brand => 9,
            Field::Category => 6,
            Field::Subtitle => 3,
            Field::Variant => 2,
            Field::Detail => 1,
        }
    }

    /// Long prose only counts whole words.
    fn prefix(self) -> bool {
        !matches!(self, Field::Detail)
    }
}

/// Words left out of highlights and specs. There, ساعة is nearly always "hours" (battery life,
/// shelf life), but it is also the word for a watch, and a search for watches shouldn't list
/// earbuds.
const DETAIL_SKIP: [&str; 4] = ["ساعه", "ساعات", "hour", "hours"];

/// The keys of every word of a field, sorted for binary search.
struct Keys(Vec<String>);

impl Keys {
    fn of<'a>(texts: impl IntoIterator<Item = &'a str>) -> Keys {
        Self::keep(texts, |_| true)
    }

    fn details<'a>(texts: impl IntoIterator<Item = &'a str>) -> Keys {
        Self::keep(texts, |w| !word_keys(w).iter().any(|k| DETAIL_SKIP.contains(&k.as_str())))
    }

    fn keep<'a>(texts: impl IntoIterator<Item = &'a str>, keep: impl Fn(&str) -> bool) -> Keys {
        let mut keys: Vec<String> = texts
            .into_iter()
            .flat_map(|t| tokens(t))
            .filter(|w| keep(w))
            .flat_map(|w| word_keys(&w))
            .collect();
        keys.sort();
        keys.dedup();
        Keys(keys)
    }

    fn has(&self, t: &str) -> bool {
        self.0.binary_search_by(|k| k.as_str().cmp(t)).is_ok()
    }

    fn has_prefix(&self, t: &str) -> bool {
        let i = self.0.partition_point(|k| k.as_str() < t);
        self.0.get(i).is_some_and(|k| k.starts_with(t))
    }
}

struct Doc {
    fields: Vec<(Field, Keys)>,
}

/// One document per product (same index order as the catalogue's product list).
pub struct SearchIndex {
    docs: Vec<Doc>,
}

pub struct DocText<'a> {
    pub names: [&'a str; 2],
    pub brands: [&'a str; 2],
    pub categories: [&'a str; 2],
    pub subtitles: [&'a str; 2],
    /// Option values, both languages ("256 GB", "Midnight").
    pub variants: Vec<&'a str>,
    /// Highlights and spec values, both languages (whole words only).
    pub details: Vec<&'a str>,
}

impl SearchIndex {
    pub fn build<'a>(texts: impl Iterator<Item = DocText<'a>>) -> Self {
        let docs = texts
            .map(|t| Doc {
                fields: vec![
                    (Field::Name, Keys::of(t.names)),
                    (Field::Brand, Keys::of(t.brands)),
                    (Field::Category, Keys::of(t.categories)),
                    (Field::Subtitle, Keys::of(t.subtitles)),
                    (Field::Variant, Keys::of(t.variants)),
                    (Field::Detail, Keys::details(t.details)),
                ],
            })
            .collect();
        SearchIndex { docs }
    }

    /// Product index → score, for products matching **every** query word.
    pub fn matches(&self, q: &str) -> HashMap<usize, u32> {
        let query = Query::parse(q);
        let mut out = HashMap::new();
        if query.is_empty() {
            return out;
        }
        'doc: for (i, doc) in self.docs.iter().enumerate() {
            let mut score = 0u32;
            for t in &query.terms {
                let best = doc.fields.iter().map(|(f, keys)| t.score(*f, keys)).max().unwrap_or(0);
                if best == 0 {
                    continue 'doc;
                }
                score += best;
            }
            out.insert(i, score);
        }
        out
    }
}

/// Split `text` into highlighted / plain runs for the query's words.
pub fn highlight(text: &str, q: &str) -> Vec<Seg> {
    let query = Query::parse(q);
    let orig: Vec<char> = text.chars().collect();
    let mut folded = Vec::new();
    let mut map = Vec::new();
    for (i, c) in orig.iter().enumerate() {
        if let Some(f) = fold_char(*c) {
            folded.push(f);
            map.push(i);
        }
    }
    let mut hit = vec![false; orig.len()];
    let mut s = 0;
    while s < folded.len() {
        if folded[s] == ' ' {
            s += 1;
            continue;
        }
        let e = (s..folded.len()).find(|&i| folded[i] == ' ').unwrap_or(folded.len());
        let word: String = folded[s..e].iter().collect();
        let span = query
            .terms
            .iter()
            .filter_map(|t| t.span(&word))
            .max_by_key(|(from, to)| to - from);
        if let Some((from, to)) = span {
            let start = map[s + from];
            let mut end = map[s + to - 1];
            // keep trailing diacritics with their letter
            while end + 1 < orig.len() && fold_char(orig[end + 1]).is_none() {
                end += 1;
            }
            hit[start..=end].iter_mut().for_each(|h| *h = true);
        }
        s = e;
    }
    let mut segs: Vec<Seg> = Vec::new();
    for (c, h) in orig.iter().zip(hit) {
        match segs.last_mut() {
            Some(s) if s.hit == h => s.text.push(*c),
            _ => segs.push(Seg { text: c.to_string(), hit: h }),
        }
    }
    segs
}

#[cfg(test)]
mod tests {
    use super::*;

    fn index(names: &[(&str, &str)]) -> SearchIndex {
        SearchIndex::build(names.iter().map(|(en, ar)| DocText {
            names: [en, ar],
            brands: ["", ""],
            categories: ["", ""],
            subtitles: ["", ""],
            variants: vec![],
            details: vec![],
        }))
    }

    #[test]
    fn arabic_folding() {
        assert_eq!(fold("أُرز مصريّ"), "ارز مصري");
        assert_eq!(fold("طحينة"), fold("طحينه"));
        assert_eq!(fold("٢٥٦ GB"), "256 gb");
        let segs = highlight("أرز مصري", "ارز");
        assert_eq!(segs[0], Seg { text: "أرز".into(), hit: true });
    }

    #[test]
    fn article_stems_and_synonyms() {
        let ix = index(&[("Egyptian rice", "أرز مصري"), ("Dates", "تمر سيوي"), ("Earbuds", "سماعة لاسلكية")]);
        for q in ["الرز", "رز", "بالتمر", "التمر", "سماعات", "headphones", "phones"] {
            let hits = ix.matches(q);
            let want = match q {
                "الرز" | "رز" => Some(0),
                "بالتمر" | "التمر" => Some(1),
                "سماعات" | "headphones" => Some(2),
                _ => None,
            };
            assert_eq!(hits.keys().next().copied(), want, "{q}");
        }
    }

    #[test]
    fn highlights_skip_the_article() {
        let segs = highlight("والطحينة البلدي", "طحينة");
        assert_eq!(segs[0], Seg { text: "وال".into(), hit: false });
        assert_eq!(segs[1], Seg { text: "طحينة".into(), hit: true });
    }
}
