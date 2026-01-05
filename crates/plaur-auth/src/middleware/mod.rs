use axum::{
    extract::{Request, State},
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::Response,
};
use plaur_common::{
    auth::AuthService,
    error::{PlaurdError, Result},
    models::user::JwtClaims,
};
use std::sync::Arc;

pub async fn auth_middleware(
    State(auth_service): State<Arc<AuthService>>,
    mut request: Request,
    next: Next,
) -> Result<Response> {
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "));

    let token = auth_header.ok_or_else(|| PlaurdError::Authentication {
        message: "Missing authorization header".to_string(),
    })?;

    let claims = auth_service.verify_token(token)?;
    
    // Add claims to request extensions
    request.extensions_mut().insert(claims);
    
    Ok(next.run(request).await)
}

pub fn extract_claims(request: &Request) -> Result<&JwtClaims> {
    request
        .extensions()
        .get::<JwtClaims>()
        .ok_or_else(|| PlaurdError::Authentication {
            message: "Missing authentication claims".to_string(),
        })
}