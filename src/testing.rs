use deadpool_diesel::mysql::Pool;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use rand::{distributions::Alphanumeric, Rng}; // 0.8
use std::env;
use std::time::SystemTime;

use crate::app::mysql_pool;
use crate::database::establish_connection;
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

pub struct TestContext {
    db_url: String,
}

impl TestContext {
    pub fn new() -> Self {
        let duration_since_epoch = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let timestamp_nanos = duration_since_epoch.as_nanos();
        let random_string = TestContext::generate_random_string();
        let unique_name = format!("{}_{}", timestamp_nanos, random_string);

        TestContext {
            db_url: TestContext::create_new_database(&unique_name),
        }
    }

    pub fn pool(&self) -> Pool {
       mysql_pool(&self.db_url)
    }

    pub fn conn(&self) -> MysqlConnection {
       establish_connection(&self.db_url)
    }

    fn create_new_database(name: &str) -> String {
        dotenv().ok();

        let database_server_url = env::var("DATABASE_SERVER").expect("DATABASE_SERVER must be set");
        let mut conn = MysqlConnection::establish(&database_server_url)
            .expect("Error connecting to sql server");

        diesel::sql_query(format!("CREATE DATABASE {}", name))
            .execute(&mut conn)
            .expect("Error creating database with name ");

        let database_url = format!("{database_server_url}{name}");
        let mut conn = establish_connection(&database_url);

        conn.run_pending_migrations(MIGRATIONS)
            .expect("failed to run migrations");

        database_url.to_string()
    }

    fn generate_random_string() -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(6)
            .map(char::from)
            .collect()
    }
}

impl Default for TestContext {
    fn default() -> Self {
        TestContext::new()
    }
}

impl Drop for TestContext {
    fn drop(&mut self) {
        dotenv().ok();

        let database_server_url = env::var("DATABASE_SERVER").expect("DATABASE_SERVER must be set");
        let mut conn = MysqlConnection::establish(&database_server_url)
            .expect("Error connecting to sql server {}");

        let db_name = self.db_url.split('/').last().unwrap();
        diesel::sql_query(format!("DROP DATABASE {}", db_name))
            .execute(&mut conn)
            .expect("Error dropping database");
    }
}
