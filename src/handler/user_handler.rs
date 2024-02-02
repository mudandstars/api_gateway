use crate::models::{NewUser, User};
use crate::store_user_with_api_key;
use axum::{extract::State, http::StatusCode, response::Json};

pub async fn store_user(
    State(pool): State<deadpool_diesel::mysql::Pool>,
    Json(new_user): Json<NewUser>,
) -> Result<(StatusCode, Json<User>), (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let res = conn
        .interact(move |conn| store_user_with_api_key(conn, &new_user))
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    Ok((StatusCode::CREATED, Json(res)))
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
