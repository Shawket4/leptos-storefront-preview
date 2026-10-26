use serde::{Deserialize, Serialize};

/// A string in both storefront languages.
///
/// Origin: Madar's `name_translations: {"en": …, "ar": …}` on categories, items and addons. Madar
/// keeps a primary `name` plus the translations map; the storefront always has both languages,
/// so it is flattened into two fields.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct L10n {
    pub en: String,
    pub ar: String,
}

impl L10n {
    pub fn new(en: impl Into<String>, ar: impl Into<String>) -> Self {
        L10n { en: en.into(), ar: ar.into() }
    }

    /// `ar == true` picks Arabic. (The UI crate wraps this with its `Lang` enum.)
    pub fn pick(&self, ar: bool) -> &str {
        if ar { &self.ar } else { &self.en }
    }
}
