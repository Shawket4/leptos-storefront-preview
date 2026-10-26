//! The product gallery (an island).
//!
//! Without JS it is a native scroll-snap strip (swipe, momentum and RTL come from the browser) with
//! anchor thumbnails. Once hydrated it takes the gesture over, Apple-style, on the design system's
//! springs (`components::motion`): the strip follows the finger 1:1 from where it was grabbed, the
//! release velocity decides next/previous, a slow release lands where its momentum projects, the
//! ends rubber-band, and every movement starts from the live position — grab it mid-flight and it
//! stops under the finger; a flick hands its speed to the spring.
//!
//! Desktop: arrows, thumbnails, hover zoom-to-cursor. Everywhere: tap/click opens a fullscreen
//! viewer (same strip engine) with double-tap and pinch zoom and pan. Keyboard: arrows, Home/End,
//! Enter opens, Escape closes. A colour variant whose image differs swaps the photo (through the
//! page store the purchase panel writes).

use super::model::{GalleryData, STAGE_SIZES, text};
use super::use_product_store;
use crate::components::button::IconButton;
use crate::components::icons::Icon;
use crate::components::image::Picture;
use crate::components::motion::{Spring, SpringValue, VelocityTracker, project, rubberband};
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
/// Apple's rubber-band constant.
const RUBBER: f64 = 0.55;
/// After a flick only: a little life at the landing (between DEFAULT and BOUNCY).
const FLICK: Spring = Spring {
    damping: 0.88,
    response: 0.32,
};

#[component]
pub fn Gallery(data: GalleryData) -> impl IntoView {
    view! { <GalleryBody data=data/> }
}

/// A spring that exists only in the browser. `SpringValue` keeps its state in thread-local
/// storage; created during SSR it would be dropped later by whichever tokio worker ends the
/// response stream, and a thread-local value dropped on another thread panics.
fn client_spring(initial: f64, draw: impl Fn(f64) + 'static) -> Option<SpringValue> {
    #[cfg(feature = "hydrate")]
    {
        Some(SpringValue::new(initial, draw))
    }
    #[cfg(not(feature = "hydrate"))]
    {
        let _ = (initial, draw);
        None
    }
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
    /// Strip position when grabbed.
    pos0: f64,
    /// The finger's (un-rubber-banded) position, for the release.
    raw: f64,
    axis: Axis,
    tracker: VelocityTracker,
}

#[derive(Default)]
struct Strip {
    w: f64,
    n: usize,
    rtl: bool,
    index: usize,
    drag: Option<Drag>,
}

impl Strip {
    fn max(&self) -> f64 {
        self.n.saturating_sub(1) as f64 * self.w
    }

    /// The on-screen position for a raw (finger) position: rubber-banded past either end.
    fn shown(&self, raw: f64) -> f64 {
        if raw < 0.0 {
            -rubberband(-raw, self.w, RUBBER)
        } else if raw > self.max() {
            self.max() + rubberband(raw - self.max(), self.w, RUBBER)
        } else {
            raw
        }
    }
}

/// What a finished pointer gesture on a strip turned out to be.
enum Release {
    /// It never moved past the slop: a tap.
    Tap,
    Swipe,
    /// Another pointer, or the browser took it (vertical scroll).
    Nothing,
}

/// Paint a strip at logical position `pos` (px along the reading direction).
fn draw_strip(st: StoredValue<Strip>, track: NodeRef<html::Div>, pos: f64) {
    let rtl = st.try_with_value(|s| s.rtl).unwrap_or(false);
    let x = if rtl { pos } else { -pos };
    if let Some(t) = track.get_untracked() {
        let _ = web_sys::HtmlElement::style(&t)
            .set_property("transform", &format!("translate3d({x:.2}px,0,0)"));
    }
}

#[derive(Clone, Copy)]
struct StripRef {
    st: StoredValue<Strip>,
    viewport: NodeRef<html::Div>,
    track: NodeRef<html::Div>,
    /// The committed index (dots, counter, thumbnails, aria).
    active: RwSignal<usize>,
    /// Position along the reading direction, px (slide i rests at `i·w`). Browser only.
    x: Option<SpringValue>,
}

