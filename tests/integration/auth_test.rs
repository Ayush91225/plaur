use plaur_common::{
    config::Config,
    models::user::{CreateUserRequest, LoginRequest, UserRole},
};
use reqwest::Client;
use serde_json::json;
use tokio;

#[tokio::test]
async fn test_user_registration_and_login() {
    let client = Client::new();
    let base_url = "http://localhost:8081";

    // Test user registration
    let register_request = CreateUserRequest {
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
        first_name: "Test".to_string(),
        last_name: "User".to_string(),
        role: UserRole::Owner,
    };

    let response = client
        .post(&format!("{}/auth/register", base_url))
        .json(&register_request)
        .send()
        .await
        .expect("Failed to send registration request");

    assert_eq!(response.status(), 200);

    // Test user login
    let login_request = LoginRequest {
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    };

    let response = client
        .post(&format!("{}/auth/login", base_url))
        .json(&login_request)
        .send()
        .await
        .expect("Failed to send login request");

    assert_eq!(response.status(), 200);

    let login_response: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse login response");

    assert!(login_response["access_token"].is_string());
    assert!(login_response["refresh_token"].is_string());
}