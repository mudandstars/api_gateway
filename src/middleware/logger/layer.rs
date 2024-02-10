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
    use crate::{app::create_app, models::NewUser, store_user_with_api_key, testing::TestContext};
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

        let user_logs = user.logs(&mut test_context.conn());
        let log = user_logs.first().unwrap();

        assert_eq!(log.method, http::Method::GET.to_string());
        assert_eq!(log.uri, String::from("/users"));
        assert_eq!(log.status, http::status::StatusCode::OK);
    }
}
