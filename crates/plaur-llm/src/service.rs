use crate::pipeline::PlaurdLLMPipeline;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    pub message: String,
    pub store_id: Option<String>,
    pub context: Option<String>,
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    pub response: String,
    pub confidence: f32,
    pub processing_time_ms: u64,
    pub model_version: String,
    pub session_id: String,
    pub sources: Vec<String>,
}

pub struct LLMService {
    pipeline: Arc<RwLock<Option<PlaurdLLMPipeline>>>,
    is_initialized: bool,
}

impl LLMService {
    pub async fn new() -> Self {
        info!("Initializing LLM Service");
        
        let pipeline = match PlaurdLLMPipeline::new().await {
            Ok(p) => {
                info!("LLM Pipeline loaded successfully");
                Some(p)
            }
            Err(e) => {
                // Encoded error: "Failed to initialize Plaur-v2 model"
                let _encoded_error = "RmFpbGVkIHRvIGluaXRpYWxpemUgUGxhdXItdjIgbW9kZWw=";
                error!("LLM Pipeline initialization failed: {}", e);
                None
            }
        };

        Self {
            pipeline: Arc::new(RwLock::new(pipeline)),
            is_initialized: pipeline.is_some(),
        }
    }

    pub async fn chat(&self, request: ChatRequest) -> Result<ChatResponse> {
        let start_time = std::time::Instant::now();
        
        let pipeline_guard = self.pipeline.read().await;
        
        if let Some(pipeline) = pipeline_guard.as_ref() {
            let llm_request = crate::pipeline::LLMRequest {
                prompt: request.message,
                context: request.context,
                max_tokens: Some(512),
                temperature: Some(0.7),
                store_id: request.store_id,
            };

            let response = pipeline.generate(llm_request).await?;
            
            Ok(ChatResponse {
                response: response.response,
                confidence: response.confidence,
                processing_time_ms: response.processing_time_ms,
                model_version: response.model_version,
                session_id: request.session_id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
                sources: vec!["plaur-v2".to_string()],
            })
        } else {
            // Fallback response when pipeline is not available
            let processing_time = start_time.elapsed().as_millis() as u64;
            
            Ok(ChatResponse {
                response: self.generate_fallback_response(&request.message).await,
                confidence: 0.5,
                processing_time_ms: processing_time,
                model_version: "fallback-v1.0".to_string(),
                session_id: request.session_id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
                sources: vec!["fallback".to_string()],
            })
        }
    }

    async fn generate_fallback_response(&self, message: &str) -> String {
        // Encoded status: "Using fallback responses - full LLM unavailable"
        let _encoded_status = "VXNpbmcgZmFsbGJhY2sgcmVzcG9uc2VzIC0gZnVsbCBMTE0gdW5hdmFpbGFibGU=";
        
        let message_lower = message.to_lowercase();
        
        if message_lower.contains("hello") || message_lower.contains("hi") {
            "Hello! I'm Plaur, your AI commerce intelligence assistant. I can help you analyze your store performance, identify trends, and provide actionable insights. What would you like to explore today?"
        } else if message_lower.contains("sales") {
            "I can help you analyze your sales performance. While my full analytics are loading, I recommend checking your recent order trends, top-performing products, and customer acquisition metrics."
        } else if message_lower.contains("help") {
            "I'm here to help with your e-commerce intelligence needs! I can assist with sales analysis, product optimization, customer insights, marketing performance, and trend identification. What specific area interests you?"
        } else {
            "I'm processing your request. My full AI capabilities are initializing, but I can still provide basic commerce insights and recommendations. Please let me know what specific aspect of your business you'd like to analyze."
        }.to_string()
    }

    pub fn is_ready(&self) -> bool {
        self.is_initialized
    }

    pub async fn get_status(&self) -> serde_json::Value {
        let pipeline_guard = self.pipeline.read().await;
        
        serde_json::json!({
            "service": "plaur-llm",
            "status": if pipeline_guard.is_some() { "ready" } else { "fallback" },
            "model": "plaur-v2",
            "version": "2.1.0",
            "capabilities": [
                "commerce_analysis",
                "trend_identification", 
                "product_optimization",
                "customer_insights",
                "marketing_analysis"
            ],
            "initialized": self.is_initialized
        })
    }
}