//! Code excerpts on the showcase are cut from this repository's own source at compile time
//! (`include_str!`), between an anchor line and an end line. They can't drift from the code: if an
//! anchor disappears, the excerpt disappears with it (and a debug build logs which one).
//!
//! Server only: the source text is in the server binary, never in the browser bundle.

use leptos::prelude::*;

/// A file the showcase quotes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Src {
    // Rust
    Lib,
    Shell,
    Stream,
    ApiMod,
    Errors,
    Sim,
    Cache,
    Tenant,
    Request,
    ProductHead,
    Toast,
    DomainCart,
    BuildRs,
    ClientJs,
    // TypeScript
    Main,
    Router,
    ClientCache,
    Net,
    Store,
    Toasts,
    CartTs,
    Dom,
    Motion,
    BuildMjs,
    ShowcaseTs,
}

impl Src {
    pub fn path(self) -> &'static str {
        match self {
            Src::Lib => "crates/app/src/lib.rs",
            Src::Shell => "crates/app/src/components/shell.rs",
            Src::Stream => "crates/app/src/pages/showcase/stream.rs",
            Src::ApiMod => "crates/app/src/api/mod.rs",
            Src::Errors => "crates/app/src/pages/errors.rs",
            Src::Sim => "crates/app/src/server/sim.rs",
            Src::Cache => "crates/app/src/server/cache.rs",
            Src::Tenant => "crates/app/src/tenant.rs",
            Src::Request => "crates/app/src/request.rs",
            Src::ProductHead => "crates/app/src/pages/product/head.rs",
            Src::Toast => "crates/app/src/components/toast.rs",
            Src::DomainCart => "crates/domain/src/cart.rs",
            Src::BuildRs => "crates/app/build.rs",
            Src::ClientJs => "crates/app/src/client_js.rs",
            Src::Main => "client/src/main.ts",
            Src::Router => "client/src/core/router.ts",
            Src::ClientCache => "client/src/core/cache.ts",
            Src::Net => "client/src/core/net.ts",
            Src::Store => "client/src/core/store.ts",
            Src::Toasts => "client/src/stores/toasts.ts",
            Src::CartTs => "client/src/stores/cart.ts",
            Src::Dom => "client/src/core/dom.ts",
            Src::Motion => "client/src/core/motion.ts",
            Src::BuildMjs => "client/build.mjs",
            Src::ShowcaseTs => "client/src/features/showcase.ts",
        }
    }

    #[cfg(feature = "ssr")]
    fn text(self) -> &'static str {
        match self {
            Src::Lib => include_str!("../../lib.rs"),
            Src::Shell => include_str!("../../components/shell.rs"),
            Src::Stream => include_str!("stream.rs"),
            Src::ApiMod => include_str!("../../api/mod.rs"),
            Src::Errors => include_str!("../errors.rs"),
            Src::Sim => include_str!("../../server/sim.rs"),
            Src::Cache => include_str!("../../server/cache.rs"),
            Src::Tenant => include_str!("../../tenant.rs"),
            Src::Request => include_str!("../../request.rs"),
            Src::ProductHead => include_str!("../product/head.rs"),
            Src::Toast => include_str!("../../components/toast.rs"),
            Src::DomainCart => include_str!("../../../../domain/src/cart.rs"),
            Src::BuildRs => include_str!("../../../build.rs"),
            Src::ClientJs => include_str!("../../client_js.rs"),
            Src::Main => include_str!("../../../../../client/src/main.ts"),
            Src::Router => include_str!("../../../../../client/src/core/router.ts"),
            Src::ClientCache => include_str!("../../../../../client/src/core/cache.ts"),
            Src::Net => include_str!("../../../../../client/src/core/net.ts"),
            Src::Store => include_str!("../../../../../client/src/core/store.ts"),
            Src::Toasts => include_str!("../../../../../client/src/stores/toasts.ts"),
            Src::CartTs => include_str!("../../../../../client/src/stores/cart.ts"),
            Src::Dom => include_str!("../../../../../client/src/core/dom.ts"),
            Src::Motion => include_str!("../../../../../client/src/core/motion.ts"),
            Src::BuildMjs => include_str!("../../../../../client/build.mjs"),
            Src::ShowcaseTs => include_str!("../../../../../client/src/features/showcase.ts"),
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

#[cfg(not(feature = "ssr"))]
pub fn excerpt(_: Cut) -> Option<Excerpt> {
    None
}

/// A short, real excerpt in a collapsible block: `path:line` as the summary, scrollable LTR code.
#[component]
pub fn Code(cut: Cut, lang: crate::prefs::Lang, #[prop(optional)] open: bool) -> impl IntoView {
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
    "export",
    "import",
    "from",
    "interface",
    "class",
    "extends",
    "private",
    "readonly",
    "this",
    "void",
    "undefined",
    "keyof",
    "declare",
    "satisfies",
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
/// `inner_html`) rather than one view per token, so a long excerpt stays one text node of markup.
/// The input is this repository's own source, escaped here.
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
