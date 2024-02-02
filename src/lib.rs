pub mod database;
pub mod handler;
pub mod models;
pub mod schema;
pub mod app;

use database::api_keys;
use database::users;
use diesel::prelude::*;
use diesel::result;
use models::NewUser;

use self::models::User;

pub fn store_user_with_api_key(conn: &mut MysqlConnection, new_user: &NewUser) -> Result<User, result::Error> {
    let user = users::store_user(conn, new_user)?;

    api_keys::store_new_api_key(conn, user.id);

    Ok(user)
}

#[cfg(test)]
mod tests {
    use super::*;
    use diesel::result::Error;

    #[test]
    fn test_can_store_a_user() {
        database::establish_connection().test_transaction::<_, Error, _>(|conn| {
            let name = String::from("test user");
            let email = String::from("test@user.com");

            let user = store_user_with_api_key(
                conn,
                &NewUser {
                    name: name.clone(),
                    email: email.clone(),
                },
            ).unwrap();

            assert_eq!(user.name, name);
            assert_eq!(user.email, email);

            Ok(())
        });
    }
}
