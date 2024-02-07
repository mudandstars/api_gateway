use super::internal_error;
use crate::models::{NewUser, User};
use crate::schema::users;
use crate::store_user_with_api_key;
use axum::{extract::State, http::StatusCode, response::Json};
use diesel::QueryDsl;

use diesel::prelude::*;

pub async fn store_user(
    State(pool): State<deadpool_diesel::mysql::Pool>,
    Json(new_user): Json<NewUser>,
) -> Result<(StatusCode, Json<User>), (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let user = conn
        .interact(move |conn| store_user_with_api_key(conn, &new_user))
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    Ok((StatusCode::CREATED, Json(user)))
}

#[derive(serde::Serialize)]
pub struct IndexUserResponse {
    users: Vec<User>,
}

pub async fn index_users(
    State(pool): State<deadpool_diesel::mysql::Pool>,
) -> Result<Json<IndexUserResponse>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let users = conn
        .interact(move |conn| users::table.order_by(users::id).load::<User>(conn))
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    Ok(Json(IndexUserResponse { users }))
}
