use dotenvy::dotenv;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use api_gateway::app::{create_app, mysql_pool};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "TRACING_LEVEL=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let app = create_app(mysql_pool(&db_url)).await;

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
