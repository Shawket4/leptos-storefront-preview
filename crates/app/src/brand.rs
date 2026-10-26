//! Tenant colour arithmetic.
//!
//! Origin: a direct port of `MadarDashboard/src/features/public-shell/brand-color.ts`
//! (`luminance`, `contrast`, `mix`, `readableOn`, `brandTokens`). A tenant supplies one brand colour;
//! the page derives a fill that clears WCAG AA (4.5:1) on the light and the dark page ground, plus
//! a label colour that clears AA on the fill. Computed on the server, inlined into `<head>`.

pub const AA: f64 = 4.5;

/// The page grounds the storefront paints (`--page` in `style/tokens.css`). Keep in step.
pub const GROUND_LIGHT: &str = "#EFF3F4";
pub const GROUND_DARK: &str = "#14181E";
const INK_LABEL: &str = "#12222A";

fn rgb(hex: &str) -> Option<[u8; 3]> {
    let h = hex.strip_prefix('#')?;
    if h.len() != 6 {
        return None;
    }
    let p = |i: usize| u8::from_str_radix(&h[i..i + 2], 16).ok();
    Some([p(0)?, p(2)?, p(4)?])
}

pub fn luminance(hex: &str) -> f64 {
    let Some(c) = rgb(hex) else { return 0.0 };
    let ch = |v: u8| {
        let c = v as f64 / 255.0;
        if c <= 0.03928 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
    };
    0.2126 * ch(c[0]) + 0.7152 * ch(c[1]) + 0.0722 * ch(c[2])
}

pub fn contrast(a: &str, b: &str) -> f64 {
    let (la, lb) = (luminance(a), luminance(b));
    let (hi, lo) = if la > lb { (la, lb) } else { (lb, la) };
    (hi + 0.05) / (lo + 0.05)
}

pub fn mix(a: &str, b: &str, t: f64) -> String {
    let (Some(x), Some(y)) = (rgb(a), rgb(b)) else { return a.to_owned() };
    let c = |i: usize| (x[i] as f64 + (y[i] as f64 - x[i] as f64) * t).round() as u8;
    format!("#{:02x}{:02x}{:02x}", c(0), c(1), c(2))
}

/// Walk `color` toward black (light ground) or white (dark ground) until it reads as text on
/// `ground`, keeping the hue.
pub fn readable_on(color: &str, ground: &str) -> String {
    if rgb(color).is_none() || rgb(ground).is_none() || contrast(color, ground) >= AA {
        return color.to_owned();
    }
    let target = if luminance(ground) > 0.5 { "#000000" } else { "#ffffff" };
    let mut out = color.to_owned();
    let mut t = 0.08;
    while t <= 1.0001 {
        out = mix(color, target, t);
        if contrast(&out, ground) >= AA {
            break;
        }
        t += 0.08;
    }
    out
}

fn label_on(fill: &str) -> &'static str {
    if contrast("#FFFFFF", fill) >= contrast(INK_LABEL, fill) { "#FFFFFF" } else { INK_LABEL }
}

/// `(fill, label)` for one theme: the fill reads on the page ground, the label reads on the fill.
pub fn brand_tokens(color: &str, dark: bool) -> (String, &'static str) {
    let ground = if dark { GROUND_DARK } else { GROUND_LIGHT };
    let walked = readable_on(color, ground);
    let label = label_on(&walked);
    let away = if label == "#FFFFFF" { "#000000" } else { "#FFFFFF" };
    let mut fill = walked.clone();
    let mut t = 0.04;
    while contrast(label, &fill) < AA && t <= 1.0001 {
        fill = mix(&walked, away, t);
        t += 0.04;
    }
    (fill, label)
}

/// The inline `<style>` that skins the page for a tenant (both themes), computed per request.
pub fn tenant_css(accent: &str, accent_dark: &str) -> String {
    let (l_fill, l_label) = brand_tokens(accent, false);
    let (d_fill, d_label) = brand_tokens(accent_dark, true);
    format!(
        ":root{{--brand-l:{l_fill};--on-brand-l:{l_label};--brand-d:{d_fill};--on-brand-d:{d_label};\
         --brand-src-l:{accent};--brand-src-d:{accent_dark}}}"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokens_clear_aa() {
        for (c, dark) in [("#0D6273", false), ("#2E94A6", true), ("#A8431E", false), ("#F08A5D", true), ("#F6C445", false)] {
            let (fill, label) = brand_tokens(c, dark);
            let ground = if dark { GROUND_DARK } else { GROUND_LIGHT };
            assert!(contrast(&fill, ground) >= AA, "{c} on ground");
            assert!(contrast(label, &fill) >= AA, "{c} label");
        }
    }
}
