//! Bilingual product search with Arabic-aware folding.
//!
//! Arabic shoppers type the same word many ways (أرز / ارز / إرز, طحينة / طحينه, with or without
//! diacritics), and digits in either system. Text and queries are folded to one form before
//! matching; highlights are mapped back onto the original text, so what the shopper sees is the
//! catalogue's spelling with the matched part marked.

use crate::api::types::Seg;
use std::collections::HashMap;

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

#[derive(Clone, Copy)]
enum Field {
    Name = 4,
    Brand = 3,
    Category = 2,
    Extra = 1,
}

struct Doc {
    fields: Vec<(Field, Vec<String>)>,
}

/// One document per product (same index order as the catalogue's product list).
pub struct SearchIndex {
    docs: Vec<Doc>,
}

pub struct DocText<'a> {
    pub names: [&'a str; 2],
    pub brands: [&'a str; 2],
    pub categories: [&'a str; 2],
    pub extra: Vec<&'a str>,
}

impl SearchIndex {
    pub fn build<'a>(texts: impl Iterator<Item = DocText<'a>>) -> Self {
        let toks = |s: &str| fold(s).split(' ').filter(|t| !t.is_empty()).map(str::to_owned).collect::<Vec<_>>();
        let docs = texts
            .map(|t| {
                let mut fields = Vec::new();
                for n in t.names {
                    fields.push((Field::Name, toks(n)));
                }
                for b in t.brands {
                    fields.push((Field::Brand, toks(b)));
                }
                for c in t.categories {
                    fields.push((Field::Category, toks(c)));
                }
                for e in t.extra {
                    fields.push((Field::Extra, toks(e)));
                }
                Doc { fields }
            })
            .collect();
        SearchIndex { docs }
    }

    /// Product index → score, for products matching **every** query token (prefix match on a word).
    pub fn matches(&self, q: &str) -> HashMap<usize, u32> {
        let qt = tokens(q);
        let mut out = HashMap::new();
        if qt.is_empty() {
            return out;
        }
        'doc: for (i, doc) in self.docs.iter().enumerate() {
            let mut score = 0u32;
            for t in &qt {
                let mut best = 0u32;
                for (field, words) in &doc.fields {
                    for w in words {
                        if w.starts_with(t.as_str()) {
                            let exact = if w == t { 2 } else { 0 };
                            best = best.max(*field as u32 * 3 + exact);
                        }
                    }
                }
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

/// Split `text` into highlighted / plain runs for the query's tokens (word-prefix matches).
pub fn highlight(text: &str, q: &str) -> Vec<Seg> {
    let qt: Vec<Vec<char>> = tokens(q).into_iter().map(|t| t.chars().collect()).collect();
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
    for t in &qt {
        if t.is_empty() {
            continue;
        }
        let mut i = 0;
        while i + t.len() <= folded.len() {
            let word_start = i == 0 || folded[i - 1] == ' ';
            if word_start && folded[i..i + t.len()] == t[..] {
                let (start, mut end) = (map[i], map[i + t.len() - 1]);
                // keep trailing diacritics with their letter
                while end + 1 < orig.len() && fold_char(orig[end + 1]).is_none() {
                    end += 1;
                }
                hit[start..=end].iter_mut().for_each(|h| *h = true);
                i += t.len();
            } else {
                i += 1;
            }
        }
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

    #[test]
    fn arabic_folding() {
        assert_eq!(fold("أُرز مصريّ"), "ارز مصري");
        assert_eq!(fold("طحينة"), fold("طحينه"));
        assert_eq!(fold("٢٥٦ GB"), "256 gb");
        let segs = highlight("أرز مصري", "ارز");
        assert_eq!(segs[0], Seg { text: "أرز".into(), hit: true });
    }
}
