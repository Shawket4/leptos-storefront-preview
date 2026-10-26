//! `/checkout?step=address|slot|payment|review` — a multi-step checkout that is a set of plain
//! HTML forms first and an app second.
//!
//! Each step is a real `<form method="post">` to its own server function. The function validates
//! (`rules`), stores a draft server-side keyed by the `visitor` cookie (`draft`), and calls
//! `leptos_axum::redirect` to the next step. The cart is read from the browser's `cart` cookie and
//! priced again at every step; placing the order re-prices it, checks stock and clears the cookie.
//! With JS the client router posts the same forms; `features/checkout.ts` adds constraint
//! messages, focus on the first server error, a pending state and a double-submit guard.
//!
//! How that meets server_fn's own redirect for form posts (verified in the 0.8.22 source): a post
//! that `Accept`s `text/html` (a browser without JS, or the client router) makes server_fn set
//! `Location: <Referer>` itself — with `?__path=…&__err=…` appended on error — and *then*
//! `leptos_axum` applies the `ResponseOptions`, whose `Location` replaces it (`HeaderMap::extend`
//! replaces). So:
//! - success → our `redirect` wins: 302 to the next step;
//! - failure → we redirect too, to the *clean* step URL with the error encoded exactly the way
//!   server_fn would (`ServerFnUrlError`), instead of appending another `__err` pair to a Referer
//!   that may already carry one; the page decodes it and renders the messages inline, next to the
//!   fields, while the typed-in values come back from the draft (personal data stays out of URLs);

mod address;
pub mod draft;
pub mod rules;
mod steps;

use crate::api::types::{CartView, DeliverySlot};
use crate::components::icons::Icon;
use crate::format;
use crate::prefs::{Lang, Numerals, tr};
use crate::request::req;
use domain::PaymentMethod;
use leptos::either::{Either, EitherOf5};
use leptos::prelude::*;
use leptos::server_fn::codec::JsonEncoding;
use leptos::server_fn::error::{FromServerFnError, ServerFnErrorErr, ServerFnUrlError};
use leptos_meta::{Meta, Title};
use leptos_router::hooks::use_query_map;
use rules::{AddressInput, FieldError};
use serde::{Deserialize, Serialize};

pub use address::AddressForm;

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum Step {
    #[default]
    Address,
    Slot,
    Payment,
    Review,
}

impl Step {
    pub const ALL: [Step; 4] = [Step::Address, Step::Slot, Step::Payment, Step::Review];

    pub fn code(self) -> &'static str {
        match self {
            Step::Address => "address",
            Step::Slot => "slot",
            Step::Payment => "payment",
            Step::Review => "review",
        }
    }

    pub fn parse(s: &str) -> Option<Step> {
        Step::ALL.into_iter().find(|x| x.code() == s)
    }

    pub fn href(self) -> String {
        format!("/checkout?step={}", self.code())
    }

    pub fn index(self) -> usize {
        self as usize
    }

    pub fn label(self, lang: Lang) -> &'static str {
        match self {
            Step::Address => tr(lang, "Address", "العنوان"),
            Step::Slot => tr(lang, "Delivery", "التوصيل"),
            Step::Payment => tr(lang, "Payment", "الدفع"),
            Step::Review => tr(lang, "Review", "المراجعة"),
        }
    }
}

/// What a checkout server function can fail with (encoded into the redirect URL by
/// `ServerFnUrlError`, decoded by the page that shows it).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CheckoutError {
    /// Problems with address fields.
    Invalid(Vec<FieldError>),
    NoSlot,
    /// The chosen delivery window closed meanwhile.
    SlotGone,
    NoPayment,
    EmptyCart,
    /// An earlier step isn't done.
    Missing(Step),
    /// The cart changed after the review page was shown.
    CartChanged,
    /// Card and wallet payments aren't switched on yet.
    PaymentUnavailable,
    /// An item sold out, or has less stock than the cart asks for.
    Stock,
    /// Transport, simulated outage, or a malformed request.
    Server(String),
}

impl FromServerFnError for CheckoutError {
    type Encoder = JsonEncoding;