impl StripRef {
    fn new(n: usize, start: usize) -> Self {
        let st = StoredValue::new(Strip {
            n,
            index: start,
            ..Default::default()
        });
        let track = NodeRef::<html::Div>::new();
        StripRef {
            st,
            viewport: NodeRef::new(),
            track,
            active: RwSignal::new(start),
            x: client_spring(0.0, move |pos| draw_strip(st, track, pos)),
        }
    }

    fn index(self) -> usize {
        self.st.with_value(|s| s.index)
    }

    fn pos(self) -> f64 {
        self.x.map(|x| x.get()).unwrap_or_default()
    }

    /// Put the strip somewhere now (stops any spring in flight).
    fn set(self, pos: f64) {
        if let Some(x) = self.x {
            x.set(pos);
        }
    }

    /// Re-read the slide width (on mount, resize, open, direction change) and rest on the index.
    fn measure(self, rtl: bool) {
        let Some(vp) = self.viewport.get_untracked() else {
            return;
        };
        let w = vp.client_width() as f64;
        let rest = self
            .st
            .try_update_value(|s| {
                s.rtl = rtl;
                if w > 0.0 && s.drag.is_none() {
                    s.w = w;
                    Some(s.index as f64 * w)
                } else {
                    None
                }
            })
            .flatten();
        match rest {
            Some(p) => self.set(p),
            None => draw_strip(self.st, self.track, self.pos()),
        }
    }

    /// Spring to slide `i` from the live position, handing over velocity `v` (px/s).
    fn go(self, i: usize, v: f64, spring: Spring) {
        let Some((i, target)) = self.st.try_update_value(|s| {
            let i = i.min(s.n.saturating_sub(1));
            s.index = i;
            (i, i as f64 * s.w)
        }) else {
            return;
        };
        self.active.set(i);
        if let Some(x) = self.x {
            x.animate_to(target, spring, Some(v), None);
        }
    }

    fn jump(self, i: usize) {
        let target = self.st.try_update_value(|s| {
            s.index = i.min(s.n.saturating_sub(1));
            s.index as f64 * s.w
        });
        self.active.set(self.index());
        if let Some(t) = target {
            self.set(t);
        }
    }

    fn step_by(self, d: isize) {
        let (i, n) = self.st.with_value(|s| (s.index as isize, s.n as isize));
        let next = (i + d).clamp(0, (n - 1).max(0)) as usize;
        self.go(next, 0.0, Spring::DEFAULT);
    }

    /// Pointer down: grab the strip where it is on screen (interrupting any spring).
    fn down(self, ev: &PointerEvent) {
        let pos0 = self.pos();
        self.set(pos0);
        let (x, y, t) = (ev.client_x() as f64, ev.client_y() as f64, ev.time_stamp());
        self.st.update_value(|s| {
            let mut tracker = VelocityTracker::default();
            tracker.push(t, pos0);
            s.drag = Some(Drag {
                id: ev.pointer_id(),
                x0: x,
                y0: y,
                pos0,
                raw: pos0,
                axis: Axis::Undecided,
                tracker,
            });
        });
    }

    /// Pointer move: decide the axis after the slop, then follow 1:1. Returns the axis.
    fn moved(self, ev: &PointerEvent) -> Axis {
        let (x, y, t, id) = (
            ev.client_x() as f64,
            ev.client_y() as f64,
            ev.time_stamp(),
            ev.pointer_id(),
        );
        let (axis, shown) = self
            .st
            .try_update_value(|s| {
                let sign = if s.rtl { 1.0 } else { -1.0 };
                let Some(d) = s.drag.as_mut().filter(|d| d.id == id) else {
                    return (Axis::Undecided, None);
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
                let raw = d.pos0 + dx * sign;
                if axis == Axis::X {
                    d.raw = raw;
                    d.tracker.push(t, raw);
                }
                if axis == Axis::Y {
                    s.drag = None;
                    return (axis, None);
                }
                (axis, (axis == Axis::X && s.w > 0.0).then(|| s.shown(raw)))
            })
            .unwrap_or_default();
        if let Some(p) = shown {
            self.set(p);
        }
        axis
    }

    /// Pointer up: a tap, or a swipe that springs to the slide its velocity points at.
    fn up(self, ev: &PointerEvent, cancelled: bool) -> Release {
        let (t, id) = (ev.time_stamp(), ev.pointer_id());
        let Some(mut d) = self
            .st
            .try_update_value(|s| s.drag.take().filter(|d| d.id == id))
            .flatten()
        else {
            return Release::Nothing;
        };
        match d.axis {
            Axis::Undecided if !cancelled => {
                let i = self.index();
                self.go(i, 0.0, Spring::SNAPPY);
                Release::Tap
            }
            Axis::X => {
                // A sample at release: a finger that rested before lifting carries no speed.
                d.tracker.push(t, d.raw);
                let v = if cancelled { 0.0 } else { d.tracker.velocity() };
                let (w, index, n) = self.st.with_value(|s| (s.w, s.index, s.n));
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
                    // Otherwise the slide nearest to where its momentum would carry it.
                    let landing = d.raw + project(v, 0.998);
                    ((landing / w).round().max(0.0) as usize)
                        .clamp(index.saturating_sub(1), index + 1)
                };
                let flick = v.abs() > FLICK_V;
                self.go(
                    target.min(n.saturating_sub(1)),
                    v,
                    if flick { FLICK } else { Spring::DEFAULT },
                );
                Release::Swipe
            }
            _ => {
                let i = self.index();
                self.go(i, 0.0, Spring::DEFAULT);
                Release::Nothing
            }
        }
    }

