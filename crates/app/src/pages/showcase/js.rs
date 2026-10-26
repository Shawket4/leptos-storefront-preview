//! A few browser APIs the showcase islands need that the app's `web-sys` feature list doesn't
//! cover (Performance, fetch `Response` headers and body streams, XPath). Reached through
//! `js_sys::Reflect` so no new crate features or dependencies are needed, plus a tiny
//! Promise → Future adapter (the app doesn't depend on `wasm-bindgen-futures` directly).
//!
//! Browser only (`hydrate`); islands call these from effects and event handlers.

use js_sys::{Array, Function, Object, Promise, Reflect};
use std::{
    cell::RefCell,
    future::Future,
    pin::Pin,
    rc::Rc,
    task::{Context, Poll, Waker},
};
use wasm_bindgen::{JsCast, JsValue, closure::Closure};

// ─── Reflection helpers ────────────────────────────────────────────────────────────────────────

pub fn global() -> JsValue {
    js_sys::global().into()
}

pub fn get(o: &JsValue, key: &str) -> JsValue {
    Reflect::get(o, &JsValue::from_str(key)).unwrap_or(JsValue::UNDEFINED)
}

pub fn set(o: &JsValue, key: &str, v: &JsValue) {
    let _ = Reflect::set(o, &JsValue::from_str(key), v);
}

pub fn call(o: &JsValue, method: &str, args: &[JsValue]) -> Result<JsValue, JsValue> {
    let f: Function = get(o, method).dyn_into()?;
    f.apply(o, &args.iter().cloned().collect::<Array>())
}

pub fn num(o: &JsValue, key: &str) -> f64 {
    get(o, key).as_f64().unwrap_or(0.0)
}

pub fn string(o: &JsValue, key: &str) -> String {
    get(o, key).as_string().unwrap_or_default()
}

pub fn object(pairs: &[(&str, JsValue)]) -> JsValue {
    let o: JsValue = Object::new().into();
    for (k, v) in pairs {
        set(&o, k, v);
    }
    o
}

// ─── Promise → Future ──────────────────────────────────────────────────────────────────────────

struct Slot {
    result: Option<Result<JsValue, JsValue>>,
    waker: Option<Waker>,
}

/// Await a JS promise from Rust.
pub struct Await(Rc<RefCell<Slot>>);

impl Future for Await {
    type Output = Result<JsValue, JsValue>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut slot = self.0.borrow_mut();
        match slot.result.take() {
            Some(r) => Poll::Ready(r),
            None => {
                slot.waker = Some(cx.waker().clone());
                Poll::Pending
            }
        }
    }
}

pub fn wait(p: JsValue) -> Await {
    let slot = Rc::new(RefCell::new(Slot {
        result: None,
        waker: None,
    }));
    let settle = |ok: bool| {
        let slot = slot.clone();
        // `once_into_js` frees itself when called; the branch that never fires is a few bytes.
        Closure::once_into_js(move |v: JsValue| {
            let waker = {
                let mut s = slot.borrow_mut();
                s.result = Some(if ok { Ok(v) } else { Err(v) });
                s.waker.take()
            };
            if let Some(w) = waker {
                w.wake();
            }
        })
    };
    let p: JsValue = if p.is_instance_of::<Promise>() {
        p
    } else {
        Promise::resolve(&p).into()
    };
    let _ = call(&p, "then", &[settle(true), settle(false)]);
    Await(slot)
}

// ─── Time ──────────────────────────────────────────────────────────────────────────────────────

fn performance() -> JsValue {
    get(&global(), "performance")
}

/// `performance.now()` — ms since this document started loading.
pub fn now() -> f64 {
    call(&performance(), "now", &[])
        .ok()
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0)
}

/// `performance.timeOrigin` — Unix ms when this document started loading. It only changes on a
/// real page load, so it proves a navigation was not one.
pub fn time_origin() -> f64 {
    num(&performance(), "timeOrigin")
}