    fn from_server_fn_error(value: ServerFnErrorErr) -> Self {
        CheckoutError::Server(value.to_string())
    }
}

impl std::fmt::Display for CheckoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for CheckoutError {}

impl CheckoutError {
    /// A message for errors that don't belong to one field (MSA, like Madar's error copy).
    pub fn message(&self, lang: Lang) -> String {
        let (en, ar) = match self {
            CheckoutError::Invalid(_) => (
                "Some details need attention.",
                "بعض البيانات تحتاج إلى مراجعة.",
            ),
            CheckoutError::NoSlot => ("Choose a delivery time.", "اختر موعد التوصيل."),
            CheckoutError::SlotGone => (
                "That delivery time just closed. Please choose another one.",
                "انتهى موعد التوصيل الذي اخترته. يرجى اختيار موعد آخر.",
            ),
            CheckoutError::NoPayment => ("Choose how you'd like to pay.", "اختر طريقة الدفع."),
            CheckoutError::EmptyCart => ("Your cart is empty.", "سلة التسوق فارغة."),
            CheckoutError::Missing(_) => (
                "Please complete the earlier steps first.",
                "يرجى إكمال الخطوات السابقة أولًا.",
            ),
            CheckoutError::CartChanged => (
                "Your cart changed since you reviewed it. Please check the new total.",
                "تغيّرت السلة منذ مراجعتها. يرجى مراجعة الإجمالي الجديد.",
            ),
            CheckoutError::PaymentUnavailable => (
                "That payment method isn't available yet. Please choose cash on delivery.",
                "طريقة الدفع هذه غير متاحة بعد. يرجى اختيار الدفع نقدًا عند الاستلام.",
            ),
            CheckoutError::Stock => (
                "Some items in your cart changed since you added them. Fix them in your cart to continue.",
                "تغيّر توفّر بعض المنتجات منذ إضافتها. يرجى تعديلها في السلة للمتابعة.",
            ),
            CheckoutError::Server(_) => (
                "Something went wrong on our side. Please try again.",
                "حدث خطأ من جهتنا. يرجى المحاولة مرة أخرى.",
            ),
        };
        tr(lang, en, ar).to_owned()
    }

    /// The step whose page shows this error.
    pub fn step(&self) -> Step {
        match self {
            CheckoutError::Invalid(_) => Step::Address,
            CheckoutError::NoSlot | CheckoutError::SlotGone => Step::Slot,
            CheckoutError::NoPayment | CheckoutError::PaymentUnavailable => Step::Payment,
            CheckoutError::Missing(s) => *s,
            _ => Step::Review,
        }
    }
}

// ── Server functions ───────────────────────────────────────────────────────────────────────────

#[cfg(feature = "ssr")]
mod imp {
    use super::*;
    use crate::server::{cart, sim, state};
    use domain::cart::CartLine;

    /// A browser form post (no JS) or a client-router post — both ask for HTML.
    pub fn accepts_html() -> bool {
        use_context::<http::request::Parts>()
            .and_then(|p| {
                p.headers
                    .get(http::header::ACCEPT)
                    .and_then(|v| v.to_str().ok())
                    .map(|a| a.contains("text/html"))
            })
            .unwrap_or(false)
    }

    /// Fail a step. For HTML posts, send the browser back to the step's clean URL with the error
    /// encoded the way server_fn encodes it (our `Location` replaces server_fn's Referer one).
    pub fn fail(err: CheckoutError, fn_path: &str) -> CheckoutError {
        if accepts_html() {
            let to = match &err {
                CheckoutError::EmptyCart => "/cart".to_owned(),
                e => e.step().href(),
            };
            match ServerFnUrlError::new(fn_path, err.clone()).to_url(&format!("http://local{to}")) {
                Ok(url) => leptos_axum::redirect(&format!(
                    "{}?{}",
                    url.path(),
                    url.query().unwrap_or_default()
                )),
                Err(_) => leptos_axum::redirect(&to),
            }
        }
        err
    }

