use axum::{
    routing::{get, post},
    Router,
};
use plaur_common::{
    config::Config,
    database::{create_pool, run_migrations},
    cache::CacheService,
    error::Result,
};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;

use crate::handlers::{auth_handlers, health_handlers};
use crate::services::AuthService;

pub struct AppState {
    pub db_pool: plaur_common::database::DbPool,
    pub cache: CacheService,
    pub auth_service: AuthService,
}

pub async fn start(config: Config) -> Result<()> {
    info!("Starting Plaur Auth Service on {}:{}", config.server.host, config.server.port);

    // Initialize database
    let db_pool = create_pool(&config.database).await?;
    run_migrations(&db_pool).await?;

    // Initialize cache
    let cache = CacheService::new(&config.redis).await?;

    // Initialize auth service
    let auth_service = AuthService::new(&config.security.jwt_secret);

    let state = Arc::new(AppState {
        db_pool,
        cache,
        auth_service,
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
        .route("/health", get(health_handlers::health_check))
        .route("/auth/register", post(auth_handlers::register))
        .route("/auth/login", post(auth_handlers::login))
        .route("/auth/refresh", post(auth_handlers::refresh_token))
        .route("/auth/logout", post(auth_handlers::logout))
        .route("/auth/verify", get(auth_handlers::verify_token))
        .with_state(state)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive())
        )
}