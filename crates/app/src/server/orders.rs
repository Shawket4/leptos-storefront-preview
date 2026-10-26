//! Placed orders and their (simulated) progress through Madar's online-order lifecycle.
//!
//! In Madar, staff advance an order `received → confirmed → preparing → ready →
//! out_for_delivery` and finalising sets `delivered`. The preview has no staff, so an order walks
//! the same steps on a fixed timetable from the moment it is placed; tracking pages poll it.

use crate::api::types::{CartView, DeliverySlot};
use crate::format::cairo_offset_hours;
use crate::tenant::Tenant;
use domain::{Address, L10n, Order, OrderLine, OrderStatus, PaymentMethod, Piastres};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn now_ms() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as i64).unwrap_or(0)
}

/// Seconds after placing when each step is reached (demo pace).
const TIMETABLE: [(OrderStatus, i64); 6] = [
    (OrderStatus::Received, 0),
    (OrderStatus::Confirmed, 12),
    (OrderStatus::Preparing, 40),
    (OrderStatus::Ready, 90),
    (OrderStatus::OutForDelivery, 150),
    (OrderStatus::Delivered, 420),
];

pub struct OrderStore {
    orders: moka::sync::Cache<String, Arc<Order>>,
}

impl OrderStore {
    pub fn new() -> Self {
        OrderStore {
            orders: moka::sync::Cache::builder()
                .max_capacity(50_000)
                .time_to_live(Duration::from_secs(7 * 24 * 3600))
                .build(),
        }
    }

    /// Freeze the cart into an order (names and prices copied, like Madar's `CartSnapshot`).
    pub fn place(&self, tenant: &Tenant, cart: &CartView, address: Address, slot: &DeliverySlot, payment: PaymentMethod) -> Order {
        let created = now_ms();
        let id = loop {
            let n = (uuid::Uuid::new_v4().as_u128() % 90_000 + 10_000) as u32;
            let id = format!("{}-{n}", tenant.order_prefix);
            if self.orders.get(&id).is_none() {
                break id;
            }
        };
        let delivery_fee = cart.delivery_fee + slot.fee;
        let order = Order {
            id: id.clone(),
            tenant: tenant.id.clone(),
            status: OrderStatus::Received,
            created_at_ms: created,
            status_times_ms: vec![(OrderStatus::Received, created)],
            lines: cart
                .lines
                .iter()
                .map(|l| OrderLine {
                    product: l.product,
                    variant: l.variant,
                    name: l.name.clone(),
                    variant_label: l.variant_label.clone(),
                    image: Some(l.img.key.clone()),
                    quantity: l.qty,
                    unit_price: l.unit_price,
                    line_total: l.line_total,
                })
                .collect(),
            subtotal: cart.subtotal,
            delivery_fee,
            discount: Piastres::ZERO,
            tax_included: cart.tax_included,
            total: cart.subtotal + delivery_fee,
            address,
            slot: L10n::new(format!("{}, {}", slot.day.en, slot.window.en), format!("{}، {}", slot.day.ar, slot.window.ar)),
            payment,
            estimated_minutes: if slot.express { tenant.express_minutes.unwrap_or(45) } else { 120 },
        };
        self.orders.insert(id, Arc::new(order.clone()));
        order
    }

    /// The order with its status advanced to "now". Orders belong to one tenant.
    pub fn get(&self, tenant: &str, id: &str) -> Option<Order> {
        let stored = self.orders.get(&id.to_ascii_uppercase())?;
        if stored.tenant != tenant {
            return None;
        }
        let mut o = (*stored).clone();
        let elapsed = (now_ms() - o.created_at_ms) / 1000;
        o.status_times_ms = TIMETABLE
            .iter()
            .filter(|(_, at)| elapsed >= *at)
            .map(|(s, at)| (*s, o.created_at_ms + at * 1000))
            .collect();
        o.status = o.status_times_ms.last().map(|(s, _)| *s).unwrap_or(OrderStatus::Received);
        Some(o)
    }

    /// When the next step is due (for "updates in …" and poll pacing), if any.
    pub fn next_step_at(order: &Order) -> Option<(OrderStatus, i64)> {
        let i = order.status.step_index()?;
        TIMETABLE.get(i + 1).map(|(s, at)| (*s, order.created_at_ms + at * 1000))
    }
}

impl Default for OrderStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Delivery windows for the next two days in Cairo time (express first, if the tenant has it).
pub fn slots(tenant: &Tenant, now: i64) -> Vec<DeliverySlot> {
    let offset_ms = cairo_offset_hours(now) * 3_600_000;
    let local = now + offset_ms;
    let today = local.div_euclid(86_400_000);
    let hour_now = (local.rem_euclid(86_400_000)) / 3_600_000;
    let mut out = Vec::new();
    if let Some(m) = tenant.express_minutes {
        out.push(DeliverySlot {
            id: "express".into(),
            day: L10n::new("Express", "سريع"),
            window: L10n::new(format!("Within {m} minutes"), format!("خلال {m} دقيقة")),
            fee: Piastres::from_pounds(15),
            express: true,
            available: (9..23).contains(&hour_now),
        });
    }
    let windows = [(10, 13), (13, 16), (16, 19), (19, 22)];
    for d in 0..2 {
        let day = today + d;
        let label = if d == 0 { L10n::new("Today", "النهارده") } else { L10n::new("Tomorrow", "بكرة") };
        for (from, to) in windows {
            let available = d > 0 || hour_now + 1 < from;
            let fmt_en = |h: i64| format!("{}{}", if h % 12 == 0 { 12 } else { h % 12 }, if h >= 12 { " PM" } else { " AM" });
            let fmt_ar = |h: i64| format!("{} {}", if h % 12 == 0 { 12 } else { h % 12 }, if h >= 12 { "م" } else { "ص" });
            out.push(DeliverySlot {
                id: format!("d{day}-{from}"),
                day: label.clone(),
                window: L10n::new(format!("{} – {}", fmt_en(from), fmt_en(to)), format!("{} – {}", fmt_ar(from), fmt_ar(to))),
                fee: Piastres::ZERO,
                express: false,
                available,
            });
        }
    }
    out
}
