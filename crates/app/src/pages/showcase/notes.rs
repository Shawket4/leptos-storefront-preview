//! The showcase's own server functions: the "rate this showcase" board behind the ActionForm demo
//! (an in-memory list per tenant, the last 40 notes; it resets when the server restarts), and the
//! flaky read behind the `net::retry` demo.
//!
//! Shaped for `<ActionForm>`: flat, form-encodable arguments (`rating`, `note`). Without JS the
//! browser posts the form to the function's URL and server_fn redirects back to the Referer (an
//! error travels back in `?__err=`); with JS the island dispatches it and gets the `Board` back.

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

pub const MAX_NOTE: usize = 140;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Note {
    pub rating: u8,
    pub text: String,
    /// Unix ms.
    pub at: i64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Board {
    pub count: u32,
    /// Average rating × 10 (e.g. 46 = 4.6), 0 when empty.
    pub avg_x10: u32,
    /// Newest first, at most 3.
    pub recent: Vec<Note>,
}

/// Error codes the island maps to copy.
pub const ERR_RATING: &str = "rating_required";

#[cfg(feature = "ssr")]
mod store {
    use super::*;
    use std::collections::HashMap;
    use std::sync::{LazyLock, Mutex};

    static NOTES: LazyLock<Mutex<HashMap<String, Vec<Note>>>> =
        LazyLock::new(|| Mutex::new(HashMap::new()));

    pub fn board(tenant: &str) -> Board {
        let map = NOTES.lock().unwrap_or_else(|e| e.into_inner());
        let notes = map.get(tenant).map(Vec::as_slice).unwrap_or(&[]);
        let count = notes.len() as u32;
        let sum: u32 = notes.iter().map(|n| n.rating as u32).sum();
        Board {
            count,
            avg_x10: if count == 0 {
                0
            } else {
                (sum * 10 + count / 2) / count
            },
            recent: notes.iter().rev().take(3).cloned().collect(),
        }
    }

    pub fn add(tenant: &str, note: Note) -> Board {
        {
            let mut map = NOTES.lock().unwrap_or_else(|e| e.into_inner());
            let list = map.entry(tenant.to_owned()).or_default();
            list.push(note);
            if list.len() > 40 {
                let extra = list.len() - 40;
                list.drain(..extra);
            }
        }
        board(tenant)
    }
}

/// The board for the current tenant (server components).
#[cfg(feature = "ssr")]
pub fn board_now() -> Board {
    store::board(&crate::request::init().tenant.id)
}

/// Leave a 1–5 rating and an optional note (trimmed to 140 characters).
#[server]
pub async fn leave_note(rating: Option<u8>, note: Option<String>) -> Result<Board, ServerFnError> {
    let rq = crate::request::init();
    crate::server::cache::private();
    let rating = rating
        .filter(|r| (1..=5).contains(r))
        .ok_or_else(|| ServerFnError::new(ERR_RATING))?;
    let text: String = note
        .unwrap_or_default()
        .trim()
        .chars()
        .take(MAX_NOTE)
        .collect();
    let at = crate::server::orders::now_ms();
    Ok(store::add(&rq.tenant.id, Note { rating, text, at }))
}

/// The flaky read behind the retry demo: try `attempt` (1-based) fails while it is within the
/// first `fail_first` (at most 3), then succeeds. The browser numbers its tries, so nothing is
/// kept on the server and two visitors can't disturb each other.
#[server]
pub async fn flaky_read(attempt: u32, fail_first: u32) -> Result<u32, ServerFnError> {
    crate::server::cache::private();
    if attempt <= fail_first.min(3) {
        Err(ServerFnError::new("flaky"))
    } else {
        Ok(attempt)
    }
}
