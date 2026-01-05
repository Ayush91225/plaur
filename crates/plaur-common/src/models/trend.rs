use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trend {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub category: TrendCategory,
    pub keywords: Vec<String>,
    pub velocity: f64,
    pub sentiment: TrendSentiment,
    pub geographic_regions: Vec<String>,
    pub data_sources: Vec<String>,
    pub confidence_score: f64,
    pub first_detected: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendCategory {
    Fashion,
    Technology,
    Beauty,
    Home,
    Sports,
    Food,
    Travel,
    Entertainment,
    Health,
    Sustainability,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendSentiment {
    Positive,
    Negative,
    Neutral,
    Mixed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendMapping {
    pub id: Uuid,
    pub store_id: Uuid,
    pub trend_id: Uuid,
    pub product_ids: Vec<Uuid>,
    pub relevance_score: f64,
    pub opportunity_score: f64,
    pub recommended_actions: Vec<TrendAction>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendAction {
    pub action_type: TrendActionType,
    pub description: String,
    pub expected_impact: f64,
    pub effort_required: crate::models::analytics::EffortLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendActionType {
    CreateProduct,
    UpdateProductDescription,
    AdjustPricing,
    LaunchCampaign,
    InventoryAdjustment,
    ContentCreation,
}