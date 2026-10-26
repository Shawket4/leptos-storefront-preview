//! A slide-over for islands: the cart drawer (from the inline end — the left in Arabic) and the
//! filters bottom sheet.
//!
//! A modal `<dialog>`: the browser traps focus, makes the page inert and gives Escape; we restore
//! focus to the opener on close and lock page scroll in CSS. Motion is a spring
//! ([`super::motion`]): it opens and closes along the same path, follows the finger 1:1 when
//! dragged toward its edge, rubber-bands when pulled past open, and on release the velocity
//! decides — projected past halfway it closes, otherwise it springs back (with a little bounce,
//! because a flick preceded it). Reduced motion: a short cross-fade instead.

use super::button::IconButton;
use super::motion::{Spring, SpringValue, VelocityTracker, project, rubberband};
use crate::prefs::{tr, use_ui};
use leptos::html;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DrawerSide {
    /// From the inline end, full height (cart).
    #[default]
    End,
    /// From the bottom, up to 88% of the screen (filters on phones).
    Bottom,
}

#[component]
pub fn Drawer(
    /// Open/closed; the drawer animates to match and sets it back to `false` when dismissed.
    open: RwSignal<bool>,
    #[prop(optional)] side: DrawerSide,
    /// The heading (reactive: follow `use_ui().lang`).
    #[prop(into)]
    title: Signal<String>,
    /// Unique id on the page (labels the dialog); default `drawer`.
    #[prop(optional, into)]
    id: Option<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let ui = use_ui();
    let id = id.unwrap_or_else(|| "drawer".to_owned());
    let title_id = format!("{id}-title");
    let dialog = NodeRef::<html::Dialog>::new();
    let panel = NodeRef::<html::Div>::new();
    let scrim = NodeRef::<html::Div>::new();
    let bottom = side == DrawerSide::Bottom;
    let cls = if bottom {
        "drawer drawer--bottom"
    } else {
        "drawer drawer--end"
    };

    // The panel's extent along its axis (px), measured when it opens.
    let extent = StoredValue::new(400.0_f64);
    // The element that had focus before opening is tagged, to give focus back on close. (Nothing
    // thread-bound is stored: this body also runs on the server, when the island renders.)
    let opener_attr = "data-drawer-opener";
    let opener_id = StoredValue::new(id.clone());

    // p = how far the panel is from open, along its dismiss direction (0 = open, extent = gone).
    let pos = SpringValue::new(0.0, move |p| {
        let (Some(panel), Some(scrim)) = (panel.get_untracked(), scrim.get_untracked()) else {
            return;
        };
        let rtl = ui.lang.get_untracked().is_rtl();
        let t = if bottom {
            format!("translate3d(0,{p}px,0)")
        } else {
            let x = if rtl { -p } else { p };
            format!("translate3d({x}px,0,0)")
        };
        let panel: &web_sys::HtmlElement = &panel;
        let _ = panel.style().set_property("transform", &t);
        let progress = (1.0 - p / extent.get_value().max(1.0)).clamp(0.0, 1.0);
        let scrim: &web_sys::HtmlElement = &scrim;
        let _ = scrim
            .style()
            .set_property("opacity", &format!("{progress:.3}"));
    });

    let finish_close = move || {
        if let Some(d) = dialog.get_untracked() {
            if d.open() {
                d.close();
            }
        }
        let sel = opener_id.with_value(|oid| format!("[{opener_attr}=\"{oid}\"]"));
        if let Ok(Some(el)) = document().query_selector(&sel) {
            let _ = el.remove_attribute(opener_attr);
            if let Ok(el) = wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlElement>(el) {
                let _ = el.focus();
            }
        }
    };

    // Follow `open`.
    Effect::new(move |_| {
        let want = open.get();
        let Some(d) = dialog.get() else { return };
        if want {
            if !d.open() {
                if let Some(active) = document().active_element() {
                    let _ = opener_id.with_value(|oid| active.set_attribute(opener_attr, oid));
                }
                let _ = d.show_modal();
                if let Some(p) = panel.get_untracked() {
                    let r = p.get_bounding_client_rect();
                    extent.set_value(if bottom { r.height() } else { r.width() });
                }
                pos.set(extent.get_value());
            }
            pos.animate_to(0.0, Spring::DEFAULT, None, None);
        } else if d.open() {
            pos.animate_to(
                extent.get_value(),
                Spring::SNAPPY,
                None,
                Some(Box::new(finish_close)),
            );
        }
    });

    // ── Drag to dismiss ──
    // The bottom sheet drags from its grabber and head (its content scrolls); the end drawer
    // drags sideways anywhere (vertical scrolling stays native via `touch-action: pan-y`). A drag
    // only starts after 10px along the dismiss axis, and a click after a real drag is swallowed.
    let drag: StoredValue<Option<Drag>> = StoredValue::new(None);
    let dragged = StoredValue::new(false);
    let on_down = move |ev: web_sys::PointerEvent| {
        if ev.button() != 0 {
            return;
        }
        let target = ev
            .target()
            .and_then(|t| wasm_bindgen::JsCast::dyn_into::<web_sys::Element>(t).ok());
        if bottom {
            let handle = target
                .as_ref()
                .and_then(|el| el.closest(".drawer__grabber, .drawer__head").ok().flatten())
                .is_some();
            let on_button = target
                .as_ref()
                .and_then(|el| el.closest("button, a").ok().flatten())
                .is_some();
            if !handle || on_button {
                return;
            }
        }
        // Grabbing a drawer on its way out takes it back: the drag decides again.
        if !open.get_untracked() {
            open.set(true);
        }
        let mut tracker = VelocityTracker::default();
        tracker.push(ev.time_stamp(), 0.0);
        dragged.set_value(false);
        drag.set_value(Some(Drag {
            pointer: ev.pointer_id(),
            x0: ev.client_x() as f64,
            y0: ev.client_y() as f64,
            p0: pos.get(),
            active: false,
            tracker,
        }));
    };
    let on_move = move |ev: web_sys::PointerEvent| {
        let rtl = ui.lang.get_untracked().is_rtl();
        // Some(p): move to p; None inside: abandon (not our gesture).
        let step = drag.try_update_value(|d| -> Option<Option<(f64, bool)>> {
            let d = d.as_mut()?;
            if d.pointer != ev.pointer_id() {
                return None;
            }
            let dx = ev.client_x() as f64 - d.x0;
            let dy = ev.client_y() as f64 - d.y0;
            // Movement toward the edge the panel lives on is positive.
            let (along, across) = if bottom {
                (dy, dx)
            } else {
                (if rtl { -dx } else { dx }, dy)
            };
            let mut started = false;
            if !d.active {
                if along.abs() < 10.0 && across.abs() < 10.0 {
                    return None;
                }
                if across.abs() > along.abs() {
                    return Some(None);
                }
                d.active = true;
                started = true;
            }
            d.tracker.push(ev.time_stamp(), along);
            let raw = d.p0 + along;
            // Pulled past open: resist progressively, never a hard stop.
            let p = if raw < 0.0 {
                -rubberband(-raw, extent.get_value(), 0.55)
            } else {
                raw
            };
            Some(Some((p, started)))
        });
        match step.flatten() {
            Some(Some((p, started))) => {
                if started {
                    dragged.set_value(true);
                    if let Some(el) = ev
                        .current_target()
                        .and_then(|t| wasm_bindgen::JsCast::dyn_into::<web_sys::Element>(t).ok())
                    {
                        let _ = el.set_pointer_capture(ev.pointer_id());
                    }
                }
                ev.prevent_default();
                pos.set(p);
            }
            Some(None) => drag.set_value(None),
            None => {}
        }
    };
    let on_up = move |ev: web_sys::PointerEvent| {
        let Some(d) = drag.get_value() else { return };
        drag.set_value(None);
        if !d.active || d.pointer != ev.pointer_id() {
            return;
        }
        let v = d.tracker.velocity();
        let p = pos.get();
        let size = extent.get_value();
        if p + project(v, 0.998) > size / 2.0 {
            // Commit: carry the finger's speed all the way out, then close for real.
            pos.animate_to(
                size,
                Spring::DEFAULT,
                Some(v.max(0.0)),
                Some(Box::new(move || {
                    open.set(false);
                    finish_close();
                })),
            );
        } else {
            // Back to open; a flick earns a little bounce.
            let spring = if v.abs() > 300.0 {
                Spring::BOUNCY
            } else {
                Spring::DEFAULT
            };
            pos.animate_to(0.0, spring, Some(v), None);
        }
    };
    let on_click_capture = move |ev: web_sys::MouseEvent| {
        if dragged.get_value() {
            dragged.set_value(false);
            ev.prevent_default();
            ev.stop_propagation();
        }
    };

    let close_label = move || tr(ui.lang.get(), "Close", "إغلاق");
    let labelled_by = title_id.clone();
    view! {
        <dialog
            class=cls
            node_ref=dialog
            aria-labelledby=labelled_by
            on:cancel=move |ev: web_sys::Event| {
                // Escape: leave along the way it came, not in one frame.
                ev.prevent_default();
                open.set(false);
            }
        >
            <div class="drawer__scrim" node_ref=scrim on:click=move |_| open.set(false)></div>
            <div
                class="drawer__panel"
                node_ref=panel
                on:pointerdown=on_down
                on:pointermove=on_move
                on:pointerup=on_up
                on:pointercancel=on_up
                on:click:capture=on_click_capture
            >
                {bottom.then(|| view! { <div class="drawer__grabber" aria-hidden="true"></div> })}
                <header class="drawer__head">
                    <h2 class="drawer__title" id=title_id>{move || title.get()}</h2>
                    <IconButton
                        icon="x"
                        label=close_label
                        kind=crate::components::button::ButtonKind::Ghost
                        class="drawer__close"
                        on:click=move |_| open.set(false)
                    />
                </header>
                <div class="drawer__body">{children()}</div>
            </div>
        </dialog>
    }
}

#[derive(Clone)]
struct Drag {
    pointer: i32,
    x0: f64,
    y0: f64,
    p0: f64,
    active: bool,
    tracker: VelocityTracker,
}
