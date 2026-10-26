//! Islands-era network helpers, kept only until their last callers are ported to the browser code
//! (`client/src/core/net.ts` is the one network layer now: timeouts, retries, offline). Everything
//! here runs on the server, where there is no connection to lose: `is_online` is always true,
//! nothing waits, and a read runs once.

use std::future::Future;

/// How many times a read was tried in all (islands era; documentation for the showcase).
pub const TRIES: u32 = 4;
/// The wait before the second try; doubled after each failure (islands era).
pub const FIRST_DELAY_MS: u64 = 300;
/// The longest wait between tries (islands era).
pub const MAX_DELAY_MS: u64 = 2400;

/// Whether the device says it has a connection: always `true` on the server.
pub fn is_online() -> bool {
    true
}

/// A no-op on the server.
pub async fn sleep(ms: u64) {
    let _ = ms;
}

/// Resolves at once on the server.
pub async fn until_online() {}

/// Run a read once (the server has nothing to retry against).
pub async fn retry<T, E, F, Fut>(f: F) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
{
    retry_if(f, |_| true).await
}

/// Like [`retry`]: the read runs once on the server.
pub async fn retry_if<T, E, F, Fut, P>(mut f: F, should_retry: P) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
    P: Fn(&E) -> bool,
{
    let _ = should_retry;
    f().await
}
