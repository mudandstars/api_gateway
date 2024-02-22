use api_gateway::app::create_app;
use api_gateway::models::{ApiKey, NewUser, User};
use api_gateway::schema::{api_keys, users};
use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use diesel::prelude::*;
use diesel::QueryDsl;
use http_body_util::BodyExt;
use serde_json::Value;
use tower::util::ServiceExt;

use api_gateway::testing::TestContext;

#[tokio::test]
async fn test_user_can_be_stored() {
    let test_context = TestContext::new();

    let app = create_app(test_context.pool()).await;

    let new_user = NewUser {
        name: String::from("example_user"),
        email: String::from("user@example.com"),
    };

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/users")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(serde_json::to_vec(&new_user).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body["email"].as_str().unwrap().to_string(), new_user.email);
    assert_eq!(body["name"].as_str().unwrap().to_string(), new_user.name);

    let user_id = body["id"].as_u64().unwrap();

    let user: Result<User, diesel::result::Error> = users::table
        .find(user_id as u32)
        .first(&mut test_context.conn());

    match user {
        Ok(user) => {
            assert_eq!(user.name, new_user.name);
            assert_eq!(user.email, new_user.email);
        }
        Err(_) => panic!("An error occurred while fetching user with id {}", user_id),
    };
}

#[tokio::test]
async fn test_stores_an_api_key_with_the_user() {
    let test_context = TestContext::new();

    let app = create_app(test_context.pool()).await;

    let new_user = NewUser {
        name: String::from("example_user"),
        email: String::from("user@example.com"),
    };

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/users")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(serde_json::to_vec(&new_user).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    let user_id = body["id"].as_u64().unwrap();

    let api_key: Result<ApiKey, diesel::result::Error> = api_keys::table
        .filter(api_keys::user_id.eq(user_id as u32))
        .first::<ApiKey>(&mut test_context.conn());

    match api_key {
        Ok(api_key) => {
            assert_ne!(api_key.key, "");
        }
        Err(_) => panic!(
            "An error occurred while fetching api_key with user_id {}",
            user_id
        ),
    }
}
