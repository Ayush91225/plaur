use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use plaur_common::{config::Config, telemetry::init_telemetry};
use serde_json::Value;
use tracing::{error, info};

async fn function_handler(event: LambdaEvent<Value>) -> Result<Value, Error> {
    info!("Processing Shopify webhook: {:?}", event.payload);
    
    // Process the webhook payload
    match process_webhook(event.payload).await {
        Ok(_) => Ok(serde_json::json!({"statusCode": 200, "body": "OK"})),
        Err(e) => {
            error!("Error processing webhook: {}", e);
            Ok(serde_json::json!({"statusCode": 500, "body": "Internal Server Error"}))
        }
    }
}

async fn process_webhook(payload: Value) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let config = Config::load()?;
    
    // Extract webhook type and data
    let webhook_type = payload.get("headers")
        .and_then(|h| h.get("x-shopify-topic"))
        .and_then(|t| t.as_str())
        .unwrap_or("unknown");
    
    info!("Processing webhook type: {}", webhook_type);
    
    match webhook_type {
        "orders/create" | "orders/updated" | "orders/cancelled" => {
            // Process order webhook
            info!("Processing order webhook");
        }
        "products/create" | "products/update" | "products/delete" => {
            // Process product webhook
            info!("Processing product webhook");
        }
        "app/uninstalled" => {
            // Handle app uninstall
            info!("Processing app uninstall webhook");
        }
        _ => {
            info!("Unknown webhook type: {}", webhook_type);
        }
    }
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    init_telemetry().map_err(|e| Error::from(e.to_string()))?;
    run(service_fn(function_handler)).await
}