    pub struct Visit {
        pub tenant: crate::tenant::Tenant,
        /// The `visitor` cookie (the draft's owner), if the browser has one yet.
        pub visitor: Option<String>,
        pub key: Option<String>,
        /// The `cart` cookie, as sent.
        pub lines: Vec<CartLine>,
        /// …priced now.
        pub cart: CartView,
        /// A line can't be sent as it is (sold out, fewer left, gone from the catalogue).
        pub blocked: bool,
    }

    /// The visitor's tenant, cart (priced now) and draft key. Synchronous: everything is in memory.
    pub fn visit() -> Visit {
        let rq = crate::request::init();
        let st = state();
        let tenant = st.tenant(&rq.tenant.id).clone();
        let lines = cart::from_request();
        let priced = cart::price(&lines, &tenant, &st.catalog(&tenant.id), st.media());
        let visitor = super::draft::visitor();
        let key = visitor.as_deref().map(|v| super::draft::key(&tenant.id, v));
        Visit {
            tenant,
            visitor,
            key,
            blocked: priced.blocked(),
            cart: priced.view,
            lines,
        }
    }

    pub fn slots(tenant: &crate::tenant::Tenant) -> Vec<DeliverySlot> {
        crate::server::orders::slots(tenant, crate::server::orders::now_ms())
    }

    /// Common start of every step: private response, simulated latency/outage, a non-empty cart
    /// and a visitor id (created on the first step).
    pub async fn begin(fn_path: &str) -> Result<(Visit, String), CheckoutError> {
        crate::server::cache::private();
        // Everything from the request first: after the `.await` the client may have gone.
        let mut v = visit();
        let visitor = super::draft::ensure_visitor();
        sim::check_fail(sim::Widget::Checkout)
            .map_err(|e| fail(CheckoutError::Server(e.to_string()), fn_path))?;
        sim::delay(sim::Widget::Checkout).await;
        if v.lines.is_empty() {
            return Err(fail(CheckoutError::EmptyCart, fn_path));
        }
        let key = super::draft::key(&v.tenant.id, &visitor);
        v.visitor = Some(visitor);
        v.key = Some(key.clone());
        Ok((v, key))
    }
}

/// Step 1: the delivery address, validated with `rules::validate` (the browser code only
/// pre-checks the form's HTML5 constraints).
#[server]
pub async fn checkout_address(
    name: String,
    phone: String,
    city: Option<String>,
    area: Option<String>,
    street: String,
    building: String,
    floor: Option<String>,
    apartment: Option<String>,
    landmark: Option<String>,
    notes: Option<String>,
) -> Result<(), CheckoutError> {
    let path = <CheckoutAddress as leptos::server_fn::ServerFn>::PATH;
    let (_, key) = imp::begin(path).await?;
    let input = AddressInput {
        name,
        phone,
        city: city.unwrap_or_default(),
        area: area.unwrap_or_default(),
        street,
        building,
        floor: floor.unwrap_or_default(),
        apartment: apartment.unwrap_or_default(),
        landmark: landmark.unwrap_or_default(),
        notes: notes.unwrap_or_default(),
    };
    let errors = rules::validate(&input);
    if !errors.is_empty() {
        draft::update(&key, |d| d.attempt = Some(input));
        return Err(imp::fail(CheckoutError::Invalid(errors), path));
    }
    draft::update(&key, |d| {
        d.address = Some(rules::clean(&input));
        d.attempt = None;
        d.placed = None;
    });
    leptos_axum::redirect(&Step::Slot.href());
    Ok(())
}

/// Step 2: a delivery window that is still open.
#[server]
pub async fn checkout_slot(slot: Option<String>) -> Result<(), CheckoutError> {
    let path = <CheckoutSlot as leptos::server_fn::ServerFn>::PATH;
    // Whatever happens next, the page that comes back shows the same choice.
    if let Some(k) = imp::visit().key {
        draft::update(&k, |d| d.attempt_slot = slot.clone());
    }
    let (v, key) = imp::begin(path).await?;
    let Some(id) = slot.filter(|s| !s.is_empty()) else {
        return Err(imp::fail(CheckoutError::NoSlot, path));
    };
    if !imp::slots(&v.tenant)
        .iter()
        .any(|s| s.id == id && s.available)
    {
        return Err(imp::fail(CheckoutError::SlotGone, path));
    }
    draft::update(&key, |d| {
        d.slot = Some(id);
        d.attempt_slot = None;
    });
    leptos_axum::redirect(&Step::Payment.href());
    Ok(())
}

