use axum::Router;
use plaur_common::{config::Config, error::Result};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;

use crate::routes::{auth_routes, store_routes, analytics_routes, ai_routes};

pub struct AppState {
    pub config: Config,
    pub http_client: reqwest::Client,
}

pub async fn start(config: Config) -> Result<()> {
    info!("Starting Plaur API Gateway on {}:{}", config.server.host, config.server.port);

    let http_client = reqwest::Client::new();
    
    let state = Arc::new(AppState {
        config: config.clone(),
        http_client,
    });

    let app = create_router(state);

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", config.server.host, config.server.port))
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .nest("/api/v1/auth", auth_routes::create_routes())
        .nest("/api/v1/stores", store_routes::create_routes())
        .nest("/api/v1/analytics", analytics_routes::create_routes())
        .nest("/api/v1/ai", ai_routes::create_routes())
        .with_state(state)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive())
        )
}