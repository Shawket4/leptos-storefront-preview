//! The product gallery (an island).
//!
//! Without JS it is a native scroll-snap strip (swipe, momentum and RTL come from the browser) with
//! anchor thumbnails. Once hydrated it takes the gesture over, Apple-style: the strip follows the
//! finger 1:1 from where it was grabbed, the release velocity decides next/previous, a flick's
//! landing point is projected (`(v/1000)·d/(1−d)`), the ends rubber-band, and every movement is a
//! spring that starts from the live position — grab it mid-flight and it stops under the finger.
//!
//! Desktop: arrows, thumbnails, hover zoom-to-cursor. Everywhere: tap/click opens a fullscreen
//! viewer (same strip engine) with double-tap and pinch zoom and pan. Keyboard: arrows, Home/End,
//! Enter opens, Escape closes. A colour variant whose image differs swaps the photo (through the
//! page store the purchase panel writes).

use super::model::{GalleryData, STAGE_SIZES, text};
use super::motion::{Frames, Spring, SpringCfg, Tracker, project, reduced_motion, rubberband};
use super::use_product_store;
use crate::components::button::IconButton;
use crate::components::icons::Icon;
use crate::components::image::Picture;
use crate::format;
use crate::prefs::{tr, use_ui};
use leptos::html;
use leptos::prelude::*;
use web_sys::PointerEvent;

const THUMB_SIZES: &str = "72px";
/// Faster than this (px/s) and the release direction wins over the position.
const FLICK_V: f64 = 280.0;
/// Movement (px) before a gesture commits to a direction.
const SLOP: f64 = 8.0;
const TAP_ZOOM: f64 = 2.5;
const MAX_ZOOM: f64 = 4.0;
const DOUBLE_TAP_MS: f64 = 300.0;

#[island]
pub fn Gallery(data: GalleryData) -> impl IntoView {
    view! { <GalleryBody data=data/> }
}

// ── The strip engine (stage and viewer) ─────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum Axis {
    #[default]
    Undecided,
    X,
    Y,
}

#[derive(Clone, Debug, Default)]
struct Drag {
    id: i32,
    x0: f64,
    y0: f64,
    pos0: f64,
    axis: Axis,
    tracker: Tracker,
}

#[derive(Default)]
struct Strip {
    /// Logical position along the reading direction, px (slide i rests at `i·w`).
    pos: Spring,
    w: f64,
    n: usize,
    rtl: bool,
    index: usize,
    drag: Option<Drag>,
    frames: Frames,
}

impl Strip {
    fn max(&self) -> f64 {
        self.n.saturating_sub(1) as f64 * self.w
    }

    /// The on-screen position for a raw (finger) position: rubber-banded past either end.
    fn shown(&self, raw: f64) -> f64 {
        if raw < 0.0 {
            -rubberband(-raw, self.w)
        } else if raw > self.max() {
            self.max() + rubberband(raw - self.max(), self.w)
        } else {
            raw
        }
    }

    fn css(&self) -> String {
        let x = if self.rtl { self.pos.x } else { -self.pos.x };
        format!("translate3d({x:.2}px,0,0)")
    }
}

/// What a finished pointer gesture on a strip turned out to be.
enum Release {
    /// It never moved past the slop: a tap.
    Tap,
    Swipe,
    /// Another pointer / the browser took it (vertical scroll).
    Nothing,
}

#[derive(Clone, Copy)]
struct StripRef {
    st: StoredValue<Strip>,
    viewport: NodeRef<html::Div>,
    track: NodeRef<html::Div>,
    /// The committed index (dots, counter, thumbnails, aria).
    active: RwSignal<usize>,
}

impl StripRef {
    fn new(n: usize, start: usize) -> Self {
        StripRef {
            st: StoredValue::new(Strip {
                n,
                index: start,
                ..Default::default()
            }),
            viewport: NodeRef::new(),
            track: NodeRef::new(),
            active: RwSignal::new(start),
        }
    }

    fn index(self) -> usize {
        self.st.with_value(|s| s.index)
    }

    fn paint(self) {
        let css = self.st.with_value(|s| s.css());
        if let Some(t) = self.track.get_untracked() {
            let _ = web_sys::HtmlElement::style(&t).set_property("transform", &css);
        }
    }

    /// Re-read the slide width (on mount, resize, open) and rest on the current index.
    fn measure(self, rtl: bool) {
        let Some(vp) = self.viewport.get_untracked() else {
            return;
        };
        let w = vp.client_width() as f64;
        self.st.update_value(|s| {
            s.rtl = rtl;
            if w > 0.0 && s.drag.is_none() {
                s.frames.stop();
                s.w = w;
                s.pos = Spring::at(s.index as f64 * w);
            }
        });
        self.paint();
    }

