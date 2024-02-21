use axum::{body::Body, http::Request, response::Response};
use chrono::Utc;
use deadpool_diesel::mysql::Pool;
use diesel::insert_into;
use futures_util::future::BoxFuture;
use std::task::{Context, Poll};
use std::time::Instant;
use tower::Service;

use crate::models::{ApiKey, LogType, NewLog};
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
        let now = Utc::now();
        let start_time = Instant::now();
        let pool = self.pool.clone();
        let uri = req.uri().clone().to_string();
        let method = req.method().clone().to_string();
        let api_key = req.headers().get(super::super::API_KEY_NAME).cloned(); // Assuming API-Key is the header

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
            let _ = conn
                .interact(move |conn| {
                    println!(
                        "{} {{method={} uri={:?} status={} duration(μs)={} API_KEY={}}}",
                        now.to_rfc3339_opts(chrono::SecondsFormat::Micros, true),
                        method,
                        uri,
                        status,
                        duration_in_microseconds,
                        api_key_str
                    );
                    let api_key = api_keys::table
                        .filter(api_keys::key.eq(api_key_str))
                        .first::<ApiKey>(conn)
                        .optional();

                    if let Ok(Some(api_key)) = api_key {
                        insert_into(logs::table)
                            .values(&NewLog {
                                method,
                                uri,
                                status,
                                duration_in_microseconds,
                                api_key_id: api_key.id,
                                type_: LogType::INFO.into(),
                            })
                            .execute(conn)
                            .expect("DB interaction failed");
                    };
                })
                .await;

            Ok(response)
        })
    }
}
