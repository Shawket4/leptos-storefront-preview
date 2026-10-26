//! Code excerpts on the showcase are cut from this repository's own source at compile time
//! (`include_str!`), between an anchor line and an end line. They can't drift from the code: if an
//! anchor disappears, the excerpt disappears with it (and a debug build logs which one).
//!
//! Server only — the source text never reaches the WASM bundle.

use leptos::prelude::*;

/// A file the showcase quotes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Src {
    Frontend,
    Lib,
    Client,
    Stores,
    Request,
    Tenant,
    Media,
    ApiMod,
    ApiTypes,
    Cache,
    Sim,
    ServerMain,
    Home,
    Errors,
    Islands,
    Demos,
    Motion,
    ShellJs,
    Net,
    ApiCart,
    CartPage,
    Product,
    ProductPanel,
    ProductHead,
    Gallery,
    Explorer,
    Steps,
}

impl Src {
    pub fn path(self) -> &'static str {
        match self {
            Src::Frontend => "crates/frontend/src/lib.rs",
            Src::Lib => "crates/app/src/lib.rs",
            Src::Client => "crates/app/src/client.rs",
            Src::Stores => "crates/app/src/stores.rs",
            Src::Request => "crates/app/src/request.rs",
            Src::Tenant => "crates/app/src/tenant.rs",
            Src::Media => "crates/app/src/media.rs",
            Src::ApiMod => "crates/app/src/api/mod.rs",
            Src::ApiTypes => "crates/app/src/api/types.rs",
            Src::Cache => "crates/app/src/server/cache.rs",
            Src::Sim => "crates/app/src/server/sim.rs",
            Src::ServerMain => "crates/server/src/main.rs",
            Src::Home => "crates/app/src/pages/home.rs",
            Src::Errors => "crates/app/src/pages/errors.rs",
            Src::Islands => "crates/app/src/pages/showcase/islands.rs",
            Src::Demos => "crates/app/src/pages/showcase/demos.rs",
            Src::Motion => "crates/app/src/components/motion.rs",
            Src::ShellJs => "crates/app/src/components/shell.js",
            Src::Net => "crates/app/src/components/net.rs",
            Src::ApiCart => "crates/app/src/api/cart.rs",
            Src::CartPage => "crates/app/src/pages/cart/page.rs",
            Src::Product => "crates/app/src/pages/product.rs",
            Src::ProductPanel => "crates/app/src/pages/product/panel.rs",
            Src::ProductHead => "crates/app/src/pages/product/head.rs",
            Src::Gallery => "crates/app/src/pages/product/gallery.rs",
            Src::Explorer => "crates/app/src/pages/category/explorer.rs",
            Src::Steps => "crates/app/src/pages/checkout/steps.rs",
        }
    }

    #[cfg(feature = "ssr")]
    fn text(self) -> &'static str {
        match self {
            Src::Frontend => include_str!("../../../../frontend/src/lib.rs"),
            Src::Lib => include_str!("../../lib.rs"),
            Src::Client => include_str!("../../client.rs"),
            Src::Stores => include_str!("../../stores.rs"),
            Src::Request => include_str!("../../request.rs"),
            Src::Tenant => include_str!("../../tenant.rs"),
            Src::Media => include_str!("../../media.rs"),
            Src::ApiMod => include_str!("../../api/mod.rs"),
            Src::ApiTypes => include_str!("../../api/types.rs"),
            Src::Cache => include_str!("../../server/cache.rs"),
            Src::Sim => include_str!("../../server/sim.rs"),
            Src::ServerMain => include_str!("../../../../server/src/main.rs"),
            Src::Home => include_str!("../home.rs"),
            Src::Errors => include_str!("../errors.rs"),
            Src::Islands => include_str!("islands.rs"),
            Src::Demos => include_str!("demos.rs"),
            Src::Motion => include_str!("../../components/motion.rs"),
            Src::ShellJs => include_str!("../../components/shell.js"),
            Src::Net => include_str!("../../components/net.rs"),
            Src::ApiCart => include_str!("../../api/cart.rs"),
            Src::CartPage => include_str!("../cart/page.rs"),
            Src::Product => include_str!("../product.rs"),
            Src::ProductPanel => include_str!("../product/panel.rs"),
            Src::ProductHead => include_str!("../product/head.rs"),
            Src::Gallery => include_str!("../product/gallery.rs"),
            Src::Explorer => include_str!("../category/explorer.rs"),
            Src::Steps => include_str!("../checkout/steps.rs"),
        }
    }
}

