use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSession {
    pub id: Uuid,
    pub store_id: Uuid,
    pub user_id: Uuid,
    pub title: Option<String>,
    pub messages: Vec<ChatMessage>,
    pub context: ChatContext,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: Uuid,
    pub role: MessageRole,
    pub content: String,
    pub metadata: MessageMetadata,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageMetadata {
    pub sources: Vec<DataSource>,
    pub confidence: Option<f64>,
    pub processing_time_ms: Option<u64>,
    pub tokens_used: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSource {
    pub source_type: SourceType,
    pub reference: String,
    pub relevance_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SourceType {
    OrderData,
    ProductData,
    AnalyticsReport,
    TrendData,
    MarketingData,
    CustomerData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatContext {
    pub store_context: StoreContext,
    pub conversation_memory: Vec<ContextItem>,
    pub user_preferences: UserPreferences,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreContext {
    pub business_type: String,
    pub primary_categories: Vec<String>,
    pub target_audience: String,
    pub key_metrics: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextItem {
    pub key: String,
    pub value: serde_json::Value,
    pub timestamp: DateTime<Utc>,
    pub ttl: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub preferred_metrics: Vec<String>,
    pub notification_frequency: String,
    pub report_format: String,
    pub voice_enabled: bool,
}