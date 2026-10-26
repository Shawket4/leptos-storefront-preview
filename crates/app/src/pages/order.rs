//! `/orders/:id` — order tracking.
//!
//! The server renders the whole page — status timeline, expected times, summary — so it's
//! complete without JS (with a refresh link). The moving parts ([`live::OrderLive`]) are kept
//! current by `features/checkout.ts`, which polls the fixed endpoint `GET /api/orders/track?id=`
//! ([`order_track`], shape `OrderTrack` in `client/src/api/types.ts`) — the same [`TrackJson`]
//! the page was rendered from.
//!
//! Tracking links can be shared, so only the visitor who placed the order (same `visitor` cookie)
//! sees the name, phone and street; anyone else sees the status and the area.

mod live;

use crate::components::icons::Icon;
use crate::components::image::Picture;
use crate::components::price::{Price, PriceSize};
use crate::components::states::{SectionError, StateTone, StateView};
use crate::format;
use crate::media::Img;
use crate::pages::cart::{ArtImage, art};
use crate::pages::checkout::rules;
use crate::prefs::{Lang, Numerals, tr};
use crate::request::req;
use domain::{Order, OrderStatus as Status, PaymentMethod, Piastres};
use leptos::either::Either;
use leptos::prelude::*;
use leptos::server_fn::codec::GetUrl;
use leptos_meta::{Meta, Title};
use leptos_router::hooks::{use_params_map, use_query_map};
use serde::{Deserialize, Serialize};

pub use live::OrderLive;

/// The moving parts of an order, formatted in the visitor's language and numerals: what the
/// tracking block renders and what `GET /api/orders/track` answers (`OrderTrack` in TS).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TrackJson {
    pub id: String,
    /// `received`, `confirmed`, `preparing`, `ready`, `out_for_delivery`, `delivered`,
    /// `cancelled`, `rejected`.
    pub status: String,
    pub headline: String,
    pub eta: String,
    /// One per timeline step, in order.
    pub steps: Vec<TrackStep>,
    /// How far the timeline's line has grown, 0..=1.
    pub progress: f32,
    pub cancelled: bool,
    /// A final status: nothing will change any more (stop polling).
    pub done: bool,
    /// When the next step is due, from now (ms) — for pacing the polls.
    pub next_in_ms: Option<i64>,
    /// "Updated 06:02 PM".
    pub updated: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TrackStep {
    /// `done`, `current` or `todo`.
    pub state: String,
    /// When it happened, or "Expected around …" for the next one.
    pub time: Option<String>,
}

fn status_code(s: Status) -> &'static str {
    match s {
        Status::Received => "received",
        Status::Confirmed => "confirmed",
        Status::Preparing => "preparing",
        Status::Ready => "ready",
        Status::OutForDelivery => "out_for_delivery",
        Status::Delivered => "delivered",
        Status::Cancelled => "cancelled",
        Status::Rejected => "rejected",
    }
}

/// The order's tracking view now.
pub fn track_json(o: &Order, lang: Lang, num: Numerals) -> TrackJson {
    let now = crate::server::orders::now_ms();
    let at = |ms| format::time_cairo(ms, lang, num);
    let ar = lang.is_rtl();
    let next = crate::server::orders::OrderStore::next_step_at(o);
    let express = o.slot.en.starts_with("Express");
    let cancelled = matches!(o.status, Status::Cancelled | Status::Rejected);
    let terminal = o.status.is_terminal();
    let cur = o.status.step_index().unwrap_or(0);
    let eta = match o.status {
        Status::Delivered => {
            let when = o.status_times_ms.iter().find(|(s, _)| *s == Status::Delivered).map(|(_, ms)| at(*ms)).unwrap_or_default();
            match lang {
                Lang::En => format!("Delivered at {when}. Enjoy!"),
                Lang::Ar => format!("اتوصّل الساعة {when}. بالهنا والشفا!"),
            }
        }
        Status::Cancelled | Status::Rejected => {
            tr(lang, "If you paid, the amount will be refunded.", "لو كنت دفعت، المبلغ هيرجعلك.").to_owned()
        }
        _ if express => {
            let eta = at(o.created_at_ms + o.estimated_minutes as i64 * 60_000);
            match lang {
                Lang::En => format!("Arriving by {eta}"),
                Lang::Ar => format!("هيوصلك قبل {eta}"),
            }
        }
        _ => match lang {
            Lang::En => format!("Delivery window: {}", o.slot.en),
            Lang::Ar => format!("ميعاد التوصيل: {}", o.slot.ar),
        },
    };
    let steps = Status::STEPS
        .into_iter()
        .enumerate()
        .map(|(i, step)| {
            let state = if cancelled {
                "todo"
            } else if i < cur || (i == cur && terminal) {
                "done"
            } else if i == cur {
                "current"
            } else {
                "todo"
            };
            let time = if let Some((_, ms)) = o.status_times_ms.iter().find(|(s, _)| *s == step) {
                Some(at(*ms))
            } else if !cancelled && next.is_some_and(|(s, _)| s == step) {
                let when = at(next.map(|(_, ms)| ms).unwrap_or_default());
                Some(match lang {
                    Lang::En => format!("Expected around {when}"),
                    Lang::Ar => format!("متوقع حوالي {when}"),
                })
            } else {
                None
            };
            TrackStep { state: state.to_owned(), time }
        })
        .collect();
    TrackJson {
        id: o.id.clone(),
        status: status_code(o.status).to_owned(),
        headline: o.status.headline().pick(ar).to_owned(),
        eta,
        steps,
        progress: if cancelled { 0.0 } else { cur as f32 / (Status::STEPS.len() - 1) as f32 },
        cancelled,
        done: terminal,
        next_in_ms: next.map(|(_, ms)| (ms - now).max(0)),
        updated: format!("{} {}", tr(lang, "Updated", "آخر تحديث"), at(now)),
    }
}