/// Where to cut: from the first line containing `from` to the first later line containing `to`
/// (inclusive), at most `max` lines. A `from` starting with `^` must equal a whole line once
/// trimmed (`"^<For"` skips doc comments that mention `<For>`); a `to` starting with `^` must
/// match a whole line exactly (`"^}"` = the closing brace of a top-level item).
#[derive(Clone, Copy, Debug)]
pub struct Cut {
    pub src: Src,
    pub from: &'static str,
    pub to: &'static str,
    pub max: usize,
}

pub const fn cut(src: Src, from: &'static str, to: &'static str, max: usize) -> Cut {
    Cut { src, from, to, max }
}

pub struct Excerpt {
    pub path: &'static str,
    /// 1-based line number of the first line.
    pub line: usize,
    pub code: String,
}

#[cfg(feature = "ssr")]
pub fn excerpt(c: Cut) -> Option<Excerpt> {
    let lines: Vec<&str> = c.src.text().lines().collect();
    let starts = |l: &&str| match c.from.strip_prefix('^') {
        Some(exact) => l.trim() == exact,
        None => l.contains(c.from),
    };
    let Some(start) = lines.iter().position(starts) else {
        tracing::warn!(
            "showcase excerpt: '{}' not found in {}",
            c.from,
            c.src.path()
        );
        return None;
    };
    let mut end = (start + c.max.max(1) - 1).min(lines.len() - 1);
    if !c.to.is_empty() {
        let hit = |l: &&str| match c.to.strip_prefix('^') {
            Some(exact) => *l == exact,
            None => l.contains(c.to),
        };
        if let Some(off) = lines[start + 1..].iter().position(hit) {
            end = end.min(start + 1 + off);
        }
    }
    let mut picked: Vec<&str> = lines[start..=end].to_vec();
    while picked.last().is_some_and(|l| l.trim().is_empty()) {
        picked.pop();
    }
    let indent = picked
        .iter()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.len() - l.trim_start().len())
        .min()
        .unwrap_or(0);
    let code = picked
        .iter()
        .map(|l| {
            if l.len() >= indent {
                &l[indent..]
            } else {
                l.trim_start()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    Some(Excerpt {
        path: c.src.path(),
        line: start + 1,
        code,
    })
}

/// The hydrate build never renders excerpts (cards are server components).
#[cfg(not(feature = "ssr"))]
pub fn excerpt(c: Cut) -> Option<Excerpt> {
    let _ = (c.src.path(), c.from, c.to, c.max);
    None
}

/// A short, real excerpt in a collapsible block: `path:line` as the summary, scrollable LTR code.
#[component]
pub fn Code(cut: Cut, #[prop(optional)] open: bool) -> impl IntoView {
    let lang = crate::prefs::lang_now();
    excerpt(cut).map(|ex| {
        let lines = ex.code.lines().count();
        let label = format!("{}:{}", ex.path, ex.line);
        // One- to three-line excerpts are shown open: a toggle for one line is just friction.
        let open = open || lines <= 3;
        view! {
            <details class="sc-code" open=open>
                <summary class="sc-code__sum">
                    <span class="sc-code__file mono" dir="ltr">{label}</span>
                    <span class="sc-code__meta">
                        {crate::prefs::tr(lang, "Show code", "اعرض الكود")}
                        " · "
                        <span class="mono">{lines}</span>
                    </span>
                </summary>
                <pre class="sc-code__pre" dir="ltr" tabindex="0"><code inner_html=highlight(&ex.code)></code></pre>
            </details>
        }
    })
}

/// Token classes for the tiny highlighter.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Tok {
    Plain,
    Comment,
    Str,
    Kw,
    Ty,
    Mac,
    Attr,
    Num,
    Tag,
}

impl Tok {
    fn class(self) -> &'static str {
        match self {
            Tok::Plain => "",
            Tok::Comment => "hl-c",
            Tok::Str => "hl-s",
            Tok::Kw => "hl-k",
            Tok::Ty => "hl-t",
            Tok::Mac => "hl-m",
            Tok::Attr => "hl-a",
            Tok::Num => "hl-n",
            Tok::Tag => "hl-g",
        }
    }
}

const KEYWORDS: &[&str] = &[
    "as",
    "async",
    "await",
    "break",
    "const",
    "continue",
    "crate",
    "else",
    "enum",
    "fn",
    "for",
    "if",
    "impl",
    "in",
    "let",
    "loop",
    "match",
    "mod",
    "move",
    "mut",
    "pub",
    "ref",
    "return",
    "self",
    "Self",
    "static",
    "struct",
    "super",
    "trait",
    "type",
    "unsafe",
    "use",
    "where",
    "while",
    "true",
    "false",
    "Some",
    "None",
    "Ok",
    "Err",
    "function",
    "of",
    "new",
    "try",
    "catch",
    "null",
    "typeof",
    "instanceof",
];

