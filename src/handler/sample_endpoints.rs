use axum::{extract::State, http::StatusCode};

pub async fn sample_endpoint(
    State(_pool): State<deadpool_diesel::mysql::Pool>,
) -> Result<(), (StatusCode, String)> {
    Ok(())
}

pub async fn failing_endpoint(
    State(_pool): State<deadpool_diesel::mysql::Pool>,
) -> Result<(), (StatusCode, String)> {
    Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        String::from("This is a secret internal error"),
    ))
}
