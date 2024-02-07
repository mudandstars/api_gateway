use axum::{extract::State, http::StatusCode};

pub async fn sample_endpoint(
    State(_pool): State<deadpool_diesel::mysql::Pool>,
) -> Result<(), (StatusCode, String)> {
    Ok(())
}
