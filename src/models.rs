use crate::schema::users;
use crate::schema::api_keys;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
}


#[derive(Queryable, Selectable)]
#[diesel(table_name = api_keys)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct ApiKey {
    pub id: u32,
    pub key: String,
    pub user_id: u32,
}

#[derive(Insertable)]
#[diesel(table_name = api_keys)]
pub struct NewApiKey<'a> {
    pub key: &'a str,
    pub user_id: u32,
}
