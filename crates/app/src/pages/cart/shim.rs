//! Stand-ins for the design system's tier-3 components (drawer, toasts), with the contract's names
//! and props, until they land on main. Then: delete this file (and the marked block at the end of
//! `style/pages/cart.css`) and import from `crate::components` instead.

pub mod toast {
    //! A small toast queue (polite live region; errors are assertive). The kit's `toast::show`
    //! replaces it; the `Toaster` renders inside the cart drawer island until the shell mounts the
    //! real one. `Toasts` is a `Copy` handle, so async code (after an `.await`, where there is no
    //! reactive owner to read context from) can keep one and still raise toasts.
    use crate::components::icons::Icon;
    use crate::prefs::{tr, use_ui};
    use leptos::prelude::*;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ToastKind {
        Success,
        Error,
        Info,
    }

    #[derive(Clone)]
    pub struct Toast {
        pub id: u32,
        pub kind: ToastKind,
        pub text: String,
        pub action: Option<(String, Callback<()>)>,
    }

    #[derive(Clone, Copy)]
    pub struct Toasts {
        pub list: RwSignal<Vec<Toast>>,
        next: StoredValue<u32>,
    }

    pub fn provide() -> Toasts {
        let t = Toasts {
            list: RwSignal::new(vec![]),
            next: StoredValue::new(1),
        };
        provide_context(t);
        t
    }

    pub fn use_toasts() -> Toasts {
        use_context::<Toasts>().unwrap_or_else(provide)
    }

    /// Show a toast (from an event handler); returns its id.
    pub fn show(kind: ToastKind, text: impl Into<String>) -> u32 {
        use_toasts().show(kind, text)
    }

    impl Toasts {
        pub fn show(self, kind: ToastKind, text: impl Into<String>) -> u32 {
            self.push(kind, text.into(), None)
        }

        /// A toast with one action button ("Undo", "View cart").
        pub fn show_with_action(
            self,
            kind: ToastKind,
            text: impl Into<String>,
            label: impl Into<String>,
            on: impl Fn() + Send + Sync + 'static,
        ) -> u32 {
            self.push(
                kind,
                text.into(),
                Some((label.into(), Callback::new(move |_| on()))),
            )
        }

        fn push(
            self,
            kind: ToastKind,
            text: String,
            action: Option<(String, Callback<()>)>,
        ) -> u32 {
            let id = self.next.get_value();
            self.next.set_value(id + 1);
            let long = action.is_some() || kind == ToastKind::Error;
            self.list.update(|l| {
                l.push(Toast {
                    id,
                    kind,
                    text,
                    action,
                });
                // Never more than three on screen.
                let extra = l.len().saturating_sub(3);
                l.drain(..extra);
            });
            #[cfg(feature = "hydrate")]
            set_timeout(
                move || self.dismiss(id),
                std::time::Duration::from_millis(if long { 6000 } else { 4000 }),
            );
            #[cfg(not(feature = "hydrate"))]
            let _ = long;
            id
        }

        pub fn dismiss(self, id: u32) {
            self.list.update(|l| l.retain(|x| x.id != id));
        }
    }

    /// The toast stack. `in_drawer`: a second instance inside the cart drawer, because a modal
    /// `<dialog>` sits in the top layer and would cover the page's own stack while it is open (CSS
    /// shows exactly one of the two).
    #[component]
    pub fn Toaster(#[prop(optional)] in_drawer: bool) -> impl IntoView {
        let t = use_toasts();
        let ui = use_ui();
        let class = if in_drawer {
            "toaster toaster--drawer"
        } else {
            "toaster toaster--page"
        };
        view! {
            <div class=class role="region" aria-label=move || { tr(ui.lang.get(), "Notifications", "التنبيهات") }>
                <div class="toaster__stack" role="status" aria-live="polite">
                    <For each=move || t.list.get().into_iter().filter(|x| x.kind != ToastKind::Error) key=|x| x.id let:toast>
                        <ToastView toast t/>
                    </For>
                </div>
                <div class="toaster__stack" role="alert" aria-live="assertive">
                    <For each=move || t.list.get().into_iter().filter(|x| x.kind == ToastKind::Error) key=|x| x.id let:toast>
                        <ToastView toast t/>
                    </For>
                </div>
            </div>
        }
    }

    #[component]
    fn ToastView(toast: Toast, t: Toasts) -> impl IntoView {
        let ui = use_ui();
        let id = toast.id;
        let (icon, cls) = match toast.kind {
            ToastKind::Success => ("circle-check", "toast toast--success"),
            ToastKind::Error => ("circle-alert", "toast toast--error"),
            ToastKind::Info => ("info", "toast toast--info"),
        };
        view! {
            <div class=cls>
                <Icon name=icon size=18 class="toast__icon"/>
                <p class="toast__text">{toast.text}</p>
                {toast.action.map(|(label, cb)| view! {
                    <button type="button" class="toast__action" on:click=move |_| { t.dismiss(id); cb.run(()); }>{label}</button>
                })}
                <button type="button" class="toast__close" aria-label=move || { tr(ui.lang.get(), "Dismiss", "إغلاق") }
                    on:click=move |_| t.dismiss(id)>
                    <Icon name="x" size=16/>
                </button>
            </div>
        }
    }
}

pub mod drawer {
    //! A side sheet on the native `<dialog>`: `showModal()` gives the top layer, focus trap, Esc and
    //! an inert page for free; it enters from the inline-end edge (left in Arabic) with
    //! `--ease-drawer`, and the CSS handles the exit with `@starting-style`.
    use crate::components::icons::Icon;
    use crate::prefs::{tr, use_ui};
    use leptos::prelude::*;

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub enum DrawerSide {
        #[default]
        End,
        Start,
    }

    #[component]
    pub fn Drawer(
        open: RwSignal<bool>,
        #[prop(optional)] side: DrawerSide,
        #[prop(into)] title: Signal<String>,
        /// DOM id of the `<dialog>` (for `aria-controls` on the opener).
        #[prop(optional)]
        id: &'static str,
        #[prop(optional, into)] class: String,
        children: Children,
    ) -> impl IntoView {
        let ui = use_ui();
        let dialog = NodeRef::<leptos::html::Dialog>::new();
        Effect::new(move |_| {
            let want = open.get();
            let Some(d) = dialog.get() else { return };
            if want && !d.open() {
                let _ = d.show_modal();
            } else if !want && d.open() {
                d.close();
            }
        });
        let side = match side {
            DrawerSide::End => "drawer--end",
            DrawerSide::Start => "drawer--start",
        };
        let id = if id.is_empty() { "drawer" } else { id };
        let title_id = format!("{id}-title");
        let labelled_by = title_id.clone();
        view! {
            <dialog node_ref=dialog id=id class=format!("drawer {side} {class}") aria-labelledby=labelled_by
                on:close=move |_| open.set(false)
                on:click=move |ev| {
                    // A click on the dialog box itself (not its content) is a click on the backdrop.
                    use wasm_bindgen::JsCast;
                    if ev.target().and_then(|t| t.dyn_into::<web_sys::HtmlDialogElement>().ok()).is_some() {
                        open.set(false);
                    }
                }>
                <div class="drawer__panel">
                    <header class="drawer__head">
                        <h2 id=title_id class="drawer__title">{move || title.get()}</h2>
                        <button type="button" class="icon-btn icon-btn--ghost drawer__close"
                            aria-label=move || { tr(ui.lang.get(), "Close", "إغلاق") } on:click=move |_| open.set(false)>
                            <Icon name="x" size=20/>
                        </button>
                    </header>
                    {children()}
                </div>
            </dialog>
        }
    }
}