/// Step 3: how to pay (cash on delivery; card and wallet are placeholders without a gateway).
#[server]
pub async fn checkout_payment(payment: Option<String>) -> Result<(), CheckoutError> {
    let path = <CheckoutPayment as leptos::server_fn::ServerFn>::PATH;
    let (_, key) = imp::begin(path).await?;
    let method = match payment.as_deref() {
        Some("cod") => PaymentMethod::CashOnDelivery,
        // Shown as "coming soon", never as a choice; a hand-made post still can't pick them.
        Some("card") | Some("wallet") => {
            return Err(imp::fail(CheckoutError::PaymentUnavailable, path));
        }
        _ => return Err(imp::fail(CheckoutError::NoPayment, path)),
    };
    draft::update(&key, |d| d.payment = Some(method));
    leptos_axum::redirect(&Step::Review.href());
    Ok(())
}

/// Step 4: place the order. Re-prices the cart now (like Madar re-prices every order at intake),
/// refuses if the total moved since the review page was rendered, then freezes it into an order,
/// empties the cart, refreshes the cart cookies and sends the visitor to tracking.
#[server]
pub async fn checkout_place(total: Option<i64>) -> Result<(), CheckoutError> {
    let path = <CheckoutPlace as leptos::server_fn::ServerFn>::PATH;
    // Already placed (double submit, back button): the cart is empty now — go to that order.
    {
        let v = imp::visit();
        if let Some(id) = v.key.as_deref().and_then(|k| draft::get(k).placed) {
            if v.lines.is_empty() {
                crate::server::cache::private();
                leptos_axum::redirect(&format!("/orders/{id}"));
                return Ok(());
            }
        }
    }
    let (v, key) = imp::begin(path).await?;
    let d = draft::get(&key);
    let Some(address) = d.address.clone() else {
        return Err(imp::fail(CheckoutError::Missing(Step::Address), path));
    };
    let slots = imp::slots(&v.tenant);
    let Some(slot) = d
        .slot
        .as_ref()
        .and_then(|id| slots.iter().find(|s| &s.id == id && s.available))
        .cloned()
    else {
        return Err(imp::fail(CheckoutError::SlotGone, path));
    };
    let Some(payment) = d.payment.filter(|p| *p == PaymentMethod::CashOnDelivery) else {
        return Err(imp::fail(CheckoutError::Missing(Step::Payment), path));
    };
    if v.blocked {
        return Err(imp::fail(CheckoutError::Stock, path));
    }
    let now_total = v.cart.total + slot.fee;
    if total.is_some_and(|t| t != now_total.0) {
        return Err(imp::fail(CheckoutError::CartChanged, path));
    }
    let st = crate::server::state();
    let city_name = rules::city(&address.city).map(|c| c.en).unwrap_or_default();
    let order = st.orders().place(
        &v.tenant,
        &v.cart,
        domain::Address {
            name: address.name.clone(),
            phone: address.phone.clone(),
            city: city_name.to_owned(),
            area: address.area.clone(),
            street: address.street.clone(),
            building: address.building.clone(),
            floor: address.floor.clone(),
            apartment: address.apartment.clone(),
            landmark: address.landmark.clone(),
        },
        &slot,
        payment,
    );
    // Empty the cart: clear the cookie (the browser code re-reads it after the navigation).
    if let Some(res) = use_context::<leptos_axum::ResponseOptions>() {
        res.append_header(
            http::header::SET_COOKIE,
            crate::server::cart::set_cookie(&[]),
        );
    }
    draft::remember_order(
        &order.id,
        draft::PlacedMeta {
            owner: v.visitor.clone().unwrap_or_default(),
            notes: address.notes.clone(),
            city: address.city.clone(),
            area: address.area.clone(),
        },
    );
    draft::update(&key, |d| {
        d.placed = Some(order.id.clone());
        d.slot = None;
    });
    leptos_axum::redirect(&format!("/orders/{}?placed=1", order.id));
    Ok(())
}

