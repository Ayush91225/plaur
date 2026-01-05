use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub event_type: EventType,
    pub payload: serde_json::Value,
    pub source: String,
    pub timestamp: DateTime<Utc>,
    pub correlation_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum EventType {
    // Store events
    StoreConnected { store_id: Uuid },
    StoreDisconnected { store_id: Uuid },
    
    // Order events
    OrderCreated { order_id: Uuid, store_id: Uuid },
    OrderUpdated { order_id: Uuid, store_id: Uuid },
    OrderCancelled { order_id: Uuid, store_id: Uuid },
    
    // Product events
    ProductCreated { product_id: Uuid, store_id: Uuid },
    ProductUpdated { product_id: Uuid, store_id: Uuid },
    ProductDeleted { product_id: Uuid, store_id: Uuid },
    
    // Analytics events
    AnomalyDetected { store_id: Uuid, metric: String, severity: String },
    TrendIdentified { store_id: Uuid, trend_id: Uuid },
    
    // Billing events
    SubscriptionCreated { store_id: Uuid, plan: String },
    SubscriptionUpdated { store_id: Uuid, plan: String },
    SubscriptionCancelled { store_id: Uuid },
    
    // AI events
    RecommendationGenerated { store_id: Uuid, recommendation_id: Uuid },
    ChatInteraction { store_id: Uuid, user_id: Uuid },
}

impl Event {
    pub fn new(event_type: EventType, source: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            event_type,
            payload: serde_json::Value::Null,
            source,
            timestamp: Utc::now(),
            correlation_id: None,
        }
    }

    pub fn with_payload(mut self, payload: serde_json::Value) -> Self {
        self.payload = payload;
        self
    }

    pub fn with_correlation_id(mut self, correlation_id: Uuid) -> Self {
        self.correlation_id = Some(correlation_id);
        self
    }
}