    /// Spring to slide `i`, starting from the live position and velocity `v` (px/s).
    fn go(self, i: usize, v: f64, cfg: SpringCfg) {
        let reduced = reduced_motion();
        let Some((frames, i)) = self.st.try_update_value(|s| {
            let i = i.min(s.n.saturating_sub(1));
            s.index = i;
            s.pos.target = i as f64 * s.w;
            s.pos.v = v;
            if reduced {
                s.pos = Spring::at(s.pos.target);
            }
            (s.frames.clone(), i)
        }) else {
            return;
        };
        self.active.set(i);
        self.paint();
        if reduced {
            return;
        }
        frames.start(move |dt| {
            let done = self
                .st
                .try_update_value(|s| s.pos.step(cfg, dt))
                .unwrap_or(true);
            self.paint();
            !done
        });
    }

    fn jump(self, i: usize) {
        self.st.update_value(|s| {
            s.frames.stop();
            s.index = i.min(s.n.saturating_sub(1));
            s.pos = Spring::at(s.index as f64 * s.w);
        });
        self.active.set(self.index());
        self.paint();
    }

    fn step_by(self, d: isize) {
        let (i, n) = self.st.with_value(|s| (s.index as isize, s.n as isize));
        let next = (i + d).clamp(0, (n - 1).max(0)) as usize;
        self.go(next, 0.0, SpringCfg::DEFAULT);
    }

    /// Pointer down: grab the strip where it is (interrupting any spring).
    fn down(self, ev: &PointerEvent) {
        let (x, y) = (ev.client_x() as f64, ev.client_y() as f64);
        self.st.update_value(|s| {
            s.frames.stop();
            s.pos.v = 0.0;
            let mut tracker = Tracker::default();
            tracker.reset(ev.time_stamp(), s.pos.x);
            s.drag = Some(Drag {
                id: ev.pointer_id(),
                x0: x,
                y0: y,
                pos0: s.pos.x,
                axis: Axis::Undecided,
                tracker,
            });
        });
        self.paint();
    }

    /// Pointer move: decide the axis after the slop, then follow 1:1. Returns the axis.
    fn moved(self, ev: &PointerEvent) -> Axis {
        let (x, y, t, id) = (
            ev.client_x() as f64,
            ev.client_y() as f64,
            ev.time_stamp(),
            ev.pointer_id(),
        );
        let axis = self
            .st
            .try_update_value(|s| {
                let sign = if s.rtl { 1.0 } else { -1.0 };
                let (w, raw_shown) = (s.w, s.drag.as_ref().map(|d| d.pos0));
                let Some(d) = s.drag.as_mut().filter(|d| d.id == id) else {
                    return Axis::Undecided;
                };
                let (dx, dy) = (x - d.x0, y - d.y0);
                if d.axis == Axis::Undecided && (dx.abs() > SLOP || dy.abs() > SLOP) {
                    d.axis = if dx.abs() > dy.abs() {
                        Axis::X
                    } else {
                        Axis::Y
                    };
                }
                let axis = d.axis;
                if axis == Axis::X && w > 0.0 {
                    let raw = raw_shown.unwrap_or(0.0) + dx * sign;
                    d.tracker.push(t, raw);
                    let shown = s.shown(raw);
                    s.pos = Spring::at(shown);
                }
                if axis == Axis::Y {
                    s.drag = None;
                }
                axis
            })
            .unwrap_or_default();
        if axis == Axis::X {
            self.paint();
        }
        axis
    }

    /// Pointer up: a tap, or a swipe that springs to the slide its velocity points at.
    fn up(self, ev: &PointerEvent, cancelled: bool) -> Release {
        let (x, t, id) = (ev.client_x() as f64, ev.time_stamp(), ev.pointer_id());
        let Some(d) = self
            .st
            .try_update_value(|s| s.drag.take().filter(|d| d.id == id))
            .flatten()
        else {
            return Release::Nothing;
        };
        match d.axis {
            Axis::Undecided if !cancelled => {
                // Nothing moved: settle exactly where we were and report a tap.
                let i = self.index();
                self.go(i, 0.0, SpringCfg::SNAPPY);
                Release::Tap
            }
            Axis::X => {
                let (v, raw, w, index, n) = self.st.with_value(|s| {
                    let sign = if s.rtl { 1.0 } else { -1.0 };
                    (
                        d.tracker.velocity(t),
                        d.pos0 + (x - d.x0) * sign,
                        s.w,
                        s.index,
                        s.n,
                    )
                });
                let v = if cancelled { 0.0 } else { v };
                let target = if w <= 0.0 {
                    index
                } else if v.abs() > FLICK_V {
                    // The flick's direction decides; one photo per flick.
                    if v > 0.0 {
                        index + 1
                    } else {
                        index.saturating_sub(1)
                    }
                } else {
                    // Otherwise where the projected landing point is nearest.
                    let landing = raw + project(v, 0.998);
                    let near = (landing / w).round().max(0.0) as usize;
                    near.clamp(index.saturating_sub(1), index + 1)
                };
                let target = target.min(n.saturating_sub(1));
                let flick = v.abs() > FLICK_V;
                self.go(
                    target,
                    v,
                    if flick {
                        SpringCfg::FLICK
                    } else {
                        SpringCfg::DEFAULT
                    },
                );
                Release::Swipe
            }
            _ => {
                let i = self.index();
                self.go(i, 0.0, SpringCfg::DEFAULT);
                Release::Nothing
            }
        }
    }

