use axum::{routing::post, routing::get, Router};
use deadpool_diesel::mysql::Pool;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use handler::{sample_endpoints, user_handler};
use tower_http::trace::TraceLayer;

use crate::handler;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

pub async fn create_app(pool: Pool) -> Router {
    {
        let conn = pool.get().await.unwrap();

        conn.interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
            .await
            .unwrap()
            .unwrap();
    }

    Router::new()
        .route(
            "/sample-endpoints",
            post(sample_endpoints::sample_endpoint).get(sample_endpoints::sample_endpoint),
        )
        .route(
            "/fail",
            get(sample_endpoints::failing_endpoint),
        )
        .route(
            "/users",
            post(user_handler::store_user).get(user_handler::index_users),
        )
        .with_state(pool)
        .layer(TraceLayer::new_for_http())
}

pub fn mysql_pool(db_url: &str) -> Pool {
    let manager = deadpool_diesel::mysql::Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);
    deadpool_diesel::mysql::Pool::builder(manager)
        .build()
        .unwrap()
}
