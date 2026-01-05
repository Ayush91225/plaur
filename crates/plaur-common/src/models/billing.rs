use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    pub id: Uuid,
    pub store_id: Uuid,
    pub shopify_subscription_id: Option<String>,
    pub plan_type: crate::models::store::PlanType,
    pub status: crate::models::store::SubscriptionStatus,
    pub trial_ends_at: Option<DateTime<Utc>>,
    pub current_period_start: DateTime<Utc>,
    pub current_period_end: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageRecord {
    pub id: Uuid,
    pub store_id: Uuid,
    pub subscription_id: Uuid,
    pub usage_type: UsageType,
    pub quantity: i64,
    pub unit_price: f64,
    pub total_amount: f64,
    pub recorded_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UsageType {
    AiChatQueries,
    ReportGeneration,
    DataAnalysis,
    VoiceSummaries,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingEvent {
    pub id: Uuid,
    pub store_id: Uuid,
    pub event_type: BillingEventType,
    pub amount: f64,
    pub currency: String,
    pub shopify_charge_id: Option<String>,
    pub status: ChargeStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BillingEventType {
    SubscriptionCharge,
    UsageCharge,
    Refund,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChargeStatus {
    Pending,
    Accepted,
    Declined,
    Expired,
}