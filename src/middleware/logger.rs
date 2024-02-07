use axum::{body::Body, http::Request, response::Response};
use deadpool_diesel::mysql::Pool;
use futures_util::future::BoxFuture;
use std::task::{Context, Poll};
use tower::{Layer, Service};

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
        let pool = self.pool.clone();
        let uri = req.uri().clone();

        let response = self.inner.call(req);

        Box::pin(async move {
            // Log the request URL to the database. This is where you'd integrate your actual DB logic.
            // For demonstration, we're just printing the URI.
            let conn = pool.get().await.expect("Failed to get DB connection");
            conn.interact(move |_conn| {
                println!("Logging request URL to DB: {}", uri);
                // Here you would insert the URI into your database
                Ok::<(), diesel::result::Error>(())
            })
            .await
            .expect("DB interaction failed")
            .expect("database result is error");

            response.await
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
