use axum::http::StatusCode;
use axum::{body::Body, http::Request, response::Response};
use chrono::Utc;
use deadpool_diesel::mysql::Pool;
use futures_util::future::BoxFuture;
use std::task::{Context, Poll};
use tower::Service;

use crate::models::ApiKey;
use crate::schema::api_keys;
use diesel::prelude::*;

#[derive(Clone)]
pub struct AuthorizationServiceMiddleware<S> {
    inner: S,
    pool: Pool,
}

impl<S> AuthorizationServiceMiddleware<S> {
    pub fn new(inner: S, pool: Pool) -> Self {
        AuthorizationServiceMiddleware { inner, pool }
    }
}

impl<S> Service<Request<Body>> for AuthorizationServiceMiddleware<S>
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
        let pool = self.pool.clone();
        let api_key = req.headers().get(super::super::API_KEY_NAME).cloned();
        let api_key_str = api_key.map_or_else(
            || "None".to_string(),
            |k| k.to_str().unwrap_or("Invalid").to_string(),
        );

        let future = self.inner.call(req);

        Box::pin(async move {
            let conn = pool.get().await.expect("Failed to get DB connection");
            let api_key = conn
                .interact(move |conn| {
                    api_keys::table
                        .filter(api_keys::key.eq(api_key_str))
                        .first::<ApiKey>(conn)
                })
                .await
                .expect("DB interaction failed");

            if let Ok(api_key) = api_key {
                conn.interact(move |conn| {
                    diesel::update(api_keys::table.filter(api_keys::id.eq(api_key.id)))
                        .set(api_keys::last_used_at.eq(Utc::now().naive_utc()))
                        .execute(conn)
                })
                .await
                .expect("DB interaction failed")
                .expect("update failed");
            } else {
                let resp = Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(Body::from("Unauthorized"))
                    .expect("Failed to construct response");

                return Ok(resp);
            }

            let response = future.await?;
            Ok(response)
        })
    }
}
