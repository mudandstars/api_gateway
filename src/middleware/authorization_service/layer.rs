use super::authorization_service::AuthorizationServiceMiddleware;
use deadpool_diesel::mysql::Pool;
use tower::Layer;

#[derive(Clone)]
pub struct AuthorizationServiceLayer {
    pool: Pool,
}

impl AuthorizationServiceLayer {
    pub fn new(pool: Pool) -> Self {
        AuthorizationServiceLayer { pool }
    }
}

impl<S> Layer<S> for AuthorizationServiceLayer {
    type Service = AuthorizationServiceMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthorizationServiceMiddleware::new(inner, self.pool.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{app::create_app, models::NewUser, store_user_with_api_key, testing::TestContext};
    use tower::util::ServiceExt;

    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };

    #[tokio::test]
    async fn test_allows_request_with_valid_api_key() {
        let test_context = TestContext::new();
        let pool = test_context.pool();

        let authorization_service_layer = AuthorizationServiceLayer::new(pool.clone());
        let app = create_app(pool).await.layer(authorization_service_layer);

        let user = store_user_with_api_key(
            &mut test_context.conn(),
            &NewUser {
                name: String::from("example_user"),
                email: String::from("user@example.com"),
            },
        )
        .unwrap();

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri("/users")
                    .header(
                        "API_KEY",
                        &user.api_keys(&mut test_context.conn()).first().unwrap().key,
                    )
                    .body(Body::from(()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_does_not_allow_request_with_invalid_api_key() {
        let test_context = TestContext::new();
        let pool = test_context.pool();

        let authorization_service_layer = AuthorizationServiceLayer::new(pool.clone());
        let app = create_app(pool).await.layer(authorization_service_layer);

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri("/users")
                    .header("API_KEY", "invalid key")
                    .body(Body::from(()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_updates_last_used_at_of_api_key() {
        let test_context = TestContext::new();
        let pool = test_context.pool();

        let authorization_service_layer = AuthorizationServiceLayer::new(pool.clone());
        let app = create_app(pool).await.layer(authorization_service_layer);

        let user = store_user_with_api_key(
            &mut test_context.conn(),
            &NewUser {
                name: String::from("example_user"),
                email: String::from("user@example.com"),
            },
        )
        .unwrap();

        let api_keys = user.api_keys(&mut test_context.conn());
        if let Some(api_key) = api_keys.first() {
            assert!(&api_key.last_used_at.is_none());
        } else {
            panic!("has no api key")
        }

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri("/sample-endpoints")
                    .header("API_KEY", &api_keys.first().unwrap().key)
                    .body(Body::from(()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let api_keys = user.api_keys(&mut test_context.conn());
        if let Some(api_key) = api_keys.first() {
            assert!(&api_key.last_used_at.is_some(), "api key's last_used_at was not set");
        } else {
            panic!("api key was not found")
        }
    }
}
