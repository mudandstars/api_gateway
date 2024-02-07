pub mod api_keys;
pub mod users;

use diesel::prelude::*;
use dotenvy::dotenv;

pub fn establish_connection(db_url: &str) -> MysqlConnection {
    dotenv().ok();

    MysqlConnection::establish(db_url).unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
}
