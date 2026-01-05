use axum::{
    routing::{get, post},
    Router,
};
use plaur_common::{
    config::Config,
    database::{create_pool, run_migrations},
    error::Result,
};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;

use crate::handlers::{shopify_handlers, webhook_handlers};
use crate::services::IngestionService;

pub struct AppState {
    pub db_pool: plaur_common::database::DbPool,
    pub ingestion_service: IngestionService,
    pub config: Config,
}

pub async fn start(config: Config) -> Result<()> {
    info!("Starting Plaur Ingestion Service on {}:{}", config.server.host, config.server.port);

    let db_pool = create_pool(&config.database).await?;
    run_migrations(&db_pool).await?;

    let ingestion_service = IngestionService::new(&config).await?;

    let state = Arc::new(AppState {
        db_pool,
        ingestion_service,
        config: config.clone(),
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
        .route("/health", get(|| async { "OK" }))
        .route("/webhooks/shopify", post(webhook_handlers::shopify_webhook))
        .route("/sync/shopify/:store_id", post(shopify_handlers::sync_store))
        .route("/sync/products/:store_id", post(shopify_handlers::sync_products))
        .route("/sync/orders/:store_id", post(shopify_handlers::sync_orders))
        .with_state(state)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive())
        )
}