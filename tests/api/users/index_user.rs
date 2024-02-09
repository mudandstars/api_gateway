use api_gateway::app::create_app;
use api_gateway::models::NewUser;
use api_gateway::store_user_with_api_key;
use api_gateway::testing::TestContext;
use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::Value;
use tower::util::ServiceExt;

#[tokio::test]
async fn test_users_can_be_retrieved() {
    let test_context = TestContext::new();

    let app = create_app(test_context.pool()).await;

    let new_user = NewUser {
        name: String::from("example_user"),
        email: String::from("user@example.com"),
    };

    store_user_with_api_key(&mut test_context.conn(), &new_user).unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::GET)
                .uri("/users")
                .body(Body::from(()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    let users = body["users"].as_array().unwrap();

    assert_eq!(users.len(), 1);
}