/// Everything the page renders.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderPageData {
    pub order: Order,
    pub track: TrackJson,
    /// Line images, resolved.
    pub imgs: Vec<Img>,
    /// The visitor placed this order (sees the full address).
    pub owner: bool,
    pub notes: String,
    pub city: String,
    pub area: String,
}

/// `GET /api/orders/track?id=` — the order's tracking view now (`null` for an unknown id).
#[server(endpoint = "orders/track", input = GetUrl)]
pub async fn order_track(id: String) -> Result<Option<TrackJson>, ServerFnError> {
    let rq = crate::request::init();
    crate::server::cache::private();
    crate::server::sim::check_fail(crate::server::sim::Widget::Order)?;
    crate::server::sim::delay(crate::server::sim::Widget::Order).await;
    Ok(crate::server::state()
        .orders()
        .get(&rq.tenant.id, &id)
        .map(|o| track_json(&o, rq.prefs.lang, rq.prefs.numerals)))
}

/// The tracking page's data. `None` for an unknown id (or another tenant's order).
#[server]
pub async fn order_detail(id: String) -> Result<Option<OrderPageData>, ServerFnError> {
    let rq = crate::request::init();
    crate::server::cache::private();
    let visitor = crate::pages::checkout::draft::visitor();
    crate::server::sim::check_fail(crate::server::sim::Widget::Order)?;
    crate::server::sim::delay(crate::server::sim::Widget::Order).await;
    let st = crate::server::state();
    let Some(order) = st.orders().get(&rq.tenant.id, &id) else {
        return Ok(None);
    };
    let meta = crate::pages::checkout::draft::order_meta(&order.id);
    let owner = meta
        .as_ref()
        .is_some_and(|m| !m.owner.is_empty() && visitor.as_deref() == Some(m.owner.as_str()));
    let imgs = order
        .lines
        .iter()
        .map(|l| {
            l.image
                .as_deref()
                .map(|k| st.media().img(k))
                .unwrap_or_else(|| Img::missing(""))
        })
        .collect();
    let track = track_json(&order, rq.prefs.lang, rq.prefs.numerals);
    let (notes, city, area) = meta.map(|m| (m.notes, m.city, m.area)).unwrap_or_default();
    Ok(Some(OrderPageData {
        order,
        track,
        imgs,
        owner,
        notes,
        city,
        area,
    }))
}

#[component]
pub fn OrderPage() -> impl IntoView {
    crate::server::cache::private();
    let rq = req();
    let (lang, num) = (rq.prefs.lang, rq.prefs.numerals);
    let params = use_params_map();
    let id = params.with_untracked(|p| p.get("id")).unwrap_or_default();
    let just_placed = use_query_map().with_untracked(|q| q.get("placed").is_some());
    // The status has to be decided before the first chunk streams; the lookup is in memory.
    if crate::server::state()
        .orders()
        .get(&rq.tenant.id, &id)
        .is_none()
    {
        if let Some(res) = use_context::<leptos_axum::ResponseOptions>() {
            res.set_status(http::StatusCode::NOT_FOUND);
        }
    }
    let shown_id = id.to_ascii_uppercase();
    let example = format!("{}-48213", rq.tenant.order_prefix);
    let data = Resource::new(move || id.clone(), order_detail);
    let tenant = rq.tenant.name.pick(lang.is_rtl()).to_owned();
    view! {
        <Meta name="robots" content="noindex"/>
        <div class="container order-page">
            <Suspense fallback=move || view! { <OrderSkeleton/> }>
                <ErrorBoundary fallback=move |errors| view! {
                    <SectionError errors what=tr(lang, "this order", "الطلب").to_owned()/>
                }>
                    {
                        let tenant = tenant.clone();
                        move || {
                            let (tenant, shown_id, example) = (tenant.clone(), shown_id.clone(), example.clone());
                            Suspend::new(async move {
                                let d = data.await?;
                                Ok::<_, ServerFnError>(match d {
                                    Some(d) => Either::Left(view! {
                                        <Title text=format!("{} {} · {tenant}", tr(lang, "Order", "طلب"), d.order.id)/>
                                        <OrderView d just_placed lang num/>
                                    }),
                                    None => Either::Right(view! {
                                        <Title text=format!("{} · {tenant}", tr(lang, "Order not found", "الطلب غير موجود"))/>
                                        <OrderNotFound id=shown_id.clone() example=example.clone() lang/>
                                    }),
                                })
                            })
                        }
                    }
                </ErrorBoundary>
            </Suspense>
        </div>
    }
}

