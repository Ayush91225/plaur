use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: Uuid,
    pub store_id: Uuid,
    pub shopify_product_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub vendor: Option<String>,
    pub product_type: Option<String>,
    pub handle: String,
    pub status: ProductStatus,
    pub tags: Vec<String>,
    pub images: Vec<ProductImage>,
    pub variants: Vec<ProductVariant>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "product_status", rename_all = "lowercase")]
pub enum ProductStatus {
    Active,
    Draft,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductImage {
    pub id: i64,
    pub src: String,
    pub alt: Option<String>,
    pub position: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductVariant {
    pub id: i64,
    pub title: String,
    pub price: f64,
    pub compare_at_price: Option<f64>,
    pub sku: Option<String>,
    pub inventory_quantity: i32,
    pub weight: Option<f64>,
    pub weight_unit: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductAnalytics {
    pub product_id: Uuid,
    pub views: i64,
    pub conversions: i64,
    pub revenue: f64,
    pub conversion_rate: f64,
    pub avg_order_value: f64,
    pub inventory_turnover: f64,
    pub trend_score: f64,
    pub last_updated: DateTime<Utc>,
}