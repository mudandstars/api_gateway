use crate::schema::api_keys;
use crate::schema::users;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, QueryableByName)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[derive(serde::Serialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}

impl User {
    pub fn api_keys(&self, conn: &mut MysqlConnection) -> Vec<ApiKey> {
        api_keys::table
            .filter(api_keys::user_id.eq(self.id))
            .load::<ApiKey>(conn)
            .expect("Error fetching API keys")
    }
}

#[derive(Insertable)]
#[diesel(table_name = users)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct NewUser {
    pub name: String,
    pub email: String,
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
pub struct NewApiKey {
    pub key: String,
    pub user_id: u32,
}
