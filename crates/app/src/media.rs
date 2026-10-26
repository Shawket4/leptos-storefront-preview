//! Responsive images.
//!
//! Every image is a *key* (`nile/p1001-1`) with a set of WebP widths on disk
//! (`data/media/nile/p1001-1-640.webp`, served at `/media/...`). Madar's own asset pipeline also
//! stores WebP (q80, thumb/tile/full variants, `MadarRust/src/assets/ingest.rs`); here the widths
//! form a `srcset` ladder instead of three named variants.
//!
//! [`Img`] is the serializable, self-contained description a component (or island prop, or server
//! function response) needs to render one image: no manifest lookup on the client.

use crate::prefs::Lang;
use domain::L10n;
use serde::{Deserialize, Serialize};

pub const MEDIA_BASE: &str = "/media";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Img {
    pub key: String,
    pub widths: Vec<u16>,
    /// width / height
    pub aspect: f32,
    /// Transparent packshot (the UI provides the backdrop).
    pub alpha: bool,
    /// Tiny blurred WebP as a data URI.
    pub lqip: String,
    /// Dominant colour `#rrggbb`, for the placeholder tint.
    pub color: String,
    pub alt: L10n,
}

impl Img {
    pub fn url(&self, w: u16) -> String {
        format!("{MEDIA_BASE}/{}-{w}.webp", self.key)
    }

    /// `"/media/k-240.webp 240w, /media/k-400.webp 400w, …"`.
    pub fn srcset(&self) -> String {
        self.widths
            .iter()
            .map(|w| format!("{} {w}w", self.url(*w)))
            .collect::<Vec<_>>()
            .join(", ")
    }

    /// The `src` fallback: the rung closest to `target` px wide.
    pub fn src_near(&self, target: u16) -> String {
        let w = self
            .widths
            .iter()
            .copied()
            .min_by_key(|w| (*w as i32 - target as i32).abs())
            .unwrap_or(400);
        self.url(w)
    }

    pub fn largest(&self) -> String {
        self.url(self.widths.iter().copied().max().unwrap_or(960))
    }

    pub fn alt(&self, lang: Lang) -> &str {
        self.alt.pick(lang.is_rtl())
    }

    /// A neutral stand-in for a missing key, so a typo never breaks a page.
    pub fn missing(key: &str) -> Img {
        Img {
            key: key.to_owned(),
            widths: vec![],
            aspect: 1.0,
            alpha: false,
            lqip: String::new(),
            color: "#dfe4e5".into(),
            alt: L10n::default(),
        }
    }

    pub fn is_missing(&self) -> bool {
        self.widths.is_empty()
    }
}

/// One entry of `data/media/<tenant>/manifest.json` (written by `tools/media/process.py`).
#[derive(Clone, Debug, Deserialize)]
pub struct MediaEntry {
    pub kind: String,
    pub widths: Vec<u16>,
    pub aspect: f32,
    pub alpha: bool,
    pub lqip: String,
    pub color: String,
    pub alt: L10n,
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub license: String,
    #[serde(default)]
    pub credit: String,
}

impl MediaEntry {
    pub fn to_img(&self, key: &str) -> Img {
        Img {
            key: key.to_owned(),
            widths: self.widths.clone(),
            aspect: self.aspect,
            alpha: self.alpha,
            lqip: self.lqip.clone(),
            color: self.color.clone(),
            alt: self.alt.clone(),
        }
    }
}