    fn dragging_x(self) -> bool {
        self.st
            .with_value(|s| s.drag.as_ref().is_some_and(|d| d.axis == Axis::X))
    }
}

// ── Viewer zoom: scale and pan, each its own spring ─────────────────────────────────────────

#[derive(Default)]
struct Zoom {
    /// Viewport box and the image's aspect (it is `object-fit: contain` inside the box).
    bw: f64,
    bh: f64,
    aspect: f64,
    pointers: Vec<(i32, f64, f64)>,
    /// Two fingers down: (distance, midpoint from centre, scale, offset) at the start.
    pinch: Option<(f64, (f64, f64), f64, (f64, f64))>,
    pan: Option<Pan>,
    last_tap: Option<(f64, f64, f64)>,
}

/// One finger (or the mouse) moving a zoomed photo.
#[derive(Clone, Debug, Default)]
struct Pan {
    id: i32,
    x0: f64,
    y0: f64,
    ox: f64,
    oy: f64,
    rx: f64,
    ry: f64,
    tx: VelocityTracker,
    ty: VelocityTracker,
    moved: bool,
}

impl Pan {
    fn start(id: i32, x: f64, y: f64, t: f64, ox: f64, oy: f64) -> Pan {
        let (mut tx, mut ty) = (VelocityTracker::default(), VelocityTracker::default());
        tx.push(t, ox);
        ty.push(t, oy);
        Pan {
            id,
            x0: x,
            y0: y,
            ox,
            oy,
            rx: ox,
            ry: oy,
            tx,
            ty,
            moved: false,
        }
    }
}

impl Zoom {
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
}

/// Rubber-band a pan offset beyond `[-b, b]`.
fn band(v: f64, b: f64, dim: f64) -> f64 {
    if v > b {
        b + rubberband(v - b, dim, RUBBER)
    } else if v < -b {
        -b - rubberband(-b - v, dim, RUBBER)
    } else {
        v
    }
}

/// The three springs of the zoom (scale, x, y). Browser only.
#[derive(Clone, Copy)]
struct ZoomSprings {
    s: SpringValue,
    x: SpringValue,
    y: SpringValue,
}

#[derive(Clone, Copy)]
struct ZoomRef {
    z: StoredValue<Zoom>,
    strip: StripRef,
    aspects: StoredValue<Vec<f32>>,
    springs: StoredValue<Option<ZoomSprings>>,
}

/// Paint the current viewer photo's zoom (each of the three springs calls this every frame).
fn draw_zoom(springs: StoredValue<Option<ZoomSprings>>, strip: StripRef) {
    let Some(Some(sp)) = springs.try_get_value() else {
        return;
    };
    let (s, x, y) = (sp.s.get(), sp.x.get(), sp.y.get());
    if let Some(el) = zoom_el(strip) {
        let _ = el.style().set_property(
            "transform",
            &format!("translate3d({x:.2}px,{y:.2}px,0) scale({s:.4})"),
        );
    }
    if let Some(vp) = strip.viewport.get_untracked() {
        let _ = if s > 1.01 {
            vp.class_list().add_1("is-zoomed")
        } else {
            vp.class_list().remove_1("is-zoomed")
        };
    }
}

