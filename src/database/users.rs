use diesel::prelude::*;

use crate::models::{NewUser, User};

pub fn store_user(conn: &mut MysqlConnection, new_user: &NewUser) -> Result<User, diesel::result::Error> {
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
}

