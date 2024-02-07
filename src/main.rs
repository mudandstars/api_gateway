use dotenvy::dotenv;
use std::net::SocketAddr;
use tracing::{Event, Subscriber};
use tracing_subscriber::{layer::Context, registry::LookupSpan, Layer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use api_gateway::{
    app::{create_app, mysql_pool},
    middleware::logger::RequestLoggerLayer,
};

pub struct CustomLayer;

impl<S> Layer<S> for CustomLayer
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        // Implement your custom logic here
        // For example, print the event
        println!("Custom layer got an event: {:?}", event);
    }

    // You can implement other methods from the Layer trait as needed
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .with(CustomLayer)
        .init();

    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = mysql_pool(&db_url);

    let logger_layer = RequestLoggerLayer::new(pool.clone());
    let app = create_app(pool).await.layer(logger_layer);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
