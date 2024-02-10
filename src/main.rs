use dotenvy::dotenv;
use std::net::SocketAddr;

use api_gateway::{
    app::{create_app, mysql_pool},
    middleware::{AuthorizationServiceLayer, RequestLoggerLayer},
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = mysql_pool(&db_url);

    let logger_layer = RequestLoggerLayer::new(pool.clone());
    let authorization_service_layer = AuthorizationServiceLayer::new(pool.clone());
    let app = create_app(pool)
        .await
        .layer(authorization_service_layer)
        .layer(logger_layer);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
