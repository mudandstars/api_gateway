pub mod api_keys;
pub mod users;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = if env::var("RUNTIME_ENV").expect("RUNTIME_ENV must be set") == "testing" {
        env::var("TESTING_DATABASE_URL").expect("TESTING_DATABASE_URL must be set")
    } else {
        env::var("DATABASE_URL").expect("DATABASE_URL must be set")
    };

    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