    fn dragging_x(self) -> bool {
        self.st
            .with_value(|s| s.drag.as_ref().is_some_and(|d| d.axis == Axis::X))
    }
}

// ── Viewer zoom (scale + pan, each its own spring) ──────────────────────────────────────────

#[derive(Default)]
struct Zoom {
    s: Spring,
    x: Spring,
    y: Spring,
    /// Viewport box and the image's aspect (it is `object-fit: contain` inside the box).
    bw: f64,
    bh: f64,
    aspect: f64,
    pointers: Vec<(i32, f64, f64)>,
    /// Two fingers down: (distance, midpoint from centre, scale, offset) at the start.
    pinch: Option<(f64, (f64, f64), f64, (f64, f64))>,
    pan: Option<Pan>,
    last_tap: Option<(f64, f64, f64)>,
    frames: Frames,
}

/// One finger (or the mouse) moving a zoomed photo.
#[derive(Clone, Debug, Default)]
struct Pan {
    id: i32,
    x0: f64,
    y0: f64,
    ox: f64,
    oy: f64,
    tx: Tracker,
    ty: Tracker,
    moved: bool,
}

impl Pan {
    fn start(id: i32, x: f64, y: f64, t: f64, ox: f64, oy: f64) -> Pan {
        let (mut tx, mut ty) = (Tracker::default(), Tracker::default());
        tx.reset(t, ox);
        ty.reset(t, oy);
        Pan {
            id,
            x0: x,
            y0: y,
            ox,
            oy,
            tx,
            ty,
            moved: false,
        }
    }
}

impl Zoom {
    fn zoomed(&self) -> bool {
        self.s.x > 1.01
    }

    fn fit(&self) -> (f64, f64) {
        let a = if self.aspect > 0.0 { self.aspect } else { 1.0 };
        if self.bh <= 0.0 {
            return (self.bw, self.bw / a);
        }
        if self.bw / self.bh > a {
            (self.bh * a, self.bh)
        } else {
            (self.bw, self.bw / a)
        }
    }

    /// How far the image may move from centre at scale `s`.
    fn bounds(&self, s: f64) -> (f64, f64) {
        let (iw, ih) = self.fit();
        (
            ((iw * s - self.bw) / 2.0).max(0.0),
            ((ih * s - self.bh) / 2.0).max(0.0),
        )
    }

    fn reset(&mut self) {
        self.frames.stop();
        self.s = Spring::at(1.0);
        self.x = Spring::at(0.0);
        self.y = Spring::at(0.0);
        self.pinch = None;
        self.pan = None;
        self.pointers.clear();
    }

    fn css(&self) -> String {
        format!(
            "translate3d({:.2}px,{:.2}px,0) scale({:.4})",
            self.x.x, self.y.x, self.s.x
        )
    }
}

/// Rubber-band a pan offset beyond `[-b, b]`.
fn band(v: f64, b: f64, dim: f64) -> f64 {
    if v > b {
        b + rubberband(v - b, dim)
    } else if v < -b {
        -b - rubberband(-b - v, dim)
    } else {
        v
    }
}

#[derive(Clone, Copy)]
struct ZoomRef {
    z: StoredValue<Zoom>,
    strip: StripRef,
    aspects: StoredValue<Vec<f32>>,
}

impl ZoomRef {
    fn target_el(self) -> Option<web_sys::HtmlElement> {
        use wasm_bindgen::JsCast;
        let i = self.strip.index();
        let track = self.strip.track.get_untracked()?;
        track
            .query_selector(&format!("[data-i=\"{i}\"] .pv__zoom"))
            .ok()
            .flatten()
            .and_then(|e| e.dyn_into::<web_sys::HtmlElement>().ok())
    }

