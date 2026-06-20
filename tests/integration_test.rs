use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::json;
use tower::ServiceExt;

// Helper to create test app
async fn test_app() -> axum::Router {
    dotenvy::dotenv().ok();
    
    let config = expense_tracker_api::config::Config::from_env();
    let pool = expense_tracker_api::db::create_pool(&config).await.expect("Failed to create pool");
    
    expense_tracker_api::routes::api::create_api_router(pool, config)
}

#[tokio::test]
async fn test_health_check() {
    let app = test_app().await;
    
    let response = app
        .oneshot(Request::builder()
            .uri("/health")
            .body(Body::empty())
            .unwrap())
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(body["status"], "ok");
}

#[tokio::test]
async fn test_register_and_login() {
    let app = test_app().await;
    
    let unique_email = format!("test_{}@example.com", uuid::Uuid::new_v4());
    
    // Register
    let register_body = json!({
        "email": unique_email,
        "password": "password123"
    });
    
    let response = app
        .clone()
        .oneshot(Request::builder()
            .method("POST")
            .uri("/auth/register")
            .header("Content-Type", "application/json")
            .body(Body::from(register_body.to_string()))
            .unwrap())
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::CREATED);
    
    // Login
    let login_body = json!({
        "email": unique_email,
        "password": "password123"
    });
    
    let response = app
        .oneshot(Request::builder()
            .method("POST")
            .uri("/auth/login")
            .header("Content-Type", "application/json")
            .body(Body::from(login_body.to_string()))
            .unwrap())
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    assert!(body["success"].as_bool().unwrap());
    assert!(body["token"].as_str().unwrap().len() > 0);
}

#[tokio::test]
async fn test_protected_route_without_auth() {
    let app = test_app().await;
    
    let response = app
        .oneshot(Request::builder()
            .method("GET")
            .uri("/expenses")
            .body(Body::empty())
            .unwrap())
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_validation_errors() {
    let app = test_app().await;
    
    let body = json!({
        "email": "not-an-email",
        "password": "123"
    });
    
    let response = app
        .oneshot(Request::builder()
            .method("POST")
            .uri("/auth/register")
            .header("Content-Type", "application/json")
            .body(Body::from(body.to_string()))
            .unwrap())
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(body["success"], false);
    assert!(body["error"].as_str().unwrap().contains("email") || body["error"].as_str().unwrap().contains("password"));
}