/// Let the resource-timing buffer hold more than the default 250 entries (this page loads many
/// thumbnails, and every client navigation adds one).
pub fn grow_resource_buffer() {
    let _ = call(
        &performance(),
        "setResourceTimingBufferSize",
        &[JsValue::from_f64(1500.0)],
    );
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Timing {
    pub name: String,
    pub initiator: String,
    pub start: f64,
    pub duration: f64,
    /// Bytes on the wire incl. headers (0 when served from the HTTP cache).
    pub transfer: f64,
    /// Body bytes as sent (compressed).
    pub encoded: f64,
    /// Body bytes after decompression.
    pub decoded: f64,
}

/// `performance.getEntriesByType("resource")`.
pub fn resources() -> Vec<Timing> {
    let Ok(list) = call(
        &performance(),
        "getEntriesByType",
        &[JsValue::from_str("resource")],
    ) else {
        return vec![];
    };
    Array::from(&list)
        .iter()
        .map(|e| Timing {
            name: string(&e, "name"),
            initiator: string(&e, "initiatorType"),
            start: num(&e, "startTime"),
            duration: num(&e, "duration"),
            transfer: num(&e, "transferSize"),
            encoded: num(&e, "encodedBodySize"),
            decoded: num(&e, "decodedBodySize"),
        })
        .collect()
}

// ─── DOM counting ──────────────────────────────────────────────────────────────────────────────

/// Evaluate an XPath number expression against the document, e.g. `count(//comment())`.
pub fn xpath_count(expr: &str) -> f64 {
    let doc: JsValue = leptos::prelude::document().into();
    // XPathResult.NUMBER_TYPE = 1
    call(
        &doc,
        "evaluate",
        &[
            JsValue::from_str(expr),
            doc.clone(),
            JsValue::NULL,
            JsValue::from_f64(1.0),
        ],
    )
    .map(|r| num(&r, "numberValue"))
    .unwrap_or(0.0)
}

// ─── fetch ─────────────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Head {
    pub status: u16,
    /// The requested headers' values, in order.
    pub values: Vec<Option<String>>,
    /// Time to response headers, ms.
    pub ms: f64,
}

fn init(method: &str, headers: &[(&str, &str)]) -> JsValue {
    let h = object(
        &headers
            .iter()
            .map(|(k, v)| (*k, JsValue::from_str(v)))
            .collect::<Vec<_>>(),
    );
    object(&[
        ("method", JsValue::from_str(method)),
        ("headers", h),
        ("cache", JsValue::from_str("no-store")),
        ("credentials", JsValue::from_str("same-origin")),
        ("redirect", JsValue::from_str("manual")),
    ])
}