fn zoom_el(strip: StripRef) -> Option<web_sys::HtmlElement> {
    use wasm_bindgen::JsCast;
    let i = strip.index();
    let track = strip.track.get_untracked()?;
    track
        .query_selector(&format!("[data-i=\"{i}\"] .pv__zoom"))
        .ok()
        .flatten()
        .and_then(|e| e.dyn_into::<web_sys::HtmlElement>().ok())
}

impl ZoomRef {
    fn new(strip: StripRef, aspects: Vec<f32>) -> Self {
        let springs = StoredValue::new(None::<ZoomSprings>);
        let draw = move |_| draw_zoom(springs, strip);
        if let (Some(s), Some(x), Some(y)) = (
            client_spring(1.0, draw),
            client_spring(0.0, draw),
            client_spring(0.0, draw),
        ) {
            springs.set_value(Some(ZoomSprings { s, x, y }));
        }
        ZoomRef {
            z: StoredValue::new(Zoom::default()),
            strip,
            aspects: StoredValue::new(aspects),
            springs,
        }
    }

    fn values(self) -> (f64, f64, f64) {
        self.springs
            .get_value()
            .map(|sp| (sp.s.get(), sp.x.get(), sp.y.get()))
            .unwrap_or((1.0, 0.0, 0.0))
    }

