use super::logger::RequestLoggerMiddleware;
use deadpool_diesel::mysql::Pool;
use tower::Layer;

#[derive(Clone)]
pub struct RequestLoggerLayer {
    pool: Pool,
}

impl RequestLoggerLayer {
    pub fn new(pool: Pool) -> Self {
        RequestLoggerLayer { pool }
    }
}

impl<S> Layer<S> for RequestLoggerLayer {
    type Service = RequestLoggerMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RequestLoggerMiddleware::new(inner, self.pool.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        app::create_app,
        middleware::AuthorizationServiceLayer,
        models::{LogType, NewUser},
        store_user_with_api_key,
        testing::TestContext,
    };
    use tower::util::ServiceExt;

    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };

    #[tokio::test]
    async fn test_saves_request_details_to_db() {
        let test_context = TestContext::new();
        let pool = test_context.pool();

        let logger_layer = RequestLoggerLayer::new(pool.clone());
        let app = create_app(pool).await.layer(logger_layer);

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
                    .uri("/sample-endpoints")
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

        let user_logs = user.logs(&mut test_context.conn());
        let log = user_logs.first().unwrap();

        assert_eq!(log.method, http::Method::GET.to_string());
        assert_eq!(log.uri, String::from("/sample-endpoints"));
        assert_eq!(log.status, http::status::StatusCode::OK);
        assert_eq!(log.type_, u8::from(LogType::INFO));
    }

    #[tokio::test]
    async fn test_does_not_throw_error_with_authorization_middleware_between_and_no_api_key() {
        let test_context = TestContext::new();
        let pool = test_context.pool();

        let logger_layer = RequestLoggerLayer::new(pool.clone());
        let authorization_service_layer = AuthorizationServiceLayer::new(pool.clone());
        let app = create_app(pool)
            .await
            .layer(authorization_service_layer)
            .layer(logger_layer);

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri("/sample-endpoints")
                    .body(Body::from(()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_saves_error_request_details_to_db() {
        let test_context = TestContext::new();
        let pool = test_context.pool();

        let logger_layer = RequestLoggerLayer::new(pool.clone());
        let app = create_app(pool).await.layer(logger_layer);

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
                    .uri("/fail")
                    .header(
                        "API_KEY",
                        &user.api_keys(&mut test_context.conn()).first().unwrap().key,
                    )
                    .body(Body::from(()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

        let user_logs = user.logs(&mut test_context.conn());
        let log = user_logs.first().unwrap();

        assert_eq!(log.method, http::Method::GET.to_string());
        assert_eq!(log.uri, String::from("/fail"));
        assert_eq!(log.status, http::status::StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(log.type_, u8::from(LogType::ERROR));
        assert_eq!(
            log.error_message.clone().unwrap(),
            String::from("This is a secret internal error")
        );
    }
}
