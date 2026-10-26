//! Orders and their lifecycle.

use crate::catalog::{ProductId, VariantId};
use crate::{L10n, Piastres};
use serde::{Deserialize, Serialize};

/// Origin: Postgres enum `delivery_order_status` (MadarRust
/// `migrations/20260614150000_delivery_core.sql`), the online-order lifecycle. Staff move an order
/// through `confirmed → preparing → ready → out_for_delivery`; `finalize` sets `delivered`.
/// Cancelling from `received` gives `rejected`, from a later step `cancelled`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    Received,
    Confirmed,
    Preparing,
    Ready,
    OutForDelivery,
    Delivered,
    Cancelled,
    Rejected,
}

impl OrderStatus {
    /// The happy path, in order (what the tracking timeline draws).
    pub const STEPS: [OrderStatus; 6] = [
        OrderStatus::Received,
        OrderStatus::Confirmed,
        OrderStatus::Preparing,
        OrderStatus::Ready,
        OrderStatus::OutForDelivery,
        OrderStatus::Delivered,
    ];

    pub fn is_terminal(self) -> bool {
        matches!(self, OrderStatus::Delivered | OrderStatus::Cancelled | OrderStatus::Rejected)
    }

    pub fn step_index(self) -> Option<usize> {
        Self::STEPS.iter().position(|s| *s == self)
    }

    /// Storefront label. Origin: MadarDashboard `src/i18n/locales/{en,ar}.json` `order.status.*`.
    pub fn label(self) -> L10n {
        let (en, ar) = match self {
            OrderStatus::Received => ("Order received", "تم استلام الطلب"),
            OrderStatus::Confirmed => ("Order accepted", "تم قبول الطلب"),
            OrderStatus::Preparing => ("Preparing", "جارٍ التحضير"),
            OrderStatus::Ready => ("Ready", "جاهز للتوصيل"),
            OrderStatus::OutForDelivery => ("On the way", "في الطريق"),
            OrderStatus::Delivered => ("Delivered", "تم التوصيل"),
            OrderStatus::Cancelled | OrderStatus::Rejected => ("Cancelled", "ملغي"),
        };
        L10n::new(en, ar)
    }

    /// Tracking-page headline. Origin: MadarDashboard `order.track.status.*`.
    pub fn headline(self) -> L10n {
        let (en, ar) = match self {
            OrderStatus::Received => ("We've got your order", "تم استلام الطلب"),
            OrderStatus::Confirmed => ("Your order was accepted", "تم قبول الطلب"),
            OrderStatus::Preparing => ("We're packing your order", "جارٍ تحضير طلبك"),
            OrderStatus::Ready => ("Ready for delivery", "جاهز للتوصيل"),
            OrderStatus::OutForDelivery => ("On its way to you", "في الطريق إليك"),
            OrderStatus::Delivered => ("Delivered", "تم التوصيل"),
            OrderStatus::Cancelled | OrderStatus::Rejected => ("Order cancelled", "تم إلغاء الطلب"),
        };
        L10n::new(en, ar)
    }
}

/// Origin: Madar `CartLineInput { menu_item_id, size_label, quantity, addons }` and the frozen
/// `CartSnapshot` line on `delivery_orders.cart`, flattened: names and prices are copied at order
/// time so later catalogue edits don't change a placed order.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderLine {
    pub product: ProductId,
    pub variant: VariantId,
    pub name: L10n,
    pub variant_label: L10n,
    pub image: Option<String>,
    pub quantity: u32,
    pub unit_price: Piastres,
    pub line_total: Piastres,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Address {
    pub name: String,
    pub phone: String,
    pub city: String,
    pub area: String,
    pub street: String,
    pub building: String,
    #[serde(default)]
    pub floor: String,
    #[serde(default)]
    pub apartment: String,
    #[serde(default)]
    pub landmark: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethod {
    /// Madar `payment_method_hint: "cash"`.
    CashOnDelivery,
    /// Madar `payment_method_hint: "card"` (placeholder: no gateway in the preview).
    Card,
    /// Mobile wallet / InstaPay (placeholder).
    Wallet,
}

/// Origin: MadarRust `DeliveryTracking` (`src/delivery/public.rs:2663`), trimmed. Money in piastres;
/// timestamps in Unix milliseconds (Madar uses `DateTime<Utc>`; the preview avoids chrono in WASM).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Order {
    /// Human reference, e.g. `NIL-48213` (Madar `delivery_ref`).
    pub id: String,
    pub tenant: String,
    pub status: OrderStatus,
    pub created_at_ms: i64,
    /// When each reached step happened, in `OrderStatus::STEPS` order.
    pub status_times_ms: Vec<(OrderStatus, i64)>,
    pub lines: Vec<OrderLine>,
    pub subtotal: Piastres,
    pub delivery_fee: Piastres,
    pub discount: Piastres,
    /// VAT included in `total` (Egypt: 14%, tax-inclusive prices).
    pub tax_included: Piastres,
    pub total: Piastres,
    pub address: Address,
    /// Delivery slot label as chosen at checkout.
    pub slot: L10n,
    pub payment: PaymentMethod,
    pub estimated_minutes: u32,
}
