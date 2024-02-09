use axum::{body::Body, http::Request, response::Response};
use deadpool_diesel::mysql::Pool;
use diesel::insert_into;
use futures_util::future::BoxFuture;
use std::task::{Context, Poll};
use std::time::Instant;
use tower::{Layer, Service};

use crate::models::{ApiKey, NewLog};
use crate::schema::api_keys;
use crate::schema::logs;
use diesel::prelude::*;

#[derive(Clone)]
pub struct RequestLoggerMiddleware<S> {
    inner: S,
    pool: Pool,
}

impl<S> Service<Request<Body>> for RequestLoggerMiddleware<S>
where
    S: Service<Request<Body>, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let start_time = Instant::now();
        let pool = self.pool.clone();
        let uri = req.uri().clone().to_string();
        let method = req.method().clone().to_string();
        let api_key = req.headers().get("API_KEY").cloned(); // Assuming API-Key is the header

        let future = self.inner.call(req);

        Box::pin(async move {
            let response = future.await?;
            let duration_in_microseconds = start_time.elapsed().as_millis() as u64;
            let status = response.status().as_u16();
            let api_key_str = api_key.map_or_else(
                || "None".to_string(),
                |k| k.to_str().unwrap_or("Invalid").to_string(),
            );

            let conn = pool.get().await.expect("Failed to get DB connection");
            conn.interact(move |conn| {
                println!(
                    "Method={}, uri={:?}, API Key: {}, Status: {}, Duration in microseconds: {:?}",
                    method, uri, api_key_str, status, duration_in_microseconds
                );
                let api_key = api_keys::table
                    .filter(api_keys::key.eq(api_key_str))
                    .first::<ApiKey>(conn);

                insert_into(logs::table)
                    .values(&NewLog {
                        method,
                        uri,
                        status,
                        duration_in_microseconds,
                        api_key_id: api_key.unwrap().id,
                    })
                    .execute(conn)
            })
            .await
            .expect("DB interaction failed")
            .expect("database result is error");

            Ok(response)
        })
    }
}

// Define a layer to wrap services with your middleware
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
        RequestLoggerMiddleware {
            inner,
            pool: self.pool.clone(),
        }
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