#[component]
fn OrderNotFound(id: String, example: String, lang: Lang) -> impl IntoView {
    let body = match lang {
        Lang::En => format!(
            "There's no order {id} in this shop — the link may be mistyped, or the order is older than we keep. Try the number from your confirmation message."
        ),
        Lang::Ar => format!(
            "مفيش طلب برقم {id} في المتجر ده — يمكن اللينك فيه غلطة، أو الطلب أقدم من المدة اللي بنحتفظ بيها. جرّب الرقم اللي في رسالة التأكيد."
        ),
    };
    view! {
        <div class="order-missing">
            <StateView tone=StateTone::NotFound page=true
                title=tr(lang, "We couldn't find that order", "مش لاقيين الطلب ده")
                body=body/>
            <crate::pages::orders_lookup::LookupForm value=String::new() error=None example lang/>
            <p class="order-page__more">
                <a class="link" href="/">{tr(lang, "Back to the shop", "ارجع للمتجر")}</a>
            </p>
        </div>
    }
}

#[component]
fn OrderSkeleton() -> impl IntoView {
    use crate::components::skeleton::Skeleton;
    view! {
        <div class="order-layout" aria-hidden="true">
            <div class="card order-track">
                <Skeleton width="40%" height="1rem"/>
                <Skeleton width="70%" height="2.25rem"/>
                <Skeleton width="55%" height="1rem"/>
                <Skeleton width="100%" height="16rem"/>
            </div>
            <div class="card order-side"><Skeleton width="100%" height="20rem"/></div>
        </div>
    }
}

pub fn payment_label(p: PaymentMethod, lang: Lang) -> &'static str {
    match p {
        PaymentMethod::CashOnDelivery => tr(lang, "Cash on delivery", "كاش عند الاستلام"),
        PaymentMethod::Card => tr(
            lang,
            "Card (preview — nothing charged)",
            "كارت بنكي (تجريبي — مفيش خصم)",
        ),
        PaymentMethod::Wallet => tr(
            lang,
            "Mobile wallet / InstaPay (preview)",
            "محفظة موبايل / InstaPay (تجريبي)",
        ),
    }
}

