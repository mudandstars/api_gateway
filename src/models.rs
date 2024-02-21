use crate::schema::api_keys;
use crate::schema::logs;
use crate::schema::users;
use chrono;
use diesel::backend::Backend;
use diesel::deserialize::FromSqlRow;
use diesel::deserialize::{self, FromSql};
use diesel::expression::AsExpression;
use diesel::prelude::*;
use diesel::serialize::{self, ToSql};
use diesel::sql_types::TinyInt;

#[derive(Queryable, Selectable, Identifiable, QueryableByName)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[derive(serde::Serialize, Debug)]
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
#[derive(Debug)]
pub struct ApiKey {
    pub id: u32,
    pub key: String,
    pub user_id: u32,
    pub created_at: chrono::NaiveDateTime,
    pub last_used_at: Option<chrono::NaiveDateTime>,
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
    pub type_: u8,
    pub duration_in_microseconds: u64,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = logs)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[derive(Debug)]
pub struct Log {
    pub id: u32,
    pub api_key_id: u32,
    pub method: String,
    pub uri: String,
    pub status: u16,
    pub type_: u8,
    pub error_message: Option<String>,
    pub duration_in_microseconds: u64,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow)]
#[diesel(sql_type = diesel::sql_types::TinyInt)]
pub enum LogType {
    INFO,
    ERROR,
}

impl From<LogType> for u8 {
    fn from(log_type: LogType) -> Self {
        match log_type {
            LogType::INFO => 0,
            LogType::ERROR => 1,
        }
    }
}

impl<DB> ToSql<TinyInt, DB> for LogType
where
    DB: Backend,
    u8: ToSql<TinyInt, DB>,
{
    fn to_sql<'a>(&self, out: &mut serialize::Output<DB>) -> serialize::Result {
        match *self {
            LogType::INFO => ToSql::<TinyInt, DB>::to_sql(&0, out),
            LogType::ERROR => ToSql::<TinyInt, DB>::to_sql(&1, out),
        }
    }
}

impl<DB> FromSql<TinyInt, DB> for LogType
where
    DB: Backend,
    u8: FromSql<TinyInt, DB>,
{
    fn from_sql(raw_bytes: DB::RawValue<'_>) -> deserialize::Result<Self> {
        let value = u8::from_sql(raw_bytes)?;
        match value {
            0 => Ok(LogType::INFO),
            1 => Ok(LogType::ERROR),
            _ => Err("Unrecognized value for LogType".into()),
        }
    }
}
