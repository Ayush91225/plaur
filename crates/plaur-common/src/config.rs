use anyhow::Result;
use config::{Config as ConfigBuilder, Environment, File};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub aws: AwsConfig,
    pub shopify: ShopifyConfig,
    pub ai: AiConfig,
    pub billing: BillingConfig,
    pub security: SecurityConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub url: String,
    pub pool_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwsConfig {
    pub region: String,
    pub s3_bucket: String,
    pub lambda_role_arn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopifyConfig {
    pub app_id: String,
    pub app_secret: String,
    pub webhook_secret: String,
    pub scopes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    pub openai_api_key: String,
    pub quadrant_url: String,
    pub quadrant_api_key: String,
    pub sarvam_api_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingConfig {
    pub shopify_billing_api_version: String,
    pub trial_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub jwt_secret: String,
    pub encryption_key: String,
    pub session_timeout: u64,
}

impl Config {
    pub fn load() -> Result<Self> {
        let env = env::var("PLAUR_ENV").unwrap_or_else(|_| "development".to_string());
        
        let config = ConfigBuilder::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name(&format!("config/{}", env)).required(false))
            .add_source(Environment::with_prefix("PLAUR"))
            .build()?;
            
        Ok(config.try_deserialize()?)
    }
}