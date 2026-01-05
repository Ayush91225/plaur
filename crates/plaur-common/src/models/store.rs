use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Store {
    pub id: Uuid,
    pub shopify_store_id: i64,
    pub shop_domain: String,
    pub shop_name: String,
    pub owner_id: Uuid,
    pub access_token: String,
    pub webhook_secret: String,
    pub subscription_status: SubscriptionStatus,
    pub plan_type: PlanType,
    pub trial_ends_at: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub timezone: String,
    pub currency: String,
    pub country_code: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "subscription_status", rename_all = "lowercase")]
pub enum SubscriptionStatus {
    Trial,
    Active,
    Cancelled,
    Suspended,
    PastDue,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "plan_type", rename_all = "lowercase")]
pub enum PlanType {
    Starter,
    Growth,
    Pro,
    Enterprise,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoreSettings {
    pub store_id: Uuid,
    pub notifications_enabled: bool,
    pub email_reports: bool,
    pub voice_summaries: bool,
    pub anomaly_detection: bool,
    pub trend_analysis: bool,
    pub competitor_tracking: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoreMetrics {
    pub store_id: Uuid,
    pub total_orders: i64,
    pub total_revenue: f64,
    pub total_customers: i64,
    pub total_products: i64,
    pub avg_order_value: f64,
    pub conversion_rate: f64,
    pub last_updated: DateTime<Utc>,
}