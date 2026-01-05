use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use lambda_web::{is_running_on_lambda, LambdaError};
use plaur_auth::server;
use plaur_common::{config::Config, telemetry::init_telemetry};
use serde_json::Value;

async fn function_handler(_event: LambdaEvent<Value>) -> Result<Value, Error> {
    let config = Config::load().map_err(|e| LambdaError::from(e.to_string()))?;
    server::start(config).await.map_err(|e| LambdaError::from(e.to_string()))?;
    Ok(serde_json::json!({"statusCode": 200}))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    init_telemetry().map_err(|e| LambdaError::from(e.to_string()))?;
    
    if is_running_on_lambda() {
        run(service_fn(function_handler)).await
    } else {
        let config = Config::load().map_err(|e| LambdaError::from(e.to_string()))?;
        server::start(config).await.map_err(|e| LambdaError::from(e.to_string()))?;
        Ok(())
    }
}