/// Split Rust-ish source into classified runs (good enough for short excerpts: comments, strings,
/// attributes, macros, keywords, CamelCase types, numbers, and `<Tags>` inside `view!`).
fn tokens(src: &str) -> Vec<(Tok, String)> {
    let chars: Vec<char> = src.chars().collect();
    let mut out: Vec<(Tok, String)> = Vec::new();
    let mut push = |t: Tok, s: String| {
        if let Some((lt, ls)) = out.last_mut() {
            if *lt == t {
                ls.push_str(&s);
                return;
            }
        }
        out.push((t, s));
    };
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        // line comment
        if c == '/' && chars.get(i + 1) == Some(&'/') {
            let j = chars[i..]
                .iter()
                .position(|&x| x == '\n')
                .map(|p| i + p)
                .unwrap_or(chars.len());
            push(Tok::Comment, chars[i..j].iter().collect());
            i = j;
            continue;
        }
        // string
        if c == '"' {
            let mut j = i + 1;
            while j < chars.len() && chars[j] != '"' {
                if chars[j] == '\\' {
                    j += 1;
                }
                j += 1;
            }
            let j = (j + 1).min(chars.len());
            push(Tok::Str, chars[i..j].iter().collect());
            i = j;
            continue;
        }
        // attribute #[...] / #![...]
        if c == '#'
            && (chars.get(i + 1) == Some(&'[')
                || (chars.get(i + 1) == Some(&'!') && chars.get(i + 2) == Some(&'[')))
        {
            let mut depth = 0;
            let mut j = i;
            while j < chars.len() {
                if chars[j] == '[' {
                    depth += 1;
                } else if chars[j] == ']' {
                    depth -= 1;
                    if depth == 0 {
                        j += 1;
                        break;
                    }
                }
                j += 1;
            }
            push(Tok::Attr, chars[i..j].iter().collect());
            i = j;
            continue;
        }
        // view! tags: <Name or </name
        if c == '<'
            && chars
                .get(i + 1)
                .is_some_and(|n| n.is_ascii_alphabetic() || *n == '/')
        {
            let mut j = i + 1;
            if chars.get(j) == Some(&'/') {
                j += 1;
            }
            let k = j;
            while j < chars.len()
                && (chars[j].is_ascii_alphanumeric() || chars[j] == '_' || chars[j] == '-')
            {
                j += 1;
            }
            // `<T>` in generics is short and CamelCase-only; treat lower-case names and known
            // components as tags.
            let name: String = chars[k..j].iter().collect();
            if !name.is_empty()
                && (name.chars().next().is_some_and(|f| f.is_ascii_lowercase()) || j - k > 2)
            {
                push(Tok::Tag, chars[i..j].iter().collect());
                i = j;
                continue;
            }
        }
        if c.is_ascii_digit() {
            let mut j = i;
            while j < chars.len()
                && (chars[j].is_ascii_alphanumeric() || chars[j] == '_' || chars[j] == '.')
            {
                j += 1;
            }
            push(Tok::Num, chars[i..j].iter().collect());
            i = j;
            continue;
        }
        if c.is_alphabetic() || c == '_' {
            let mut j = i;
            while j < chars.len() && (chars[j].is_alphanumeric() || chars[j] == '_') {
                j += 1;
            }
            let word: String = chars[i..j].iter().collect();
            let t = if chars.get(j) == Some(&'!') && chars.get(j + 1) != Some(&'=') {
                j += 1;
                Tok::Mac
            } else if KEYWORDS.contains(&word.as_str()) {
                Tok::Kw
            } else if word.chars().next().is_some_and(|f| f.is_ascii_uppercase()) {
                Tok::Ty
            } else {
                Tok::Plain
            };
            push(t, chars[i..j].iter().collect());
            i = j;
            continue;
        }
        push(Tok::Plain, c.to_string());
        i += 1;
    }
    out
}

/// The excerpt as escaped HTML with `<span class="hl-…">` runs. Built as one string (set with
/// `inner_html`) rather than one view per token, so a long excerpt doesn't add hundreds of
/// islands-router branch markers to the page. The input is this repository's own source.
fn highlight(src: &str) -> String {
    let mut html = String::with_capacity(src.len() * 2);
    for (t, s) in tokens(src) {
        let text = escape(&s);
        match t {
            Tok::Plain => html.push_str(&text),
            t => {
                html.push_str("<span class=\"");
                html.push_str(t.class());
                html.push_str("\">");
                html.push_str(&text);
                html.push_str("</span>");
            }
        }
    }
    html
}

fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            c => out.push(c),
        }
    }
    out
}