    /// Follow the fingers (1:1).
    fn set(self, s: f64, x: f64, y: f64) {
        if let Some(sp) = self.springs.get_value() {
            sp.s.set(s);
            sp.x.set(x);
            sp.y.set(y);
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

    /// Back to fit, at once, on every photo (after switching photos or closing).
    fn reset(self) {
        self.z.update_value(|z| {
            z.pinch = None;
            z.pan = None;
            z.pointers.clear();
        });
        if let Some(track) = self.strip.track.get_untracked()
            && let Ok(all) = track.query_selector_all(".pv__zoom")
        {
            use wasm_bindgen::JsCast;
            for i in 0..all.length() {
                if let Some(el) = all
                    .item(i)
                    .and_then(|n| n.dyn_into::<web_sys::HtmlElement>().ok())
                {
                    let _ = el.style().remove_property("transform");
                }
            }
        }
        self.set(1.0, 0.0, 0.0);
    }

    /// Spring scale and offset to `(s, x, y)`, clamped to what the image allows.
    fn settle(self, s: f64, x: f64, y: f64, vx: f64, vy: f64, spring: Spring) {
        let s = s.clamp(1.0, MAX_ZOOM);
        let (bx, by) = self.z.with_value(|z| z.bounds(s));
        if let Some(sp) = self.springs.get_value() {
            sp.s.animate_to(s, spring, None, None);
            sp.x.animate_to(x.clamp(-bx, bx), spring, Some(vx), None);
            sp.y.animate_to(y.clamp(-by, by), spring, Some(vy), None);
        }
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
        let (s, x, y) = self.values();
        if s > 1.01 {
            self.settle(1.0, 0.0, 0.0, 0.0, 0.0, Spring::SNAPPY);
        } else {
            let k = TAP_ZOOM / s.max(0.01);
            self.settle(
                TAP_ZOOM,
                fx - (fx - x) * k,
                fy - (fy - y) * k,
                0.0,
                0.0,
                Spring::DEFAULT,
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
    let zoom = ZoomRef::new(viewer, data.images.iter().map(|i| i.aspect).collect());
    let root = NodeRef::<html::Section>::new();
    let dialog = NodeRef::<html::Dialog>::new();
    // Hover zoom-to-cursor (fine pointers only): position in % of the slide.
    let hover = RwSignal::new(None::<(f64, f64)>);
    let fine = StoredValue::new(false);
    let broken_watched = StoredValue::new(false);
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
        if !broken_watched.get_value() {
            broken_watched.set_value(true);
            watch_broken(&section);
        }
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
    let online = window_event_listener(leptos::ev::online, move |_| {
        if let Some(section) = root.get_untracked() {
            retry_broken(&section);
        }
    });
    on_cleanup(move || online.remove());
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
                main.go(i, 0.0, Spring::DEFAULT);
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
        if fine.get_value()
            && ev.pointer_type() == "mouse"
            && ev.buttons() == 0
            && let Some(vp) = main.viewport.get_untracked()
        {
            let r = vp.get_bounding_client_rect();
            let x = ((ev.client_x() as f64 - r.left()) / r.width() * 100.0).clamp(0.0, 100.0);
            let y = ((ev.client_y() as f64 - r.top()) / r.height() * 100.0).clamp(0.0, 100.0);
            if hover.get_untracked().is_none() {
                sharpen(main, main.index());
            }
            hover.set(Some((x, y)));
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
            "Home" => main.go(0, 0.0, Spring::DEFAULT),
            "End" => main.go(n - 1, 0.0, Spring::DEFAULT),
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
        let (s, tx, ty) = zoom.values();
        // Grab the photo where it is (stops any settle in flight).
        zoom.set(s, tx, ty);
        let count = zoom
            .z
            .try_update_value(|z| {
                z.pointers.retain(|p| p.0 != id);
                z.pointers.push((id, x, y));
                z.pointers.len()
            })
            .unwrap_or(0);
        if count >= 2 {
            // Second finger: pinch. The strip lets go where it is.
            viewer.st.update_value(|s| s.drag = None);
            let i = viewer.index();
            viewer.go(i, 0.0, Spring::SNAPPY);
            zoom.z.update_value(|z| {
                let (a, b) = (z.pointers[0], z.pointers[1]);
                let d = ((a.1 - b.1).powi(2) + (a.2 - b.2).powi(2)).sqrt().max(1.0);
                let m = (
                    (a.1 + b.1) / 2.0 - z.bw / 2.0,
                    (a.2 + b.2) / 2.0 - z.bh / 2.0,
                );
                z.pan = None;
                z.pinch = Some((d, m, s, (tx, ty)));
            });
        } else if s > 1.01 {
            zoom.z
                .update_value(|z| z.pan = Some(Pan::start(id, x, y, ev.time_stamp(), tx, ty)));
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
        let s_now = zoom.values().0;
        let next = zoom
            .z
            .try_update_value(|z| {
                if let Some(p) = z.pointers.iter_mut().find(|p| p.0 == id) {
                    p.1 = x;
                    p.2 = y;
                }
                if let Some((d0, m0, s0, t0)) = z.pinch {
                    if z.pointers.len() < 2 {
                        return Some(None);
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
                        1.0 - rubberband(1.0 - raw, 1.0, RUBBER)
                    } else if raw > MAX_ZOOM {
                        MAX_ZOOM + rubberband(raw - MAX_ZOOM, MAX_ZOOM, RUBBER)
                    } else {
                        raw
                    };
                    let k = s / s0;
                    return Some(Some((s, m.0 - (m0.0 - t0.0) * k, m.1 - (m0.1 - t0.1) * k)));
                }
                let (bw, bh) = (z.bw, z.bh);
                let (bx, by) = z.bounds(s_now);
                if let Some(p) = z.pan.as_mut() {
                    if p.id != id {
                        return Some(None);
                    }
                    if (x - p.x0).abs() > SLOP || (y - p.y0).abs() > SLOP {
                        p.moved = true;
                    }
                    p.rx = p.ox + x - p.x0;
                    p.ry = p.oy + y - p.y0;
                    p.tx.push(t, p.rx);
                    p.ty.push(t, p.ry);
                    return Some(Some((s_now, band(p.rx, bx, bw), band(p.ry, by, bh))));
                }
                None
            })
            .flatten();
        match next {
            Some(Some((s, tx, ty))) => {
                ev.prevent_default();
                zoom.set(s, tx, ty);
            }
            Some(None) => ev.prevent_default(),
            None => {
                if viewer.moved(&ev) == Axis::X {
                    ev.prevent_default();
                }
            }
        }
    };
    let v_up = move |ev: PointerEvent, cancelled: bool| {
        let (id, t) = (ev.pointer_id(), ev.time_stamp());
        let (x, y) = (ev.client_x() as f64, ev.client_y() as f64);
        let (s_now, x_now, y_now) = zoom.values();
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
                        z.pan = Some(Pan::start(pid, px, py, t, x_now, y_now));
                        return After::Wait;
                    }
                    return After::Settle(s_now, x_now, y_now, 0.0, 0.0);
                }
                if z.pan.as_ref().is_some_and(|p| p.id != id) {
                    return After::Wait;
                }
                if let Some(mut p) = z.pan.take() {
                    if !p.moved && !cancelled {
                        return After::Tap;
                    }
                    p.tx.push(t, p.rx);
                    p.ty.push(t, p.ry);
                    let (vx, vy) = if cancelled {
                        (0.0, 0.0)
                    } else {
                        (p.tx.velocity(), p.ty.velocity())
                    };
                    return After::Settle(
                        s_now,
                        x_now + project(vx, 0.995),
                        y_now + project(vy, 0.995),
                        vx,
                        vy,
                    );
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
            After::Settle(s, sx, sy, vx, vy) => zoom.settle(s, sx, sy, vx, vy, Spring::DEFAULT),
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
                    <span class="pg__fallback" aria-hidden="true">
                        <Icon name="image" size=28/>
                        <span>{move || tr(ui.lang.get(), "Photo unavailable", "الصورة غير متاحة")}</span>
                    </span>
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
                            main.go(i, 0.0, Spring::DEFAULT);
                        }
                    >
                        <Picture img=img sizes=THUMB_SIZES lang=lang/>
                        <span class="pg__fallback pg__fallback--sm" aria-hidden="true"><Icon name="image" size=16/></span>
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
                    <span class="pg__fallback" aria-hidden="true">
                        <Icon name="image" size=32/>
                        <span>{move || tr(ui.lang.get(), "Photo unavailable", "الصورة غير متاحة")}</span>
                    </span>
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
                    aria-label=move || {
                        tr(
                            ui.lang.get(),
                            "Product photos. Use arrow keys to browse, Enter to enlarge.",
                            "صور المنتج. استخدم الأسهم للتنقل وEnter للتكبير.",
                        )
                    }
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
                    // `dir` on an inner span: on the positioned element it would flip its inset.
                    <p class="pg__count mono" aria-hidden="true">
                        <span dir="ltr">{move || count_text(main.active.get())}</span>
                    </p>
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
                    <p class="pv__count mono">
                        <span dir="ltr">{move || multi.then(|| count_text(viewer.active.get()))}</span>
                    </p>
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
                    <span class="pv__hint-touch">
                        {move || tr(ui.lang.get(), "Double-tap or pinch to zoom", "دوس مرتين أو قرّب بصباعين للتكبير")}
                    </span>
                    <span class="pv__hint-mouse">
                        {move || tr(ui.lang.get(), "Double-click to zoom · drag to move", "دوس مرتين للتكبير · اسحب للتحريك")}
                    </span>
                </p>
            </dialog>
        </section>
    }
}

/// Photos that fail (offline, a dead link) become a quiet placeholder in their slot instead of
/// the browser's broken-image icon. Covers images that already failed before hydration.
fn watch_broken(root: &web_sys::Element) {
    use wasm_bindgen::{JsCast, closure::Closure};
    fn mark(img: &web_sys::Element) {
        if let Ok(Some(slot)) = img.closest(".pg__slide, .pg__thumb, .pv__slide") {
            let _ = slot.class_list().add_1("is-broken");
        }
    }
    let Ok(imgs) = root.query_selector_all("img") else {
        return;
    };
    for i in 0..imgs.length() {
        let Some(img) = imgs
            .item(i)
            .and_then(|n| n.dyn_into::<web_sys::Element>().ok())
        else {
            continue;
        };
        let prop = |k: &str| js_sys::Reflect::get(&img, &k.into()).ok();
        let complete = prop("complete").and_then(|v| v.as_bool()).unwrap_or(false);
        let width = prop("naturalWidth").and_then(|v| v.as_f64()).unwrap_or(1.0);
        if complete && width == 0.0 {
            mark(&img);
        }
        let el = img.clone();
        let cb = Closure::<dyn FnMut(web_sys::Event)>::new(move |_| mark(&el));
        let _ = img.add_event_listener_with_callback("error", cb.as_ref().unchecked_ref());
        cb.forget();
    }
}

/// Back online: try the failed photos again (re-setting the attributes reloads them).
fn retry_broken(root: &web_sys::Element) {
    use wasm_bindgen::JsCast;
    let Ok(slots) = root.query_selector_all(".is-broken") else {
        return;
    };
    for i in 0..slots.length() {
        let Some(slot) = slots
            .item(i)
            .and_then(|n| n.dyn_into::<web_sys::Element>().ok())
        else {
            continue;
        };
        let _ = slot.class_list().remove_1("is-broken");
        if let Ok(Some(img)) = slot.query_selector("img") {
            for attr in ["srcset", "src"] {
                if let Some(v) = img.get_attribute(attr) {
                    let _ = img.set_attribute(attr, &v);
                }
            }
        }
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
