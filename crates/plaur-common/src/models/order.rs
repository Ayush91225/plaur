use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Order {
    pub id: Uuid,
    pub store_id: Uuid,
    pub shopify_order_id: i64,
    pub order_number: String,
    pub customer_id: Option<Uuid>,
    pub email: Option<String>,
    pub total_price: f64,
    pub subtotal_price: f64,
    pub total_tax: f64,
    pub total_discounts: f64,
    pub currency: String,
    pub financial_status: FinancialStatus,
    pub fulfillment_status: Option<FulfillmentStatus>,
    pub source_name: Option<String>,
    pub referring_site: Option<String>,
    pub landing_site: Option<String>,
    pub line_items: Vec<OrderLineItem>,
    pub shipping_address: Option<Address>,
    pub billing_address: Option<Address>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "financial_status", rename_all = "snake_case")]
pub enum FinancialStatus {
    Pending,
    Authorized,
    PartiallyPaid,
    Paid,
    PartiallyRefunded,
    Refunded,
    Voided,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "fulfillment_status", rename_all = "snake_case")]
pub enum FulfillmentStatus {
    Fulfilled,
    Null,
    Partial,
    Restocked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderLineItem {
    pub id: i64,
    pub variant_id: Option<i64>,
    pub product_id: i64,
    pub title: String,
    pub quantity: i32,
    pub price: f64,
    pub total_discount: f64,
    pub sku: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub company: Option<String>,
    pub address1: Option<String>,
    pub address2: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub country: Option<String>,
    pub zip: Option<String>,
    pub phone: Option<String>,
}