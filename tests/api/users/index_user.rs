use api_gateway::app::{app, mysql_pool};
use api_gateway::database::establish_connection;
use api_gateway::models::NewUser;
use api_gateway::schema::users;
use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use diesel::prelude::*;
use diesel::QueryDsl;
use http_body_util::BodyExt;
use serde_json::Value;
use tower::util::ServiceExt;

#[tokio::test]
async fn test_users_can_be_retrieved() {
    let app = app(mysql_pool()).await;

    let new_user = NewUser {
        name: String::from("example_user"),
        email: String::from("user@example.com"),
    };

    let users_count: Result<i64, diesel::result::Error> =
        users::table.count().get_result(&mut establish_connection());

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::GET)
                .uri("/users")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(serde_json::to_vec(&new_user).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    let users = body["users"].as_array().unwrap();

    assert_eq!(users.len(), users_count.unwrap() as usize);
}
