use axum::http::StatusCode;

pub mod sample_endpoints;
pub mod user_handler;

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
