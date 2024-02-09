use crate::schema::api_keys;
use crate::schema::logs;
use crate::schema::users;
use chrono;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, QueryableByName)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[derive(serde::Serialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl User {
    pub fn api_keys(&self, conn: &mut MysqlConnection) -> Vec<ApiKey> {
        api_keys::table
            .filter(api_keys::user_id.eq(self.id))
            .load::<ApiKey>(conn)
            .expect("Error fetching API keys")
    }

    pub fn logs(&self, conn: &mut MysqlConnection) -> Vec<Log> {
        self.api_keys(conn)
            .iter()
            .flat_map(|api_key| api_key.logs(conn))
            .collect()
    }
}

#[derive(Insertable)]
#[diesel(table_name = users)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct NewUser {
    pub name: String,
    pub email: String,
}

#[derive(Insertable)]
#[diesel(table_name = api_keys)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct NewApiKey {
    pub key: String,
    pub user_id: u32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = api_keys)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct ApiKey {
    pub id: u32,
    pub key: String,
    pub user_id: u32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl ApiKey {
    pub fn logs(&self, conn: &mut MysqlConnection) -> Vec<Log> {
        logs::table
            .filter(logs::api_key_id.eq(self.id))
            .load::<Log>(conn)
            .expect("Error fetching API keys")
    }
}

#[derive(Insertable)]
#[diesel(table_name = logs)]
pub struct NewLog {
    pub api_key_id: u32,
    pub method: String,
    pub uri: String,
    pub status: u16,
    pub duration_in_microseconds: u64,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = logs)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Log {
    pub id: u32,
    pub api_key_id: u32,
    pub method: String,
    pub uri: String,
    pub status: u16,
    pub duration_in_microseconds: u64,
    pub created_at: chrono::NaiveDateTime,
}
