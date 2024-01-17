use diesel::prelude::*;

use crate::models::{NewUser, User};

pub fn store(conn: &mut MysqlConnection, new_user: &NewUser) -> User {
    use crate::schema::users;

    conn.transaction(|conn| {
        diesel::insert_into(users::table)
            .values(new_user)
            .execute(conn)?;

        users::table
            .order(users::id.desc())
            .select(User::as_select())
            .first(conn)
    })
    .expect("Error while saving post")
}
