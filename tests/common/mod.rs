use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

pub fn clear_database(conn: &MysqlConnection) -> QueryResult<()> {
    conn.transaction::<(), diesel::result::Error, _>(|| {
        let table_names = diesel::sql_query("SHOW TABLES")
            .load::<(String,)>(conn)?
            .into_iter()
            .map(|(name,)| name)
            .collect::<Vec<String>>();

        for table_name in table_names {
            diesel::sql_query(format!("DELETE FROM `{}`", table_name)).execute(conn)?;
        }

        Ok(())
    })
}