    fn paint(self) {
        let css = self.z.with_value(|z| z.css());
        let zoomed = self.z.with_value(|z| z.zoomed());
        if let Some(el) = self.target_el() {
            let _ = el.style().set_property("transform", &css);
        }
        if let Some(vp) = self.strip.viewport.get_untracked() {
            let _ = if zoomed {
                vp.class_list().add_1("is-zoomed")
            } else {
                vp.class_list().remove_1("is-zoomed")
            };
        }
    }

    /// The box and the current image's aspect, before a gesture.
    fn measure(self) {
        let Some(vp) = self.strip.viewport.get_untracked() else {
            return;
        };
        let r = vp.get_bounding_client_rect();
        let i = self.strip.index();
        let aspect = self
            .aspects
            .with_value(|a| a.get(i).copied().unwrap_or(1.0)) as f64;
        self.z.update_value(|z| {
            z.bw = r.width();
            z.bh = r.height();
            z.aspect = aspect;
        });
    }

    fn reset(self) {
        if let Some(el) = self.target_el() {
            let _ = el.style().set_property("transform", "none");
        }
        self.z.update_value(|z| z.reset());
        self.paint();
    }

    /// Spring scale and offset to `(s, x, y)`, clamped to what the image allows.
    fn settle(self, s: f64, x: f64, y: f64, vx: f64, vy: f64, cfg: SpringCfg) {
        let reduced = reduced_motion();
        let frames = self.z.try_update_value(|z| {
            let s = s.clamp(1.0, MAX_ZOOM);
            let (bx, by) = z.bounds(s);
            z.s.target = s;
            z.x.target = x.clamp(-bx, bx);
            z.y.target = y.clamp(-by, by);
            z.x.v = vx;
            z.y.v = vy;
            if reduced {
                z.s = Spring::at(z.s.target);
                z.x = Spring::at(z.x.target);
                z.y = Spring::at(z.y.target);
            }
            z.frames.clone()
        });
        self.paint();
        let Some(frames) = frames else { return };
        if reduced {
            return;
        }
        frames.start(move |dt| {
            let done = self
                .z
                .try_update_value(|z| {
                    let a = z.s.step_eps(cfg, dt, 0.002);
                    let b = z.x.step(cfg, dt);
                    let c = z.y.step(cfg, dt);
                    a && b && c
                })
                .unwrap_or(true);
            self.paint();
            !done
        });
    }

    /// Double-tap: zoom in to the tapped point, or back out.
    fn toggle_at(self, cx: f64, cy: f64) {
        self.measure();
        let Some(vp) = self.strip.viewport.get_untracked() else {
            return;
        };
        let r = vp.get_bounding_client_rect();
        let (fx, fy) = (
            cx - r.left() - r.width() / 2.0,
            cy - r.top() - r.height() / 2.0,
        );
        let (zoomed, x, y, s) = self.z.with_value(|z| (z.zoomed(), z.x.x, z.y.x, z.s.x));
        if zoomed {
            self.settle(1.0, 0.0, 0.0, 0.0, 0.0, SpringCfg::SNAPPY);
        } else {
            let k = TAP_ZOOM / s.max(0.01);
            self.settle(
                TAP_ZOOM,
                fx - (fx - x) * k,
                fy - (fy - y) * k,
                0.0,
                0.0,
                SpringCfg::DEFAULT,
            );
        }
    }
}

// ── The component ───────────────────────────────────────────────────────────────────────────