// ── The page ───────────────────────────────────────────────────────────────────────────────────

/// Everything a step needs, resolved on the server before the first byte.
#[derive(Clone, Default)]
pub struct Ctx {
    pub cart: CartView,
    /// A line can't be sent as it is: the review step says so and can't be placed.
    pub blocked: bool,
    /// The lines' snapshots now (JSON): the browser code compares them with what its cart showed
    /// and says so when a price moved (`features/checkout.ts`).
    pub snapshots_json: String,
    pub step: Step,
    /// Which steps are complete.
    pub done: [bool; 4],
    pub address: Option<AddressInput>,
    pub attempt: Option<AddressInput>,
    pub slots: Vec<DeliverySlot>,
    pub slot: Option<String>,
    /// The window picked on the last failed submit.
    pub attempt_slot: Option<String>,
    pub payment: Option<PaymentMethod>,
    /// `forward` or `back`, from the Referer's step (picks the view-transition direction).
    pub dir: &'static str,
    pub now_ms: i64,
}

fn load(asked: Step) -> Option<Ctx> {
    let v = imp::visit();
    if v.lines.is_empty() {
        return None;
    }
    let d = v.key.as_deref().map(draft::get).unwrap_or_default();
    let slots = imp::slots(&v.tenant);
    let snapshots_json = {
        let lang = crate::request::init().prefs.lang;
        let st = crate::server::state();
        let catalog = st.catalog(&v.tenant.id);
        let snaps: Vec<_> = v
            .lines
            .iter()
            .map(|l| crate::server::cart::snapshot(l, &catalog, st.media(), lang))
            .collect();
        crate::pages::cart::snapshots_json(&v.lines, &snaps)
    };
    let slot_ok = d
        .slot
        .as_ref()
        .is_some_and(|id| slots.iter().any(|s| &s.id == id && s.available));
    let paid_ok = d.payment == Some(PaymentMethod::CashOnDelivery);
    let done = [d.address.is_some(), slot_ok, paid_ok, false];
    let reachable = Step::ALL
        .into_iter()
        .find(|s| !done[s.index()])
        .unwrap_or(Step::Review);
    let step = if asked > reachable {
        // Not there yet (a bookmarked review page, a stale tab): to the first open step.
        leptos_axum::redirect(&reachable.href());
        reachable
    } else {
        asked
    };
    let came_from = use_context::<http::request::Parts>()
        .and_then(|p| {
            p.headers
                .get(http::header::REFERER)
                .and_then(|r| r.to_str().ok())
                .map(str::to_owned)
        })
        .and_then(|r| {
            let q = r.split_once('?')?.1.to_owned();
            q.split('&')
                .find_map(|kv| kv.strip_prefix("step=").and_then(Step::parse))
        });
    let dir = match came_from {
        Some(prev) if prev > step => "back",
        _ => "forward",
    };
    Some(Ctx {
        cart: v.cart,
        blocked: v.blocked,
        snapshots_json,
        step,
        done,
        address: d.address,
        attempt: d.attempt,
        slots,
        slot: d.slot.filter(|_| slot_ok),
        attempt_slot: d.attempt_slot,
        payment: d.payment,
        dir,
        now_ms: crate::server::orders::now_ms(),
    })
}

#[component]
pub fn CheckoutPage() -> impl IntoView {
    // Personal: never cached. Headers (and any redirect) are set before the first chunk.
    crate::server::cache::private();
    let rq = req();
    let (lang, num) = (rq.prefs.lang, rq.prefs.numerals);
    let q = use_query_map();
    let asked = q
        .with_untracked(|q| q.get("step"))
        .and_then(|s| Step::parse(&s))
        .unwrap_or(Step::Address);
    // An error a no-JS post brought back (server_fn's `__err` encoding; the newest is last).
    let url_error = q
        .with_untracked(|q| q.get_all("__err"))
        .and_then(|all| all.last().cloned())
        .map(|e| ServerFnUrlError::<CheckoutError>::decode_err(&e));
    let ctx = load(asked);
    let title = format!(
        "{} · {}",
        tr(lang, "Checkout", "إتمام الطلب"),
        rq.tenant.name.pick(lang.is_rtl())
    );

    let body = match ctx {
        None => Either::Left(view! { <steps::CheckoutEmpty lang/> }),
        Some(ctx) => {
            let error = url_error.filter(|e| e.step() == ctx.step);
            Either::Right(view! { <CheckoutBody ctx error lang num/> })
        }
    };
    view! {
        <Title text=title/>
        <Meta name="robots" content="noindex"/>
        <div class="container checkout" data-ts="checkout">{body}</div>
    }
}

