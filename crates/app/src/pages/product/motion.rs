//! A small spring animator for the gallery, in Apple's terms (damping ratio + response), local
//! until `components::motion` lands (then swap and delete).
//!
//! - Springs integrate from the *live* value and velocity, so any animation can be grabbed and
//!   redirected mid-flight (interruptible) and a release hands its velocity to the spring.
//! - `project` is Apple's momentum projection, `rubberband` the edge resistance.
//! - `Frames` runs a requestAnimationFrame loop with high-resolution timestamps until the frame
//!   callback says it is done, or until it is stopped (a new gesture grabbed the value).

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpringCfg {
    /// 1.0 = critically damped (no overshoot); < 1 overshoots.
    pub damping: f64,
    /// Seconds to (visually) reach the target; not a duration.
    pub response: f64,
}

impl SpringCfg {
    /// The house default: moves and repositions.
    pub const DEFAULT: SpringCfg = SpringCfg {
        damping: 1.0,
        response: 0.35,
    };
    /// Small, quick corrections (zoom snapping back, a tap jump).
    pub const SNAPPY: SpringCfg = SpringCfg {
        damping: 1.0,
        response: 0.25,
    };
    /// Only after a flick: a little life at the landing.
    pub const FLICK: SpringCfg = SpringCfg {
        damping: 0.86,
        response: 0.32,
    };

    fn stiffness(self) -> f64 {
        let w = std::f64::consts::TAU / self.response.max(0.01);
        w * w
    }
    fn friction(self) -> f64 {
        4.0 * std::f64::consts::PI * self.damping / self.response.max(0.01)
    }
}

/// One animated dimension (decompose 2-D motion into independent springs).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Spring {
    pub x: f64,
    /// Units per second.
    pub v: f64,
    pub target: f64,
}

impl Spring {
    pub fn at(x: f64) -> Self {
        Spring {
            x,
            v: 0.0,
            target: x,
        }
    }

    pub fn settled(&self) -> bool {
        (self.x - self.target).abs() < 0.25 && self.v.abs() < 6.0
    }

    /// Advance by `dt` seconds (sub-stepped for stability). Returns `true` once at rest.
    pub fn step(&mut self, cfg: SpringCfg, dt: f64) -> bool {
        let (k, c) = (cfg.stiffness(), cfg.friction());
        let mut left = dt;
        while left > 0.0 {
            let h = left.min(1.0 / 240.0);
            let a = -k * (self.x - self.target) - c * self.v;
            self.v += a * h;
            self.x += self.v * h;
            left -= h;
        }
        if self.settled() {
            self.x = self.target;
            self.v = 0.0;
            true
        } else {
            false
        }
    }

    /// Same, for a spring whose unit is not pixels (scale): `eps` is the rest tolerance.
    pub fn step_eps(&mut self, cfg: SpringCfg, dt: f64, eps: f64) -> bool {
        let (k, c) = (cfg.stiffness(), cfg.friction());
        let mut left = dt;
        while left > 0.0 {
            let h = left.min(1.0 / 240.0);
            let a = -k * (self.x - self.target) - c * self.v;
            self.v += a * h;
            self.x += self.v * h;
            left -= h;
        }
        if (self.x - self.target).abs() < eps && self.v.abs() < eps * 20.0 {
            self.x = self.target;
            self.v = 0.0;
            true
        } else {
            false
        }
    }
}

/// Where a flick at `v` (units/s) comes to rest: `(v/1000)·d/(1−d)`, `d ≈ 0.998` (scroll feel).
pub fn project(v: f64, decel: f64) -> f64 {
    (v / 1000.0) * decel / (1.0 - decel)
}

/// Progressive resistance past an edge: the further past, the less it follows.
pub fn rubberband(overshoot: f64, dimension: f64) -> f64 {
    const C: f64 = 0.55;
    if dimension <= 0.0 {
        return 0.0;
    }
    (overshoot * dimension * C) / (dimension + C * overshoot.abs())
}

/// Velocity from the last ~100 ms of pointer samples (`(timestamp ms, position)`).
#[derive(Clone, Debug, Default)]
pub struct Tracker {
    samples: Vec<(f64, f64)>,
}

impl Tracker {
    pub fn reset(&mut self, t: f64, x: f64) {
        self.samples.clear();
        self.samples.push((t, x));
    }

    pub fn push(&mut self, t: f64, x: f64) {
        self.samples.push((t, x));
        let cutoff = t - 100.0;
        if self.samples.len() > 2 {
            let keep_from = self
                .samples
                .iter()
                .position(|(ts, _)| *ts >= cutoff)
                .unwrap_or(0)
                .min(self.samples.len() - 2);
            self.samples.drain(..keep_from);
        }
    }

    /// Units per second; 0 if the pointer rested (>80 ms since the last move counts as a stop).
    pub fn velocity(&self, now: f64) -> f64 {
        let (Some(first), Some(last)) = (self.samples.first(), self.samples.last()) else {
            return 0.0;
        };
        let dt = last.0 - first.0;
        if dt <= 0.0 || now - last.0 > 80.0 {
            return 0.0;
        }
        (last.1 - first.1) / dt * 1000.0
    }
}

/// A requestAnimationFrame loop. `frame(dt_seconds)` returns `true` to keep going.
///
/// `Send + Sync` (an atomic generation, not an `Rc`), so the state holding it can live in an
/// ordinary `StoredValue`: island bodies also run during SSR on a multi-threaded runtime, where a
/// thread-local value dropped by another worker panics.
#[derive(Clone, Debug, Default)]
pub struct Frames {
    generation: Arc<AtomicU64>,
}

impl Frames {
    /// Stop whatever loop is running (the value stays where it is on screen).
    pub fn stop(&self) {
        self.generation.fetch_add(1, Ordering::Relaxed);
    }

    /// Replace any running loop with this one.
    pub fn start(&self, frame: impl FnMut(f64) -> bool + 'static) {
        self.stop();
        let mine = self.generation.load(Ordering::Relaxed);
        schedule(self.generation.clone(), mine, None, Box::new(frame));
    }
}

fn schedule(
    generation: Arc<AtomicU64>,
    mine: u64,
    last: Option<f64>,
    mut frame: Box<dyn FnMut(f64) -> bool>,
) {
    use wasm_bindgen::{JsCast, closure::Closure};
    let cb = Closure::once_into_js(move |ts: f64| {
        if generation.load(Ordering::Relaxed) != mine {
            return;
        }
        // The first frame only fixes the clock; cap dt so a background tab doesn't teleport.
        let dt = last
            .map(|l| ((ts - l) / 1000.0).clamp(0.0, 1.0 / 30.0))
            .unwrap_or(1.0 / 120.0);
        if frame(dt) {
            schedule(generation, mine, Some(ts), frame);
        }
    });
    if let Some(w) = web_sys::window() {
        let _ = w.request_animation_frame(cb.unchecked_ref());
    }
}

/// `prefers-reduced-motion: reduce` (springs become instant jumps / cross-fades).
pub fn reduced_motion() -> bool {
    web_sys::window()
        .and_then(|w| {
            w.match_media("(prefers-reduced-motion: reduce)")
                .ok()
                .flatten()
        })
        .map(|m| m.matches())
        .unwrap_or(false)
}