#[component]
pub fn GalleryBody(data: GalleryData) -> impl IntoView {
    let ui = use_ui();
    let lang = ui.lang.get_untracked();
    let store = use_product_store();
    let pid = data.product;
    let n = data.images.len().max(1);
    let start = data.start.min(n - 1);
    let name = text(&data.name, lang);

    let main = StripRef::new(n, start);
    let viewer = StripRef::new(n, start);
    let zoom = ZoomRef {
        z: StoredValue::new(Zoom {
            s: Spring::at(1.0),
            ..Default::default()
        }),
        strip: viewer,
        aspects: StoredValue::new(data.images.iter().map(|i| i.aspect).collect()),
    };
    let root = NodeRef::<html::Section>::new();
    let dialog = NodeRef::<html::Dialog>::new();
    // Hover zoom-to-cursor (fine pointers only): position in % of the slide.
    let hover = RwSignal::new(None::<(f64, f64)>);
    let fine = StoredValue::new(false);
    let rtl = move || ui.lang.get_untracked().is_rtl();

    // Hydrated: take over from the native strip at the photo the visitor is on.
    Effect::new(move |_| {
        let (Some(vp), Some(section)) = (main.viewport.get(), root.get()) else {
            return;
        };
        let w = vp.client_width() as f64;
        let scrolled = (vp.scroll_left() as f64).abs();
        let at = if scrolled > 1.0 && w > 0.0 {
            (scrolled / w).round() as usize
        } else {
            start
        };
        main.st.update_value(|s| s.index = at.min(n - 1));
        main.measure(rtl());
        let _ = section.class_list().add_1("is-js");
        vp.set_scroll_left(0);
        main.active.set(main.index());
        fine.set_value(
            window()
                .match_media("(hover: hover) and (pointer: fine) and (min-width: 768px)")
                .ok()
                .flatten()
                .is_some_and(|m| m.matches()),
        );
    });
    // The language toggle flips the reading direction.
    Effect::watch(
        move || ui.lang.get(),
        move |l, _, _| {
            main.measure(l.is_rtl());
            viewer.measure(l.is_rtl());
        },
        false,
    );
    let resize = window_event_listener(leptos::ev::resize, move |_| {
        main.measure(rtl());
        viewer.measure(rtl());
    });
    on_cleanup(move || resize.remove());
    // A variant whose photo differs (written by the purchase panel).
    let wanted = Memo::new(move |_| {
        store
            .picked
            .get()
            .filter(|p| p.product == pid)
            .and_then(|p| p.image)
    });
    Effect::watch(
        move || wanted.get(),
        move |img, _, _| {
            if let Some(i) = *img
                && i < n
                && i != main.index()
            {
                main.go(i, 0.0, SpringCfg::DEFAULT);
            }
        },
        false,
    );

    let open = move |i: usize| {
        let Some(d) = dialog.get_untracked() else {
            return;
        };
        if d.show_modal().is_err() {
            return;
        }
        if let Some(html) = document().document_element() {
            let _ = html.class_list().add_1("pv-open");
        }
        viewer.st.update_value(|s| s.index = i);
        viewer.measure(rtl());
        viewer.active.set(i);
        zoom.reset();
    };
    let on_closed = move |_| {
        if let Some(html) = document().document_element() {
            let _ = html.class_list().remove_1("pv-open");
        }
        zoom.reset();
        main.jump(viewer.index());
    };
    let close = move || {
        if let Some(d) = dialog.get_untracked() {
            d.close();
        }
    };

    // ── Stage pointer handling ──
    let stage_down = move |ev: PointerEvent| {
        if ev.pointer_type() == "mouse" && ev.button() != 0 {
            return;
        }
        hover.set(None);
        main.down(&ev);
        if let Some(vp) = main.viewport.get_untracked() {
            let _ = vp.set_pointer_capture(ev.pointer_id());
        }
    };
    let stage_move = move |ev: PointerEvent| {
        if main.moved(&ev) == Axis::X {
            ev.prevent_default();
            return;
        }
        // Hovering with a mouse: zoom to the cursor (not while a button is held).
        if fine.get_value() && ev.pointer_type() == "mouse" && ev.buttons() == 0 {
            if let Some(vp) = main.viewport.get_untracked() {
                let r = vp.get_bounding_client_rect();
                let x = ((ev.client_x() as f64 - r.left()) / r.width() * 100.0).clamp(0.0, 100.0);
                let y = ((ev.client_y() as f64 - r.top()) / r.height() * 100.0).clamp(0.0, 100.0);
                if hover.get_untracked().is_none() {
                    sharpen(main, main.index());
                }
                hover.set(Some((x, y)));
            }
        }
    };
    let stage_up = move |ev: PointerEvent| {
        if let Release::Tap = main.up(&ev, false) {
            open(main.index());
        }
    };
    let stage_cancel = move |ev: PointerEvent| {
        main.up(&ev, true);
    };
    let stage_key = move |ev: web_sys::KeyboardEvent| {
        let back = if rtl() { "ArrowRight" } else { "ArrowLeft" };
        let fwd = if rtl() { "ArrowLeft" } else { "ArrowRight" };
        match ev.key().as_str() {
            k if k == fwd => main.step_by(1),
            k if k == back => main.step_by(-1),
            "Home" => main.go(0, 0.0, SpringCfg::DEFAULT),
            "End" => main.go(n - 1, 0.0, SpringCfg::DEFAULT),
            "Enter" | " " => open(main.index()),
            _ => return,
        }
        ev.prevent_default();
    };

    // ── Viewer pointer handling: pinch, pan when zoomed, swipe otherwise, double-tap ──
    let v_down = move |ev: PointerEvent| {
        if ev.pointer_type() == "mouse" && ev.button() != 0 {
            return;
        }
        if let Some(vp) = viewer.viewport.get_untracked() {
            let _ = vp.set_pointer_capture(ev.pointer_id());
        }
        zoom.measure();
        let (x, y, id) = (ev.client_x() as f64, ev.client_y() as f64, ev.pointer_id());
        let count = zoom
            .z
            .try_update_value(|z| {
                z.frames.stop();
                z.pointers.retain(|p| p.0 != id);
                z.pointers.push((id, x, y));
                z.pointers.len()
            })
            .unwrap_or(0);
        if count >= 2 {
            // Second finger: pinch. The strip lets go where it is.
            viewer.st.update_value(|s| s.drag = None);
            let i = viewer.index();
            viewer.go(i, 0.0, SpringCfg::SNAPPY);
            zoom.z.update_value(|z| {
                let (a, b) = (z.pointers[0], z.pointers[1]);
                let d = ((a.1 - b.1).powi(2) + (a.2 - b.2).powi(2)).sqrt().max(1.0);
                let m = (
                    (a.1 + b.1) / 2.0 - z.bw / 2.0,
                    (a.2 + b.2) / 2.0 - z.bh / 2.0,
                );
                z.pan = None;
                z.pinch = Some((d, m, z.s.x, (z.x.x, z.y.x)));
            });
        } else if zoom.z.with_value(|z| z.zoomed()) {
            zoom.z.update_value(|z| {
                z.pan = Some(Pan::start(id, x, y, ev.time_stamp(), z.x.x, z.y.x))
            });
        } else {
            viewer.down(&ev);
        }
    };
    let v_move = move |ev: PointerEvent| {
        let (x, y, id, t) = (
            ev.client_x() as f64,
            ev.client_y() as f64,
            ev.pointer_id(),
            ev.time_stamp(),
        );
        let handled = zoom
            .z
            .try_update_value(|z| {
                if let Some(p) = z.pointers.iter_mut().find(|p| p.0 == id) {
                    p.1 = x;
                    p.2 = y;
                }
                if let Some((d0, m0, s0, t0)) = z.pinch {
                    if z.pointers.len() < 2 {
                        return true;
                    }
                    let (a, b) = (z.pointers[0], z.pointers[1]);
                    let d = ((a.1 - b.1).powi(2) + (a.2 - b.2).powi(2)).sqrt().max(1.0);
                    let m = (
                        (a.1 + b.1) / 2.0 - z.bw / 2.0,
                        (a.2 + b.2) / 2.0 - z.bh / 2.0,
                    );
                    let raw = s0 * d / d0;
                    // Resist past the limits instead of stopping dead.
                    let s = if raw < 1.0 {
                        1.0 - rubberband(1.0 - raw, 1.0)
                    } else if raw > MAX_ZOOM {
                        MAX_ZOOM + rubberband(raw - MAX_ZOOM, MAX_ZOOM)
                    } else {
                        raw
                    };
                    let k = s / s0;
                    z.s = Spring::at(s);
                    z.x = Spring::at(m.0 - (m0.0 - t0.0) * k);
                    z.y = Spring::at(m.1 - (m0.1 - t0.1) * k);
                    return true;
                }
                if let Some(p) = z.pan.as_mut() {
                    if p.id != id {
                        return true;
                    }
                    if (x - p.x0).abs() > SLOP || (y - p.y0).abs() > SLOP {
                        p.moved = true;
                    }
                    let (rx, ry) = (p.ox + x - p.x0, p.oy + y - p.y0);
                    p.tx.push(t, rx);
                    p.ty.push(t, ry);
                    let s = z.s.x;
                    let (bx, by) = z.bounds(s);
                    let (bw, bh) = (z.bw, z.bh);
                    z.x = Spring::at(band(rx, bx, bw));
                    z.y = Spring::at(band(ry, by, bh));
                    return true;
                }
                false
            })
            .unwrap_or(false);
        if handled {
            ev.prevent_default();
            zoom.paint();
        } else if viewer.moved(&ev) == Axis::X {
            ev.prevent_default();
        }
    };
    let v_up = move |ev: PointerEvent, cancelled: bool| {
        let (id, t) = (ev.pointer_id(), ev.time_stamp());
        let (x, y) = (ev.client_x() as f64, ev.client_y() as f64);
        enum After {
            Strip,
            Settle(f64, f64, f64, f64, f64),
            Tap,
            Wait,
        }
        let after = zoom
            .z
            .try_update_value(|z| {
                z.pointers.retain(|p| p.0 != id);
                if z.pinch.is_some() {
                    if z.pointers.len() >= 2 {
                        return After::Wait;
                    }
                    z.pinch = None;
                    // One finger left: it carries on as a pan; settle when it lifts.
                    if let Some(&(pid, px, py)) = z.pointers.first() {
                        z.pan = Some(Pan::start(pid, px, py, t, z.x.x, z.y.x));
                        return After::Wait;
                    }
                    return After::Settle(z.s.x, z.x.x, z.y.x, 0.0, 0.0);
                }
                if z.pan.as_ref().is_some_and(|p| p.id != id) {
                    return After::Wait;
                }
                if let Some(p) = z.pan.take() {
                    if !p.moved && !cancelled {
                        return After::Tap;
                    }
                    let (vx, vy) = if cancelled {
                        (0.0, 0.0)
                    } else {
                        (p.tx.velocity(t), p.ty.velocity(t))
                    };
                    let (px, py) = (z.x.x + project(vx, 0.995), z.y.x + project(vy, 0.995));
                    return After::Settle(z.s.x, px, py, vx, vy);
                }
                After::Strip
            })
            .unwrap_or(After::Strip);
        // Two quick taps in the same spot (any pointer type) toggle the zoom there.
        let double_tap = move || {
            let hit = zoom
                .z
                .try_update_value(|z| {
                    let hit = z.last_tap.is_some_and(|(lt, lx, ly)| {
                        t - lt < DOUBLE_TAP_MS && (lx - x).abs() < 30.0 && (ly - y).abs() < 30.0
                    });
                    z.last_tap = if hit { None } else { Some((t, x, y)) };
                    hit
                })
                .unwrap_or(false);
            if hit {
                zoom.toggle_at(x, y);
            }
        };
        match after {
            After::Settle(s, sx, sy, vx, vy) => zoom.settle(s, sx, sy, vx, vy, SpringCfg::DEFAULT),
            After::Tap => double_tap(),
            After::Wait => {}
            After::Strip => {
                let before = viewer.index();
                match viewer.up(&ev, cancelled) {
                    Release::Tap => double_tap(),
                    Release::Swipe if viewer.index() != before => zoom.reset(),
                    _ => {}
                }
            }
        }
    };
    let v_key = move |ev: web_sys::KeyboardEvent| {
        let back = if rtl() { "ArrowRight" } else { "ArrowLeft" };
        let fwd = if rtl() { "ArrowLeft" } else { "ArrowRight" };
        let k = ev.key();
        if k == fwd || k == back {
            ev.prevent_default();
            zoom.reset();
            viewer.step_by(if k == fwd { 1 } else { -1 });
        }
    };

    // ── Markup ──
    let count_text = move |i: usize| {
        let num = ui.numerals.get();
        format!(
            "{} / {}",
            format::count(i as u32 + 1, num),
            format::count(n as u32, num)
        )
    };
    let slide_label = move |i: usize| {
        let num = ui.numerals.get();
        let (a, b) = (
            format::count(i as u32 + 1, num),
            format::count(n as u32, num),
        );
        match ui.lang.get() {
            crate::prefs::Lang::En => format!("{a} of {b}"),
            crate::prefs::Lang::Ar => format!("{a} من {b}"),
        }
    };
    let images = data.images.clone();
    let multi = n > 1;

    let slides = images
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, img)| {
            view! {
                <div
                    class="pg__slide"
                    id=format!("pg{pid}-{i}")
                    data-i=i
                    role="group"
                    aria-roledescription=move || tr(ui.lang.get(), "photo", "صورة")
                    aria-label=move || slide_label(i)
                    class:is-start={i == start}
                    class:is-zoom=move || hover.get().is_some() && main.active.get() == i
                    style=move || {
                        hover.get().filter(|_| main.active.get() == i).map(|(x, y)| format!("--zx:{x:.1}%;--zy:{y:.1}%"))
                    }
                >
                    {if i == start {
                        // The card → page morph lands on the photo the page opens with.
                        view! { <Picture img=img sizes=STAGE_SIZES priority=true vt_name=format!("p{pid}") lang=lang/> }.into_any()
                    } else {
                        view! { <Picture img=img sizes=STAGE_SIZES lang=lang/> }.into_any()
                    }}
                </div>
            }
        })
        .collect_view();

    let thumbs = multi.then(|| {
        let items = images
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, img)| {
                view! {
                    <a
                        class="pg__thumb"
                        href=format!("#pg{pid}-{i}")
                        aria-label=move || format!("{} {}", tr(ui.lang.get(), "Photo", "صورة"), slide_label(i))
                        aria-current=move || (main.active.get() == i).then_some("true")
                        on:click=move |ev| {
                            ev.prevent_default();
                            main.go(i, 0.0, SpringCfg::DEFAULT);
                        }
                    >
                        <Picture img=img sizes=THUMB_SIZES lang=lang/>
                    </a>
                }
            })
            .collect_view();
        view! { <div class="pg__thumbs">{items}</div> }
    });

    let dots = multi.then(|| {
        view! {
            <div class="pg__dots" aria-hidden="true">
                {(0..n).map(|i| view! { <span class="pg__dot" class:is-on=move || main.active.get() == i></span> }).collect_view()}
            </div>
        }
    });

    let viewer_slides = images
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, img)| {
            let alt = img.alt(lang).to_owned();
            let missing = img.is_missing();
            view! {
                <div class="pv__slide" data-i=i>
                    <div class="pv__zoom">
                        {(!missing).then(|| view! {
                            <img
                                class="pv__img"
                                src=img.largest()
                                srcset=img.srcset()
                                sizes="100vw"
                                alt=alt
                                loading="lazy"
                                decoding="async"
                                draggable="false"
                            />
                        })}
                    </div>
                </div>
            }
        })
        .collect_view();

    view! {
        <section
            class="pg"
            node_ref=root
            aria-roledescription=move || tr(ui.lang.get(), "gallery", "معرض صور")
            aria-label=format!("{} — {name}", tr(lang, "Photos", "الصور"))
        >
            <div class="pg__stage">
                <div
                    class="pg__viewport"
                    node_ref=main.viewport
                    tabindex="0"
                    aria-label=move || tr(ui.lang.get(), "Product photos. Use arrow keys to browse, Enter to enlarge.", "صور المنتج. استخدم الأسهم للتنقل وEnter للتكبير.")
                    on:pointerdown=stage_down
                    on:pointermove=stage_move
                    on:pointerup=stage_up
                    on:pointercancel=stage_cancel
                    on:pointerleave=move |_| {
                        if !main.dragging_x() {
                            hover.set(None);
                        }
                    }
                    on:keydown=stage_key
                    on:dragstart=|ev| ev.prevent_default()
                >
                    <div class="pg__track" node_ref=main.track>{slides}</div>
                </div>
                {multi.then(|| view! {
                    <IconButton
                        icon="chevron-left"
                        label=move || tr(ui.lang.get(), "Previous photo", "الصورة السابقة").to_owned()
                        class="pg__nav pg__nav--prev"
                        disabled=Signal::derive(move || main.active.get() == 0)
                        on:click=move |_| main.step_by(-1)
                    />
                    <IconButton
                        icon="chevron-right"
                        label=move || tr(ui.lang.get(), "Next photo", "الصورة التالية").to_owned()
                        class="pg__nav pg__nav--next"
                        disabled=Signal::derive(move || main.active.get() + 1 >= n)
                        on:click=move |_| main.step_by(1)
                    />
                    <p class="pg__count mono" aria-hidden="true" dir="ltr">{move || count_text(main.active.get())}</p>
                })}
                <IconButton
                    icon="zoom-in"
                    label=move || tr(ui.lang.get(), "View full screen", "عرض بملء الشاشة").to_owned()
                    class="pg__expand"
                    on:click=move |_| open(main.index())
                />
                <p class="sr-only" aria-live="polite">
                    {move || multi.then(|| format!("{} {}", tr(ui.lang.get(), "Photo", "صورة"), slide_label(main.active.get())))}
                </p>
            </div>
            {dots}
            {thumbs}
            <dialog
                class="pv"
                node_ref=dialog
                aria-label=format!("{} — {name}", tr(lang, "Photos", "الصور"))
                on:close=on_closed
                on:keydown=v_key
            >
                <div class="pv__bar">
                    <p class="pv__count mono" dir="ltr">{move || multi.then(|| count_text(viewer.active.get()))}</p>
                    <IconButton
                        icon="x"
                        label=move || tr(ui.lang.get(), "Close", "إغلاق").to_owned()
                        class="pv__close"
                        on:click=move |_| close()
                    />
                </div>
                <div
                    class="pv__viewport"
                    node_ref=viewer.viewport
                    on:pointerdown=v_down
                    on:pointermove=v_move
                    on:pointerup=move |ev| v_up(ev, false)
                    on:pointercancel=move |ev| v_up(ev, true)
                    on:dragstart=|ev| ev.prevent_default()
                >
                    <div class="pv__track" node_ref=viewer.track>{viewer_slides}</div>
                </div>
                <p class="pv__hint" aria-hidden="true">
                    <Icon name="zoom-in" size=14/>
                    <span class="pv__hint-touch">{move || tr(ui.lang.get(), "Double-tap or pinch to zoom", "دوس مرتين أو قرّب بصباعين للتكبير")}</span>
                    <span class="pv__hint-mouse">{move || tr(ui.lang.get(), "Double-click to zoom · drag to move", "دوس مرتين للتكبير · اسحب للتحريك")}</span>
                </p>
            </dialog>
        </section>
    }
}

/// Before zooming a stage photo 2×, ask the browser for its largest file.
fn sharpen(main: StripRef, i: usize) {
    let Some(track) = main.track.get_untracked() else {
        return;
    };
    if let Ok(Some(img)) = track.query_selector(&format!("[data-i=\"{i}\"] img")) {
        let _ = img.set_attribute("sizes", "1200px");
    }
}
