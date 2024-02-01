use crate::models::{ApiKey, NewApiKey};
use diesel::prelude::*;

pub fn store(conn: &mut MysqlConnection, user_id: u32) -> ApiKey {
    use crate::schema::api_keys;

    let new_api_key = NewApiKey {
        key: String::from("something"),
        user_id,
    };

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
