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
//! - they are listed as synonyms (رز / أرز / rice, عيش / خبز / bread), whole words only. The
//!   table is data (`data/search/synonyms.txt`, plus `synonyms.<shop>.txt` per shop), read live,
//!   and may list phrases ("لاب توب"), which are matched as one word.
//!
//! Names weigh most, then brand, aisle, subtitle and variant labels ("256 GB"). Highlights and
//! spec values count least and only on whole words, so a prefix like "blue" doesn't pull in every
//! product with Bluetooth in its specs. Descriptions aren't indexed: they're prose that names
//! other products ("serve with tahini").

use crate::api::types::Seg;
use std::{cmp::Reverse, collections::HashMap};

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

/// The folded words of `s`, each with whether it may start with the article. A word written with
/// a hamza on its first alef (ألبان dairy, ألعاب games, إلكتروني) starts with that alef, not with
/// ال — folding would otherwise turn ألبان into "ال + بان" and match Ray-Ban (راي بان).
fn words(s: &str) -> Vec<(String, bool)> {
    let mut out = Vec::new();
    let mut word = String::new();
    let mut hamza = false;
    for c in s.chars() {
        match fold_char(c) {
            None => {}
            Some(' ') => {
                if !word.is_empty() {
                    out.push((std::mem::take(&mut word), !hamza));
                }
            }
            Some(f) => {
                if word.is_empty() {
                    hamza = matches!(c, 'أ' | 'إ' | 'آ');
                }
                word.push(f);
            }
        }
    }
    if !word.is_empty() {
        out.push((word, !hamza));
    }
    out
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

/// English plurals whose singular means something else: "glasses" isn't more glass, "sweets"
/// aren't sweet paprika, "chips" aren't phone chips, "pulses" aren't heart rates.
const NO_STEM: [&str; 11] =
    ["glasses", "sunglasses", "shorts", "series", "news", "sweets", "goods", "greens", "chips", "wipes", "pulses"];

/// A light stem. Arabic: a sound feminine plural loses ات (مشروبات → مشروب, سماعات → سماع), and a
/// tā' marbūṭa word maps to that plural (سماعه → سماعات), so سماعة meets سماعات without its bare
/// stem landing on another word (معلقة "spoon" is not معلق "hanging", قدرة "power" is not قدر
/// "pot"). English: plurals (phones → phone, watches → watch, berries → berry, tomatoes →
/// tomato). Both sides of a match are stemmed alike, so a stem only has to be consistent.
fn stem(word: &str) -> Option<String> {
    if NO_STEM.contains(&word) {
        return None;
    }
    let n = word.chars().count();
    if word.chars().any(is_arabic) {
        return if n >= 5 && word.ends_with("ات") {
            Some(drop_last(word, 2))
        } else if n >= 4 && word.ends_with('ه') {
            Some(drop_last(word, 1) + "ات")
        } else {
            None
        };
    }
    if !word.bytes().all(|b| b.is_ascii_lowercase()) {
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

/// The forms a folded word is indexed and matched under: as written, without its article (when
/// it may have one), and the stems of both.
fn word_keys(word: &str, article: bool) -> Vec<String> {
    let mut keys = vec![word.to_owned()];
    if let Some((rest, _)) = strip_article(word).filter(|_| article) {
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

/// The synonym table: groups of words that find each other, and the multi-word entries in them.
#[derive(Default)]
pub struct Synonyms {
    /// Each group as the keys of all its words; a phrase is one key, its words joined.
    groups: Vec<Vec<String>>,
    /// Multi-word entries as folded words, longest first.
    phrases: Vec<Vec<String>>,
}

impl Synonyms {
    /// One group per line, entries separated by commas (`,` or `،`); a line starting with `#` is
    /// a comment. Entries are folded like everything else, so spelling variants that differ only
    /// in hamza, tā' marbūṭa, diacritics or case need listing once.
    pub fn parse(text: &str) -> Synonyms {
        let mut syn = Synonyms::default();
        for line in text.lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with('#')) {
            let mut group = Vec::new();
            for entry in line.split([',', '،']) {
                let ws = words(entry);
                match ws.as_slice() {
                    [] => {}
                    [(w, article)] => group.extend(word_keys(w, *article)),
                    _ => {
                        let phrase: Vec<String> = ws.into_iter().map(|(w, _)| w).collect();
                        group.push(phrase.concat());
                        syn.phrases.push(phrase);
                    }
                }
            }
            group.sort();
            group.dedup();
            if group.len() > 1 {
                syn.groups.push(group);
            }
        }
        syn.phrases.sort();
        syn.phrases.dedup();
        syn.phrases.sort_by_key(|p| Reverse(p.len()));
        syn
    }

    /// Number of groups.
    pub fn len(&self) -> usize {
        self.groups.len()
    }

    pub fn is_empty(&self) -> bool {
        self.groups.is_empty()
    }

    /// `words` with every listed phrase replaced by its joined token (queries). A joined token
    /// never sheds an article.
    fn join(&self, words: Vec<(String, bool)>) -> Vec<(String, bool)> {
        if self.phrases.is_empty() {
            return words;
        }
        let plain: Vec<String> = words.iter().map(|(w, _)| w.clone()).collect();
        let mut out = Vec::with_capacity(words.len());
        let mut i = 0;
        while i < words.len() {
            match self.phrases.iter().find(|p| plain[i..].starts_with(p)) {
                Some(p) => {
                    out.push((p.concat(), false));
                    i += p.len();
                }
                None => {
                    out.push(words[i].clone());
                    i += 1;
                }
            }
        }
        out
    }

    /// The joined tokens of the listed phrases found in `words` (indexed beside the words).
    fn phrases_in(&self, words: &[String]) -> Vec<String> {
        (0..words.len())
            .flat_map(|i| self.phrases.iter().filter(move |p| words[i..].starts_with(p)))
            .map(|p| p.concat())
            .collect()
    }
}

/// One query word, ready to match: `prefix` forms may match the start of a catalogue word,
/// `whole` forms (stems, synonyms) only a whole one.
struct Term {
    prefix: Vec<String>,
    whole: Vec<String>,
}

impl Term {
    fn new(word: &str, article: bool, syn: &Synonyms) -> Term {
        let mut prefix = vec![word.to_owned()];
        if let Some((rest, _)) = strip_article(word).filter(|_| article) {
            prefix.push(rest.to_owned());
        }
        let mut whole: Vec<String> = prefix.iter().filter_map(|p| stem(p)).collect();
        let own = whole.clone();
        for group in &syn.groups {
            if group.iter().any(|k| prefix.contains(k) || own.contains(k)) {
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
    fn span(&self, word: &str, article: bool) -> Option<(usize, usize)> {
        let bare = strip_article(word).filter(|_| article);
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
        if !self.whole.is_empty() && word_keys(word, article).iter().any(|k| self.whole.contains(k)) {
            take((bare.map_or(0, |(_, off)| off), word.chars().count()));
        }
        best
    }
}

/// A parsed query: one [`Term`] per meaningful word. When it contains a listed phrase it is read
/// both ways — the phrase as one word («لاب توب» → laptop) and word by word (a catalogue that
/// never says "smartwatch" still answers «ساعة ذكية» through ساعة + ذكية) — and a product matching
/// either reading matches.
pub struct Query {
    readings: Vec<Vec<Term>>,
}

impl Query {
    pub fn parse(q: &str, syn: &Synonyms) -> Query {
        let plain = words(q);
        let joined = syn.join(plain.clone());
        let read = |ws: Vec<(String, bool)>| {
            let meaningful: Vec<&(String, bool)> =
                ws.iter().filter(|(w, _)| !STOP_WORDS.contains(&w.as_str())).collect();
            let ws: Vec<&(String, bool)> = if meaningful.is_empty() { ws.iter().collect() } else { meaningful };
            ws.into_iter().map(|(w, article)| Term::new(w, *article, syn)).collect::<Vec<_>>()
        };
        let mut readings = Vec::new();
        if joined != plain {
            readings.push(read(joined));
        }
        readings.push(read(plain));
        readings.retain(|r| !r.is_empty());
        Query { readings }
    }

    pub fn is_empty(&self) -> bool {
        self.readings.is_empty()
    }

    fn terms(&self) -> impl Iterator<Item = &Term> {
        self.readings.iter().flatten()
    }

    /// Split `text` into highlighted / plain runs for this query's words.
    pub fn highlight(&self, text: &str) -> Vec<Seg> {
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
            let article = !matches!(orig[map[s]], 'أ' | 'إ' | 'آ');
            let span = self.terms().filter_map(|t| t.span(&word, article)).max_by_key(|(from, to)| to - from);
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

/// The keys of every word of a field (and of the listed phrases in it), sorted for binary search.
struct Keys(Vec<String>);

impl Keys {
    fn of<'a>(texts: impl IntoIterator<Item = &'a str>, syn: &Synonyms) -> Keys {
        Self::keep(texts, syn, |_| true)
    }

    fn details<'a>(texts: impl IntoIterator<Item = &'a str>, syn: &Synonyms) -> Keys {
        Self::keep(texts, syn, |w| !word_keys(w, true).iter().any(|k| DETAIL_SKIP.contains(&k.as_str())))
    }

    fn keep<'a>(texts: impl IntoIterator<Item = &'a str>, syn: &Synonyms, keep: impl Fn(&str) -> bool) -> Keys {
        let mut keys = Vec::new();
        for text in texts {
            let ws = words(text);
            let plain: Vec<String> = ws.iter().map(|(w, _)| w.clone()).collect();
            keys.extend(syn.phrases_in(&plain));
            keys.extend(ws.iter().filter(|(w, _)| keep(w)).flat_map(|(w, article)| word_keys(w, *article)));
        }
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

/// One document per product (same index order as the catalogue's product list), and the shop's
/// synonym table.
pub struct SearchIndex {
    docs: Vec<Doc>,
    synonyms: Synonyms,
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
    pub fn build<'a>(texts: impl Iterator<Item = DocText<'a>>, synonyms: Synonyms) -> Self {
        let syn = &synonyms;
        let docs = texts
            .map(|t| Doc {
                fields: vec![
                    (Field::Name, Keys::of(t.names, syn)),
                    (Field::Brand, Keys::of(t.brands, syn)),
                    (Field::Category, Keys::of(t.categories, syn)),
                    (Field::Subtitle, Keys::of(t.subtitles, syn)),
                    (Field::Variant, Keys::of(t.variants, syn)),
                    (Field::Detail, Keys::details(t.details, syn)),
                ],
            })
            .collect();
        SearchIndex { docs, synonyms }
    }

    /// Parse a query with this shop's synonyms.
    pub fn query(&self, q: &str) -> Query {
        Query::parse(q, &self.synonyms)
    }

    /// Product index → score, for products matching **every** query word.
    pub fn matches(&self, q: &str) -> HashMap<usize, u32> {
        self.matches_query(&self.query(q))
    }

    pub fn matches_query(&self, query: &Query) -> HashMap<usize, u32> {
        let mut out = HashMap::new();
        if query.is_empty() {
            return out;
        }
        for (i, doc) in self.docs.iter().enumerate() {
            let score = query.readings.iter().filter_map(|terms| Self::score(doc, terms)).max();
            if let Some(score) = score {
                out.insert(i, score);
            }
        }
        out
    }

    /// A document's score for one reading of a query: `None` unless every word is found.
    fn score(doc: &Doc, terms: &[Term]) -> Option<u32> {
        terms.iter().try_fold(0u32, |sum, t| {
            let best = doc.fields.iter().map(|(f, keys)| t.score(*f, keys)).max().unwrap_or(0);
            (best > 0).then_some(sum + best)
        })
    }

    /// Does every word of the query match somewhere in `text`? (Aisle shortcuts.)
    pub fn text_matches(&self, query: &Query, text: &str) -> bool {
        let keys = Keys::of([text], &self.synonyms);
        query.readings.iter().any(|terms| terms.iter().all(|t| t.score(Field::Category, &keys) > 0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn synonyms() -> Synonyms {
        Synonyms::parse(
            "# test table\nrice, أرز, رز\nheadphones, earbuds, سماعة، سماعات\ndates, تمر, بلح\n\
             tahini, طحينة\nflour, دقيق, طحين\nlaptop, لابتوب, لاب توب\n",
        )
    }

    fn index(names: &[(&'static str, &'static str)]) -> SearchIndex {
        SearchIndex::build(
            names.iter().map(|&(en, ar)| DocText {
                names: [en, ar],
                brands: ["", ""],
                categories: ["", ""],
                subtitles: ["", ""],
                variants: vec![],
                details: vec![],
            }),
            synonyms(),
        )
    }

    fn only(ix: &SearchIndex, q: &str) -> Vec<usize> {
        let mut hits: Vec<usize> = ix.matches(q).into_keys().collect();
        hits.sort();
        hits
    }

    #[test]
    fn arabic_folding() {
        assert_eq!(fold("أُرز مصريّ"), "ارز مصري");
        assert_eq!(fold("طحينة"), fold("طحينه"));
        assert_eq!(fold("٢٥٦ GB"), "256 gb");
        let segs = Query::parse("ارز", &Synonyms::default()).highlight("أرز مصري");
        assert_eq!(segs[0], Seg { text: "أرز".into(), hit: true });
    }

    #[test]
    fn article_stems_and_synonyms() {
        let ix = index(&[
            ("Egyptian rice", "أرز مصري"),
            ("Dates", "تمر سيوي"),
            ("Earbuds", "سماعة لاسلكية"),
            ("Tahini", "طحينة"),
            ("Flour", "دقيق"),
        ]);
        for q in ["الرز", "رز", "بالرز"] {
            assert_eq!(only(&ix, q), [0], "{q}");
        }
        for q in ["بالتمر", "التمر", "بلح"] {
            assert_eq!(only(&ix, q), [1], "{q}");
        }
        for q in ["سماعات", "headphones"] {
            assert_eq!(only(&ix, q), [2], "{q}");
        }
        // طحينة and طحين (flour) must not meet through a stem.
        assert_eq!(only(&ix, "طحينة"), [3]);
        assert_eq!(only(&ix, "flour"), [4]);
        assert!(only(&ix, "phones").is_empty());
    }

    #[test]
    fn phrases_match_as_one_word() {
        let ix = index(&[("Lenovo laptop", "لينوفو"), ("Laptop bag", "شنطة لاب توب")]);
        assert_eq!(only(&ix, "لاب توب"), [0, 1]);
        assert_eq!(only(&ix, "laptop"), [0, 1]);
        assert_eq!(only(&ix, "لابتوب لينوفو"), [0]);
    }

    #[test]
    fn feminine_singular_meets_its_plural_only() {
        let ix = index(&[("Spoons", "معلقة شاي"), ("Hanging chair", "كرسي معلق"), ("Speakers", "سماعات")]);
        assert_eq!(only(&ix, "معلقة"), [0]);
        assert_eq!(only(&ix, "سماعة"), [2]);
    }

    #[test]
    fn a_phrase_query_also_reads_word_by_word() {
        let syn = Synonyms::parse("smartwatch, ساعة ذكية\n");
        let ix = SearchIndex::build(
            [("Watch SE", "ساعة ذكية")].iter().map(|&(en, ar)| DocText {
                names: [en, ar],
                brands: ["", ""],
                categories: ["", ""],
                subtitles: ["", ""],
                variants: vec![],
                details: vec![],
            }),
            syn,
        );
        assert_eq!(ix.matches("smartwatch").len(), 1);
        assert_eq!(ix.matches("ساعة ذكية").len(), 1);
    }

    #[test]
    fn hamza_alef_is_not_the_article() {
        let ix = index(&[("Dairy", "ألبان"), ("Ray-Ban Aviator", "نظارة راي بان")]);
        assert_eq!(only(&ix, "ألبان"), [0]);
        assert_eq!(only(&ix, "بان"), [1]);
    }

    #[test]
    fn highlights_skip_the_article() {
        let segs = Query::parse("طحينة", &synonyms()).highlight("والطحينة البلدي");
        assert_eq!(segs[0], Seg { text: "وال".into(), hit: false });
        assert_eq!(segs[1], Seg { text: "طحينة".into(), hit: true });
    }
}
