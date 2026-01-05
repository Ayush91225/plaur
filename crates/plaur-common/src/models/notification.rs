use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: Uuid,
    pub store_id: Uuid,
    pub user_id: Option<Uuid>,
    pub notification_type: NotificationType,
    pub title: String,
    pub message: String,
    pub priority: NotificationPriority,
    pub channels: Vec<NotificationChannel>,
    pub status: NotificationStatus,
    pub metadata: serde_json::Value,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub sent_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationType {
    AnomalyAlert,
    ReportReady,
    RecommendationGenerated,
    BillingUpdate,
    SystemMaintenance,
    SecurityAlert,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationPriority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationChannel {
    InApp,
    Email,
    Sms,
    Slack,
    Webhook,
    Voice,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationStatus {
    Pending,
    Sent,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationTemplate {
    pub id: Uuid,
    pub name: String,
    pub notification_type: NotificationType,
    pub subject_template: String,
    pub body_template: String,
    pub variables: Vec<String>,
    pub is_active: bool,
}