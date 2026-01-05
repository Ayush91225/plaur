use axum::{extract::State, http::StatusCode, Json};
use plaur_common::{
    error::Result,
    models::user::{CreateUserRequest, LoginRequest, LoginResponse, UserProfile},
    validation::validate_request,
};
use std::sync::Arc;
use tracing::{error, info};

use crate::{server::AppState, services::UserService};

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateUserRequest>,
) -> Result<Json<UserProfile>> {
    validate_request(&request)?;
    
    info!("Registering new user: {}", request.email);
    
    let user_service = UserService::new(&state.db_pool, &state.cache, &state.auth_service);
    let user = user_service.create_user(request).await?;
    
    Ok(Json(user))
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<LoginResponse>> {
    validate_request(&request)?;
    
    info!("User login attempt: {}", request.email);
    
    let user_service = UserService::new(&state.db_pool, &state.cache, &state.auth_service);
    let response = user_service.authenticate_user(request).await?;
    
    Ok(Json(response))
}

pub async fn refresh_token(
    State(state): State<Arc<AppState>>,
    Json(refresh_token): Json<String>,
) -> Result<Json<LoginResponse>> {
    let user_service = UserService::new(&state.db_pool, &state.cache, &state.auth_service);
    let response = user_service.refresh_token(&refresh_token).await?;
    
    Ok(Json(response))
}

pub async fn logout(
    State(state): State<Arc<AppState>>,
    Json(token): Json<String>,
) -> Result<StatusCode> {
    let user_service = UserService::new(&state.db_pool, &state.cache, &state.auth_service);
    user_service.logout(&token).await?;
    
    Ok(StatusCode::NO_CONTENT)
}

pub async fn verify_token(
    State(state): State<Arc<AppState>>,
    Json(token): Json<String>,
) -> Result<Json<UserProfile>> {
    let user_service = UserService::new(&state.db_pool, &state.cache, &state.auth_service);
    let user = user_service.verify_token(&token).await?;
    
    Ok(Json(user))
}