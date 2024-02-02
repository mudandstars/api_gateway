
use axum::{routing::post, Router};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::handler;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

pub async fn app() -> Router {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "TRACING_LEVEL=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // set up connection pool
    let manager = deadpool_diesel::mysql::Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);
    let pool = deadpool_diesel::mysql::Pool::builder(manager)
        .build()
        .unwrap();

    // run the migrations on server startup
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
        .route("/users", post(handler::user_handler::store_user))
        .with_state(pool)
        .layer(TraceLayer::new_for_http())
}
