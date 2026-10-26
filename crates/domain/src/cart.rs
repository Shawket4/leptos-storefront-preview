//! The cart as the browser keeps it: the `cart` cookie. The client owns it (adds, removes and
//! changes quantities locally — `client/src/stores/cart.ts`); the server reads the same cookie when
//! it needs to price the cart (/cart, the drawer's quote, checkout) and writes it only for no-JS
//! forms and when an order is placed (cleared).
//!
//! Format — keep identical to `client/src/stores/cart.ts`:
//! lines joined by `|`, fields by `:`, add-on ids by `+`:
//! `product:variant:qty[:addon+addon]`, e.g. `2307:1:2|2501:3:1:as-is+pro-install`.

use crate::catalog::{ProductId, VariantId};

pub const COOKIE: &str = "cart";
/// At most this many lines, and this many of one line (the client enforces them too).
pub const MAX_LINES: usize = 50;
pub const MAX_QTY: u32 = 99;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CartLine {
    pub product: ProductId,
    pub variant: VariantId,
    pub qty: u32,
    /// Add-on (modifier option) ids, sorted.
    pub addons: Vec<String>,
}

impl CartLine {
    /// The line's identity: `product:variant[:addons]` (same as the client's `lineKey`).
    pub fn key(&self) -> String {
        let mut k = format!("{}:{}", self.product.0, self.variant.0);
        if !self.addons.is_empty() {
            k.push(':');
            k.push_str(&self.addons.join("+"));
        }
        k
    }
}

fn addon_ok(a: &str) -> bool {
    !a.is_empty() && a.len() <= 40 && a.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
}

/// Parse the cookie value (already percent-decoded). Bad lines are skipped, limits applied,
/// duplicate keys merged — a tampered cookie can't do more than an honest one.
pub fn parse(raw: &str) -> Vec<CartLine> {
    let mut lines: Vec<CartLine> = Vec::new();
    for part in raw.split('|') {
        let mut f = part.split(':');
        let (Some(p), Some(v), Some(q)) = (f.next(), f.next(), f.next()) else { continue };
        let (Ok(p), Ok(v), Ok(q)) = (p.parse::<u32>(), v.parse::<u32>(), q.parse::<u32>()) else { continue };
        if q == 0 {
            continue;
        }
        let mut addons: Vec<String> = f.next().map(|a| a.split('+').filter(|a| addon_ok(a)).map(str::to_owned).collect()).unwrap_or_default();
        addons.sort();
        addons.dedup();
        let line = CartLine { product: ProductId(p), variant: VariantId(v), qty: q.min(MAX_QTY), addons };
        let key = line.key();
        if let Some(l) = lines.iter_mut().find(|l| l.key() == key) {
            l.qty = (l.qty + line.qty).min(MAX_QTY);
        } else if lines.len() < MAX_LINES {
            lines.push(line);
        }
    }
    lines
}

pub fn serialize(lines: &[CartLine]) -> String {
    lines
        .iter()
        .map(|l| {
            let mut s = format!("{}:{}:{}", l.product.0, l.variant.0, l.qty);
            if !l.addons.is_empty() {
                s.push(':');
                s.push_str(&l.addons.join("+"));
            }
            s
        })
        .collect::<Vec<_>>()
        .join("|")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_and_hardening() {
        let raw = "2307:1:2|2501:3:1:pro-install+as-is|2307:1:200|bad|9:9:0|1:1:1:x<y";
        let lines = parse(raw);
        assert_eq!(serialize(&lines), "2307:1:99|2501:3:1:as-is+pro-install|1:1:1");
        assert_eq!(lines[1].key(), "2501:3:as-is+pro-install");
    }
}
