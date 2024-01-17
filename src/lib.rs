pub mod models;
pub mod schema;
pub mod handler;

use diesel::prelude::*;
use dotenvy::dotenv;
use models::ApiKey;
use std::env;

use self::models::{User, NewUser, NewApiKey};

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn store_user_with_api_key(conn: &mut MysqlConnection, name: &str, email: &str) -> User {
    let user = store_user(conn, name, email);

    store_api_key(conn, user.id);

    user
}

fn store_user(conn: &mut MysqlConnection, name: &str, email: &str) -> User {
    use crate::schema::users;

    let new_user = NewUser { name, email };

    conn.transaction(|conn| {
        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(conn)?;

        users::table
            .order(users::id.desc())
            .select(User::as_select())
            .first(conn)
    })
    .expect("Error while saving post")
}

fn store_api_key(conn: &mut MysqlConnection, user_id: u32) -> ApiKey {
    use crate::schema::api_keys;

    let new_api_key = NewApiKey { key: "something", user_id };

    conn.transaction(|conn| {
        diesel::insert_into(api_keys::table)
            .values(&new_api_key)
            .execute(conn)?;

        api_keys::table
            .order(api_keys::id.desc())
            .select(ApiKey::as_select())
            .first(conn)
    })
    .expect("Error while saving api key")
}


#[cfg(test)]
mod tests {
    use super::*;
    use diesel::result::Error;


    #[test]
    fn test_can_store_a_user() {

        establish_connection().test_transaction::<_, Error, _>(|conn| {
            let name = "test user";
            let email = "test@user.com";

            let user = store_user_with_api_key(conn, name, email);

            assert_eq!(user.name, name);
            assert_eq!(user.email, email);

            Ok(())
        });
    }
}
