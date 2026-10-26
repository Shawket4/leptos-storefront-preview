//! Network helpers for islands: whether the device is online, waiting until it is, sleeping, and
//! retrying an idempotent read with exponential backoff that pauses while offline.
//!
//! ```ignore
//! // In an island: a server-function read that survives a flaky connection.
//! let view = LocalResource::new(move || net::retry(|| api::cart::cart_get()));
//! // Only retry what can succeed on a second try:
//! net::retry_if(|| api::product_detail(slug.clone()), |e| !e.to_string().contains("not_found")).await
//! ```
//!
//! Backoff: 300 ms, 600 ms, 1.2 s (capped at 2.4 s), at most 4 tries. While the device reports
//! itself offline no try is spent: the helper waits for the `online` event and goes on. On the
//! server there is nothing to wait for: the read runs once.
//!
//! All futures here are `Send` (shared state is `Arc<Mutex<_>>`, listener handles are `Send`), so
//! they also fit `Resource::new`.

use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

/// How many times a read is tried in all.
pub const TRIES: u32 = 4;
/// The wait before the second try; doubles after each failure.
pub const FIRST_DELAY_MS: u64 = 300;
/// The longest wait between tries.
pub const MAX_DELAY_MS: u64 = 2400;

/// Whether the device says it has a connection (always `true` on the server).
pub fn is_online() -> bool {
    #[cfg(feature = "hydrate")]
    {
        leptos::prelude::window().navigator().on_line()
    }
    #[cfg(not(feature = "hydrate"))]
    {
        true
    }
}

#[derive(Default)]
#[cfg_attr(not(feature = "hydrate"), allow(dead_code))]
struct Flag {
    done: bool,
    waker: Option<Waker>,
}

/// Completes once its flag is fired.
#[cfg_attr(not(feature = "hydrate"), allow(dead_code))]
struct Gate(Arc<Mutex<Flag>>);

impl Future for Gate {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let mut f = self.0.lock().unwrap_or_else(|e| e.into_inner());
        if f.done {
            Poll::Ready(())
        } else {
            f.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

#[cfg(feature = "hydrate")]
fn fire(flag: &Arc<Mutex<Flag>>) {
    let waker = {
        let mut f = flag.lock().unwrap_or_else(|e| e.into_inner());
        f.done = true;
        f.waker.take()
    };
    if let Some(w) = waker {
        w.wake();
    }
}

/// Wait `ms` milliseconds (browser timer; no wait on the server).
pub async fn sleep(ms: u64) {
    #[cfg(feature = "hydrate")]
    {
        let flag = Arc::new(Mutex::new(Flag::default()));
        let f2 = flag.clone();
        leptos::prelude::set_timeout(move || fire(&f2), std::time::Duration::from_millis(ms));
        Gate(flag).await;
    }
    #[cfg(not(feature = "hydrate"))]
    {
        let _ = ms;
    }
}

/// Resolve as soon as the device is online (at once if it already is).
pub async fn until_online() {
    #[cfg(feature = "hydrate")]
    {
        if is_online() {
            return;
        }
        let flag = Arc::new(Mutex::new(Flag::default()));
        let f2 = flag.clone();
        let handle = leptos::prelude::window_event_listener_untyped("online", move |_| fire(&f2));
        Gate(flag).await;
        handle.remove();
    }
}

/// Run an idempotent read, retrying failures with backoff (see the module docs).
pub async fn retry<T, E, F, Fut>(f: F) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
{
    retry_if(f, |_| true).await
}

/// Like [`retry`], but only errors `should_retry` accepts are tried again (a "not found" won't
/// succeed on the second try).
pub async fn retry_if<T, E, F, Fut, P>(mut f: F, should_retry: P) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
    P: Fn(&E) -> bool,
{
    let mut delay = FIRST_DELAY_MS;
    let mut attempt = 1;
    loop {
        // Offline: don't spend a try, wait for the connection.
        until_online().await;
        match f().await {
            Ok(v) => return Ok(v),
            Err(e) if attempt >= TRIES || !should_retry(&e) || cfg!(not(feature = "hydrate")) => {
                return Err(e);
            }
            Err(_) => {
                attempt += 1;
                if is_online() {
                    sleep(delay).await;
                    delay = (delay * 2).min(MAX_DELAY_MS);
                }
            }
        }
    }
}