/// `fetch(url, {method})` and read some response headers. Resolves when the headers arrive (a
/// streamed page's body keeps coming; for `HEAD` there is none).
pub async fn head(url: &str, method: &str, names: &[&str]) -> Result<Head, String> {
    let t0 = now();
    let resp = wait(
        call(
            &global(),
            "fetch",
            &[JsValue::from_str(url), init(method, &[])],
        )
        .map_err(err)?,
    )
    .await
    .map_err(err)?;
    let ms = now() - t0;
    let headers = get(&resp, "headers");
    let values = names
        .iter()
        .map(|n| {
            call(&headers, "get", &[JsValue::from_str(n)])
                .ok()
                .and_then(|v| v.as_string())
        })
        .collect();
    // Drain/cancel the body so the connection is released.
    if let Ok(body) = get(&resp, "body").dyn_into::<Object>() {
        let _ = call(&body, "cancel", &[]);
    }
    Ok(Head {
        status: num(&resp, "status") as u16,
        values,
        ms,
    })
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Chunk {
    /// ms after the request started.
    pub at: f64,
    /// Decompressed bytes in this read.
    pub bytes: f64,
    /// Out-of-order chunks (`<template id=…>`) that completed in this read.
    pub templates: u32,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Stream {
    pub status: u16,
    pub chunks: Vec<Chunk>,
    pub total_ms: f64,
    pub bytes: f64,
    /// `<!--bo-…-->` / `<!--bc-…-->` comments and their bytes.
    pub markers: u32,
    pub marker_bytes: f64,
    pub templates: u32,
}

/// Fetch a page and record when each piece of its streamed body arrives.
pub async fn stream(url: &str, headers: &[(&str, &str)]) -> Result<Stream, String> {
    let t0 = now();
    let resp = wait(
        call(
            &global(),
            "fetch",
            &[JsValue::from_str(url), init("GET", headers)],
        )
        .map_err(err)?,
    )
    .await
    .map_err(err)?;
    let mut out = Stream {
        status: num(&resp, "status") as u16,
        ..Default::default()
    };
    let body = get(&resp, "body");
    let reader = call(&body, "getReader", &[]).map_err(err)?;
    let decoder = Reflect::construct(
        &get(&global(), "TextDecoder")
            .dyn_into::<Function>()
            .map_err(err)?,
        &Array::new(),
    )
    .map_err(err)?;
    let opts = object(&[("stream", JsValue::TRUE)]);
    const CLOSE: &str = "</template>";
    let mut text = String::new();
    // Next byte to search for a closing tag; a tag split across two reads is found next time.
    let mut search_from = 0usize;
    loop {
        let r = wait(call(&reader, "read", &[]).map_err(err)?)
            .await
            .map_err(err)?;
        if get(&r, "done").as_bool().unwrap_or(true) {
            break;
        }
        let value = get(&r, "value");
        let bytes = num(&value, "byteLength");
        let piece = call(&decoder, "decode", &[value, opts.clone()])
            .ok()
            .and_then(|v| v.as_string())
            .unwrap_or_default();
        text.push_str(&piece);
        let mut templates = 0;
        while let Some(i) = text[search_from..].find(CLOSE) {
            templates += 1;
            search_from += i + CLOSE.len();
        }
        search_from = search_from.max(floor_char(&text, text.len().saturating_sub(CLOSE.len())));
        out.templates += templates;
        out.bytes += bytes;
        out.chunks.push(Chunk {
            at: now() - t0,
            bytes,
            templates,
        });
    }
    out.total_ms = now() - t0;
    let (n, b) = markers(&text);
    out.markers = n;
    out.marker_bytes = b as f64;
    Ok(out)
}

fn floor_char(s: &str, mut i: usize) -> usize {
    while i > 0 && !s.is_char_boundary(i) {
        i -= 1;
    }
    i
}

/// Count islands-router branch markers (`<!--bo-…-->`, `<!--bc-…-->`) and their bytes.
fn markers(html: &str) -> (u32, usize) {
    let mut n = 0;
    let mut bytes = 0;
    let mut rest = html;
    while let Some(i) = rest.find("<!--b") {
        let tail = &rest[i..];
        let is_marker = tail
            .as_bytes()
            .get(5)
            .is_some_and(|c| *c == b'o' || *c == b'c')
            && tail.as_bytes().get(6) == Some(&b'-');
        match tail.find("-->") {
            Some(end) => {
                if is_marker {
                    n += 1;
                    bytes += end + 3;
                }
                rest = &tail[end + 3..];
            }
            None => break,
        }
    }
    (n, bytes)
}

fn err(e: JsValue) -> String {
    e.as_string()
        .or_else(|| js_sys::JSON::stringify(&e).ok().and_then(|s| s.as_string()))
        .unwrap_or_else(|| "request failed".into())
}

// ─── Feature detection ─────────────────────────────────────────────────────────────────────────

pub fn has_view_transitions() -> bool {
    let doc: JsValue = leptos::prelude::document().into();
    Reflect::has(&doc, &JsValue::from_str("startViewTransition")).unwrap_or(false)
}

pub fn matches_media(q: &str) -> bool {
    leptos::prelude::window()
        .match_media(q)
        .ok()
        .flatten()
        .is_some_and(|m| m.matches())
}
