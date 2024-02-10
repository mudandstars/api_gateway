use axum::{body::Body, http::Request, response::Response};
use deadpool_diesel::mysql::Pool;
use diesel::insert_into;
use futures_util::future::BoxFuture;
use std::task::{Context, Poll};
use std::time::Instant;
use tower::Service;

use crate::models::{ApiKey, NewLog};
use crate::schema::api_keys;
use crate::schema::logs;
use diesel::prelude::*;

#[derive(Clone)]
pub struct RequestLoggerMiddleware<S> {
    inner: S,
    pool: Pool,
}

impl<S> RequestLoggerMiddleware<S> {
    pub fn new(inner: S, pool: Pool) -> Self {
        RequestLoggerMiddleware { inner, pool }
    }
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
            let duration_in_microseconds = start_time.elapsed().as_micros() as u64;
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
