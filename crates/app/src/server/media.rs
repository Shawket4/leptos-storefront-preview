//! The image manifests (`data/media/<tenant>/manifest.json`), merged into one lookup.

use crate::media::{Img, MediaEntry};
use std::{collections::HashMap, path::Path, sync::RwLock};

/// Reloadable: the dev server picks up new manifests while the asset pipeline is still running.
pub struct Media {
    entries: RwLock<HashMap<String, MediaEntry>>,
}

impl Media {
    pub fn load(dir: &Path) -> Media {
        Media { entries: RwLock::new(Self::read_dir(dir)) }
    }

    pub fn reload(&self, dir: &Path) {
        let fresh = Self::read_dir(dir);
        if let Ok(mut e) = self.entries.write() {
            *e = fresh;
        }
    }

    fn read_dir(dir: &Path) -> HashMap<String, MediaEntry> {
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
        entries
    }

    pub fn len(&self) -> usize {
        self.entries.read().map(|e| e.len()).unwrap_or(0)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn has(&self, key: &str) -> bool {
        self.entries.read().is_ok_and(|e| e.contains_key(key))
    }

    /// The image for a key, or a neutral placeholder so a missing file never breaks a page.
    pub fn img(&self, key: &str) -> Img {
        self.try_img(key).unwrap_or_else(|| Img::missing(key))
    }

    pub fn try_img(&self, key: &str) -> Option<Img> {
        self.entries.read().ok()?.get(key).map(|e| e.to_img(key))
    }

    pub fn entry(&self, key: &str) -> Option<MediaEntry> {
        self.entries.read().ok()?.get(key).cloned()
    }
}
