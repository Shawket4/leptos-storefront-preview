//! The image manifests (`data/media/<tenant>/manifest.json`), merged into one lookup.

use crate::media::{Img, MediaEntry};
use std::{collections::HashMap, path::Path};

pub struct Media {
    entries: HashMap<String, MediaEntry>,
}

impl Media {
    pub fn load(dir: &Path) -> Media {
        let mut entries = HashMap::new();
        if let Ok(read) = std::fs::read_dir(dir) {
            for sub in read.flatten() {
                let manifest = sub.path().join("manifest.json");
                let Ok(text) = std::fs::read_to_string(&manifest) else { continue };
                match serde_json::from_str::<HashMap<String, MediaEntry>>(&text) {
                    Ok(map) => entries.extend(map),
                    Err(e) => tracing::warn!("{}: {e}", manifest.display()),
                }
            }
        }
        Media { entries }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn has(&self, key: &str) -> bool {
        self.entries.contains_key(key)
    }

    /// The image for a key, or a neutral placeholder so a missing file never breaks a page.
    pub fn img(&self, key: &str) -> Img {
        self.entries.get(key).map(|e| e.to_img(key)).unwrap_or_else(|| Img::missing(key))
    }

    pub fn try_img(&self, key: &str) -> Option<Img> {
        self.entries.get(key).map(|e| e.to_img(key))
    }

    pub fn entry(&self, key: &str) -> Option<&MediaEntry> {
        self.entries.get(key)
    }
}
