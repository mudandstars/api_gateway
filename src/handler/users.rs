use crate::models::{NewUser, User};
use axum::{extract::State, http::StatusCode, response::Json};

use diesel::prelude::*;

pub async fn store_user(
    State(pool): State<deadpool_diesel::mysql::Pool>,
    Json(new_user): Json<NewUser>,
) -> Result<Json<User>, (StatusCode, String)> {
    use crate::schema::users::dsl::*;
    let conn = pool.get().await.map_err(internal_error)?;

    let res = conn
        .interact(|conn| {
            diesel::insert_into(users).values(new_user).execute(conn)?;

            users.order(id.desc()).select(User::as_select()).first(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    Ok(Json(res))
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