#[component]
fn OrderView(d: OrderPageData, just_placed: bool, lang: Lang, num: Numerals) -> impl IntoView {
    let ar = lang.is_rtl();
    let o = d.order.clone();
    let first_name = o
        .address
        .name
        .split_whitespace()
        .next()
        .unwrap_or_default()
        .to_owned();
    let placed_at = format!(
        "{} · {}",
        format::date_cairo(o.created_at_ms, lang, num),
        format::time_cairo(o.created_at_ms, lang, num)
    );
    let city = rules::city(&d.city)
        .map(|c| c.name(lang))
        .unwrap_or_default();
    let area = rules::area_name(&d.area, lang).unwrap_or_default();
    let place = format!("{area}{}{city}", if ar { "، " } else { ", " });
    let address = if d.owner {
        let a = &o.address;
        let mut bits = vec![format!("{} {}", tr(lang, "Building", "عمارة"), a.building)];
        if !a.floor.is_empty() {
            bits.push(format!("{} {}", tr(lang, "floor", "الدور"), a.floor));
        }
        if !a.apartment.is_empty() {
            bits.push(format!("{} {}", tr(lang, "apt", "شقة"), a.apartment));
        }
        let sep = if ar { "، " } else { ", " };
        let mut lines = vec![
            format!(
                "{} · {}",
                a.name,
                format::ltr(&format::digits(&rules::pretty_phone(&a.phone), num))
            ),
            format!("{}{sep}{}", a.street, bits.join(sep)),
            place.clone(),
        ];
        if !a.landmark.is_empty() {
            lines.push(format!(
                "{}: {}",
                tr(lang, "Landmark", "علامة مميزة"),
                a.landmark
            ));
        }
        if !d.notes.is_empty() {
            lines.push(format!("{}: {}", tr(lang, "Notes", "ملاحظات"), d.notes));
        }
        lines
    } else {
        vec![place.clone()]
    };
    let lines: Vec<(domain::OrderLine, Img)> = o
        .lines
        .iter()
        .cloned()
        .zip(d.imgs.iter().cloned())
        .collect();
    let count: u32 = o.lines.iter().map(|l| l.quantity).sum();
    let vat = crate::pages::cart::vat_note(o.tax_included, lang, num);
    view! {
        {(just_placed && d.owner).then(|| view! {
            <section class="placed card" aria-labelledby="placed-title">
                <ArtImage art=art("order-placed") fallback_icon="package-check" lang/>
                <div class="placed__text">
                    <p class="placed__title" id="placed-title">
                        {match lang {
                            Lang::En => format!("Thank you, {first_name}! Your order is in."),
                            Lang::Ar => format!("شكرًا يا {first_name}! طلبك وصلنا."),
                        }}
                    </p>
                    <p class="placed__sub">
                        {tr(lang, "We'll keep this page up to date — no need to refresh.", "الصفحة دي هتتحدث لوحدها، مش محتاج تعمل تحديث.")}
                    </p>
                </div>
            </section>
        })}
        <div class="order-layout">
            <section class="card order-track" aria-labelledby="order-headline">
                <p class="order-track__kicker">
                    <span>{tr(lang, "Order", "طلب")}</span>
                    " "
                    <span class="mono order-id" dir="ltr">{o.id.clone()}</span>
                    <span class="order-track__dot" aria-hidden="true">"·"</span>
                    <span>{placed_at}</span>
                </p>
                <div class="order-track__body">
                    <ArtImage art=art("delivery") fallback_icon="truck" lang/>
                    <OrderLive t=d.track.clone() lang/>
                </div>
            </section>
            <aside class="card order-side" aria-label=tr(lang, "Order details", "تفاصيل الطلب")>
                <dl class="review">
                    <div class="review__row">
                        <dt><Icon name="map-pin" size=18/><span>{tr(lang, "Delivering to", "التوصيل على")}</span></dt>
                        <dd>{address.into_iter().map(|l| view! { <span class="review__line">{l}</span> }).collect_view()}</dd>
                    </div>
                    <div class="review__row">
                        <dt><Icon name="credit-card" size=18/><span>{tr(lang, "Payment", "الدفع")}</span></dt>
                        <dd><span class="review__line">{payment_label(o.payment, lang)}</span></dd>
                    </div>
                </dl>
                <h2 class="co-sub">{format::items(count, lang, num)}</h2>
                <ul class="mini-lines" role="list">
                    {lines.into_iter().map(|(l, img)| view! {
                        <li class="mini-line">
                            <span class="mini-line__media"><Picture img sizes="48px" lang/></span>
                            <span class="mini-line__main">
                                <span class="mini-line__name">{l.name.pick(ar).to_owned()}</span>
                                <span class="mini-line__meta">
                                    <span class="mono">{format!("{}×", format::count(l.quantity, num))}</span>
                                    {(!l.variant_label.pick(ar).is_empty()).then(|| view! { <span>{l.variant_label.pick(ar).to_owned()}</span> })}
                                </span>
                            </span>
                            <span class="mini-line__total"><Price amount=l.line_total size=PriceSize::Sm no_badge=true lang numerals=num/></span>
                        </li>
                    }).collect_view()}
                </ul>
                <dl class="totals">
                    <div class="totals__row">
                        <dt>{tr(lang, "Subtotal", "المجموع")}</dt>
                        <dd><Price amount=o.subtotal size=PriceSize::Sm no_badge=true lang numerals=num/></dd>
                    </div>
                    <div class="totals__row">
                        <dt>{tr(lang, "Delivery", "التوصيل")}</dt>
                        <dd>{if o.delivery_fee == Piastres::ZERO {
                            Either::Left(view! { <span class="totals__free">{tr(lang, "Free", "مجاني")}</span> })
                        } else {
                            Either::Right(view! { <Price amount=o.delivery_fee size=PriceSize::Sm no_badge=true lang numerals=num/> })
                        }}</dd>
                    </div>
                    <div class="totals__row totals__row--total">
                        <dt>{tr(lang, "Total", "الإجمالي")}</dt>
                        <dd><Price amount=o.total size=PriceSize::Lg no_badge=true lang numerals=num/></dd>
                    </div>
                </dl>
                <p class="totals__vat">{vat}</p>
            </aside>
        </div>
        <p class="order-page__more">
            <a class="link" href="/">{tr(lang, "Continue shopping", "كمّل تسوق")}</a>
        </p>
    }
}