#[component]
fn CheckoutBody(
    ctx: Ctx,
    error: Option<CheckoutError>,
    lang: Lang,
    num: Numerals,
) -> impl IntoView {
    let step = ctx.step;
    let extra_fee = match step {
        Step::Payment | Step::Review => ctx
            .slot
            .as_ref()
            .and_then(|id| ctx.slots.iter().find(|s| &s.id == id))
            .map(|s| s.fee)
            .unwrap_or_default(),
        _ => Default::default(),
    };
    let field_errors = match &error {
        Some(CheckoutError::Invalid(es)) => es.clone(),
        _ => vec![],
    };
    let step_error = error
        .as_ref()
        .filter(|e| !matches!(e, CheckoutError::Invalid(_)))
        .map(|e| e.message(lang));
    // One branch per step (and a separate one for an address re-render with server errors), so
    // a step change replaces the step wholesale.
    let panel = match step {
        Step::Address if field_errors.is_empty() => EitherOf5::A(view! {
            <AddressForm initial=ctx.attempt.clone().or(ctx.address.clone()).unwrap_or_default() errors=vec![] form_error=step_error.clone()/>
        }),
        Step::Address => EitherOf5::B(view! {
            <AddressForm initial=ctx.attempt.clone().or(ctx.address.clone()).unwrap_or_default() errors=field_errors form_error=step_error.clone()/>
        }),
        Step::Slot => EitherOf5::C(view! {
            <steps::SlotStep slots=ctx.slots.clone()
                chosen={if step_error.is_some() { ctx.attempt_slot.clone().or(ctx.slot.clone()) } else { ctx.slot.clone() }}
                error=step_error.clone() now_ms=ctx.now_ms lang num/>
        }),
        Step::Payment => EitherOf5::D(
            view! { <steps::PaymentStep chosen=ctx.payment error=step_error.clone() lang/> },
        ),
        Step::Review => EitherOf5::E(view! {
            <steps::ReviewStep ctx=ctx.clone() extra_fee error=step_error.clone() lang num/>
        }),
    };
    view! {
        <header class="co-head">
            <a class="co-back" href={if step == Step::Address { "/cart".to_owned() } else { Step::ALL[step.index() - 1].href() }}>
                <Icon name="arrow-left" size=18 class="flip-rtl"/>
                <span>{if step == Step::Address { tr(lang, "Back to cart", "ارجع للسلة") } else { tr(lang, "Back", "رجوع") }}</span>
            </a>
            <h1 class="t-h2 co-title">{tr(lang, "Checkout", "إتمام الطلب")}</h1>
        </header>
        <steps::Stepper step done=ctx.done lang num/>
        <script type="application/json" data-cart-snapshots="" inner_html=ctx.snapshots_json.clone()></script>
        <crate::pages::cart::PriceChanged lang/>
        <div class={if step == Step::Review { "co-layout co-layout--single" } else { "co-layout" }}>
            <section class=format!("co-main card co-main--{}", ctx.dir) aria-labelledby="co-step-title">
                {panel}
            </section>
            // The review step lists the items and totals itself.
            {(step != Step::Review).then(|| view! { <steps::Summary cart=ctx.cart.clone() extra_fee lang num/> })}
        </div>
    }
}

/// `+ EGP 15.00` / `مجاني`: a slot's fee as shown on its card.
pub fn fee_label(fee: domain::Piastres, lang: Lang, num: Numerals) -> String {
    if fee.0 == 0 {
        tr(lang, "Free", "مجاني").to_owned()
    } else {
        format!("+ {}", format::money(fee, lang, num))
    }
}
