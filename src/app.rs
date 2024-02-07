use axum::{routing::post, Router};
use deadpool_diesel::mysql::Pool;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use tower_http::trace::TraceLayer;

use crate::handler;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

pub async fn app(pool: Pool) -> Router {
    {
        let conn = pool.get().await.unwrap();

        conn.interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
            .await
            .unwrap()
            .unwrap();
    }

    // build our application with some routes
    Router::new()
        // .route("/hello", get(list_users))
        .route(
            "/users",
            post(handler::user_handler::store_user).get(handler::user_handler::index_users),
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
