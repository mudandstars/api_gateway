use api_gateway::app::app;
use api_gateway::models::{NewUser, User};
use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::Value;
use tower::util::ServiceExt;
use api_gateway::schema::users;
use diesel::prelude::*;
use api_gateway::database;

#[tokio::test]
async fn test_user_can_be_stored() {
    let app = app().await;

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
    println!("{:?}", body);

    assert_eq!( body["email"].as_str().unwrap().to_string(), new_user.email );
    assert_eq!( body["name"].as_str().unwrap().to_string(), new_user.name );

    let user_id = body["id"].as_u64().unwrap();

    let user = users::table
        .filter(users::id.eq(user_id as u32))
        .first::<User>(&mut database::establish_connection());

    match user {
        Ok(user) => {
            assert_eq!(user.name, new_user.name);
            assert_eq!(user.email, new_user.email);
        },
        Err(_) => panic!("An error occurred while fetching user with id {}", user_id),
    }
}
