//! Springs for gesture-driven UI (drawers, bottom sheets, gallery swipes), after Apple's
//! *Designing Fluid Interfaces*: motion starts from the value that is on screen now, inherits
//! the finger's velocity, projects momentum to where a flick is going, rubber-bands at edges, and
//! can be grabbed and redirected at any instant.
//!
//! ```ignore
//! let x = SpringValue::new(width, move |v| set_transform(v));   // the callback draws each frame
//! x.set(drag_offset);                                           // 1:1 while the finger is down
//! let v = tracker.velocity();                                   // px/s at release
//! let target = if x.get() + project(v, 0.998) > width / 2.0 { width } else { 0.0 };
//! x.animate_to(target, if target == 0.0 { Spring::BOUNCY } else { Spring::DEFAULT }, Some(v), None);
//! ```
//!
//! The spring is solved in closed form each frame, so it is exact at any frame rate; a retarget
//! keeps the current value and velocity, so reversing mid-flight never hits a "brick wall". Under
//! `prefers-reduced-motion` animations land at once.
//!
//! **Islands era.** The browser code's `client/src/core/motion.ts` is the motion system now; this
//! module keeps the spring maths (and an inert `SpringValue`) only for the showcase's remaining
//! Rust references, and goes when they do.

use std::collections::VecDeque;

/// Apple's two designer-facing spring parameters.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Spring {
    /// 1 = critically damped (no overshoot); below 1 overshoots.
    pub damping: f64,
    /// Seconds to (mostly) reach the target. Not a duration: settling emerges from the physics.
    pub response: f64,
}

impl Spring {
    /// The house default (CSS `--spring-default`): no overshoot.
    pub const DEFAULT: Spring = Spring {
        damping: 1.0,
        response: 0.35,
    };
    /// Small, quick things (CSS `--spring-snappy`).
    pub const SNAPPY: Spring = Spring {
        damping: 1.0,
        response: 0.25,
    };
    /// Large surfaces settling (CSS `--spring-gentle`).
    pub const GENTLE: Spring = Spring {
        damping: 1.0,
        response: 0.5,
    };
    /// Only after a flick or throw: a little overshoot (CSS `--spring-bouncy`).
    pub const BOUNCY: Spring = Spring {
        damping: 0.8,
        response: 0.3,
    };

    /// Advance a displacement from the target `x` with velocity `v` by `dt` seconds.
    /// Unit mass, stiffness (2π/response)², damping 4πζ/response — solved exactly.
    pub fn step(&self, x: f64, v: f64, dt: f64) -> (f64, f64) {
        let w0 = std::f64::consts::TAU / self.response.max(0.01);
        let z = self.damping.clamp(0.0, 1.0);
        if z >= 0.9999 {
            // Critically damped: x(t) = (A + B t) e^{-w0 t}
            let b = v + w0 * x;
            let e = (-w0 * dt).exp();
            let nx = (x + b * dt) * e;
            let nv = (v - w0 * b * dt) * e;
            (nx, nv)
        } else {
            // Under-damped
            let wd = w0 * (1.0 - z * z).sqrt();
            let e = (-z * w0 * dt).exp();
            let (s, c) = (wd * dt).sin_cos();
            let a = x;
            let b = (v + z * w0 * x) / wd;
            let nx = e * (a * c + b * s);
            let nv = e * ((b * wd - z * w0 * a) * c - (a * wd + z * w0 * b) * s);
            (nx, nv)
        }
    }
}

/// Where a flick at `velocity` (px/s) would come to rest, Apple's exponential-decay projection.
/// `deceleration` ≈ 0.998 feels like scrolling; 0.99 is snappier.
pub fn project(velocity: f64, deceleration: f64) -> f64 {
    (velocity / 1000.0) * deceleration / (1.0 - deceleration)
}

/// How far an element follows a drag `overshoot` px past an edge: progressively less, never a
/// hard stop. `dimension` is the size of the element along the drag; `constant` 0.55 is Apple's.
pub fn rubberband(overshoot: f64, dimension: f64, constant: f64) -> f64 {
    let d = dimension.max(1.0);
    (overshoot * d * constant) / (d + constant * overshoot.abs())
}

/// Velocity from a short history of pointer positions (the last 100ms), in px/s.
#[derive(Clone, Debug, Default)]
pub struct VelocityTracker {
    samples: VecDeque<(f64, f64)>,
}

impl VelocityTracker {
    pub fn reset(&mut self) {
        self.samples.clear();
    }

    /// Record a position at a time in milliseconds (an event's `timeStamp`).
    pub fn push(&mut self, t_ms: f64, pos: f64) {
        self.samples.push_back((t_ms, pos));
        while self.samples.len() > 2 && self.samples.front().is_some_and(|(t, _)| t_ms - t > 100.0)
        {
            self.samples.pop_front();
        }
        if self.samples.len() > 8 {
            self.samples.pop_front();
        }
    }

    /// px/s over the recorded window (0 if the finger stopped before letting go).
    pub fn velocity(&self) -> f64 {
        let (Some((t0, p0)), Some((t1, p1))) = (self.samples.front(), self.samples.back()) else {
            return 0.0;
        };
        let dt = t1 - t0;
        if dt < 1.0 {
            0.0
        } else {
            (p1 - p0) / dt * 1000.0
        }
    }
}

/// Whether the visitor asked for less motion: unknown on the server, so `false`.
pub fn reduced() -> bool {
    false
}

/// Islands-era spring handle, inert: the server never animates. Gesture motion is
/// `client/src/core/motion.ts` (`SpringValue`) now; this stays only until the last Rust caller
/// (the showcase) is ported, then the module goes.
#[derive(Clone, Copy)]
pub struct SpringValue {}

impl SpringValue {
    pub fn new(_initial: f64, _on_update: impl Fn(f64) + 'static) -> Self {
        SpringValue {}
    }
    pub fn get(&self) -> f64 {
        0.0
    }
    pub fn velocity(&self) -> f64 {
        0.0
    }
    pub fn is_animating(&self) -> bool {
        false
    }
    pub fn set(&self, _x: f64) {}
    pub fn animate_to(
        &self,
        _target: f64,
        _spring: Spring,
        _velocity: Option<f64>,
        on_rest: Option<Box<dyn FnOnce()>>,
    ) {
        if let Some(f) = on_rest {
            f();
        }
    }
    pub fn stop(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn critically_damped_settles_without_overshoot() {
        let s = Spring::DEFAULT;
        let (mut x, mut v) = (100.0, 0.0);
        let mut min = x;
        for _ in 0..60 {
            (x, v) = s.step(x, v, 1.0 / 60.0);
            min = f64::min(min, x);
        }
        assert!(x.abs() < 0.5, "settled: {x}");
        assert!(min > -0.001, "no overshoot: {min}");
        let _ = v;
    }

    #[test]
    fn bouncy_overshoots_a_little() {
        let s = Spring::BOUNCY;
        let (mut x, mut v) = (100.0, 0.0);
        let mut min = x;
        for _ in 0..60 {
            (x, v) = s.step(x, v, 1.0 / 60.0);
            min = f64::min(min, x);
        }
        assert!(min < 0.0 && min > -3.0, "small overshoot: {min}");
    }

    #[test]
    fn projection_and_rubberband() {
        assert!((project(1000.0, 0.998) - 499.0).abs() < 0.01);
        let r = rubberband(100.0, 400.0, 0.55);
        assert!(r > 0.0 && r < 55.0);
    }
}
