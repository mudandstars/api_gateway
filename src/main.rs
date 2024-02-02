use std::net::SocketAddr;

use api_gateway::app::app;

#[tokio::main]
async fn main() {
    let app = app().await;

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
